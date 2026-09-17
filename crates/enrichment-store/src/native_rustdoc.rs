//! Owned, externally bounded rustdoc syntax extraction into native Arrow fact inputs.
use crate::{
    native_worker::{self, Request},
    provider::FileWitness,
};
use arrow::ipc::{reader::StreamReader, writer::StreamWriter};
use datafusion::error::{DataFusionError, Result};
use enrichment_core::{
    canonical,
    evidence::Artifact,
    producer::rustdoc::facts::{self, Fact},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Extraction {
    artifact_id: String,
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
    streams: BTreeMap<Fact, StreamReceipt>,
}

pub(crate) struct Captured {
    pub directory: Arc<tempfile::TempDir>,
    pub retention: Arc<File>,
    pub producer_revision: String,
}

/// The mandatory worker owns decoding and rendering allocations. Native admission and
/// public reachability start only after its exact Arrow output and EOS receipts are checked.
pub async fn from_artifact(
    runtime: &crate::runtime::QueryRuntime,
    blobs: crate::BlobStore,
    artifact: Artifact,
    summary_chars: usize,
) -> Result<crate::rust_normalize::RustFacts> {
    let captured = native_worker::run_blocking(move |cancelled| {
        let retention = crate::leases::shared(
            blobs
                .root()
                .parent()
                .ok_or_else(|| invalid("blob store parent missing"))?,
        )?;
        let mut capture = blobs.capture(&artifact, facts::MAX_BYTES)?;
        let mut json = tempfile::NamedTempFile::new_in(blobs.root().join(".staging"))?;
        io::copy(&mut capture, &mut json)?;
        json.flush()?;
        let (directory, producer_revision) = prepare(json.path(), &artifact, &cancelled)?;
        Ok(Captured {
            directory: Arc::new(directory),
            retention,
            producer_revision,
        })
    })
    .await?;
    Box::pin(crate::rust_normalize::RustFacts::open(
        runtime,
        captured,
        summary_chars,
    ))
    .await
}

fn prepare(
    path: &Path,
    artifact: &Artifact,
    cancelled: &AtomicBool,
) -> Result<(tempfile::TempDir, String)> {
    let directory = tempfile::Builder::new()
        .prefix("rustdoc-facts-")
        .tempdir_in(
            path.parent()
                .ok_or_else(|| invalid("capture parent missing"))?,
        )?;
    let report = native_worker::run_request(
        Request {
            extraction: Extraction {
                artifact_id: artifact.artifact_id.clone(),
                output: directory.path().to_owned(),
            },
            path: path.to_owned(),
            digest: artifact.sha256.clone(),
            bytes: artifact.size_bytes,
            deadline: std::time::Duration::from_secs(30),
        },
        cancelled,
    )?;
    validate_receipt(directory.path(), &report.rustdoc)?;
    Ok((directory, report.producer_revision))
}

fn validate_receipt(directory: &Path, receipt: &Receipt) -> Result<()> {
    if receipt.streams.len() != Fact::ALL.len() {
        return Err(invalid("rustdoc fact inventory"));
    }
    for fact in Fact::ALL {
        let stream = receipt
            .streams
            .get(&fact)
            .ok_or_else(|| invalid("rustdoc fact missing"))?;
        if !(8..=facts::MAX_BYTES).contains(&stream.bytes) || stream.rows > facts::MAX_ROWS {
            return Err(invalid("rustdoc fact stream bound"));
        }
        let path = directory.join(fact.name()).with_extension("arrow");
        let witness = FileWitness::read(&path)?;
        let (digest, bytes) = canonical::sha256_reader(File::open(&path)?, facts::MAX_BYTES)?;
        if digest != stream.digest || bytes != stream.bytes {
            return Err(invalid("rustdoc fact receipt mismatch"));
        }
        let mut file = File::open(&path)?;
        file.seek(SeekFrom::End(-8))?;
        let mut eos = [0; 8];
        file.read_exact(&mut eos)?;
        file.rewind()?;
        if eos != [255, 255, 255, 255, 0, 0, 0, 0] {
            return Err(invalid("rustdoc fact EOS missing"));
        }
        let mut reader = StreamReader::try_new(&mut file, None)?;
        if reader.schema() != fact.schema() {
            return Err(invalid("rustdoc fact schema mismatch"));
        }
        let mut rows = 0;
        for batch in &mut reader {
            let batch = batch?;
            if batch.num_rows() > facts::BATCH_ROWS
                || datafusion::common::utils::memory::get_record_batch_memory_size(&batch)
                    > facts::BATCH_BYTES + 65536
            {
                return Err(invalid("rustdoc fact batch allocation bound"));
            }
            for array in batch.columns() {
                array.to_data().validate_full()?;
            }
            rows += batch.num_rows() as u64;
        }
        drop(reader);
        if rows != stream.rows
            || file.stream_position()? != bytes
            || FileWitness::read(&path)? != witness
        {
            return Err(invalid("rustdoc fact framing or witness mismatch"));
        }
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
    source
        .take(facts::MAX_BYTES + 1)
        .read_to_string(&mut payload)?;
    if payload.len() as u64 != request.bytes {
        return Err(invalid("native rustdoc size mismatch"));
    }
    let mut outputs = BTreeMap::new();
    for fact in Fact::ALL {
        let output = crate::dataset::BoundedFile {
            file: File::options()
                .write(true)
                .create_new(true)
                .open(extraction.output.join(fact.name()).with_extension("arrow"))?,
            bytes: 0,
            limit: facts::MAX_BYTES,
        };
        outputs.insert(
            fact,
            (StreamWriter::try_new(output, &fact.schema())?, 0_u64),
        );
    }
    facts::extract(&request.path, &payload, &mut |fact, batch| {
        let (writer, rows) = outputs.get_mut(&fact).ok_or_else(|| {
            arrow::error::ArrowError::InvalidArgumentError("fact writer missing".into())
        })?;
        *rows += batch.num_rows() as u64;
        writer.write(&batch)
    })?;
    let mut streams = BTreeMap::new();
    for (fact, (mut writer, rows)) in outputs {
        writer.finish()?;
        writer.get_ref().file.sync_all()?;
        drop(writer);
        let path = extraction.output.join(fact.name()).with_extension("arrow");
        let (digest, bytes) = canonical::sha256_reader(File::open(path)?, facts::MAX_BYTES)?;
        streams.insert(
            fact,
            StreamReceipt {
                digest,
                bytes,
                rows,
            },
        );
    }
    File::open(&extraction.output)?.sync_all()?;
    Ok(Receipt { streams })
}
fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
