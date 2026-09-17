//! Bounded operating-system observations. Native plans decide whether ownership ended.
use enrichment_core::operation::retention::{NativeProcess, ProcessObservation};
use std::io::{self, Read};

fn text(path: impl AsRef<std::path::Path>, limit: u64) -> io::Result<String> {
    let mut value = String::new();
    std::fs::File::open(path)?
        .take(limit + 1)
        .read_to_string(&mut value)?;
    if value.len() as u64 > limit || value.trim().is_empty() {
        return Err(io::Error::other("invalid process observation size"));
    }
    Ok(value.trim().into())
}

fn start_ticks(pid: u32) -> io::Result<u64> {
    let stat = text(format!("/proc/{pid}/stat"), 8192)?;
    // comm is parenthesized and may itself contain spaces, ')' and newlines.
    let (_, fields) = stat
        .rsplit_once(')')
        .ok_or_else(|| io::Error::other("invalid proc process stat"))?;
    fields
        .split_whitespace()
        .nth(19)
        .ok_or_else(|| io::Error::other("process start field missing"))?
        .parse()
        .map_err(io::Error::other)
}

pub(crate) fn current() -> io::Result<NativeProcess> {
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::MetadataExt;
        let namespace = std::fs::metadata("/proc/self/ns/pid")?;
        let pid = std::process::id();
        Ok(NativeProcess {
            machine: text("/etc/machine-id", 256)?,
            boot: text("/proc/sys/kernel/random/boot_id", 256)?,
            pid_namespace: format!("{}:{}", namespace.dev(), namespace.ino()),
            pid,
            start_ticks: start_ticks(pid)?,
        })
    }
    #[cfg(not(target_os = "linux"))]
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "native process identity requires Linux procfs",
    ))
}

pub(crate) fn observe(process: NativeProcess, observer: &NativeProcess) -> ProcessObservation {
    let (present, ticks) = match start_ticks(process.pid) {
        Ok(ticks) => (Some(true), Some(ticks)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => (Some(false), None),
        // Permission, parsing and I/O failures never become evidence of absence.
        Err(_) => (None, None),
    };
    ProcessObservation {
        process,
        observer: observer.clone(),
        present,
        start_ticks: ticks,
    }
}
