//! Captured native routing for exact retained evidence, offline requests and acquisition.
use crate::{control::ControlSnapshot, runtime::QueryRuntime};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use enrichment_core::{
    identity::{ContextId, EnvironmentId, SnapshotId},
    native_union::{NativeStruct, NativeUnion, Rule},
};
use std::sync::Arc;

enrichment_core::native_struct! {
    pub struct Scope {
        ecosystem: enrichment_core::identity::Ecosystem => Rule::Text,
        name: String => Rule::NonEmpty,
        registry: String => Rule::NonEmpty,
        version: Option<String> => Rule::Text,
        environment_id: EnvironmentId => Rule::Text,
        mode: enrichment_core::identity::ResearchMode => Rule::Text,
        allow_local_build: bool => Rule::Text,
        freshness: enrichment_core::request::FreshnessMode => Rule::Text,
        profiles: Vec<String> => Rule::Set,
    }
}
enrichment_core::native_union! {
    pub enum Route {
        Retained = "retained" { context_id: ContextId => Rule::Text, snapshot_id: SnapshotId => Rule::Text },
        Acquire = "acquire" { normalized_name: String => Rule::NonEmpty },
        Offline = "offline",
        Disabled = "disabled",
    }
}
enrichment_core::native_struct! {
    pub struct Retained {
        context_id: ContextId => Rule::Text,
        snapshot_id: SnapshotId => Rule::Text,
    }
}

pub struct ResolutionPolicy {
    runtime: QueryRuntime,
    session: SessionContext,
    pub catalog: Arc<ControlSnapshot>,
}

impl ResolutionPolicy {
    pub async fn new(
        runtime: &QueryRuntime,
        catalog: Arc<ControlSnapshot>,
        scope: Scope,
    ) -> Result<Self> {
        let session = catalog.session(runtime).await?;
        crate::native_catalog::batch(&session, "resolution_scope", Scope::batch(&[scope])?)?;
        let input = session.sql("SELECT * REPLACE (CASE WHEN ecosystem='python' THEN regexp_replace(lower(name),'[-_.]+','-','g') ELSE lower(name) END AS name) FROM resolution_scope").await?;
        crate::native_catalog::work(&session, "resolution_input", input.into_view())?;
        let releases = session.sql("SELECT r.* FROM state.records.releases r JOIN resolution_input q ON r.key.ecosystem=q.ecosystem AND r.key.registry=q.registry AND r.key.version=q.version AND CASE WHEN r.key.ecosystem='python' THEN regexp_replace(lower(r.key.package),'[-_.]+','-','g') ELSE replace(lower(r.key.package),'_','-') END=CASE WHEN q.ecosystem='python' THEN q.name ELSE replace(q.name,'_','-') END").await?;
        crate::native_catalog::work(&session, "resolution_releases", releases.into_view())?;
        runtime.require_empty(session.sql("SELECT 'ambiguous_release' AS witness FROM resolution_releases HAVING count(*)>1").await?, "resolution_release_identity", "resolution_routing").await?;
        let retained = session.sql(&format!(r#"
            SELECT c.context_id,s.snapshot_id FROM resolution_releases r
            JOIN state.records.contexts c ON r.release_id=c.release_id
            JOIN resolution_input q ON c.environment_id=q.environment_id AND c.mode=q.mode
            JOIN state.records.selections h ON c.context_id=h.context_id
            JOIN state.records.snapshots s ON h.snapshot_id=s.snapshot_id AND c.context_id=s.context_id
            WHERE NOT q.allow_local_build OR NOT array_has(s.publication.missing,'public_api')
                OR EXISTS (SELECT 1 FROM state.records.attempts a WHERE a.snapshot_id=s.snapshot_id AND a.producer='{}')
        "#, enrichment_core::producer::rustdoc::LOCAL_PRODUCER)).await?;
        crate::native_catalog::work(&session, "resolution_retained", retained.into_view())?;
        runtime.require_empty(session.sql("SELECT 'ambiguous_retained_context' AS witness FROM resolution_retained HAVING count(*)>1").await?, "resolution_context_identity", "resolution_routing").await?;
        Ok(Self {
            runtime: runtime.clone(),
            session,
            catalog,
        })
    }
    pub async fn route(&self) -> Result<Route> {
        let selected = self.session.sql("SELECT CASE WHEN q.freshness<>'revalidate' AND r.snapshot_id IS NOT NULL THEN 'retained' WHEN q.freshness='offline' THEN 'offline' WHEN array_has(q.profiles,'static') THEN 'acquire' ELSE 'disabled' END AS kind,r.context_id,r.snapshot_id,q.name AS normalized_name FROM resolution_input q LEFT JOIN resolution_retained r ON true").await?;
        crate::native_catalog::work(&self.session, "resolution_decision", selected.into_view())?;
        let frame = self.session.sql("SELECT kind, CASE WHEN kind='retained' THEN named_struct('context_id',context_id,'snapshot_id',snapshot_id) END AS retained, CASE WHEN kind='acquire' THEN named_struct('normalized_name',normalized_name) END AS acquire FROM resolution_decision").await?;
        let output = self.output(frame).await?;
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&output)?;
        Route::decode(rows.row(0)).map_err(DataFusionError::from)
    }
    /// Used after exact upstream selection, with the same native eligibility relation.
    pub async fn retained(&self) -> Result<Option<Retained>> {
        let output = self
            .runtime
            .execute(
                self.session
                    .table("resolution_retained")
                    .await?
                    .limit(0, Some(2))?,
            )
            .await?;
        if output.rows > 1 {
            return datafusion::common::exec_err!("ambiguous retained resolution");
        }
        let mut selected = None;
        for batch in output.batches {
            let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch)?;
            for index in 0..batch.num_rows() {
                selected = Some(Retained::decode(rows.row(index))?);
            }
        }
        Ok(selected)
    }
    async fn output(&self, frame: DataFrame) -> Result<arrow::record_batch::RecordBatch> {
        let output = self.runtime.execute(frame.limit(0, Some(2))?).await?;
        if output.rows != 1 {
            return datafusion::common::exec_err!(
                "resolution requires exactly one routing decision"
            );
        }
        output
            .batches
            .into_iter()
            .find(|batch| batch.num_rows() == 1)
            .ok_or_else(|| DataFusionError::Internal("missing resolution routing decision".into()))
    }
}
