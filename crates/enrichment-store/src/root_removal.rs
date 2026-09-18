//! One native visibility and reverse-dependency policy for publication/result retirement.
//! Tombstones and cleanup obligations commit together; physical readers retain their leases.
use crate::{
    control::{ControlStore, Table},
    native_catalog::{self, BindingKind, BoundCatalog, Tables},
    retention::{CleanupObligation, RetentionRoot, RetentionStore, RootRemoval},
    runtime::QueryRuntime,
};
use datafusion::{
    common::{DataFusionError, Result},
    dataframe::DataFrame,
    prelude::*,
};
use enrichment_core::native_union::{NativeStruct, Rule};
use std::{collections::BTreeMap, sync::Arc};

enrichment_core::native_struct! { struct RequestedRoot { root_id: String => Rule::NonEmpty } }
enrichment_core::native_struct! { struct InvalidatedTable { table_uri: String => Rule::NonEmpty } }

/// Folded facts remain available to admission and reconciliation. Every newly captured
/// product catalog applies this same policy, including results, selection and recovery.
pub(crate) async fn visible_records(runtime: &QueryRuntime, mut records: Tables) -> Result<Tables> {
    let catalog = BoundCatalog::default().with_schema(BindingKind::FoldedRecords, records.clone());
    let session = runtime.bound_session(BTreeMap::from([(
        "state".into(),
        Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
    )]))?;
    for (table, key) in [
        (Table::Snapshots, "snapshot_id"),
        (Table::SearchProjections, "projection_id"),
        (Table::RetainedResults, "result_artifact_id"),
    ] {
        let frame = session.table(table.reference()).await?;
        let label = enrichment_core::native_id::diagnostic(
            frame.schema().field_with_unqualified_name(key)?,
            col(key),
        )?;
        let input = format!("visibility_{}", table.name());
        native_catalog::work(
            &session,
            &input,
            frame.with_column("__root_label", label)?.into_view(),
        )?;
        let plan = session.sql(&format!("SELECT t.* EXCLUDE (__root_label) FROM {input} t JOIN state.records.retention_roots r ON t.__root_label=r.root_id WHERE NOT r.removed")).await?;
        records.insert(table.name().into(), plan.into_view());
    }
    // These views refer to the already filtered native root families. They do not
    // choose a historical fallback when the current selection has been removed.
    let catalog = BoundCatalog::default().with_schema(BindingKind::FoldedRecords, records.clone());
    let session = runtime.bound_session(BTreeMap::from([(
        "state".into(),
        Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
    )]))?;
    for (table, sql) in [
        (
            Table::Selections,
            "SELECT t.* FROM state.records.selections t JOIN state.records.snapshots s ON t.snapshot_id=s.snapshot_id",
        ),
        (
            Table::Attempts,
            "SELECT t.* FROM state.records.attempts t JOIN state.records.snapshots s ON t.snapshot_id=s.snapshot_id",
        ),
        (
            Table::JobPublications,
            "SELECT t.* FROM state.records.job_publications t JOIN state.records.snapshots s ON t.snapshot_id=s.snapshot_id JOIN state.records.retained_results r ON t.delivery.artifact_id=r.result_artifact_id",
        ),
        (
            Table::ComparisonPublications,
            "SELECT t.* FROM state.records.comparison_publications t JOIN state.records.snapshots b ON t.before_snapshot_id=b.snapshot_id JOIN state.records.snapshots a ON t.after_snapshot_id=a.snapshot_id JOIN state.records.retained_results r ON t.delivery.artifact_id=r.result_artifact_id",
        ),
        (
            Table::ArtifactReceipts,
            "SELECT t.* FROM state.records.artifact_receipts t LEFT SEMI JOIN (SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed) r ON r.dependency.kind='artifact' AND t.artifact.artifact_id=r.dependency.artifact.artifact_id",
        ),
        (
            Table::JobTransitions,
            "SELECT t.* REPLACE (CASE WHEN r.removed THEN NULL ELSE t.result END AS result) FROM state.records.job_transitions t LEFT JOIN state.records.retention_roots r ON t.result.artifact_id=r.root_id",
        ),
    ] {
        records.insert(table.name().into(), session.sql(sql).await?.into_view());
    }
    Ok(records)
}

/// A child is retired when an input disappears. References between retained results
/// flow backwards: removing a referenced result also removes its referring results.
async fn edges(session: &SessionContext) -> Result<DataFrame> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    let mut graph = session.sql("SELECT ref.artifact_id AS parent,result_artifact_id AS child FROM (SELECT result_artifact_id,unnest(references) AS ref FROM state.records.retained_results)").await?;
    for (table, key, child) in [
        (
            Table::SearchProjections,
            "snapshot_id",
            col("projection_id"),
        ),
        (
            Table::JobPublications,
            "snapshot_id",
            col("delivery").field("artifact_id"),
        ),
        (
            Table::ComparisonPublications,
            "before_snapshot_id",
            col("delivery").field("artifact_id"),
        ),
        (
            Table::ComparisonPublications,
            "after_snapshot_id",
            col("delivery").field("artifact_id"),
        ),
    ] {
        let frame = session.table(table.reference()).await?;
        let parent = enrichment_core::native_id::diagnostic(
            frame.schema().field_with_unqualified_name(key)?,
            col(key),
        )?;
        graph = graph.union(frame.select(vec![parent.alias("parent"), child.alias("child")])?)?;
    }
    let snapshots = session.table(Table::RetainedResults.reference()).await?;
    let snapshots = crate::field_admission::identity_values(
        snapshots,
        Table::RetainedResults.schema()?.as_ref(),
        "result_artifact_id",
        enrichment_core::native_union::Domain::Snapshot,
    )?
    .ok_or_else(|| {
        DataFusionError::Plan("retained results have no declared snapshot references".into())
    })?;
    let parent = enrichment_core::native_id::diagnostic(
        snapshots.schema().field_with_unqualified_name("value")?,
        col("value"),
    )?;
    graph
        .union(snapshots.select(vec![parent.alias("parent"), col("owner").alias("child")])?)?
        .distinct()
}

async fn selection(
    runtime: &QueryRuntime,
    session: &SessionContext,
    requested: &[String],
    edges: DataFrame,
) -> Result<DataFrame> {
    if requested.is_empty() || requested.len() > 256 {
        return datafusion::common::exec_err!("root removal requires between 1 and 256 roots");
    }
    native_catalog::work(
        session,
        "requested_roots",
        crate::native_catalog::batch(
            session,
            "root_removal",
            RequestedRoot::batch(
                &requested
                    .iter()
                    .map(|root_id| RequestedRoot {
                        root_id: root_id.clone(),
                    })
                    .collect::<Vec<_>>(),
            )?,
        )?
        .distinct()?
        .into_view(),
    )?;
    runtime.require_empty(session.sql("SELECT q.root_id AS witness FROM requested_roots q LEFT ANTI JOIN state.records.retention_roots r ON q.root_id=r.root_id").await?, "removal_root_present", "retention").await?;
    let metadata =
        datafusion::common::metadata::FieldMetadata::from(RequestedRoot::fields()[0].as_ref());
    let edges = edges.select(vec![
        col("parent").alias_with_metadata("parent", Some(metadata.clone())),
        col("child").alias_with_metadata("child", Some(metadata)),
    ])?;
    native_catalog::work(session, "removal_edges", edges.into_view())?;
    let walk = session.sql("WITH RECURSIVE walk AS (SELECT root_id,make_array(root_id) AS path,0 AS depth,false AS cycle FROM requested_roots UNION ALL SELECT e.child,array_append(w.path,e.child),w.depth+1,array_has(w.path,e.child) FROM walk w JOIN removal_edges e ON w.root_id=e.parent WHERE w.depth<64 AND NOT w.cycle) SELECT * FROM walk").await?;
    native_catalog::work(session, "removal_walk", walk.into_view())?;
    runtime
        .require_empty(
            session
                .sql("SELECT root_id AS witness FROM removal_walk WHERE cycle OR depth=64")
                .await?,
            "removal_dependency_cycle_depth",
            "retention",
        )
        .await?;
    let selected = session.sql("SELECT r.* FROM state.records.retention_roots r JOIN (SELECT DISTINCT root_id FROM removal_walk) w ON r.root_id=w.root_id").await?;
    native_catalog::work(session, "removal_roots", selected.clone().into_view())?;
    runtime
        .require_empty(
            session
                .sql("SELECT 'removal_count' AS witness FROM removal_roots HAVING count(*)>256")
                .await?,
            "removal_closure_bound",
            "retention",
        )
        .await?;
    selected.sort(vec![col("root_id").sort(true, false)])
}

pub(crate) async fn require_roots(
    runtime: &QueryRuntime,
    session: &SessionContext,
    roots: &[String],
) -> Result<()> {
    if roots.is_empty() {
        return Ok(());
    }
    if roots.len() > 256 {
        return datafusion::common::exec_err!("read root bound exceeded");
    }
    native_catalog::work(
        session,
        "read_roots",
        crate::native_catalog::batch(
            session,
            "root_removal",
            RequestedRoot::batch(
                &roots
                    .iter()
                    .map(|root_id| RequestedRoot {
                        root_id: root_id.clone(),
                    })
                    .collect::<Vec<_>>(),
            )?,
        )?
        .into_view(),
    )?;
    runtime.require_empty(session.sql("SELECT q.root_id AS witness FROM read_roots q LEFT ANTI JOIN state.records.retention_roots r ON q.root_id=r.root_id AND NOT r.removed").await?, "read_root_current", "retention_enrollment").await
}

impl ControlStore {
    /// Preview or atomically retire an exact publication/result dependency closure.
    /// No byte deletion occurs here. Repeating an acknowledged or unknown-ack removal
    /// observes its tombstones and never creates a second cleanup owner.
    pub async fn remove_roots(&self, requested: &[String], apply: bool) -> Result<RootRemoval> {
        if apply {
            self.require_write()?;
        }
        let delta = self.delta_namespace();
        let runtime = &delta.runtime;
        for _ in 0..16 {
            let pin = self.capture().await?;
            let session = pin.transition_session(runtime).await?;
            let selected = selection(runtime, &session, requested, edges(&session).await?).await?;
            let roots = runtime.records::<RetentionRoot>(selected, 256).await?;
            if !apply {
                return Ok(RootRemoval {
                    generation: pin.generation(),
                    applied: false,
                    roots,
                });
            }
            let next = pin
                .generation()
                .checked_add(1)
                .ok_or_else(|| DataFusionError::Execution("removal generation overflow".into()))?;
            let mut changes = Vec::new();
            let process = crate::native_process::current()
                .map_err(|e| DataFusionError::External(Box::new(e)))?;
            for root in roots.iter().filter(|root| !root.removed) {
                changes.push((
                    Table::RetentionRoots,
                    RetentionRoot::batch(&[RetentionRoot {
                        removed: true,
                        sequence: next,
                        ..root.clone()
                    }])?,
                ));
                changes.push((
                    Table::CleanupObligations,
                    CleanupObligation::batch(&[CleanupObligation {
                        obligation_id: enrichment_core::identity::CleanupObligationId::new(),
                        label: format!("retired-root/{}", root.root_id),
                        process: process.clone(),
                        dependencies: root.dependencies.clone(),
                        physical_released: true,
                        settled: false,
                        sequence: next,
                    }])?,
                ));
            }
            let generation = if changes.is_empty() {
                pin.generation()
            } else {
                match self
                    .commit_native(
                        pin.generation(),
                        changes,
                        roots
                            .iter()
                            .map(|r| format!("retention-root/{}", r.root_id))
                            .collect(),
                    )
                    .await?
                {
                    Some(version) => version,
                    None => continue,
                }
            };
            // Invalidation happens after the durable authority change. It is repeatable
            // after unknown acknowledgment. Live readers keep their own immutable Arcs.
            let tables = session.sql(&format!("SELECT DISTINCT {} AS table_uri FROM (SELECT unnest(dependencies) AS dependency FROM removal_roots) WHERE dependency.kind IN ('table','cdf_window','table_scope','pending_row')", crate::retention::table_scope("dependency"))).await?;
            for table in runtime.records::<InvalidatedTable>(tables, 8192).await? {
                delta.invalidate(&table.table_uri)?;
            }
            return Ok(RootRemoval {
                generation,
                applied: true,
                roots: roots
                    .into_iter()
                    .map(|r| RetentionRoot {
                        removed: true,
                        sequence: if r.removed { r.sequence } else { generation },
                        ..r
                    })
                    .collect(),
            });
        }
        datafusion::common::exec_err!("root removal conflict bound")
    }
}

/// An old captured catalog is not fresh authority to enroll another reader. Preserve
/// already enrolled leases, but refuse a new dependency held only by removed roots.
pub(crate) async fn removed_dependencies(
    session: &SessionContext,
    input: &str,
) -> Result<DataFrame> {
    session.sql(&format!("WITH incoming AS (SELECT unnest(dependencies) AS dependency FROM {input}), removed AS (SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE removed), retained AS (SELECT unnest(dependencies) AS dependency FROM state.records.retention_roots WHERE NOT removed) SELECT DISTINCT i.dependency AS witness FROM incoming i JOIN removed r ON i.dependency=r.dependency LEFT ANTI JOIN retained a ON i.dependency=a.dependency")).await
}

impl RetentionStore {
    pub(crate) async fn require_current_dependencies(
        &self,
        runtime: &QueryRuntime,
        session: &SessionContext,
        input: &str,
    ) -> Result<()> {
        runtime
            .require_empty(
                removed_dependencies(session, input).await?,
                "retention_removed_root",
                "retention_enrollment",
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retention::Dependency;
    use arrow::{
        array::StringArray,
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    };

    fn root(id: &str, removed: bool) -> RetentionRoot {
        RetentionRoot {
            root_id: id.into(),
            dependencies: vec![Dependency::Artifact {
                artifact_id: format!("artifact/{id}"),
            }],
            removed,
            sequence: 1,
        }
    }
    fn catalog(
        runtime: &QueryRuntime,
        roots: &[RetentionRoot],
    ) -> Result<(SessionContext, Tables)> {
        let mut records = Tables::new();
        for table in Table::ALL {
            let batch = if table == Table::RetentionRoots {
                RetentionRoot::batch(roots)?
            } else {
                RecordBatch::new_empty(table.schema()?)
            };
            records.insert(
                table.name().into(),
                crate::native_catalog::batch(&runtime.session(), "root_removal", batch)?
                    .into_view(),
            );
        }
        let bound =
            BoundCatalog::default().with_schema(BindingKind::FoldedRecords, records.clone());
        Ok((
            runtime.bound_session(BTreeMap::from([(
                "state".into(),
                Arc::new(bound) as Arc<dyn datafusion::catalog::CatalogProvider>,
            )]))?,
            records,
        ))
    }
    fn graph(
        session: &SessionContext,
        parents: Vec<&str>,
        children: Vec<&str>,
    ) -> Result<DataFrame> {
        crate::native_catalog::batch(
            session,
            "root_removal",
            RecordBatch::try_new(
                Arc::new(Schema::new(vec![
                    Field::new("parent", DataType::Utf8, false),
                    Field::new("child", DataType::Utf8, false),
                ])),
                vec![
                    Arc::new(StringArray::from(parents)),
                    Arc::new(StringArray::from(children)),
                ],
            )?,
        )
    }

    #[tokio::test]
    async fn plan19_native_root_removal_closure_visibility_and_refusal() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let roots = [
            root("snapshot", false),
            root("search", false),
            root("result", false),
            root("parent-result", false),
            root("unrelated", false),
        ];
        let (session, records) = catalog(&runtime, &roots)?;
        // Plan every production visibility relation and edge against actual declarations.
        for (_, provider) in visible_records(&runtime, records).await? {
            runtime.execute(session.read_table(provider)?).await?;
        }
        runtime.execute(edges(&session).await?).await?;
        let selected = selection(
            &runtime,
            &session,
            &["snapshot".into()],
            graph(
                &session,
                vec!["snapshot", "snapshot", "result"],
                vec!["search", "result", "parent-result"],
            )?,
        )
        .await?;
        let mut found = runtime
            .records::<RetentionRoot>(selected, 256)
            .await?
            .into_iter()
            .map(|r| r.root_id)
            .collect::<Vec<_>>();
        found.sort();
        assert_eq!(found, vec!["parent-result", "result", "search", "snapshot"]);
        let (session, _) = catalog(&runtime, &roots)?;
        assert!(
            selection(
                &runtime,
                &session,
                &["unknown".into()],
                graph(&session, vec![], vec![])?
            )
            .await
            .is_err()
        );
        let (session, _) = catalog(&runtime, &roots)?;
        assert!(
            selection(
                &runtime,
                &session,
                &["snapshot".into()],
                graph(
                    &session,
                    vec!["snapshot", "search"],
                    vec!["search", "snapshot"]
                )?
            )
            .await
            .is_err()
        );
        // Shared dependencies remain admissible; a removed-only dependency does not.
        let retired = root("retired", true);
        let mut shared = root("shared", false);
        shared.dependencies = retired.dependencies.clone();
        for (roots, expected) in [
            (vec![retired.clone()], 1),
            (vec![retired.clone(), shared], 0),
        ] {
            let (session, _) = catalog(&runtime, &roots)?;
            native_catalog::work(
                &session,
                "incoming",
                crate::native_catalog::batch(
                    &session,
                    "root_removal",
                    RetentionRoot::batch(std::slice::from_ref(&retired))?,
                )?
                .into_view(),
            )?;
            assert_eq!(
                runtime
                    .execute(removed_dependencies(&session, "incoming").await?)
                    .await?
                    .rows,
                expected
            );
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
