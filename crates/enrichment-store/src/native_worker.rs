//! Bounded physical worker for exact rustdoc decoding. No relational storage decoder lives here.

use crate::provider::FileWitness;
use datafusion::error::{DataFusionError, Result};
use enrichment_core::{
    canonical,
    execution::rustdoc_decoder::{
        CONTROL_BYTES, DEADLINE_SECONDS, MEMORY_BYTES, PROTOCOL, Report, Request, STDERR_BYTES,
    },
};
use std::{
    fs::File,
    io::{self, Read, Seek},
    path::PathBuf,
    process::Stdio,
    sync::Arc,
    time::Duration,
};

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

/// Input/output ownership is enrolled before the request crosses stdin. The fixed sibling
/// executable and matching compiled report remain the native decoder identity boundary.
pub(crate) async fn run_request(
    runtime: &crate::runtime::QueryRuntime,
    request: Request,
    directory: Arc<crate::PrivateDirectory>,
) -> Result<Report> {
    if request.root != directory.path() {
        return Err(invalid(
            "native decoder requires its exact private input/output root",
        ));
    }
    let held = directory.clone();
    runtime
        .blocking(move || held.directory(crate::private_directory::Kind::Rustdoc, held.path()))
        .await??;
    let parent = crate::native_effect::authorize().await?;
    let executable = runtime.blocking(executable).await??;
    let prepared =
        Arc::new(crate::rustdoc_decoder_plan::prepare(parent, request, executable).await?);
    let request = &prepared.value.request;
    let deadline = tokio::time::Instant::now()
        + Duration::from_secs(request.deadline_seconds).min(runtime.remaining()?);
    let permit = tokio::time::timeout_at(deadline, runtime.parser_permit())
        .await
        .map_err(|_| invalid("native decoder admission deadline"))?
        .map_err(|e| invalid(e.to_string()))?;
    canonical::serialized_size(&request, CONTROL_BYTES).map_err(|e| invalid(e.to_string()))?;
    let bytes = enrichment_core::json_output::JsonOutput::serialize(
        &runtime.session().runtime_env().memory_pool,
        &request,
    )?;
    prepared.parent.check().await?;
    let memory = crate::static_worker::parser_memory(runtime, prepared.value.memory_bytes)?;
    let mut command = tokio::process::Command::new(&prepared.value.executable);
    command
        .args(&prepared.value.argv)
        .env_clear()
        .envs(&prepared.value.environment)
        .current_dir(&prepared.value.cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = crate::static_worker::OwnedChild::start(
        runtime,
        &mut command,
        (directory.clone(), permit, memory, prepared.clone()),
    )?;
    let pid = child
        .child()
        .id()
        .ok_or_else(|| invalid("native decoder PID missing"))?;
    tokio::time::timeout_at(deadline, directory.protect_child(pid))
        .await
        .map_err(|_| invalid("native decoder enrollment deadline"))??;
    let output = tokio::select! {
        value = supervise(child, bytes.as_str().as_bytes(), deadline) => value?,
        error = prepared.parent.revoked() => return Err(error),
    };
    let report: Report = serde_json::from_slice(&output).map_err(|e| invalid(e.to_string()))?;
    crate::rustdoc_decoder_plan::admit_report(
        runtime,
        &report,
        &canonical::sha256_hex(bytes.as_str().as_bytes()),
    )
    .await?;
    prepared.parent.check().await?;
    Ok(report)
}

// Bounded protocol mechanics shared with isolated pipe/exit/cancellation tests.
async fn supervise(
    mut owned: crate::static_worker::OwnedChild,
    input: &[u8],
    deadline: tokio::time::Instant,
) -> Result<Vec<u8>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let child = owned.child();
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| invalid("missing decoder input"))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| invalid("missing decoder output"))?
        .take(CONTROL_BYTES as u64 + 1);
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| invalid("missing decoder diagnostics"))?
        .take(STDERR_BYTES as u64 + 1);
    let mut output = Vec::new();
    let mut diagnostics = Vec::new();
    let run = async {
        let (_, _, _, status) = tokio::try_join!(
            async {
                stdin.write_all(input).await?;
                drop(stdin);
                Ok::<_, io::Error>(())
            },
            async {
                stdout.read_to_end(&mut output).await?;
                if output.len() > CONTROL_BYTES {
                    return Err(io::Error::other("native decoder report too large"));
                }
                Ok(())
            },
            async {
                stderr.read_to_end(&mut diagnostics).await?;
                if diagnostics.len() > STDERR_BYTES {
                    return Err(io::Error::other("native decoder diagnostic byte bound"));
                }
                Ok(())
            },
            child.wait(),
        )?;
        Ok::<_, io::Error>(status)
    };
    let status = tokio::time::timeout_at(deadline, run)
        .await
        .map_err(|_| invalid("native decoder wall deadline exceeded"))??;
    if !status.success() {
        return Err(invalid(format!(
            "native decoder rejected input ({status}): {}",
            String::from_utf8_lossy(&diagnostics)
        )));
    }
    Ok(output)
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
    let mut output =
        enrichment_core::native_json::BoundedWriter::new(io::stdout().lock(), CONTROL_BYTES);
    serde_json::to_writer(&mut output, &report).map_err(|e| invalid(e.to_string()))?;
    Ok(())
}

fn decode(request: &Request) -> Result<Report> {
    if request.protocol != PROTOCOL
        || request.bytes == 0
        || request.bytes > enrichment_core::producer::rustdoc::facts::MAX_BYTES
        || !(1..=DEADLINE_SECONDS).contains(&request.deadline_seconds)
        || !request.root.is_absolute()
        || request.artifact_id != enrichment_core::evidence::artifact_id_for(&request.sha256)
    {
        return Err(invalid("invalid native decoder request bounds"));
    }
    let path = request.input();
    let witness = FileWitness::read(&path)?;
    let mut options = File::options();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x20000 | 0x800); // O_NOFOLLOW | O_NONBLOCK
    }
    let mut source = options.open(&path)?;
    if !source.metadata()?.is_file() {
        return Err(invalid("native input is not a regular file"));
    }
    let (digest, count) = canonical::sha256_reader(
        &mut source,
        enrichment_core::producer::rustdoc::facts::MAX_BYTES,
    )?;
    if digest != request.sha256 || count != request.bytes {
        return Err(invalid("native input digest mismatch"));
    }
    source.rewind()?;
    let streams = crate::native_rustdoc::extract(&mut source, request)?;
    if FileWitness::read(&path)? != witness {
        return Err(invalid("native rustdoc input changed"));
    }
    Ok(Report {
        protocol: PROTOCOL.into(),
        request_digest: String::new(),
        producer_revision: crate::runtime::DEFINITION_REVISION.into(),
        streams,
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
    for (resource, cap) in [(9, MEMORY_BYTES), (0, DEADLINE_SECONDS), (4, 0)] {
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
    fn child(
        runtime: &crate::runtime::QueryRuntime,
        program: &str,
        args: &[&str],
        owner: impl Send + Sync + 'static,
    ) -> crate::static_worker::OwnedChild {
        let mut command = tokio::process::Command::new(program);
        command
            .args(args)
            .env_clear()
            .current_dir("/")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        crate::static_worker::OwnedChild::start(runtime, &mut command, owner).unwrap()
    }
    #[tokio::test]
    async fn decoder_supervision_rejects_overflow_abnormal_exit_and_reaps_timeout() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
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
            let spawned = child(&runtime, program, &args, ());
            let failure = supervise(
                spawned,
                b"",
                tokio::time::Instant::now() + Duration::from_millis(100),
            )
            .await
            .unwrap_err();
            assert!(failure.to_string().contains(expected), "{failure}");
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }
    #[tokio::test]
    async fn dropped_admission_future_cancels_and_reaps_running_decoder() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(
            root.path(),
            crate::runtime::QueryLimits {
                concurrency: 1,
                native: enrichment_core::config::NativeQueryConfig {
                    parser_concurrency: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;
        let permit = runtime.parser_permit().await?;
        let pool = runtime.session().runtime_env().memory_pool.clone();
        let baseline = pool.reserved();
        let memory = crate::static_worker::parser_memory(&runtime, MEMORY_BYTES)?;
        assert_eq!(pool.reserved(), baseline + MEMORY_BYTES as usize);
        let mut spawned = child(&runtime, "/bin/sleep", &["30"], (permit, memory));
        let pid = spawned.child().id().unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(20), runtime.parser_permit())
                .await
                .is_err()
        );
        assert_eq!(
            runtime
                .execute(runtime.session().sql("SELECT 1 AS value").await?)
                .await?
                .rows,
            1
        );
        let task = tokio::spawn(supervise(
            spawned,
            b"",
            tokio::time::Instant::now() + Duration::from_secs(60),
        ));
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        runtime.close_diagnostics().await?;
        assert!(runtime.parser_permit().await.is_err());
        assert!(!std::path::Path::new(&format!("/proc/{pid}")).exists());
        assert_eq!(pool.reserved(), baseline);
        Ok(())
    }
}
