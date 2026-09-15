//! Durable replacement of one small service-owned record. Multi-file visibility belongs to the catalog coordinator.
use std::{
    fs, io,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Write `bytes` to `path` atomically: a sibling temp file, then a rename over the target.
///
/// # Errors
///
/// Fails on I/O error; the target is never left half-written.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("path has no parent"))?;
    fs::create_dir_all(parent)?;
    let n = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let temp = parent.join(format!(
        ".{}.{}-{n}.tmp",
        path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("record"),
        std::process::id()
    ));
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(&temp);
        return Err(error);
    }
    if let Err(err) = fs::rename(&temp, path) {
        let _ = fs::remove_file(&temp);
        return Err(err);
    }
    fs::File::open(parent)?.sync_all()
}
