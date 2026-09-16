//! Delta-backed HTTP observations. Native plans own selection, freshness and validator reuse.
use crate::{
    blob::BlobStore,
    control::ControlStore,
    control_jobs::encode,
    native_delta::{DeltaStore, StorageContract, missing_table, transaction_conflict},
    registry::rows,
    runtime::QueryRuntime,
};
use datafusion::{
    common::ScalarValue,
    error::{DataFusionError, Result},
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{Artifact, ArtifactKind},
    http::{Fetched, cache_schema, response_schema},
};
use serde::{Deserialize, Serialize};
use std::{io::Read, path::Path, sync::Arc};

#[derive(Clone)]
pub struct HttpCache {
    runtime: QueryRuntime,
    delta: DeltaStore,
    contract: StorageContract,
    blobs: BlobStore,
    catalog: ControlStore,
    limit: u64,
    negative_ttl: u64,
    initialization: Arc<tokio::sync::Mutex<()>>,
}

#[derive(Serialize, Deserialize)]
struct Record {
    #[serde(flatten)]
    response: Fetched,
    request_url: String,
    accept: Option<String>,
    body_digest: String,
    body_bytes: u64,
}

#[derive(Deserialize)]
pub struct Cached {
    #[serde(flatten)]
    record: Record,
    pub reuse: bool,
    pub validator_name: Option<String>,
    pub validator_value: Option<String>,
}

impl Cached {
    pub fn response(&self) -> &Fetched {
        &self.record.response
    }
    fn artifact(&self) -> Artifact {
        // Decode the admitted cache's flat physical descriptor. No state selection occurs here.
        Artifact {
            artifact_id: enrichment_core::evidence::artifact_id_for(&self.record.body_digest),
            sha256: self.record.body_digest.clone(),
            size_bytes: self.record.body_bytes,
            media_type: self
                .record
                .response
                .content_type
                .clone()
                .unwrap_or_else(|| "application/octet-stream".into()),
            kind: ArtifactKind::Other,
            source_uri: self.record.request_url.clone(),
            final_url: Some(self.record.response.final_url.clone()),
            retrieved_at: self.record.response.retrieved_at.clone(),
            etag: self.record.response.etag.clone(),
            last_modified: self.record.response.last_modified.clone(),
            compression: None,
        }
    }
}

impl HttpCache {
    pub fn new(
        data_root: &Path,
        runtime: &QueryRuntime,
        limit: u64,
        negative_ttl: u64,
    ) -> Result<Self> {
        Ok(Self {
            runtime: runtime.clone(),
            delta: DeltaStore::new(&data_root.join("delta"), runtime.clone())?,
            contract: StorageContract::new(cache_schema())?,
            blobs: BlobStore::open(data_root)?,
            catalog: ControlStore::open(data_root, runtime.clone())?,
            limit,
            negative_ttl,
            initialization: Arc::new(tokio::sync::Mutex::new(())),
        })
    }
    async fn table(&self) -> Result<deltalake::DeltaTable> {
        let _initialization = self.initialization.lock().await;
        self.delta.prepare_root("http_responses")?;
        match self.delta.load("http_responses", None).await {
            Ok(table) => Ok(table),
            Err(error) if missing_table(&error) => {
                match self
                    .delta
                    .create_with_rules(
                        "http_responses",
                        &self.contract,
                        true,
                        &[
                            ("http_status", "status IN (200,404,410)".into()),
                            (
                                "http_digest",
                                "regexp_like(body_digest,'^[0-9a-f]{64}$')".into(),
                            ),
                            (
                                "http_timestamp",
                                "try_cast(retrieved_at AS TIMESTAMP) IS NOT NULL".into(),
                            ),
                        ],
                    )
                    .await
                {
                    Ok(table) => Ok(table),
                    Err(error) if transaction_conflict(&error) => {
                        self.delta.load("http_responses", None).await
                    }
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
    pub async fn select(
        &self,
        url: &url::Url,
        accept: Option<&str>,
        force: bool,
    ) -> Result<Option<Cached>> {
        let table = self
            .table()
            .await
            .map_err(|e| e.context("HTTP response table"))?;
        let session = self.runtime.session();
        crate::native_catalog::work(
            &session,
            "http_responses",
            self.delta.provider(&table, &self.contract).await?,
        )?;
        let frame = session.sql(r#"
            SELECT *, (NOT CAST($3 AS BOOLEAN) AND status IN (404,410) AND CAST($4 AS DOUBLE)>0
                AND try_cast(retrieved_at AS TIMESTAMP)<=now()
                AND date_part('epoch',now())-date_part('epoch',try_cast(retrieved_at AS TIMESTAMP))<CAST($4 AS DOUBLE)) AS reuse,
                CASE WHEN status=200 AND etag IS NOT NULL THEN 'if-none-match'
                     WHEN status=200 AND last_modified IS NOT NULL THEN 'if-modified-since' END AS validator_name,
                CASE WHEN status=200 THEN coalesce(etag,last_modified) END AS validator_value
            FROM http_responses WHERE request_url=$1 AND (accept IS NOT DISTINCT FROM CAST($2 AS VARCHAR))
                AND body_bytes<=$5
            ORDER BY try_cast(retrieved_at AS TIMESTAMP) DESC,body_digest,status LIMIT 1
        "#).await?.with_param_values(vec![ScalarValue::from(url.as_str()), ScalarValue::Utf8(accept.map(str::to_owned)), ScalarValue::Boolean(Some(force)), ScalarValue::UInt64(Some(self.negative_ttl)), ScalarValue::UInt64(Some(self.limit))])?;
        Ok(rows(&self.runtime, frame, 1)
            .await
            .map_err(|e| e.context("HTTP response selection"))?
            .pop())
    }
    pub async fn body(&self, cached: &Cached) -> Result<Vec<u8>> {
        let artifact = cached.artifact();
        let blobs = self.blobs.clone();
        let limit = self.limit;
        Ok(self
            .runtime
            .blocking(move || {
                let mut file = blobs.capture(&artifact, limit)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                Ok::<_, std::io::Error>(bytes)
            })
            .await??)
    }
    pub async fn record(
        &self,
        url: &url::Url,
        accept: Option<&str>,
        response: &Fetched,
    ) -> Result<()> {
        let artifact = Artifact::describe(
            &response.bytes,
            ArtifactKind::Other,
            response
                .content_type
                .as_deref()
                .unwrap_or("application/octet-stream"),
            url.as_str(),
            &response.retrieved_at,
        );
        let record = Record {
            response: response.metadata(),
            request_url: url.to_string(),
            accept: accept.map(str::to_owned),
            body_digest: artifact.sha256.clone(),
            body_bytes: artifact.size_bytes,
        };
        let frame = crate::native_catalog::batch(
            &self.runtime.session(),
            "http_observation",
            encode(cache_schema(), &[record])?,
        )?
        .filter(col("status").in_list(vec![lit(200u16), lit(404u16), lit(410u16)], false))?;
        if self.runtime.execute(frame.clone()).await?.rows == 0 {
            return Ok(());
        }
        let bytes = response.bytes.clone();
        let blobs = self.blobs.clone();
        let mut artifact = artifact;
        artifact.final_url = Some(response.final_url.clone());
        artifact.etag = response.etag.clone();
        artifact.last_modified = response.last_modified.clone();
        let stored = self
            .runtime
            .blocking(move || blobs.put(&bytes, |_| artifact))
            .await??;
        self.catalog
            .retain_artifacts(&self.runtime, &[stored.acquired])
            .await?;
        for _ in 0..16 {
            let table = self.table().await?;
            match self
                .delta
                .append(table, &self.contract, frame.clone(), vec![])
                .await
            {
                Ok(_) => return Ok(()),
                Err(error) if transaction_conflict(&error) => continue,
                Err(error) => return Err(error),
            }
        }
        Err(DataFusionError::Execution(
            "HTTP response commit contention".into(),
        ))
    }
    pub async fn revalidated(&self, fresh: &Fetched, cached: Option<&Cached>) -> Result<Fetched> {
        let session = self.runtime.session();
        session.register_batch(
            "fresh_response",
            encode(response_schema(), std::slice::from_ref(fresh))?,
        )?;
        let previous = cached
            .map(|c| c.response().clone())
            .into_iter()
            .collect::<Vec<_>>();
        session.register_batch("previous_response", encode(response_schema(), &previous)?)?;
        self.runtime.require_empty(session.sql("SELECT 'http_not_modified' AS witness FROM fresh_response f LEFT JOIN previous_response p ON p.status=200 WHERE f.status<>304 OR p.status IS NULL LIMIT 1").await?, "http_unsolicited_not_modified", "http_revalidation").await?;
        let frame = session.sql("SELECT CAST(200 AS SMALLINT UNSIGNED) AS status,coalesce(f.content_type,p.content_type) AS content_type,coalesce(f.etag,p.etag) AS etag,coalesce(f.last_modified,p.last_modified) AS last_modified,f.final_url,f.retrieved_at FROM fresh_response f CROSS JOIN previous_response p").await?;
        rows(&self.runtime, frame, 1)
            .await?
            .pop()
            .ok_or_else(|| DataFusionError::Internal("missing HTTP revalidation output".into()))
    }
}
