//! Admission of retained execution coordinates against actual content-addressed UTF-8 bytes.
use crate::{BlobStore, projection, runtime::QueryRuntime};
use datafusion::{
    error::{DataFusionError, Result},
    prelude::SessionContext,
};
use enrichment_core::{canonical, evidence::execution::*};
use std::{
    collections::BTreeMap,
    io::Read,
    sync::{Arc, Mutex},
};

struct Documents {
    blobs: BlobStore,
    inputs: BTreeMap<String, (String, u64)>,
    texts: BTreeMap<String, Arc<str>>,
    bytes: usize,
}
impl Documents {
    fn text(&mut self, id: &str) -> Result<Arc<str>> {
        if let Some(text) = self.texts.get(id) {
            return Ok(text.clone());
        }
        let (digest, expected) = self
            .inputs
            .get(id)
            .ok_or_else(|| invalid("execution document is outside its input closure"))?;
        if *expected > 1024 * 1024 {
            return Err(invalid(
                "execution document exceeds the 1 MiB validation bound",
            ));
        }
        let file = std::fs::File::open(self.blobs.path_for(digest))?;
        let mut bytes = Vec::new();
        file.take(expected + 1).read_to_end(&mut bytes)?;
        if bytes.len() as u64 != *expected || canonical::sha256_hex(&bytes) != *digest {
            return Err(invalid(
                "execution document bytes disagree with their digest",
            ));
        }
        let text: Arc<str> = String::from_utf8(bytes)
            .map_err(|e| invalid(e.to_string()))?
            .into();
        if self.bytes + text.len() > 16 * 1024 * 1024 || self.texts.len() >= 256 {
            self.texts.clear();
            self.bytes = 0;
        }
        self.bytes += text.len();
        self.texts.insert(id.into(), text.clone());
        Ok(text)
    }
    fn range(&mut self, id: &str, range: Utf8Range) -> Result<()> {
        let text = self.text(id)?;
        range.validate().map_err(invalid)?;
        range.start.validate(&text).map_err(invalid)?;
        range.end.validate(&text).map_err(invalid)
    }
    fn validate(&mut self, observation: &ExecutionObservation) -> Result<()> {
        let (digest, _) = self
            .inputs
            .get(&observation.source.artifact_id)
            .ok_or_else(|| invalid("canonical execution result is not retained"))?;
        if canonical::sha256_hex(&observation.payload.canonical_bytes().map_err(invalid)?)
            != *digest
        {
            return Err(invalid(
                "retained execution payload differs from its canonical result blob",
            ));
        }
        match &observation.payload {
            ExecutionPayload::SemanticQuery(q) => {
                let text = self.text(&q.document_artifact_id)?;
                if let Some(position) = q.position {
                    position.validate(&text).map_err(invalid)?;
                }
                for diagnostic in &q.diagnostics {
                    self.range(&q.document_artifact_id, diagnostic.range)?;
                }
                for target in &q.locations {
                    if let ExecutionTarget::Artifact { artifact_id, range } = target {
                        self.range(artifact_id, *range)?;
                    }
                }
            }
            ExecutionPayload::UsageProbe(q) => {
                self.text(&q.snippet_artifact_id)?;
            }
            ExecutionPayload::RuntimeObject(_) => {}
        }
        Ok(())
    }
}

pub async fn validate(
    runtime: &QueryRuntime,
    session: &SessionContext,
    blobs: BlobStore,
    max_rows: usize,
) -> Result<()> {
    let mut inputs = BTreeMap::new();
    let output = runtime
        .execute(
            session
                .sql("SELECT DISTINCT artifact_id, sha256, size_bytes FROM input_artifacts")
                .await?,
        )
        .await?;
    for batch in output.batches {
        use arrow::array::Array;
        let ids = projection::TextColumn::new(batch.column(0).as_ref())?;
        let hashes = projection::TextColumn::new(batch.column(1).as_ref())?;
        let sizes = batch
            .column(2)
            .as_any()
            .downcast_ref::<arrow::array::UInt64Array>()
            .ok_or_else(|| invalid("input size projection has an invalid physical type"))?;
        for i in 0..batch.num_rows() {
            if sizes.is_null(i) {
                return Err(invalid("input size cannot be null"));
            }
            let id = ids.required(i)?.to_owned();
            let descriptor = (hashes.required(i)?.to_owned(), sizes.value(i));
            if inputs
                .insert(id, descriptor.clone())
                .is_some_and(|prior| prior != descriptor)
            {
                return Err(invalid(
                    "an artifact handle names conflicting execution inputs",
                ));
            }
        }
    }
    let documents = Arc::new(Mutex::new(Documents {
        blobs,
        inputs,
        texts: BTreeMap::new(),
        bytes: 0,
    }));
    runtime
        .visit_async(
            session.table("execution_observations").await?,
            max_rows,
            |batch| {
                let documents = documents.clone();
                async move {
                    tokio::task::spawn_blocking(move || {
                        let mut documents = documents
                            .lock()
                            .map_err(|_| invalid("document validation cache poisoned"))?;
                        for observation in projection::execution::decode(&batch)? {
                            documents.validate(&observation)?;
                        }
                        Ok(())
                    })
                    .await
                    .map_err(|e| invalid(e.to_string()))?
                }
            },
        )
        .await?;
    Ok(())
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
