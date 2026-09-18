//! Native checksum syntax, complete inventory and immutable bundle admission.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::native_union::{NativeStruct, Rule};

pub(crate) const FILE_LIMIT: u64 = 256 * 1024 * 1024;
pub(crate) const TOTAL_LIMIT: u64 = 4 * 1024 * 1024 * 1024;
pub(crate) const FILE_COUNT: usize = 16_384;
pub(crate) const TEXT_LIMIT: u64 = 4 * 1024 * 1024;
pub(crate) const VERSION: &str = "delta-evidence-bundle/2";

enrichment_core::native_struct! { struct Line { text:String => Rule::Text } }
enrichment_core::native_struct! { pub(crate) struct Checksum { path:String => Rule::MemberPath, digest:String => Rule::Sha256 } }
enrichment_core::native_struct! { pub(crate) struct Observation {
    path:String => Rule::MemberPath,
    digest:Option<String> => Rule::Sha256,
    bytes:Option<u64> => Rule::UnsignedRange { min:0, max:FILE_LIMIT },
    error:Option<String> => Rule::Text
} }
enrichment_core::native_struct! { pub(crate) struct Problem { text:String => Rule::Text } }
enrichment_core::native_struct! { pub(crate) struct Copy {
    artifact_id:String => Rule::ArtifactIdentity { digest:"sha256".into() },
    sha256:String => Rule::Sha256,
    size_bytes:u64 => Rule::UnsignedRange { min:0, max:FILE_LIMIT }
} }

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

async fn admit_fields(
    runtime: &QueryRuntime,
    frame: datafusion::dataframe::DataFrame,
) -> Result<()> {
    if let Some(violations) = enrichment_core::native_schema::intrinsic_violations(
        frame.clone(),
        frame.schema().as_arrow(),
    )? {
        runtime
            .require_empty(
                violations.select(vec![
                    datafusion::prelude::lit("bundle_field").alias("witness"),
                ])?,
                "bundle_fields",
                "bundle_admission",
            )
            .await?;
    }
    Ok(())
}

/// The host splits bounded text into lines; native expressions own its syntax and policy.
pub(crate) async fn checksums(runtime: &QueryRuntime, text: &str) -> Result<Vec<Checksum>> {
    if text.len() as u64 > TEXT_LIMIT {
        return Err(invalid("bundle manifest byte bound"));
    }
    let lines = text
        .lines()
        .take(FILE_COUNT + 1)
        .map(|text| Line { text: text.into() })
        .collect::<Vec<_>>();
    if lines.is_empty() || lines.len() > FILE_COUNT {
        return Err(invalid("bundle manifest entry bound"));
    }
    let session = runtime.session();
    native_catalog::input(&session, "bundle_lines", Line::batch(&lines)?)?;
    runtime.require_empty(session.sql("SELECT text AS witness FROM bundle_lines WHERE NOT regexp_like(text,'^[0-9a-f]{64}  .+$')").await?,"bundle_checksum_syntax","bundle_admission").await?;
    let frame = session
        .sql("SELECT substr(text,67) AS path,substr(text,1,64) AS digest FROM bundle_lines")
        .await?;
    let frame =
        crate::native_delta::project(frame, &arrow::datatypes::Schema::new(Checksum::fields()))?;
    native_catalog::work(&session, "bundle_checksums", frame.clone().into_view())?;
    admit_fields(runtime, frame.clone()).await?;
    runtime.require_empty(session.sql("SELECT path AS witness FROM bundle_checksums WHERE path='MANIFEST.sha256' OR regexp_like(path,'[\\r\\n]') UNION ALL SELECT path FROM bundle_checksums GROUP BY path HAVING count(*)<>1").await?,"bundle_checksum_authority","bundle_admission").await?;
    runtime
        .records(
            frame.sort(vec![datafusion::prelude::col("path").sort(true, true)])?,
            FILE_COUNT,
        )
        .await
}

/// Every physical file has exactly one observation. Failure and missing-file facts stay
/// distinct from an empty or successful checksum. Hashing itself remains a bounded I/O driver.
pub(crate) async fn problems(
    runtime: &QueryRuntime,
    expected: &[Checksum],
    observed: &[Observation],
) -> Result<Vec<Problem>> {
    if observed.len() > FILE_COUNT || expected.len() > FILE_COUNT {
        return Err(invalid("bundle inventory entry bound"));
    }
    let session = runtime.session();
    native_catalog::input(&session, "bundle_expected", Checksum::batch(expected)?)?;
    native_catalog::input(&session, "bundle_observed", Observation::batch(observed)?)?;
    let actual = session.table("bundle_observed").await?;
    admit_fields(runtime, actual).await?;
    runtime.require_empty(session.sql("SELECT path AS witness FROM bundle_observed GROUP BY path HAVING count(*)<>1 UNION ALL SELECT path FROM bundle_observed WHERE (error IS NULL AND (digest IS NULL OR bytes IS NULL)) OR (error IS NOT NULL AND (digest IS NOT NULL OR bytes IS NOT NULL)) UNION ALL SELECT 'bundle_total_bytes' FROM bundle_observed HAVING sum(bytes)>$1").await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(TOTAL_LIMIT))])?,"bundle_physical_inventory","bundle_admission").await?;
    let frame=session.sql("SELECT concat(e.path,': ',CASE WHEN o.path IS NULL THEN 'missing file' WHEN o.error IS NOT NULL THEN o.error ELSE 'content does not match recorded digest' END) AS text FROM bundle_expected e LEFT JOIN bundle_observed o ON e.path=o.path WHERE o.path IS NULL OR o.error IS NOT NULL OR o.digest<>e.digest UNION ALL SELECT concat(o.path,': present but not listed') AS text FROM bundle_observed o LEFT ANTI JOIN bundle_expected e ON o.path=e.path WHERE o.path<>'MANIFEST.sha256' ORDER BY text").await?;
    runtime.records(frame, 2 * FILE_COUNT).await
}

/// Export and verification use the same inventory rules and canonical checksum ordering.
pub(crate) async fn manifest(runtime: &QueryRuntime, observed: &[Observation]) -> Result<String> {
    let session = runtime.session();
    if observed.is_empty() || observed.len() > FILE_COUNT {
        return Err(invalid("bundle manifest inventory bound"));
    }
    native_catalog::input(&session, "bundle_observed", Observation::batch(observed)?)?;
    runtime.require_empty(session.sql("SELECT path AS witness FROM bundle_observed WHERE error IS NOT NULL OR digest IS NULL OR bytes IS NULL OR path='MANIFEST.sha256'").await?,"bundle_export_inventory","bundle_admission").await?;
    runtime.require_empty(session.sql("SELECT 'manifest_bytes' AS witness FROM bundle_observed HAVING sum(octet_length(path)+67)>$1").await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(TEXT_LIMIT))])?,"bundle_manifest_bytes","bundle_admission").await?;
    let values:Vec<Line>=runtime.records(session.sql("SELECT concat(string_agg(concat(digest,'  ',path),chr(10) ORDER BY path),chr(10)) AS text FROM bundle_observed").await?,1).await?;
    let text = values
        .into_iter()
        .next()
        .ok_or_else(|| invalid("bundle manifest missing"))?
        .text;
    let selected = checksums(runtime, &text).await?;
    if !problems(runtime, &selected, observed).await?.is_empty() {
        return Err(invalid("bundle export inventory mismatch"));
    }
    Ok(text)
}

pub(crate) async fn admit_catalog(
    runtime: &QueryRuntime,
    session: &datafusion::prelude::SessionContext,
    description: &crate::bundle::Description,
    identity: &str,
) -> Result<()> {
    native_catalog::input(
        session,
        "bundle_descriptor",
        crate::bundle::Description::batch(std::slice::from_ref(description))?,
    )?;
    admit_fields(runtime, session.table("bundle_descriptor").await?).await?;
    runtime.require_empty(session.sql("SELECT d.control AS witness FROM bundle_descriptor d LEFT ANTI JOIN state.records.selections s ON d.context_id=s.context_id AND d.snapshot_id=s.snapshot_id UNION ALL SELECT d.control FROM bundle_descriptor d LEFT ANTI JOIN state.records.snapshots s ON d.context_id=s.context_id AND d.snapshot_id=s.snapshot_id AND d.context_id=s.publication.context_id AND d.snapshot_id=s.publication.snapshot_id UNION ALL SELECT control FROM bundle_descriptor WHERE control<>$1").await?.with_param_values(vec![datafusion::common::ScalarValue::from(identity)])?,"bundle_catalog_identity","bundle_admission").await
}

/// Union complete semantic and operational inputs before choosing physical copies.
/// Equal content has one copy; contradictory lengths never become a first-writer choice.
pub(crate) async fn copies(
    runtime: &QueryRuntime,
    inputs: Vec<datafusion::dataframe::DataFrame>,
) -> Result<Vec<Copy>> {
    use datafusion::prelude::col;
    let session = runtime.session();
    let schema = arrow::datatypes::Schema::new(Copy::fields());
    let mut frame = crate::native_catalog::batch(&session, "bundle_plan", Copy::batch(&[])?)?;
    for input in inputs {
        let input = input.select(vec![col("artifact_id"), col("sha256"), col("size_bytes")])?;
        frame = frame.union(crate::native_delta::project(input, &schema)?)?;
    }
    admit_fields(runtime, frame.clone()).await?;
    native_catalog::work(&session, "bundle_copy_candidates", frame.into_view())?;
    runtime.require_empty(session.sql("SELECT sha256 AS witness FROM bundle_copy_candidates GROUP BY sha256 HAVING count(DISTINCT size_bytes)<>1 OR count(DISTINCT artifact_id)<>1").await?,"bundle_copy_identity","bundle_admission").await?;
    let unique = session
        .sql("SELECT DISTINCT * FROM bundle_copy_candidates")
        .await?;
    native_catalog::work(&session, "bundle_copies", unique.into_view())?;
    runtime.require_empty(session.sql("SELECT 'bundle_copy_bound' AS witness FROM bundle_copies HAVING count(*)>16384 OR sum(size_bytes)>536870912").await?,"bundle_copy_bound","bundle_admission").await?;
    runtime
        .records(
            session
                .sql("SELECT * FROM bundle_copies ORDER BY sha256")
                .await?,
            FILE_COUNT,
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_bundle_copies_reject_conflicting_identity_before_io() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let copy = Copy {
            artifact_id: format!("art_{}", "a".repeat(64)),
            sha256: "a".repeat(64),
            size_bytes: 7,
        };
        let input = |values: &[Copy]| {
            crate::native_catalog::batch(&runtime.session(), "bundle_plan", Copy::batch(values)?)
        };
        assert_eq!(
            copies(
                &runtime,
                vec![
                    input(std::slice::from_ref(&copy))?,
                    input(std::slice::from_ref(&copy))?
                ]
            )
            .await?,
            vec![copy.clone()]
        );
        let conflict = Copy {
            size_bytes: 8,
            ..copy.clone()
        };
        assert!(
            copies(&runtime, vec![input(&[copy.clone(), conflict])?])
                .await
                .is_err()
        );
        let wrong = Copy {
            artifact_id: format!("art_{}", "b".repeat(64)),
            ..copy.clone()
        };
        assert!(copies(&runtime, vec![input(&[wrong])?]).await.is_err());
        let oversized = Copy {
            size_bytes: FILE_LIMIT + 1,
            ..copy
        };
        assert!(copies(&runtime, vec![input(&[oversized])?]).await.is_err());
        let full = (0..3)
            .map(|i| {
                let sha256 = format!("{i:064x}");
                Copy {
                    artifact_id: format!("art_{sha256}"),
                    sha256,
                    size_bytes: FILE_LIMIT,
                }
            })
            .collect::<Vec<_>>();
        assert!(copies(&runtime, vec![input(&full)?]).await.is_err());
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn native_bundle_catalog_admits_only_exact_selection_and_current_format() -> Result<()> {
        use crate::native_catalog::{BindingKind, BoundCatalog};
        use enrichment_core::identity::{ContextId, SnapshotId};
        use std::{collections::BTreeMap, sync::Arc};
        enrichment_core::native_struct! { struct Publication { context_id:ContextId=>Rule::Text, snapshot_id:SnapshotId=>Rule::Text } }
        enrichment_core::native_struct! { struct Snapshot { context_id:ContextId=>Rule::Text, snapshot_id:SnapshotId=>Rule::Text, publication:Publication=>Rule::Text } }
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let value = crate::bundle::Description {
            bundle_version: VERSION.into(),
            context_id: format!("ctx_{}", "a".repeat(64)).try_into().unwrap(),
            snapshot_id: format!("snap_{}", "b".repeat(64)).try_into().unwrap(),
            source_control: "source:4".into(),
            control: "export:1".into(),
            exported_at: enrichment_core::native_time::ObservationTime::now().unwrap(),
        };
        let scope = Publication {
            context_id: value.context_id.clone(),
            snapshot_id: value.snapshot_id.clone(),
        };
        let session = || -> Result<_> {
            let native = runtime.session();
            let records = BTreeMap::from([
                (
                    "selections".into(),
                    crate::native_catalog::batch(
                        &native,
                        "bundle_plan",
                        Publication::batch(std::slice::from_ref(&scope))?,
                    )?
                    .into_view(),
                ),
                (
                    "snapshots".into(),
                    crate::native_catalog::batch(
                        &native,
                        "bundle_plan",
                        Snapshot::batch(&[Snapshot {
                            context_id: scope.context_id.clone(),
                            snapshot_id: scope.snapshot_id.clone(),
                            publication: scope.clone(),
                        }])?,
                    )?
                    .into_view(),
                ),
            ]);
            runtime.bound_session(BTreeMap::from([(
                "state".into(),
                Arc::new(BoundCatalog::default().with_schema(BindingKind::FoldedRecords, records))
                    as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))
        };
        admit_catalog(&runtime, &session()?, &value, "export:1").await?;
        assert!(
            admit_catalog(&runtime, &session()?, &value, "export:2")
                .await
                .is_err()
        );
        let mut changed = value.clone();
        changed.snapshot_id = format!("snap_{}", "c".repeat(64)).try_into().unwrap();
        assert!(
            admit_catalog(&runtime, &session()?, &changed, "export:1")
                .await
                .is_err()
        );
        let mut changed = value;
        changed.bundle_version = "delta-evidence-bundle/1".into();
        assert!(
            admit_catalog(&runtime, &session()?, &changed, "export:1")
                .await
                .is_err()
        );
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn native_bundle_manifest_has_exact_file_authority() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let observations = vec![
            Observation {
                path: "data/Ω table/a.parquet".into(),
                digest: Some("a".repeat(64)),
                bytes: Some(4),
                error: None,
            },
            Observation {
                path: "bundle.json".into(),
                digest: Some("b".repeat(64)),
                bytes: Some(2),
                error: None,
            },
        ];
        let text = manifest(&runtime, &observations).await?;
        assert!(text.starts_with(&format!("{}  bundle.json\n", "b".repeat(64))));
        let expected = checksums(&runtime, &text).await?;
        assert!(
            problems(&runtime, &expected, &observations)
                .await?
                .is_empty()
        );
        let missing = problems(&runtime, &expected, &observations[..1]).await?;
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].text, "bundle.json: missing file");
        let mut changed = observations.clone();
        changed[0].digest = Some("c".repeat(64));
        changed.push(Observation {
            path: "extra".into(),
            digest: Some("d".repeat(64)),
            bytes: Some(1),
            error: None,
        });
        assert_eq!(problems(&runtime, &expected, &changed).await?.len(), 2);
        for path in [
            "../outside",
            "/absolute",
            "a/../b",
            "a\\b",
            "MANIFEST.sha256",
            "a\rb",
        ] {
            assert!(
                checksums(&runtime, &format!("{}  {path}\n", "a".repeat(64)))
                    .await
                    .is_err(),
                "{path}"
            );
        }
        assert!(checksums(&runtime, &format!("{text}{text}")).await.is_err());
        assert!(checksums(&runtime, "not a checksum\n").await.is_err());
        assert!(checksums(&runtime, "").await.is_err());
        let mut duplicate = observations.clone();
        duplicate.push(observations[0].clone());
        assert!(problems(&runtime, &expected, &duplicate).await.is_err());
        runtime.close_diagnostics().await
    }
}
