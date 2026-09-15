//! Private executable boundary for native Parquet parsing, never a selectable query engine.
//!
//! The pinned decoder can allocate from untrusted page/footer lengths before returning an
//! error. A separate process installs address-space/CPU limits before parsing. Only a complete
//! successful decode admits bytes to the ordinary native DataFusion provider.

use crate::{
    admission::{AdmissionLimits, Relation},
    catalog_generation::Table,
    provider::FileWitness,
};
use datafusion::error::{DataFusionError, Result};
use enrichment_core::canonical;
use parquet::arrow::arrow_reader::{
    ArrowReaderMetadata, ArrowReaderOptions, ParquetRecordBatchReaderBuilder,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read, Seek, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

const PROTOCOL: &str = "native-admission/2";
const CONTROL_BYTES: usize = 16 * 1024;
const ADDRESS_BYTES: u64 = 1024 * 1024 * 1024;
// Serializes actual decoder processes, including tasks whose callers were cancelled.
static DECODER: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum Domain {
    Evidence(Relation),
    Catalog(Table),
    Rustdoc(crate::native_rustdoc::Extraction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Request {
    pub relation: Domain,
    pub path: PathBuf,
    pub digest: String,
    pub bytes: u64,
    pub rows: u64,
    pub limits: AdmissionLimits,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Report {
    protocol: String,
    request_digest: String,
    rows: u64,
    row_groups: usize,
    metadata_bytes: usize,
    peak_batch_bytes: usize,
    pub(crate) rustdoc: Option<crate::native_rustdoc::Receipt>,
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

/// No user/PATH-selected executable. Installed binaries are siblings; Cargo test executables
/// live one level below their package binaries in `deps`.
fn executable() -> io::Result<PathBuf> {
    let current = std::env::current_exe()?;
    let mut directory = current
        .parent()
        .ok_or_else(|| io::Error::other("no executable directory"))?;
    if directory.file_name().is_some_and(|n| n == "deps") {
        directory = directory
            .parent()
            .ok_or_else(|| io::Error::other("no Cargo binary directory"))?;
    }
    let path = directory.join("library-enrichment-native-worker");
    FileWitness::read(&path).map_err(|e| {
        io::Error::other(format!(
            "native admission executable {}: {e}; build/install the workspace binaries",
            path.display()
        ))
    })?;
    Ok(path)
}

struct Running(Child);
impl Drop for Running {
    fn drop(&mut self) {
        // Covers errors, cancellation of the owning blocking task, and the wall deadline.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(test)]
fn validate(request: Request) -> Result<()> {
    validate_cancellable(request, &AtomicBool::new(false))
}

struct CancelOnDrop(Arc<AtomicBool>);
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

pub(crate) async fn run_blocking<T: Send + 'static>(
    work: impl FnOnce(Arc<AtomicBool>) -> Result<T> + Send + 'static,
) -> Result<T> {
    let flag = Arc::new(AtomicBool::new(false));
    let _cancel = CancelOnDrop(Arc::clone(&flag));
    tokio::task::spawn_blocking(move || work(flag))
        .await
        .map_err(|e| invalid(e.to_string()))?
}

pub(crate) fn validate_cancellable(request: Request, cancelled: &AtomicBool) -> Result<()> {
    run_request(request, cancelled).map(|_| ())
}

pub(crate) fn run_request(request: Request, cancelled: &AtomicBool) -> Result<Report> {
    let started = Instant::now();
    // Waiting tasks cannot launch fresh decoders after their own admission deadline.
    let guard = loop {
        if cancelled.load(Ordering::Acquire) || started.elapsed() >= request.limits.deadline {
            return Err(DataFusionError::ResourcesExhausted(
                "native decoder queue cancelled or deadline exceeded".into(),
            ));
        }
        match DECODER.try_lock() {
            Ok(guard) => break guard,
            Err(std::sync::TryLockError::WouldBlock) => {
                std::thread::sleep(Duration::from_millis(2))
            }
            Err(_) => return Err(invalid("native decoder coordinator poisoned")),
        }
    };
    canonical::serialized_size(&request, CONTROL_BYTES).map_err(|e| invalid(e.to_string()))?;
    let bytes = serde_json::to_vec(&request).map_err(|e| invalid(e.to_string()))?;
    let mut child = Running(
        Command::new(executable()?)
            .env_clear()
            .current_dir("/")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?,
    );
    child
        .0
        .stdin
        .take()
        .ok_or_else(|| invalid("missing decoder input"))?
        .write_all(&bytes)?;
    let output = supervise(child, started, request.limits.deadline, cancelled)?;
    let report: Report = serde_json::from_slice(&output).map_err(|e| invalid(e.to_string()))?;
    if report.protocol != PROTOCOL
        || report.request_digest != canonical::sha256_hex(&bytes)
        || report.rows != request.rows
    {
        return Err(invalid("native decoder report identity mismatch"));
    }
    drop(guard);
    Ok(report)
}

// Supervision is shared with tests that exercise actual exit, timeout and pipe behavior.
// There is no executable selection in the production interface.
fn supervise(
    mut child: Running,
    started: Instant,
    deadline: Duration,
    cancelled: &AtomicBool,
) -> Result<Vec<u8>> {
    let source = child
        .0
        .stdout
        .take()
        .ok_or_else(|| invalid("missing decoder output"))?;
    let overflow = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&overflow);
    let output = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        source
            .take(CONTROL_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > CONTROL_BYTES {
            flag.store(true, Ordering::Release);
        }
        Ok::<_, io::Error>(bytes)
    });
    let status = loop {
        if cancelled.load(Ordering::Acquire) || started.elapsed() >= deadline {
            break Err(DataFusionError::ResourcesExhausted(
                "native decoder cancelled or wall deadline exceeded".into(),
            ));
        }
        if overflow.load(Ordering::Acquire) {
            break Err(invalid("native decoder report too large"));
        }
        match child.0.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => std::thread::sleep(Duration::from_millis(2)),
            Err(e) => break Err(e.into()),
        }
    };
    // Closing/reaping the native child closes its output; always join the bounded drain thread.
    drop(child);
    let bytes = output
        .join()
        .map_err(|_| invalid("native decoder output reader panicked"))??;
    if bytes.len() > CONTROL_BYTES {
        return Err(invalid("native decoder report too large"));
    }
    let status = status?;
    if !status.success() {
        return Err(invalid(format!(
            "native admission rejected input or exceeded process limits ({status})"
        )));
    }
    Ok(bytes)
}

/// Entry point for the dedicated trusted native validator executable. Never call from the daemon.
/// # Errors
/// Unsupported process limits, malformed control input, native decode or domain errors reject.
pub fn worker_main() -> Result<()> {
    process_limits()?;
    let mut bytes = Vec::new();
    io::stdin()
        .take(CONTROL_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > CONTROL_BYTES {
        return Err(invalid("native decoder request too large"));
    }
    let request: Request = serde_json::from_slice(&bytes).map_err(|e| invalid(e.to_string()))?;
    let mut report = decode(&request)?;
    report.request_digest = canonical::sha256_hex(&bytes);
    serde_json::to_writer(io::stdout().lock(), &report).map_err(|e| invalid(e.to_string()))?;
    Ok(())
}

fn decode(request: &Request) -> Result<Report> {
    let limits = &request.limits;
    if request.bytes > limits.file_bytes
        || request.rows > limits.table_rows as u64
        || limits.batch_rows == 0
        || limits.batch_bytes == 0
        || limits.record_bytes == 0
        || limits.row_group_bytes <= 0
        || !request.path.is_absolute()
    {
        return Err(invalid("invalid native admission request bounds"));
    }
    let witness = FileWitness::read(&request.path)?;
    let mut options = File::options();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x20000 | 0x800); // O_NOFOLLOW | O_NONBLOCK
    }
    let mut source = options.open(&request.path)?;
    if !source.metadata()?.is_file() {
        return Err(invalid("native input is not a regular file"));
    }
    let (digest, count) = canonical::sha256_reader(&mut source, limits.file_bytes)?;
    if digest != request.digest || count != request.bytes {
        return Err(invalid("native input digest mismatch"));
    }
    source.rewind()?;
    if let Domain::Rustdoc(extraction) = &request.relation {
        let receipt = crate::native_rustdoc::extract(&mut source, extraction, request)?;
        if FileWitness::read(&request.path)? != witness {
            return Err(invalid("native rustdoc input changed"));
        }
        return Ok(Report {
            protocol: PROTOCOL.into(),
            request_digest: String::new(),
            rows: 0,
            row_groups: 0,
            metadata_bytes: 0,
            peak_batch_bytes: 0,
            rustdoc: Some(receipt),
        });
    }
    let metadata = ArrowReaderMetadata::load(&source, ArrowReaderOptions::default())?;
    let schema = match &request.relation {
        Domain::Evidence(r) => r.schema()?,
        Domain::Catalog(t) => t.schema()?,
        Domain::Rustdoc(_) => return Err(invalid("rustdoc reached Parquet decoding")),
    };
    if metadata.schema() != &schema
        || u64::try_from(metadata.metadata().file_metadata().num_rows()).ok() != Some(request.rows)
    {
        return Err(invalid(
            "physical schema, semantic metadata or row count mismatch",
        ));
    }
    let mut report = Report {
        protocol: PROTOCOL.into(),
        request_digest: String::new(),
        rows: 0,
        row_groups: metadata.metadata().num_row_groups(),
        metadata_bytes: metadata.metadata().memory_size(),
        peak_batch_bytes: 0,
        rustdoc: None,
    };
    if report.metadata_bytes > 16 * 1024 * 1024 || report.row_groups > 1024 {
        return Err(invalid("native metadata exceeds budget"));
    }
    for group in 0..report.row_groups {
        let rg = metadata.metadata().row_group(group);
        if rg.total_byte_size() < 0 || rg.total_byte_size() > limits.row_group_bytes {
            return Err(invalid(
                "uncompressed Parquet row group exceeds byte budget",
            ));
        }
        // Native readers never combine adjacent groups into a larger batch. A row bound is not
        // an allocation ceiling: the process limit protects footer/page/dictionary expansion.
        let reader = ParquetRecordBatchReaderBuilder::new_with_metadata(
            source.try_clone()?,
            metadata.clone(),
        )
        .with_row_groups(vec![group])
        .with_batch_size(limits.batch_rows)
        .build()?;
        let mut group_bytes = 0usize;
        for batch in reader {
            let batch = batch?;
            let bytes = batch.get_array_memory_size();
            report.peak_batch_bytes = report.peak_batch_bytes.max(bytes);
            group_bytes = group_bytes
                .checked_add(bytes)
                .ok_or_else(|| invalid("decoded group byte overflow"))?;
            if bytes > limits.batch_bytes || group_bytes > limits.row_group_bytes as usize {
                return Err(invalid(
                    "decoded Arrow batch or row group exceeds byte budget",
                ));
            }
            match &request.relation {
                Domain::Evidence(r) => r.validate(&batch, limits.record_bytes)?,
                Domain::Catalog(t) => t.validate(&batch)?,
                Domain::Rustdoc(_) => return Err(invalid("rustdoc reached Parquet validation")),
            }
            report.rows = report
                .rows
                .checked_add(batch.num_rows() as u64)
                .ok_or_else(|| invalid("row count overflow"))?;
        }
    }
    if report.rows != request.rows || FileWitness::read(&request.path)? != witness {
        return Err(invalid("native input changed or row count mismatch"));
    }
    Ok(report)
}

#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn process_limits() -> io::Result<()> {
    #[repr(C)]
    struct Limit {
        current: u64,
        maximum: u64,
    }
    unsafe extern "C" {
        fn getrlimit(resource: i32, limit: *mut Limit) -> i32;
        fn setrlimit(resource: i32, limit: *const Limit) -> i32;
    }
    for (resource, cap) in [(9, ADDRESS_BYTES), (0, 30), (4, 0)] {
        // AS, CPU, CORE
        let mut limit = Limit {
            current: 0,
            maximum: 0,
        };
        // SAFETY: Linux x86_64/aarch64 rlim_t is u64; storage is live and correctly aligned.
        // Runs after exec on the single worker thread, before any untrusted Parquet parsing.
        if unsafe { getrlimit(resource, &raw mut limit) } != 0 {
            return Err(io::Error::last_os_error());
        }
        limit.current = limit.current.min(cap);
        limit.maximum = limit.maximum.min(cap);
        if unsafe { setrlimit(resource, &raw const limit) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let mut actual = Limit {
            current: 0,
            maximum: 0,
        };
        if unsafe { getrlimit(resource, &raw mut actual) } != 0 {
            return Err(io::Error::last_os_error());
        }
        if actual.current != limit.current || actual.maximum != limit.maximum {
            return Err(io::Error::other("native process limits were not installed"));
        }
    }
    Ok(())
}
#[cfg(not(all(target_os = "linux", target_pointer_width = "64")))]
fn process_limits() -> io::Result<()> {
    Err(io::Error::other(
        "native decoder allocation limits require Linux",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::evidence::{Symbol, SymbolKind, relational::Definition};
    use parquet::arrow::ArrowWriter;

    fn request(path: PathBuf, rows: u64) -> Request {
        let (digest, bytes) =
            canonical::sha256_reader(File::open(&path).expect("input"), 16 * 1024 * 1024)
                .expect("hash");
        Request {
            relation: Domain::Evidence(Relation::Definitions),
            path,
            digest,
            bytes,
            rows,
            limits: AdmissionLimits::default(),
        }
    }

    #[test]
    fn hostile_footer_allocation_is_confined_to_native_worker() {
        let root = tempfile::tempdir().expect("root");
        let path = root.path().join("hostile.parquet");
        // Test-only compact-Thrift attack bytes: version 1, schema list of 2^31-1 structs.
        // The pinned native parser attempts this list allocation before reading its elements.
        // Never parse this fixture in the test/daemon process.
        let footer: &[u8] = &[0x15, 2, 0x19, 0xfc, 0xff, 0xff, 0xff, 0xff, 7, 0];
        let mut bytes = b"PAR1".to_vec();
        bytes.extend_from_slice(footer);
        bytes.extend_from_slice(&(footer.len() as u32).to_le_bytes());
        bytes.extend_from_slice(b"PAR1");
        std::fs::write(&path, bytes).expect("write");
        let failure = validate(request(path, 0)).expect_err("hostile allocation cannot admit");
        assert!(failure.to_string().contains("signal: 6"), "{failure}");
        // A failed decoder releases the coordinator and cannot damage the next native parse.
        let valid = root.path().join("valid.parquet");
        let batch = crate::projection::definitions(&[]).expect("schema");
        ArrowWriter::try_new(File::create(&valid).expect("file"), batch.schema(), None)
            .expect("writer")
            .close()
            .expect("close");
        validate(request(valid, 0)).expect("parent and following native worker survive");
    }

    #[test]
    fn native_decoder_isolates_row_groups_before_arrow_batching() {
        let root = tempfile::tempdir().expect("root");
        let path = root.path().join("groups.parquet");
        let rows = (0..64)
            .map(|i| {
                let path = format!("p::{}_{i}", "s".repeat(2048));
                Definition {
                    definition_id: Symbol::definition_id_for(
                        "p",
                        &path,
                        SymbolKind::Function,
                        None,
                    ),
                    kind: SymbolKind::Function,
                    definition_path: path,
                    defined_in_package: "p".into(),
                    qualifier: None,
                }
            })
            .collect::<Vec<_>>();
        let batch = crate::projection::definitions(&rows).expect("batch");
        let mut writer =
            ArrowWriter::try_new(File::create(&path).expect("file"), batch.schema(), None)
                .expect("writer");
        writer.write(&batch.slice(0, 32)).expect("group 1");
        writer.flush().expect("flush 1");
        writer.write(&batch.slice(32, 32)).expect("group 2");
        writer.close().expect("close");
        let metadata = ArrowReaderMetadata::load(
            &File::open(&path).expect("input"),
            ArrowReaderOptions::default(),
        )
        .expect("metadata");
        let isolated = (0..2)
            .map(|group| {
                ParquetRecordBatchReaderBuilder::new_with_metadata(
                    File::open(&path).expect("input"),
                    metadata.clone(),
                )
                .with_row_groups(vec![group])
                .with_batch_size(1024)
                .build()
                .expect("reader")
                .next()
                .expect("batch")
                .expect("decode")
                .get_array_memory_size()
            })
            .max()
            .expect("peak");
        let joined = ParquetRecordBatchReaderBuilder::new_with_metadata(
            File::open(&path).expect("input"),
            metadata,
        )
        .with_batch_size(1024)
        .build()
        .expect("reader")
        .next()
        .expect("batch")
        .expect("decode");
        assert_eq!(joined.num_rows(), 64);
        assert!(joined.get_array_memory_size() > isolated);
        let mut input = request(path, 64);
        input.limits.batch_bytes = isolated;
        validate(input).expect("selected groups obey the actual Arrow buffer bound");
    }

    #[test]
    fn decoder_supervision_rejects_overflow_abnormal_exit_and_reaps_timeout() {
        for (program, args, expected) in [
            (
                "/usr/bin/head",
                vec!["-c", "65536", "/dev/zero"],
                "report too large",
            ),
            (
                "/bin/sh",
                vec!["-c", "printf '{}'; exit 9"],
                "exit status: 9",
            ),
            ("/bin/sleep", vec!["5"], "wall deadline"),
        ] {
            let child = Command::new(program)
                .args(args)
                .env_clear()
                .current_dir("/")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("real child");
            let pid = child.id();
            let failure = supervise(
                Running(child),
                Instant::now(),
                Duration::from_millis(100),
                &AtomicBool::new(false),
            )
            .expect_err("invalid or stalled output");
            assert!(failure.to_string().contains(expected), "{failure}");
            assert!(
                !std::path::Path::new(&format!("/proc/{pid}")).exists(),
                "child reaped"
            );
        }
    }

    #[tokio::test]
    async fn dropped_admission_future_cancels_and_reaps_running_decoder() {
        let (ready, running) = tokio::sync::oneshot::channel();
        let (done, completion) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(run_blocking(move |cancel| {
            let child = Command::new("/bin/sleep")
                .arg("5")
                .env_clear()
                .current_dir("/")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?;
            let pid = child.id();
            let _ = ready.send(pid);
            let result = supervise(
                Running(child),
                Instant::now(),
                Duration::from_secs(10),
                &cancel,
            );
            let _ = done.send(result.is_err());
            Ok(())
        }));
        let pid = running.await.expect("started child");
        task.abort();
        assert!(task.await.expect_err("aborted caller").is_cancelled());
        assert!(
            tokio::time::timeout(Duration::from_secs(1), completion)
                .await
                .expect("prompt cancellation")
                .expect("completion")
        );
        assert!(
            !std::path::Path::new(&format!("/proc/{pid}")).exists(),
            "child reaped after caller drop"
        );
    }
}
