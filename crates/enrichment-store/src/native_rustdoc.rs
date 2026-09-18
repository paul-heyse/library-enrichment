//! Owned, externally bounded rustdoc syntax extraction into native Arrow fact inputs.
use crate::{native_worker, provider::FileWitness};
use arrow::ipc::{reader::StreamReader, writer::StreamWriter};
use datafusion::error::{DataFusionError, Result};
use enrichment_core::{
    canonical,
    evidence::Artifact,
    execution::rustdoc_decoder::{Report, Request, StreamReceipt},
    producer::rustdoc::facts::{self, Fact},
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
    sync::Arc,
};

pub(crate) struct Captured {
    pub directory: Arc<crate::PrivateDirectory>,
    pub producer_revision: String,
}
pub(crate) use enrichment_core::execution::rustdoc_decoder::INPUT_FILE;

/// The mandatory worker owns decoding and rendering allocations. Native admission and
/// public reachability start only after its exact Arrow output and EOS receipts are checked.
pub async fn from_artifact(
    runtime: &crate::runtime::QueryRuntime,
    retention: crate::retention::RetentionStore,
    blobs: crate::BlobStore,
    artifact: Artifact,
    summary_chars: usize,
) -> Result<crate::rust_normalize::RustFacts> {
    let directory = crate::PrivateDirectory::create(
        &retention,
        runtime,
        crate::private_directory::Kind::Rustdoc,
    )
    .await?;
    let protection = retention
        .enroll(
            format!("rustdoc-input/{}", uuid::Uuid::new_v4()),
            crate::retention::ProtectionKind::Query,
            vec![crate::retention::Dependency::Artifact {
                artifact_id: artifact.artifact_id.clone(),
            }],
        )
        .await?;
    let held = directory.clone();
    let input = artifact.clone();
    runtime
        .blocking(move || {
            // Exact protection follows the callback through cancellation; subsequent parsing
            // owns its independent, durably enrolled private bytes.
            let _protection = protection;
            let mut capture = blobs.capture(&input, facts::MAX_BYTES)?;
            let mut output = File::options()
                .write(true)
                .create_new(true)
                .open(held.path().join(INPUT_FILE))?;
            io::copy(&mut capture, &mut output)?;
            output.sync_all()?;
            File::open(held.path())?.sync_all()?;
            Ok::<_, DataFusionError>(())
        })
        .await??;
    let report = native_worker::run_request(
        runtime,
        crate::rustdoc_decoder_plan::request(runtime, &artifact, directory.path()).await?,
        directory.clone(),
    )
    .await?;
    let held = directory.clone();
    let producer_revision = report.producer_revision.clone();
    runtime
        .blocking(move || validate_receipt(held.path(), &report))
        .await??;
    Box::pin(crate::rust_normalize::RustFacts::open(
        runtime,
        Captured {
            directory,
            producer_revision,
        },
        summary_chars,
    ))
    .await
}

fn validate_receipt(directory: &Path, receipt: &Report) -> Result<()> {
    for stream in &receipt.streams {
        let path = directory.join(stream.fact.name()).with_extension("arrow");
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
        if reader.schema() != stream.fact.schema() {
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

pub(crate) fn extract(source: &mut File, request: &Request) -> Result<Vec<StreamReceipt>> {
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
                .open(request.root.join(fact.name()).with_extension("arrow"))?,
            bytes: 0,
            limit: facts::MAX_BYTES,
        };
        outputs.insert(
            fact,
            (StreamWriter::try_new(output, &fact.schema())?, 0_u64),
        );
    }
    facts::extract(&request.input(), &payload, &mut |fact, batch| {
        let (writer, rows) = outputs.get_mut(&fact).ok_or_else(|| {
            arrow::error::ArrowError::InvalidArgumentError("fact writer missing".into())
        })?;
        *rows += batch.num_rows() as u64;
        writer.write(&batch)
    })?;
    let mut streams = Vec::with_capacity(Fact::ALL.len());
    for (fact, (mut writer, rows)) in outputs {
        writer.finish()?;
        writer.get_ref().file.sync_all()?;
        drop(writer);
        let path = request.root.join(fact.name()).with_extension("arrow");
        let (digest, bytes) = canonical::sha256_reader(File::open(path)?, facts::MAX_BYTES)?;
        streams.push(StreamReceipt {
            fact,
            digest,
            bytes,
            rows,
        });
    }
    File::open(&request.root)?.sync_all()?;
    Ok(streams)
}
fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
