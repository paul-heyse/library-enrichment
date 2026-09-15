//! Linux capsule executor. Never spawns target code outside a private PID namespace.
use enrichment_core::{
    canonical,
    capsule_protocol::{
        self as protocol, Frame, Mode, Operation,
        inventory::{self, Entry},
    },
    execution::ProcessEnd,
};
use std::{
    fs,
    io::{self, Read, Write},
    os::unix::{fs::PermissionsExt, process::CommandExt},
    path::Path,
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

// Linux ABI; verified 2026-09-14 against kill(2), wait(2), pid_namespaces(7):
// https://man7.org/linux/man-pages/man2/kill.2.html
// https://man7.org/linux/man-pages/man2/wait.2.html
// PID 1 is mandatory before any target spawn or namespace-wide signal.
unsafe extern "C" {
    fn prctl(option: i32, ...) -> i32;
    fn kill(pid: i32, signal: i32) -> i32;
    fn waitpid(pid: i32, status: *mut i32, options: i32) -> i32;
}

fn main() {
    if let Err(error) = execute() {
        eprintln!("executor rejected operation: {error}");
        std::process::exit(125);
    }
}

fn execute() -> io::Result<()> {
    if std::process::id() != 1
        || std::env::args()
            .collect::<Vec<_>>()
            .get(1..)
            .is_none_or(|args| args != ["--operation=/operation.json"])
        || fs::read_link("/proc/self")? != Path::new("1")
    {
        return Err(io::Error::other(
            "executor requires PID 1 in an isolated Linux container",
        ));
    }
    // SAFETY: Linux PR_SET_DUMPABLE=4 with unsigned-long zero prevents target processes
    // from opening PID 1's protocol descriptors through procfs or changing its memory.
    // Verified 2026-09-14: PR_SET_DUMPABLE(2const), proc_pid_fd(5). No pointer arguments.
    if unsafe { prctl(4, 0usize, 0usize, 0usize, 0usize) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let mut bytes = Vec::new();
    fs::File::open("/operation.json")?
        .take(protocol::HEADER_LIMIT as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > protocol::HEADER_LIMIT {
        return Err(io::Error::other("operation too large"));
    }
    let operation: Operation = serde_json::from_slice(&bytes)?;
    operation.validate()?;
    let started = Instant::now();
    let deadline = Duration::from_millis(operation.deadline_millis);
    initialize(&operation)?;
    if started.elapsed() >= deadline {
        return Err(io::Error::other("input initialization exceeded deadline"));
    }
    let mut command = Command::new(&operation.argv[0]);
    command.args(&operation.argv[1..]).current_dir("/capsule");
    if operation.mode == Mode::LanguageServer {
        return Err(command.exec());
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut sink = io::stdout().lock();
    protocol::write_frame(
        &mut sink,
        &Frame::Start {
            version: protocol::VERSION,
            operation_id: operation.id(),
        },
    )?;
    let mut child = command.spawn()?;
    let overflow = Arc::new(AtomicBool::new(false));
    let stdout = bounded(
        child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("missing target stdout"))?,
        operation.output_bytes,
        overflow.clone(),
    );
    let stderr = bounded(
        child
            .stderr
            .take()
            .ok_or_else(|| io::Error::other("missing target stderr"))?,
        operation.output_bytes,
        overflow.clone(),
    );
    let (mut end, exit_code) = loop {
        if overflow.load(Ordering::Acquire) {
            break (ProcessEnd::OutputLimit, None);
        }
        if started.elapsed() >= deadline {
            break (ProcessEnd::Deadline, None);
        }
        if let Some(status) = child.try_wait()? {
            break (ProcessEnd::Exited, status.code());
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    // Reap the direct child before waitpid(any), preserving its actual exit status.
    if end != ProcessEnd::Exited {
        if let Err(error) = child.kill()
            && child.try_wait()?.is_none()
        {
            return Err(error);
        }
        child.wait()?;
    }
    quiesce(started, deadline)?;
    let stdout = stdout
        .join()
        .map_err(|_| io::Error::other("stdout reader panicked"))??;
    let stderr = stderr
        .join()
        .map_err(|_| io::Error::other("stderr reader panicked"))??;
    if overflow.load(Ordering::Acquire) {
        end = ProcessEnd::OutputLimit;
    }
    let exit_code = if end == ProcessEnd::Exited {
        exit_code
    } else {
        None
    };
    let entries = if end == ProcessEnd::Exited && exit_code == Some(0) {
        inventory::selected(
            Path::new("/capsule"),
            &operation.outputs,
            operation.data_bytes,
        )?
    } else {
        inventory::Inventory::new()
    };
    for (path, entry) in &entries {
        protocol::write_frame(
            &mut sink,
            &Frame::Entry {
                path: path.clone(),
                entry: entry.clone(),
            },
        )?;
        if let Entry::File { bytes, sha256, .. } = entry {
            let mut file = fs::File::open(Path::new("/capsule").join(path))?;
            let (actual, size) = canonical::sha256_reader(&mut file, *bytes)?;
            if actual != *sha256 || size != *bytes {
                return Err(io::Error::other("output changed after quiescence"));
            }
            use std::io::Seek;
            file.rewind()?;
            if io::copy(&mut file.take(*bytes), &mut sink)? != *bytes {
                return Err(io::Error::other("truncated output"));
            }
        }
    }
    protocol::write_frame(
        &mut sink,
        &Frame::Complete {
            operation_id: operation.id(),
            inventory_digest: canonical::digest_hex(&serde_json::json!(entries)),
            exit_code,
            end,
            stdout,
            stderr,
        },
    )?;
    // Host validates completion and removes the container. Preserve tmpfs until then.
    sink.flush()?;
    loop {
        std::thread::park();
    }
}

fn initialize(operation: &Operation) -> io::Result<()> {
    let inputs = Path::new("/inputs");
    if inventory::capture(inputs, operation.data_bytes)? != operation.inputs {
        return Err(io::Error::other("admitted input inventory mismatch"));
    }
    for (path, entry) in &operation.inputs {
        let destination = Path::new("/capsule").join(path);
        match entry {
            Entry::Directory { .. } => fs::create_dir(&destination)?,
            Entry::File { bytes, .. } => {
                if fs::copy(inputs.join(path), &destination)? != *bytes {
                    return Err(io::Error::other("input changed during copy"));
                }
                fs::set_permissions(
                    destination,
                    fs::Permissions::from_mode(entry.mode() | 0o600),
                )?;
            }
        }
    }
    for path in [
        ".executor",
        ".executor/home",
        ".executor/tmp",
        ".executor/uv",
    ] {
        let directory = Path::new("/capsule").join(path);
        fs::create_dir(&directory)?;
        fs::set_permissions(directory, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

fn bounded(
    mut source: impl Read + Send + 'static,
    limit: usize,
    overflow: Arc<AtomicBool>,
) -> std::thread::JoinHandle<io::Result<Vec<u8>>> {
    std::thread::spawn(move || {
        let mut result = Vec::new();
        let mut buffer = [0u8; 8192];
        loop {
            let read = source.read(&mut buffer)?;
            if read == 0 {
                return Ok(result);
            }
            let keep = read.min(limit.saturating_sub(result.len()));
            result.extend_from_slice(&buffer[..keep]);
            if keep < read {
                overflow.store(true, Ordering::Release);
            }
        }
    })
}

fn quiesce(started: Instant, deadline: Duration) -> io::Result<()> {
    if std::process::id() != 1 {
        return Err(io::Error::other(
            "refusing namespace signal outside executor PID 1",
        ));
    }
    let quiescence = Instant::now();
    loop {
        // SAFETY: Linux pid_t/int are i32. No pointer is passed. Mandatory PID-1 guard
        // restricts -1 to this container's descendant namespace, never the host.
        unsafe {
            kill(-1, 9);
        }
        loop {
            let mut status = 0i32;
            // SAFETY: live writable i32 storage; -1 waits for any child, WNOHANG=1
            // makes this nonblocking. This is the only remaining child-reaping thread.
            let result = unsafe { waitpid(-1, &mut status, 1) };
            if result <= 0 {
                break;
            }
        }
        let mut others = false;
        for entry in fs::read_dir("/proc")? {
            let name = entry?.file_name();
            if name
                .to_str()
                .and_then(|s| s.parse::<u32>().ok())
                .is_some_and(|id| id != 1)
            {
                others = true;
                break;
            }
        }
        if !others {
            return Ok(());
        }
        if started.elapsed() >= deadline || quiescence.elapsed() >= Duration::from_secs(2) {
            return Err(io::Error::other(
                "target descendants did not become quiescent",
            ));
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
