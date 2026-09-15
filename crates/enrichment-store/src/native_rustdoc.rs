//! Resource-bounded rustdoc extraction. The daemon retains a bounded record stream instead
//! of the raw language AST. Temporary producer transport is never a published evidence store.
use crate::{
    admission::AdmissionLimits,
    parquet_admission::{self, Domain, Request},
    provider::FileWitness,
};
use datafusion::error::{DataFusionError, Result};
use enrichment_core::{
    canonical,
    evidence::{Artifact, EvidenceFragment, Relationship, Symbol, ingest::ProducerSource},
    producer::normalize::{self, NormalizeInput},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
    path::{Path, PathBuf},
    sync::atomic::AtomicBool,
};

const RECORD_BYTES: usize = 1024 * 1024;
const STREAM_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Extraction {
    artifact_id: String,
    summary_chars: usize,
    output: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StreamReceipt {
    digest: String,
    bytes: u64,
    rows: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Receipt {
    crate_name: String,
    crate_version: Option<String>,
    target: String,
    producer_items: u64,
    symbols: StreamReceipt,
    relationships: StreamReceipt,
    fragments: StreamReceipt,
}

/// Finite, private extraction output. Every visit checks exact bytes and decodes one record.
pub struct Prepared {
    pub crate_name: String,
    pub crate_version: Option<String>,
    pub target: String,
    pub producer_items: u64,
    streams: Option<(tempfile::TempDir, Receipt)>,
    retention: Option<std::sync::Arc<File>>,
}
impl Prepared {
    #[must_use]
    pub fn source_only(crate_name: String, crate_version: Option<String>) -> Self {
        Self {
            crate_name,
            crate_version,
            target: String::new(),
            producer_items: 0,
            streams: None,
            retention: None,
        }
    }
}

/// Capture exact retained bytes and run extraction without keeping a raw AST in the daemon.
/// # Errors
/// Capture integrity, unavailable workers and bounded extraction failures are explicit.
pub async fn from_artifact(
    blobs: crate::BlobStore,
    artifact: Artifact,
    summary_chars: usize,
) -> Result<Prepared> {
    parquet_admission::run_blocking(move |cancelled| {
        let retention = crate::leases::shared(
            blobs
                .root()
                .parent()
                .ok_or_else(|| invalid("blob store parent missing"))?,
        )?;
        let mut capture = blobs.capture(&artifact, STREAM_BYTES)?;
        let mut json = tempfile::NamedTempFile::new_in(blobs.root().join(".staging"))?;
        io::copy(&mut capture, &mut json)?;
        json.flush()?;
        let mut result = prepare(json.path(), &artifact, summary_chars, &cancelled)?;
        result.retention = Some(retention);
        Ok(result)
    })
    .await
}

/// Execute the mandatory native worker against a verified private capture on a blocking task.
/// The caller owns the capture and staging root until the worker has been reaped.
/// # Errors
/// Missing worker, process/resource limits, changed bytes and malformed output fail explicitly.
pub fn prepare(
    path: &Path,
    artifact: &Artifact,
    summary_chars: usize,
    cancelled: &AtomicBool,
) -> Result<Prepared> {
    let directory = tempfile::Builder::new()
        .prefix("rustdoc-records-")
        .tempdir_in(
            path.parent()
                .ok_or_else(|| invalid("capture parent missing"))?,
        )?;
    let extraction = Extraction {
        artifact_id: artifact.artifact_id.clone(),
        summary_chars,
        output: directory.path().to_owned(),
    };
    let report = parquet_admission::run_request(
        Request {
            relation: Domain::Rustdoc(extraction),
            path: path.to_owned(),
            digest: artifact.sha256.clone(),
            bytes: artifact.size_bytes,
            rows: 0,
            limits: AdmissionLimits::default(),
        },
        cancelled,
    )?;
    let receipt = report
        .rustdoc
        .ok_or_else(|| invalid("native rustdoc receipt missing"))?;
    for (name, stream) in [
        ("symbols", &receipt.symbols),
        ("relationships", &receipt.relationships),
        ("fragments", &receipt.fragments),
    ] {
        let path = directory.path().join(name);
        if stream.bytes > STREAM_BYTES || stream.rows > 1_000_000 {
            return Err(invalid("native record stream exceeds bound"));
        }
        let _witness = FileWitness::read(&path)?;
        let (digest, bytes) = canonical::sha256_reader(File::open(path)?, stream.bytes)?;
        if digest != stream.digest || bytes != stream.bytes {
            return Err(invalid("native record receipt mismatch"));
        }
    }
    Ok(Prepared {
        crate_name: receipt.crate_name.clone(),
        crate_version: receipt.crate_version.clone(),
        target: receipt.target.clone(),
        producer_items: receipt.producer_items,
        streams: Some((directory, receipt)),
        retention: None,
    })
}

impl ProducerSource for Prepared {
    fn visit_symbols(
        &self,
        emit: &mut dyn FnMut(Symbol) -> std::result::Result<(), String>,
    ) -> std::result::Result<(), String> {
        if let Some((root, receipt)) = &self.streams {
            visit(&root.path().join("symbols"), &receipt.symbols, emit)?;
        }
        Ok(())
    }
    fn visit_relationships(
        &self,
        emit: &mut dyn FnMut(Relationship) -> std::result::Result<(), String>,
    ) -> std::result::Result<(), String> {
        if let Some((root, receipt)) = &self.streams {
            visit(
                &root.path().join("relationships"),
                &receipt.relationships,
                emit,
            )?;
        }
        Ok(())
    }
    fn visit_fragments(
        &self,
        emit: &mut dyn FnMut(EvidenceFragment) -> std::result::Result<(), String>,
    ) -> std::result::Result<(), String> {
        if let Some((root, receipt)) = &self.streams {
            visit(&root.path().join("fragments"), &receipt.fragments, emit)?;
        }
        Ok(())
    }
}

fn visit<T: serde::de::DeserializeOwned>(
    path: &Path,
    receipt: &StreamReceipt,
    emit: &mut dyn FnMut(T) -> std::result::Result<(), String>,
) -> std::result::Result<(), String> {
    let before = FileWitness::read(path).map_err(|e| e.to_string())?;
    canonical::verified_read(
        File::open(path).map_err(|e| e.to_string())?,
        &receipt.digest,
        receipt.bytes,
        |reader| {
            let mut rows = 0u64;
            let mut reader = io::BufReader::new(reader);
            loop {
                let mut record = Vec::new();
                let n = reader
                    .by_ref()
                    .take(RECORD_BYTES as u64 + 2)
                    .read_until(b'\n', &mut record)?;
                if n == 0 {
                    break;
                }
                if n > RECORD_BYTES + 1 || record.last() != Some(&b'\n') {
                    return Err(io::Error::other("native record framing exceeds bound"));
                }
                let value = serde_json::from_slice(&record)?;
                rows += 1;
                if rows > receipt.rows {
                    return Err(io::Error::other("native record count mismatch"));
                }
                emit(value).map_err(io::Error::other)?;
            }
            if rows != receipt.rows {
                return Err(io::Error::other("native record count mismatch"));
            }
            Ok(())
        },
    )
    .map_err(|e| e.to_string())?;
    if FileWitness::read(path).map_err(|e| e.to_string())? != before {
        return Err("native record stream changed".into());
    }
    Ok(())
}

pub(crate) fn extract(
    source: &mut File,
    extraction: &Extraction,
    request: &Request,
) -> Result<Receipt> {
    if !extraction.output.is_absolute()
        || extraction.artifact_id != enrichment_core::evidence::artifact_id_for(&request.digest)
    {
        return Err(invalid("invalid native extraction identity or output"));
    }
    let mut payload = String::new();
    source.take(STREAM_BYTES + 1).read_to_string(&mut payload)?;
    if payload.len() as u64 != request.bytes {
        return Err(invalid("native rustdoc size mismatch"));
    }
    let prepared = normalize::prepare(&NormalizeInput {
        payload: &payload,
        rustdoc_artifact_id: &extraction.artifact_id,
        json_path: Some(&request.path),
        summary_chars: extraction.summary_chars,
    })
    .map_err(|e| invalid(e.to_string()))?;
    drop(payload);
    let symbols = write_records(&extraction.output.join("symbols"), |emit| {
        prepared.visit_symbols(emit)
    })?;
    let relationships = write_records(&extraction.output.join("relationships"), |emit| {
        prepared.visit_relationships(emit)
    })?;
    let fragments = write_records(&extraction.output.join("fragments"), |emit| {
        prepared.visit_fragments(emit)
    })?;
    File::open(&extraction.output)?.sync_all()?;
    Ok(Receipt {
        crate_name: prepared.crate_name,
        crate_version: prepared.crate_version,
        target: prepared.target,
        producer_items: prepared.producer_items,
        symbols,
        relationships,
        fragments,
    })
}

fn write_records<T: Serialize>(
    path: &Path,
    produce: impl FnOnce(
        &mut dyn FnMut(T) -> std::result::Result<(), String>,
    ) -> std::result::Result<(), String>,
) -> Result<StreamReceipt> {
    let mut output = crate::dataset::BoundedFile {
        file: File::options().write(true).create_new(true).open(path)?,
        bytes: 0,
        limit: STREAM_BYTES,
    };
    let mut rows = 0u64;
    produce(&mut |value| {
        canonical::serialized_size(&value, RECORD_BYTES).map_err(|e| e.to_string())?;
        rows += 1;
        if rows > 1_000_000 {
            return Err("native record count exceeds bound".into());
        }
        serde_json::to_writer(&mut output, &value).map_err(|e| e.to_string())?;
        output.write_all(b"\n").map_err(|e| e.to_string())
    })
    .map_err(invalid)?;
    output.file.sync_all()?;
    let (digest, bytes) = canonical::sha256_reader(File::open(path)?, STREAM_BYTES)?;
    Ok(StreamReceipt {
        digest,
        bytes,
        rows,
    })
}
fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::evidence::ArtifactKind;

    #[tokio::test]
    async fn native_rustdoc_streams_reject_corruption_and_recover_after_bad_input() {
        let root = tempfile::tempdir().unwrap();
        let blobs = crate::BlobStore::open(root.path()).unwrap();
        crate::leases::initialize(root.path()).unwrap();
        let put = |bytes: &[u8]| {
            blobs
                .put(bytes, |_| {
                    Artifact::describe(
                        bytes,
                        ArtifactKind::RustdocJson,
                        "application/json",
                        "https://docs.rs/fixture",
                        "2026-09-14T00:00:00Z",
                    )
                })
                .unwrap()
                .acquired
        };
        let bad = put(br#"{"format_version":61,"root":"invalid"}"#);
        assert!(from_artifact(blobs.clone(), bad, 240).await.is_err());
        let bytes =
            include_bytes!("../../../tests/fixtures/rustdoc/enr-fixture-0.1.0-default.json");
        let prepared = from_artifact(blobs.clone(), put(bytes), 240).await.unwrap();
        let mut count = 0;
        prepared
            .visit_symbols(&mut |_| {
                count += 1;
                Ok(())
            })
            .unwrap();
        assert!(count > 5);
        assert!(crate::leases::exclusive(root.path()).is_err());
        let (directory, _) = prepared.streams.as_ref().unwrap();
        std::fs::write(directory.path().join("symbols"), b"{}\n").unwrap();
        assert!(prepared.visit_symbols(&mut |_| Ok(())).is_err());
        drop(prepared);
        assert!(crate::leases::exclusive(root.path()).is_ok());
    }
}
