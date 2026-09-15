//! Finite diagnostic barriers for real process-crash tests, absent from release builds.
//! A probe can only be armed by writing its control under the daemon-owned data root.
//! Research requests never accept a probe, arbitrary location, or command.
use std::{io, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Point {
    SnapshotFilesDurable,
    SnapshotRenamed,
    CatalogFilesDurable,
    CatalogRootDurable,
    JournalTerminalDurable,
}

/// # Errors
/// Malformed explicitly armed diagnostics or an unreleased barrier fail after a bounded wait.
pub fn hit(root: &Path, point: Point) -> io::Result<()> {
    #[cfg(debug_assertions)]
    {
        diagnostic::hit(root, point)
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = (root, point);
        Ok(())
    }
}

#[cfg(debug_assertions)]
mod diagnostic {
    use super::*;
    use std::{
        fs,
        io::{Read, Write},
        time::{Duration, Instant},
    };
    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    struct Arm {
        point: Point,
        token: String,
    }
    fn read(path: &Path) -> io::Result<Vec<u8>> {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.is_file() || metadata.len() > 512 {
            return Err(io::Error::other("invalid publication diagnostic control"));
        }
        let mut bytes = Vec::new();
        fs::File::open(path)?.take(513).read_to_end(&mut bytes)?;
        if bytes.len() > 512 {
            return Err(io::Error::other("publication diagnostic control grew"));
        }
        Ok(bytes)
    }
    pub(super) fn hit(root: &Path, point: Point) -> io::Result<()> {
        let directory = root.join(".publication-probe");
        match fs::symlink_metadata(&directory) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e),
            Ok(m) if !m.is_dir() => {
                return Err(io::Error::other(
                    "publication probe must be a physical data-root directory",
                ));
            }
            Ok(_) => {}
        }
        let bytes = match read(&directory.join("armed.json")) {
            Ok(v) => v,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e),
        };
        let arm: Arm = serde_json::from_slice(&bytes)?;
        if arm.point != point {
            return Ok(());
        }
        if arm.token.len() != 32 || !arm.token.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err(io::Error::other(
                "publication diagnostic token must be 32 hex digits",
            ));
        }
        let mut reached = match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(directory.join("reached.json"))
        {
            Ok(v) => v,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => return Ok(()),
            Err(e) => return Err(e),
        };
        reached.write_all(&serde_json::to_vec(&arm)?)?;
        reached.sync_all()?;
        fs::File::open(&directory)?.sync_all()?;
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            match read(&directory.join("release")) {
                Ok(bytes) if bytes == arm.token.as_bytes() => return Ok(()),
                Ok(_) => {
                    return Err(io::Error::other(
                        "publication diagnostic release token differs",
                    ));
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(e),
            }
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "publication diagnostic barrier exceeded 60 seconds",
                ));
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
