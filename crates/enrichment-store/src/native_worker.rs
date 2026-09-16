//! Bounded physical worker for exact rustdoc decoding. No relational storage decoder lives here.

use crate::provider::FileWitness;
use datafusion::error::{DataFusionError, Result};
use enrichment_core::canonical;
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

const PROTOCOL: &str = "native-rustdoc-arrow/2";
const CONTROL_BYTES: usize = 16 * 1024;
const ADDRESS_BYTES: u64 = 1024 * 1024 * 1024;
// Serializes actual decoder processes, including tasks whose callers were cancelled.
static DECODER: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Request {
    pub extraction: crate::native_rustdoc::Extraction,
    pub path: PathBuf,
    pub digest: String,
    pub bytes: u64,
    pub deadline: Duration,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Report {
    protocol: String,
    request_digest: String,
    pub(crate) producer_revision: String,
    pub(crate) rustdoc: crate::native_rustdoc::Receipt,
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
    let operation = crate::runtime::capture_operation();
    tokio::task::spawn_blocking(move || operation.run(|| work(flag)))
        .await
        .map_err(|e| invalid(e.to_string()))?
}

pub(crate) fn run_request(request: Request, cancelled: &AtomicBool) -> Result<Report> {
    let started = Instant::now();
    // Waiting tasks cannot launch fresh decoders after their own admission deadline.
    let guard = loop {
        if cancelled.load(Ordering::Acquire) || started.elapsed() >= request.deadline {
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
    let output = supervise(child, started, request.deadline, cancelled)?;
    let report: Report = serde_json::from_slice(&output).map_err(|e| invalid(e.to_string()))?;
    if report.protocol != PROTOCOL || report.request_digest != canonical::sha256_hex(&bytes) {
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
    if request.bytes > enrichment_core::producer::rustdoc::facts::MAX_BYTES
        || request.deadline.is_zero()
        || !request.path.is_absolute()
    {
        return Err(invalid("invalid native decoder request bounds"));
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
    let (digest, count) = canonical::sha256_reader(
        &mut source,
        enrichment_core::producer::rustdoc::facts::MAX_BYTES,
    )?;
    if digest != request.digest || count != request.bytes {
        return Err(invalid("native input digest mismatch"));
    }
    source.rewind()?;
    let receipt = crate::native_rustdoc::extract(&mut source, &request.extraction, request)?;
    if FileWitness::read(&request.path)? != witness {
        return Err(invalid("native rustdoc input changed"));
    }
    Ok(Report {
        protocol: PROTOCOL.into(),
        request_digest: String::new(),
        producer_revision: crate::runtime::DEFINITION_REVISION.into(),
        rustdoc: receipt,
    })
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
