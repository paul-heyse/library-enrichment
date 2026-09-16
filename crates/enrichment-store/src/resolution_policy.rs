//! Captured native routing for exact retained evidence, offline requests and acquisition.
use crate::{
    control::ControlSnapshot, control_jobs::encode, registry::rows, runtime::QueryRuntime,
};
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct Scope<'a> {
    pub ecosystem: enrichment_core::identity::Ecosystem,
    pub name: &'a str,
    pub registry: &'a str,
    pub version: Option<&'a str>,
    pub environment_id: &'a str,
    pub mode: enrichment_core::identity::ResearchMode,
    pub allow_local_build: bool,
    pub freshness: enrichment_core::request::FreshnessMode,
    pub profiles: &'a [String],
}

#[derive(Debug, Deserialize)]
#[serde(tag = "route", rename_all = "snake_case")]
pub enum Route {
    Retained {
        context_id: String,
        snapshot_id: String,
    },
    Acquire {
        normalized_name: String,
    },
    Offline,
    Disabled,
}

#[derive(Deserialize)]
pub struct Retained {
    pub context_id: String,
    pub snapshot_id: String,
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
        scope: Scope<'_>,
    ) -> Result<Self> {
        let mut fields = [
            "ecosystem",
            "name",
            "registry",
            "environment_id",
            "mode",
            "freshness",
        ]
        .map(|name| Field::new(name, DataType::Utf8, false))
        .to_vec();
        fields.extend([
            Field::new("version", DataType::Utf8, true),
            Field::new("allow_local_build", DataType::Boolean, false),
            Field::new(
                "profiles",
                DataType::List(Arc::new(Field::new("item", DataType::Utf8, true))),
                false,
            ),
        ]);
        let session = catalog.session(runtime).await?;
        session.register_batch(
            "resolution_scope",
            encode(Arc::new(Schema::new(fields)), &[scope])?,
        )?;
        let input = session.sql("SELECT * REPLACE (CASE WHEN ecosystem='python' THEN regexp_replace(lower(name),'[-_.]+','-','g') ELSE lower(name) END AS name) FROM resolution_scope").await?;
        crate::native_catalog::work(&session, "resolution_input", input.into_view())?;
        let releases = session.sql("SELECT r.* FROM state.records.releases r JOIN resolution_input q ON r.ecosystem=q.ecosystem AND r.registry=q.registry AND r.version=q.version AND CASE WHEN r.ecosystem='python' THEN regexp_replace(lower(r.package),'[-_.]+','-','g') ELSE replace(lower(r.package),'_','-') END=CASE WHEN q.ecosystem='python' THEN q.name ELSE replace(q.name,'_','-') END").await?;
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
        self.one(self.session.sql("SELECT CASE WHEN q.freshness<>'revalidate' AND r.snapshot_id IS NOT NULL THEN 'retained' WHEN q.freshness='offline' THEN 'offline' WHEN array_has(q.profiles,'static') THEN 'acquire' ELSE 'disabled' END AS route,r.context_id,r.snapshot_id,q.name AS normalized_name FROM resolution_input q LEFT JOIN resolution_retained r ON true").await?).await
    }
    /// Used after an independently verified exact upstream selection, with the same eligibility.
    pub async fn retained(&self) -> Result<Option<Retained>> {
        Ok(rows(
            &self.runtime,
            self.session.table("resolution_retained").await?,
            1,
        )
        .await?
        .pop())
    }
    async fn one<T: serde::de::DeserializeOwned>(&self, frame: DataFrame) -> Result<T> {
        rows(&self.runtime, frame, 1)
            .await?
            .pop()
            .ok_or_else(|| DataFusionError::Internal("missing resolution routing decision".into()))
    }
}
