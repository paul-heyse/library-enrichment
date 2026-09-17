//! Delta-backed HTTP observations. Native plans own selection, freshness and validator reuse.
use crate::{
    blob::BlobStore,
    control::ControlStore,
    native_delta::{DeltaStore, StorageContract, transaction_conflict},
    runtime::QueryRuntime,
};
use datafusion::{
    common::ScalarValue,
    error::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::{col, lit},
};
pub use enrichment_core::http::Cached;
use enrichment_core::{
    evidence::{Artifact, ArtifactKind},
    http::{CacheRecord as Record, Fetched, cache_schema},
    native_union::NativeStruct,
};
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
        self.delta
            .open_or_create(
                "http_responses",
                &self.contract,
                true,
                &[
                    (
                        "http_status",
                        "response.status=200 OR response.status=404 OR response.status=410".into(),
                    ),
                    (
                        "http_digest",
                        "regexp_like(body_digest,'^[0-9a-f]{64}$')".into(),
                    ),
                ],
            )
            .await
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
            SELECT *, (NOT CAST($3 AS BOOLEAN) AND response.status IN (404,410) AND CAST($4 AS DOUBLE)>0
                AND clock_instant(response.retrieved_at)<=now()
                AND date_part('epoch',now())-date_part('epoch',clock_instant(response.retrieved_at))<CAST($4 AS DOUBLE)) AS reuse,
                CASE WHEN response.status=200 AND response.etag IS NOT NULL THEN 'if-none-match'
                     WHEN response.status=200 AND response.last_modified IS NOT NULL THEN 'if-modified-since' END AS validator_name,
                CASE WHEN response.status=200 THEN coalesce(response.etag,response.last_modified) END AS validator_value
            FROM http_responses WHERE request_url=$1 AND (accept IS NOT DISTINCT FROM CAST($2 AS VARCHAR))
                AND body_bytes<=$5
            ORDER BY clock_instant(response.retrieved_at) DESC,body_digest,response.status LIMIT 1
        "#).await?.with_param_values(vec![ScalarValue::from(url.as_str()), ScalarValue::Utf8(accept.map(str::to_owned)), ScalarValue::Boolean(Some(force)), ScalarValue::UInt64(Some(self.negative_ttl)), ScalarValue::UInt64(Some(self.limit))])?;
        let record = enrichment_core::native_record::record(
            Record::fields(),
            Record::fields()
                .iter()
                .flat_map(|field| [lit(field.name()), col(field.name())])
                .collect(),
        )
        .alias("record");
        let frame = frame.select(vec![
            record,
            col("reuse"),
            col("validator_name"),
            col("validator_value"),
        ])?;
        Ok(self
            .runtime
            .records(frame, 1)
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
            response.retrieved_at,
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
            Record::batch(&[record])?,
        )?
        .filter(
            col("response")
                .field("status")
                .in_list(vec![lit(200u16), lit(404u16), lit(410u16)], false),
        )?;
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
            Fetched::batch(std::slice::from_ref(fresh))?,
        )?;
        let previous = cached
            .map(|c| c.response().clone())
            .into_iter()
            .collect::<Vec<_>>();
        session.register_batch("previous_response", Fetched::batch(&previous)?)?;
        self.runtime.require_empty(session.sql("SELECT 'http_not_modified' AS witness FROM fresh_response f LEFT JOIN previous_response p ON p.status=200 WHERE f.status<>304 OR p.status IS NULL LIMIT 1").await?, "http_unsolicited_not_modified", "http_revalidation").await?;
        let frame = session.sql("SELECT CAST(200 AS SMALLINT UNSIGNED) AS status,coalesce(f.content_type,p.content_type) AS content_type,coalesce(f.etag,p.etag) AS etag,coalesce(f.last_modified,p.last_modified) AS last_modified,f.final_url,f.retrieved_at FROM fresh_response f CROSS JOIN previous_response p").await?;
        self.runtime
            .records(frame, 1)
            .await?
            .pop()
            .ok_or_else(|| DataFusionError::Internal("missing HTTP revalidation output".into()))
    }
}
