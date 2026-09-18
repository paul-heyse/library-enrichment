//! Retained resolution is a native projection of qualified metadata and acquisition facts.
mod presentation;
use crate::{SnapshotReader, query::QueryError, runtime::QueryRuntime};
use datafusion::{common::Result, dataframe::DataFrame, prelude::*};
use enrichment_core::{
    evidence::{Artifact, Gap},
    native_union::Rule,
    producer::{docsrs::DocsRsMetadata, python::Distribution},
};
pub use presentation::{LocalCompiler, acquisition_presentation};

enrichment_core::native_struct! { pub struct Metadata {
    observed_configuration: Option<DocsRsMetadata> => Rule::Text,
    python: Option<Distribution> => Rule::Text,
} }
enrichment_core::native_struct! { struct GapRow { gap: Gap => Rule::Text } }

/// Hosted author settings establish only the documented build. Native set comparison and
/// explicit-request presence decide whether an opted-in local observation is still needed.
pub async fn local_rustdoc_required(
    runtime: &QueryRuntime,
    request: &enrichment_core::request::ResolveRequest,
    docs: Option<&DocsRsMetadata>,
    hosted_usable: bool,
) -> Result<bool> {
    use enrichment_core::native_union::NativeStruct;
    enrichment_core::native_struct! { struct Input {
        request:enrichment_core::request::ResolveRequest => Rule::Text,
        docs:Option<DocsRsMetadata> => Rule::Text,
        hosted_usable:bool => Rule::Text,
    } }
    enrichment_core::native_struct! { struct Selected { required:bool => Rule::Text } }
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "hosted_build_scope",
        Input::batch(&[Input {
            request: request.clone(),
            docs: docs.cloned(),
            hosted_usable,
        }])?,
    )?;
    let frame=session.sql(r#"
      SELECT request.allow_local_build AND (NOT hosted_usable OR (docs IS NOT NULL AND (
        (request.target IS NOT NULL AND request.target<>docs.default_target)
        OR (request.default_features IS NOT NULL AND request.default_features=docs.no_default_features)
        OR (request.features IS NOT NULL AND (docs.all_features
          OR (array_sort(array_distinct(request.features)) IS DISTINCT FROM array_sort(array_distinct(docs.features)))
          OR cardinality(docs.rustc_args)>0 OR cardinality(docs.rustdoc_args)>0
          OR cardinality(docs.cargo_args)>0))))) AS required FROM hosted_build_scope
    "#).await?;
    Ok(runtime
        .records::<Selected>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("hosted build decision missing"))?
        .required)
}
const RETAINED_NOTE: &str = "Exact validated evidence is retained without age-based expiry. This call did not consult the mutable registry; freshness=revalidate checks it.";
const RESELECTED_NOTE: &str = "The mutable registry selection was revalidated; unchanged exact artifact and environment reuse retained evidence without running extraction again.";

/// Replace only this policy's own note. Unrelated producer limitations retain their order,
/// even when appended after a retained-result note by another qualified consumer.
pub async fn retained_coverage(
    runtime: &QueryRuntime,
    coverage: &enrichment_core::wire::Coverage,
    reselected: bool,
) -> Result<enrichment_core::wire::Coverage> {
    use datafusion::functions_nested::expr_fn::{array_append, array_remove};
    use enrichment_core::native_union::NativeStruct;
    let note = if reselected {
        RESELECTED_NOTE
    } else {
        RETAINED_NOTE
    };
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "research_resolution",
        enrichment_core::wire::Coverage::batch(std::slice::from_ref(coverage))?,
    )?;
    let notes = array_append(
        array_remove(
            array_remove(col("limitations"), lit(RETAINED_NOTE)),
            lit(RESELECTED_NOTE),
        ),
        lit(note),
    );
    runtime
        .records(frame.with_column("limitations", notes)?, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Internal(
                "resolution coverage scope missing".into(),
            )
        })
}
enrichment_core::native_struct! { struct FreshnessScope {
    ecosystem: enrichment_core::identity::Ecosystem => Rule::Text,
    mode: enrichment_core::identity::ResearchMode => Rule::Text,
    explicit_version: bool => Rule::Text,
    source_version_match: enrichment_core::wire::SourceVersionMatch => Rule::Text,
} }

/// Freshness comes from the actual acquisition receipts, never the time a handler renders its
/// result. A current index selection and an exact retained source are independent observations.
pub async fn freshness(
    runtime: &QueryRuntime,
    request: &enrichment_core::request::ResolveRequest,
    source_version_match: enrichment_core::wire::SourceVersionMatch,
    artifacts: &[Artifact],
) -> Result<enrichment_core::wire::Freshness> {
    use enrichment_core::native_union::NativeStruct;
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "freshness_scope",
        FreshnessScope::batch(&[FreshnessScope {
            ecosystem: request.ecosystem,
            mode: request.effective_mode(),
            explicit_version: request.version.is_some(),
            source_version_match,
        }])?,
    )?;
    crate::native_catalog::input(&session, "freshness_receipts", Artifact::batch(artifacts)?)?;
    let plan = session
        .sql(
            "WITH registry AS (
        SELECT acquisition_time(max(clock_instant(retrieved_at))) AS checked_at,
            count(*) FILTER (WHERE kind='registry_index_entry') AS index_reads
        FROM freshness_receipts WHERE kind IN ('registry_index_entry','registry_version_metadata')
    ) SELECT checked_at AS registry_checked_at,source_version_match,
        (mode<>'revision' AND checked_at IS NOT NULL AND index_reads>0
            AND (ecosystem='rust' OR NOT explicit_version)) AS latest_verified
        FROM freshness_scope CROSS JOIN registry",
        )
        .await?;
    runtime.records(plan, 1).await?.pop().ok_or_else(|| {
        datafusion::common::DataFusionError::Internal("resolution freshness scope missing".into())
    })
}

/// Compare complete typed values, independently of their source qualifications. Maps and sets
/// use the existing declared canonical-value kernel; DataFusion owns consensus and selection.
async fn metadata(runtime: &QueryRuntime, mut input: DataFrame) -> Result<Metadata> {
    let session = runtime.session();
    for name in ["rust_docs", "python_distribution"] {
        let field = input.schema().field_with_unqualified_name(name)?.clone();
        let key = enrichment_core::native_identity::canonical_bytes(
            format!("resolution-metadata/{name}/1"),
            vec![field].into(),
        )
        .call(vec![col(name)]);
        input = input.with_column(&format!("{name}_value"), key)?;
    }
    crate::native_catalog::work(&session, "resolution_metadata", input.into_view())?;
    runtime
        .require_empty(
            session
                .sql(
                    "SELECT 'retained_metadata_conflict' AS witness
        FROM resolution_metadata HAVING
        count(DISTINCT rust_docs_value) FILTER (WHERE rust_docs IS NOT NULL)>1 OR
        count(DISTINCT python_distribution_value) FILTER (WHERE python_distribution IS NOT NULL)>1",
                )
                .await?,
            "resolution_metadata_consensus",
            "retained_resolution",
        )
        .await?;
    runtime
        .records(
            session
                .sql(
                    "SELECT
        first_value(rust_docs) FILTER (WHERE rust_docs IS NOT NULL) AS observed_configuration,
        first_value(python_distribution) FILTER (WHERE python_distribution IS NOT NULL) AS python
        FROM resolution_metadata",
                )
                .await?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Internal(
                "resolution metadata aggregate missing".into(),
            )
        })
}

async fn gaps(runtime: &QueryRuntime, input: DataFrame) -> Result<Vec<Gap>> {
    let session = runtime.session();
    crate::native_catalog::work(&session, "resolution_coverage", input.into_view())?;
    let frame = session
        .sql(
            "SELECT DISTINCT gap FROM (SELECT unnest(gaps) AS gap FROM resolution_coverage)
        ORDER BY gap",
        )
        .await?;
    Ok(runtime
        .records::<GapRow>(frame, 4096)
        .await?
        .into_iter()
        .map(|row| row.gap)
        .collect())
}

pub struct Retained {
    pub metadata: Metadata,
    pub gaps: Vec<Gap>,
    pub hosted: Option<Artifact>,
}

/// The snapshot and captured catalog supply current read protection. Only selected output
/// values cross the typed boundary; no handler hydrates metadata alternatives to fold them.
pub async fn retained(reader: &SnapshotReader) -> std::result::Result<Retained, QueryError> {
    let runtime = reader.runtime();
    let metadata = metadata(
        runtime,
        reader
            .session()
            .table("snapshot.evidence.release_metadata")
            .await?,
    )
    .await?;
    let gaps = gaps(
        runtime,
        reader.session().table("snapshot.evidence.coverage").await?,
    )
    .await?;
    let session = reader.pinned().catalog.session(runtime).await?;
    enrichment_core::native_struct! { struct ArtifactRow { artifact: Artifact => Rule::Text } }
    let hosted = runtime.records::<ArtifactRow>(session.sql("WITH receipts AS (
        SELECT unnest(acquisitions) AS artifact,started_at,attempt_id
        FROM state.records.attempts WHERE snapshot_id=$1)
        SELECT artifact FROM receipts WHERE artifact.kind='rustdoc_json'
          AND (starts_with(artifact.source_uri,'https://') OR starts_with(artifact.source_uri,'http://'))
        ORDER BY artifact.artifact_id,artifact.source_uri,started_at,attempt_id LIMIT 1")
        .await?.with_param_values(datafusion::common::ParamValues::List(vec![reader.manifest().snapshot_id.parameter()]))?, 1)
        .await?.pop().map(|row| row.artifact);
    Ok(Retained {
        metadata,
        gaps,
        hosted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{array::StructArray, record_batch::RecordBatch};
    use enrichment_core::{
        evidence::{EvidenceKind, GapReason, metadata::ReleaseDetails},
        native_union::{NativeStruct, NativeUnion},
    };

    #[tokio::test]
    async fn native_hosted_fallback_preserves_explicit_settings_and_author_scope() -> Result<()> {
        use enrichment_core::request::ResolveRequest;
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let mut request = ResolveRequest {
            allow_local_build: true,
            ..Default::default()
        };
        let mut docs = DocsRsMetadata::default();
        assert!(!local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        assert!(local_rustdoc_required(&runtime, &request, Some(&docs), false).await?);
        request.allow_local_build = false;
        assert!(!local_rustdoc_required(&runtime, &request, Some(&docs), false).await?);
        request.allow_local_build = true;
        request.features = Some(vec!["b".into(), "a".into()]);
        docs.features = vec!["a".into(), "b".into()];
        assert!(!local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        docs.features.pop();
        assert!(local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        request.features = None;
        docs.all_features = true;
        assert!(!local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        request.features = Some(vec![]);
        assert!(local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        request.features = None;
        request.default_features = Some(false);
        assert!(local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        docs.no_default_features = true;
        assert!(!local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        request.target = Some("different".into());
        assert!(local_rustdoc_required(&runtime, &request, Some(&docs), true).await?);
        assert!(!local_rustdoc_required(&runtime, &request, None, true).await?);
        request.target = None;
        request.features = Some(docs.features.clone());
        docs.all_features = false;
        for flag in 0..3 {
            let mut changed = docs.clone();
            match flag {
                0 => changed.rustc_args.push("--cfg=one".into()),
                1 => changed.rustdoc_args.push("--cfg=two".into()),
                _ => changed.cargo_args.push("--features=three".into()),
            }
            assert!(local_rustdoc_required(&runtime, &request, Some(&changed), true).await?);
        }
        runtime.close_diagnostics().await
    }

    fn metadata_rows(session: &SessionContext, values: &[ReleaseDetails]) -> Result<DataFrame> {
        let values = ReleaseDetails::encode(&values.iter().collect::<Vec<_>>())?;
        let values = values.as_any().downcast_ref::<StructArray>().unwrap();
        crate::native_catalog::batch(
            session,
            "research_resolution",
            RecordBatch::from(values.clone()),
        )
    }

    #[tokio::test]
    async fn plan19_resolution_notes_replace_only_owned_policy_text() -> Result<()> {
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let coverage = enrichment_core::wire::Coverage {
            scope: "fixture".into(),
            details: None,
            assessments: vec![],
            indexed: Default::default(),
            missing: Default::default(),
            limitations: vec![RETAINED_NOTE.into(), "a later producer limitation".into()],
        };
        let selected = retained_coverage(&runtime, &coverage, true).await?;
        assert_eq!(
            selected.limitations,
            ["a later producer limitation", RESELECTED_NOTE]
        );
        assert_eq!(
            retained_coverage(&runtime, &selected, true).await?,
            selected
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_freshness_uses_registry_receipts_and_native_selection_scope() -> Result<()> {
        use enrichment_core::{
            evidence::ArtifactKind,
            identity::{Ecosystem, ResearchMode},
            native_time::AcquisitionTime,
            request::ResolveRequest,
            wire::SourceVersionMatch,
        };
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let index_time = AcquisitionTime::from_micros(10).unwrap();
        let index = Artifact::describe(
            b"index",
            ArtifactKind::RegistryIndexEntry,
            "application/json",
            "https://example.invalid/registry",
            index_time,
        );
        let unrelated = Artifact::describe(
            b"docs",
            ArtifactKind::RustdocJson,
            "application/json",
            "https://example.invalid/docs",
            AcquisitionTime::from_micros(100).unwrap(),
        );
        let mut request = ResolveRequest {
            ecosystem: Ecosystem::Rust,
            mode: Some(ResearchMode::Upstream),
            name: "sample".into(),
            ..Default::default()
        };
        let absent = freshness(
            &runtime,
            &request,
            SourceVersionMatch::Unknown,
            std::slice::from_ref(&unrelated),
        )
        .await?;
        assert!(absent.registry_checked_at.is_none());
        assert!(!absent.latest_verified);
        let observed = freshness(
            &runtime,
            &request,
            SourceVersionMatch::Exact,
            &[index.clone(), unrelated],
        )
        .await?;
        assert_eq!(observed.registry_checked_at, Some(index_time));
        assert!(observed.latest_verified);
        request.ecosystem = Ecosystem::Python;
        request.version = Some("1.0".into());
        assert!(
            !freshness(
                &runtime,
                &request,
                SourceVersionMatch::Exact,
                std::slice::from_ref(&index)
            )
            .await?
            .latest_verified
        );
        request.version = None;
        assert!(
            freshness(
                &runtime,
                &request,
                SourceVersionMatch::Exact,
                std::slice::from_ref(&index)
            )
            .await?
            .latest_verified
        );
        request.mode = Some(ResearchMode::Revision);
        assert!(
            !freshness(&runtime, &request, SourceVersionMatch::Exact, &[index])
                .await?
                .latest_verified
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_resolution_metadata_consensus_preserves_maps_and_absence() -> Result<()> {
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let session = runtime.session();
        let rust = DocsRsMetadata::default();
        let python = Distribution {
            filename: "sample.whl".into(),
            metadata: std::collections::BTreeMap::from([
                ("name".into(), vec!["sample".into()]),
                ("classifier".into(), vec!["b".into(), "a".into()]),
            ]),
            ..Default::default()
        };
        let empty = metadata(&runtime, metadata_rows(&session, &[])?).await?;
        assert!(empty.observed_configuration.is_none() && empty.python.is_none());
        let values = [
            ReleaseDetails::PythonDistribution(python.clone()),
            ReleaseDetails::RustDocs(rust.clone()),
            ReleaseDetails::RustDocs(rust.clone()),
            ReleaseDetails::PythonDistribution(python.clone()),
        ];
        let selected = metadata(&runtime, metadata_rows(&session, &values)?).await?;
        assert_eq!(selected.observed_configuration, Some(rust.clone()));
        assert_eq!(selected.python, Some(python.clone()));
        let mut changed = rust.clone();
        changed.targets = Some(vec![]);
        assert!(
            metadata(
                &runtime,
                metadata_rows(
                    &session,
                    &[
                        ReleaseDetails::RustDocs(rust),
                        ReleaseDetails::RustDocs(changed)
                    ]
                )?
            )
            .await
            .is_err()
        );
        let mut changed = python.clone();
        changed.metadata.get_mut("classifier").unwrap().reverse();
        assert!(
            metadata(
                &runtime,
                metadata_rows(
                    &session,
                    &[
                        ReleaseDetails::PythonDistribution(python),
                        ReleaseDetails::PythonDistribution(changed)
                    ]
                )?
            )
            .await
            .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_resolution_gaps_use_native_distinct_complete_values() -> Result<()> {
        enrichment_core::native_struct! { struct CoverageRows { gaps: Vec<Gap> => Rule::Sequence } }
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let gap = Gap {
            kind: EvidenceKind::PublicApi,
            reason: GapReason::NotAttempted,
            detail: "missing".into(),
            planned_fallback: None,
        };
        let mut other = gap.clone();
        other.detail = "different producer limitation".into();
        let selected = gaps(
            &runtime,
            crate::native_catalog::batch(
                &runtime.session(),
                "research_resolution",
                CoverageRows::batch(&[
                    CoverageRows {
                        gaps: vec![gap.clone(), other.clone(), gap.clone()],
                    },
                    CoverageRows { gaps: vec![] },
                    CoverageRows {
                        gaps: vec![other.clone()],
                    },
                ])?,
            )?,
        )
        .await?;
        assert_eq!(selected.len(), 2);
        assert!(selected.contains(&gap) && selected.contains(&other));
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
