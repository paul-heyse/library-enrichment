//! Native retained-input admission and snapshot-summary agreement.
use crate::{dataset::WriteLimits, native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{Result, ScalarValue},
    prelude::SessionContext,
};
use datafusion::{
    dataframe::DataFrame,
    prelude::{col, lit},
};
use enrichment_core::evidence::snapshot::EvidenceManifest;
use enrichment_core::native_union::{NativeStruct, Rule};

enrichment_core::native_struct! { pub(crate) struct Content {
    sha256:String=>Rule::Sha256, size_bytes:u64=>Rule::Text
} }

pub(crate) async fn inputs(
    runtime: &QueryRuntime,
    session: &SessionContext,
    limits: &WriteLimits,
) -> Result<Vec<Content>> {
    let frame = session
        .sql("SELECT DISTINCT sha256,size_bytes FROM snapshot.evidence.input_artifacts")
        .await?;
    native_catalog::work(session, "validated_input_content", frame.into_view())?;
    runtime.require_empty(session.sql("SELECT sha256 AS witness FROM validated_input_content GROUP BY sha256 HAVING count(*)<>1 UNION ALL SELECT sha256 FROM validated_input_content WHERE size_bytes>$1 UNION ALL SELECT 'input_closure_bytes' FROM validated_input_content HAVING sum(size_bytes)>$2").await?.with_param_values(vec![ScalarValue::UInt64(Some(limits.file_bytes)),ScalarValue::UInt64(Some(limits.file_bytes.saturating_mul(2)))])?,"input_validation_bounds","snapshot_admission").await?;
    runtime
        .records(
            session.table("validated_input_content").await?,
            limits.table_rows,
        )
        .await
}

pub(crate) async fn manifest(
    runtime: &QueryRuntime,
    session: &SessionContext,
    manifest: &EvidenceManifest,
    limits: &WriteLimits,
) -> Result<()> {
    native_catalog::input(
        session,
        "retained_manifest",
        EvidenceManifest::batch(std::slice::from_ref(manifest))?,
    )?;
    let mut components: Option<DataFrame> = None;
    for relation in crate::admission::Relation::ALL {
        let component =
            crate::semantic::digest_plan(relation, session.table(relation.reference()).await?)?
                .select(vec![
                    lit(relation.name()).alias("relation"),
                    col("digest"),
                    col("rows"),
                ])?;
        components = Some(match components {
            None => component,
            Some(prior) => prior.union(component)?,
        });
    }
    let components = components.ok_or_else(|| {
        datafusion::common::DataFusionError::Internal("no declared evidence relations".into())
    })?;
    native_catalog::work(session, "physical_components", components.into_view())?;
    native_catalog::work(
        session,
        "physical_coverage",
        crate::coverage_plan::plan(session).await?.into_view(),
    )?;
    agreement(runtime, session, limits.table_rows).await
}

async fn agreement(
    runtime: &QueryRuntime,
    session: &SessionContext,
    max_rows: usize,
) -> Result<()> {
    runtime.require_empty(session.sql("WITH expected AS (SELECT entry.key AS relation,entry.value AS digest FROM (SELECT unnest(native_map_entries(components)) AS entry FROM retained_manifest)) SELECT coalesce(e.relation,p.relation) AS witness FROM expected e FULL OUTER JOIN physical_components p ON e.relation=p.relation WHERE (e.digest IS DISTINCT FROM p.digest) OR p.rows>$1").await?.with_param_values(vec![ScalarValue::UInt64(Some(max_rows as u64))])?,"snapshot_semantic_components","snapshot_admission").await?;
    runtime.require_empty(session.sql(r#"
        WITH bindings AS (SELECT unnest(tables) AS binding FROM retained_manifest), expected AS (
            SELECT 'definitions' AS relation, counts.definitions AS rows FROM retained_manifest
            UNION ALL SELECT 'symbols',counts.symbols FROM retained_manifest
            UNION ALL SELECT 'relationships',counts.relationships FROM retained_manifest
            UNION ALL SELECT 'fragments',counts.fragments FROM retained_manifest
        )
        SELECT e.relation AS witness FROM expected e LEFT JOIN bindings b ON e.relation=b.binding.relation
            WHERE e.rows IS DISTINCT FROM b.binding.rows
        UNION ALL SELECT coalesce(p.relation,b.binding.relation) FROM physical_components p FULL OUTER JOIN bindings b ON p.relation=b.binding.relation WHERE p.rows IS DISTINCT FROM b.binding.rows
        UNION ALL SELECT 'reexports' FROM retained_manifest WHERE counts.reexports IS DISTINCT FROM (SELECT count(*) FROM snapshot.evidence.symbols WHERE is_reexport)
        UNION ALL SELECT 'unresolved_reexports' FROM retained_manifest WHERE counts.unresolved_reexports IS DISTINCT FROM (SELECT count(*) FROM snapshot.evidence.relationships WHERE relation='reexports' AND target.kind IN ('unresolved','external'))
        UNION ALL SELECT 'producer_items' FROM retained_manifest WHERE counts.producer_items IS DISTINCT FROM metadata.producer_items
        UNION ALL SELECT 'coverage' FROM retained_manifest m CROSS JOIN physical_coverage c
            WHERE array_length(m.indexed)<>coalesce(array_length(c.indexed),0) OR array_length(m.missing)<>coalesce(array_length(c.missing),0)
            OR (array_length(m.indexed)>0 AND (m.indexed IS DISTINCT FROM c.indexed)) OR (array_length(m.missing)>0 AND (m.missing IS DISTINCT FROM c.missing))
    "#).await?,"snapshot_summary_agreement","snapshot_admission").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::evidence::{EvidenceKind, SnapshotCounts, snapshot::DeltaBinding};
    use std::sync::Arc;
    enrichment_core::native_struct! { struct Metadata {producer_items:u64=>Rule::Text} }
    enrichment_core::native_struct! { struct Summary {
        components:std::collections::BTreeMap<String,String> =>Rule::Map,
        tables:Vec<DeltaBinding> =>Rule::Sequence,
        counts:SnapshotCounts=>Rule::Text,
        metadata:Metadata=>Rule::Text,
        indexed:Vec<EvidenceKind> =>Rule::Set,
        missing:Vec<EvidenceKind> =>Rule::Set
    } }
    enrichment_core::native_struct! { struct Component {
        relation:String=>Rule::Text,digest:String=>Rule::Text,rows:i64=>Rule::Text
    } }
    #[tokio::test]
    async fn snapshot_summary_native_agreement_refuses_changed_components_counts_and_coverage()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let names = [
            "definitions",
            "symbols",
            "relationships",
            "fragments",
            "execution_observations",
        ];
        let summary = Summary {
            components: names
                .iter()
                .map(|name| ((*name).into(), "a".repeat(64)))
                .collect(),
            tables: names
                .iter()
                .map(|name| DeltaBinding {
                    source: enrichment_core::delta_reference::DeltaVersionRef {
                        table: enrichment_core::delta_reference::DeltaTableRef {
                            table_uri: format!("evidence_{name}"),
                            table_id: "unit".into(),
                            contract_id: enrichment_core::identity::SchemaContractId::try_from(
                                format!("schema_contract_{}", "b".repeat(64)),
                            )
                            .unwrap(),
                        },
                        version: 0,
                    },
                    relation: (*name).into(),
                    cohort_id: enrichment_core::identity::CohortId::try_from(
                        "cohort_385cfdbc00ec32031699460779c15099".to_owned(),
                    )
                    .unwrap(),
                    rows: 0,
                })
                .collect(),
            counts: SnapshotCounts {
                definitions: 0,
                symbols: 0,
                relationships: 0,
                fragments: 0,
                reexports: 0,
                unresolved_reexports: 0,
                producer_items: 0,
            },
            metadata: Metadata { producer_items: 0 },
            indexed: vec![],
            missing: vec![],
        };
        let check = async |summary: &Summary| -> Result<()> {
            let session = runtime.session();
            let mut tables = native_catalog::Tables::new();
            for relation in [
                crate::admission::Relation::Symbols,
                crate::admission::Relation::Relationships,
                crate::admission::Relation::Coverage,
            ] {
                tables.insert(
                    relation.name().into(),
                    native_catalog::batch(
                        &session,
                        relation.name(),
                        arrow::record_batch::RecordBatch::new_empty(relation.schema()?),
                    )?
                    .into_view(),
                );
            }
            let session = runtime.bound_session(
                [(
                    "snapshot".into(),
                    Arc::new(
                        native_catalog::BoundCatalog::default()
                            .with_schema(native_catalog::BindingKind::AdmittedEvidence, tables),
                    ) as Arc<dyn datafusion::catalog::CatalogProvider>,
                )]
                .into_iter()
                .collect(),
            )?;
            native_catalog::input(
                &session,
                "retained_manifest",
                Summary::batch(std::slice::from_ref(summary))?,
            )?;
            native_catalog::input(
                &session,
                "physical_components",
                Component::batch(
                    &names
                        .iter()
                        .map(|name| Component {
                            relation: (*name).into(),
                            digest: "a".repeat(64),
                            rows: 0,
                        })
                        .collect::<Vec<_>>(),
                )?,
            )?;
            native_catalog::work(
                &session,
                "physical_coverage",
                crate::coverage_plan::plan(&session).await?.into_view(),
            )?;
            agreement(&runtime, &session, 16).await
        };
        check(&summary).await?;
        let mut changed = summary.clone();
        changed.components.insert("symbols".into(), "c".repeat(64));
        assert!(check(&changed).await.is_err());
        let mut changed = summary.clone();
        changed.components.remove("symbols");
        assert!(check(&changed).await.is_err());
        let mut changed = summary.clone();
        changed.tables.remove(0);
        assert!(check(&changed).await.is_err());
        let mut changed = summary.clone();
        changed.tables[4].rows = 1;
        assert!(check(&changed).await.is_err());
        let mut changed = summary.clone();
        changed.counts.reexports = 1;
        assert!(check(&changed).await.is_err());
        let mut changed = summary.clone();
        changed.metadata.producer_items = 1;
        assert!(check(&changed).await.is_err());
        let mut changed = summary.clone();
        changed.indexed.push(EvidenceKind::CrateSource);
        assert!(check(&changed).await.is_err());
        runtime.close_diagnostics().await
    }
    fn bind(runtime: &QueryRuntime, rows: &[Content]) -> Result<SessionContext> {
        let session = runtime.session();
        let provider =
            native_catalog::batch(&session, "inputs", Content::batch(rows)?)?.into_view();
        runtime.bound_session(
            [(
                "snapshot".into(),
                Arc::new(native_catalog::BoundCatalog::default().with_schema(
                    native_catalog::BindingKind::AdmittedEvidence,
                    [("input_artifacts".into(), provider)].into_iter().collect(),
                )) as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]
            .into_iter()
            .collect(),
        )
    }
    #[tokio::test]
    async fn input_validation_native_bounds_count_unique_content() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let limits = WriteLimits {
            file_bytes: 8,
            ..Default::default()
        };
        let a = Content {
            sha256: "a".repeat(64),
            size_bytes: 8,
        };
        let b = Content {
            sha256: "b".repeat(64),
            size_bytes: 8,
        };
        assert_eq!(
            inputs(
                &runtime,
                &bind(&runtime, &[a.clone(), a.clone(), b.clone()])?,
                &limits
            )
            .await?
            .len(),
            2
        );
        for rows in [
            vec![
                a.clone(),
                Content {
                    size_bytes: 7,
                    ..a.clone()
                },
            ],
            vec![Content {
                size_bytes: 9,
                ..a.clone()
            }],
            vec![
                a,
                b,
                Content {
                    sha256: "c".repeat(64),
                    size_bytes: 1,
                },
            ],
        ] {
            assert!(
                inputs(&runtime, &bind(&runtime, &rows)?, &limits)
                    .await
                    .is_err()
            );
        }
        runtime.close_diagnostics().await
    }
}
