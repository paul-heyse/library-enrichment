//! Durable conservative reservations for unmounted output quarantine.
use std::{
    fs, io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

pub struct Reservation {
    pub root: PathBuf,
    record: PathBuf,
    lock: PathBuf,
}

/// A host preparation allocation. All first-party extraction/writes use its remaining bound.
/// The reservation outlives this builder's work; on drop the actual capsule remains counted.
pub struct Preparation {
    root: PathBuf,
    limit: u64,
    _reservation: Reservation,
}
impl Preparation {
    pub fn acquire(cache: &Path, root: &Path, limit: u64, budget: u64) -> io::Result<Self> {
        Ok(Self {
            root: root.to_owned(),
            limit,
            _reservation: Reservation::acquire(cache, limit, budget)?,
        })
    }
    fn used(&self) -> io::Result<u64> {
        bytes(&self.root)
    }
    pub fn archive_policy(&self) -> io::Result<enrichment_core::policy::ArchivePolicy> {
        let remaining = self
            .limit
            .checked_sub(self.used()?)
            .ok_or_else(|| io::Error::other("preparation exceeds reserved storage"))?;
        let mut policy = enrichment_core::policy::ArchivePolicy::default();
        policy.max_total_bytes = remaining.min(policy.max_total_bytes);
        policy.max_entry_bytes = remaining.min(policy.max_entry_bytes);
        Ok(policy)
    }
    pub fn write(&self, path: &Path, content: impl AsRef<[u8]>) -> io::Result<()> {
        let relative = path.strip_prefix(&self.root).map_err(io::Error::other)?;
        enrichment_core::capsule_protocol::validate_path(
            relative
                .to_str()
                .ok_or_else(|| io::Error::other("invalid preparation path"))?,
        )?;
        let old = match fs::symlink_metadata(path) {
            Ok(meta) if meta.is_file() && !meta.is_symlink() => meta.len(),
            Ok(_) => {
                return Err(io::Error::other(
                    "preparation write target is not a regular file",
                ));
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => 0,
            Err(e) => return Err(e),
        };
        let requested = content.as_ref().len() as u64;
        if self
            .used()?
            .checked_sub(old)
            .and_then(|n| n.checked_add(requested))
            .is_none_or(|n| n > self.limit)
        {
            return Err(io::Error::other(
                "preparation write exceeds its reserved byte bound",
            ));
        }
        fs::write(path, content)
    }
}

fn locked(path: &Path) -> io::Result<fs::File> {
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)?;
    file.lock()?;
    Ok(file)
}

fn bytes(path: &Path) -> io::Result<u64> {
    if !path.try_exists()? {
        return Ok(0);
    }
    // Includes manifests and lock files. Unknown or unreadable content refuses admission.
    super::inventory::capture(path, u64::MAX)?
        .values()
        .try_fold(0u64, |sum, entry| {
            sum.checked_add(entry.bytes())
                .ok_or_else(|| io::Error::other("storage count overflow"))
        })
}

impl Reservation {
    pub fn acquire(cache: &Path, requested: u64, budget: u64) -> io::Result<Self> {
        fs::create_dir_all(cache)?;
        let lock = cache.join("capsule-storage.lock");
        let _guard = locked(&lock)?;
        let reservations = cache.join("capsule-reservations");
        fs::create_dir_all(&reservations)?;
        let mut reserved = 0u64;
        for entry in fs::read_dir(&reservations)? {
            let path = entry?.path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let value: serde_json::Value = serde_json::from_slice(&fs::read(path)?)?;
            let count = value["bytes"]
                .as_u64()
                .ok_or_else(|| io::Error::other("invalid durable reservation"))?;
            reserved = reserved
                .checked_add(count)
                .ok_or_else(|| io::Error::other("reservation overflow"))?;
        }
        // Count on-disk quarantines too: this conservatively double-counts occupied bytes
        // while their reservation is alive, and never loses orphaned bytes after a crash.
        let occupied = bytes(&cache.join("capsules"))?
            .checked_add(bytes(&cache.join("handoff"))?)
            .ok_or_else(|| io::Error::other("storage overflow"))?;
        if occupied
            .checked_add(reserved)
            .and_then(|n| n.checked_add(requested))
            .is_none_or(|n| n > budget)
        {
            return Err(io::Error::other(
                "capsule storage capacity exhausted; manually clean regenerable capsules before retrying",
            ));
        }
        let id = uuid::Uuid::new_v4().simple().to_string();
        let record = reservations.join(format!("{id}.json"));
        let root = cache.join("handoff").join(&id);
        enrichment_store::atomic::write_atomic(
            &record,
            &serde_json::to_vec(
                &serde_json::json!({"version":1,"bytes":requested,"quarantine":root}),
            )?,
        )?;
        fs::File::open(&record)?.sync_all()?;
        fs::File::open(reservations)?.sync_all()?;
        fs::create_dir_all(&root)?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        Ok(Self { root, record, lock })
    }
}

impl Drop for Reservation {
    fn drop(&mut self) {
        // This tree was never mounted. Deletion is safe even if container cleanup is pending.
        // Keep the durable reservation whenever deletion fails; token drop frees no bytes.
        let result = (|| -> io::Result<()> {
            let _guard = locked(&self.lock)?;
            match fs::remove_dir_all(&self.root) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(e),
            }
            fs::remove_file(&self.record)?;
            fs::File::open(
                self.record
                    .parent()
                    .ok_or_else(|| io::Error::other("reservation parent missing"))?,
            )?
            .sync_all()
        })();
        if let Err(error) = result {
            eprintln!("library-enrichmentd: retained storage reservation: {error}");
        }
    }
}

/// Reconcile crashed host reservations after the cache owner has confirmed container absence.
/// The caller must hold `.execution-owner.lock`; no new builder can appear while this runs.
pub(crate) fn recover_orphans(cache: &Path) -> io::Result<()> {
    let _storage = enrichment_store::state::exclusive(cache, "capsule-storage.lock")?;
    let records = cache.join("capsule-reservations");
    let handoff = cache.join("handoff");
    let mut removals = Vec::new();
    for root in [&records, &handoff] {
        if !root.try_exists()? {
            continue;
        }
        if !fs::symlink_metadata(root)?.is_dir() {
            return Err(io::Error::other(
                "reservation root is not a physical directory",
            ));
        }
        for entry in fs::read_dir(root)? {
            if removals.len() >= 10_000 {
                return Err(io::Error::other(
                    "orphan reservation count exceeds recovery bound",
                ));
            }
            let path = entry?.path();
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| io::Error::other("invalid reservation filename"))?;
            if root == &records && atomic_reservation_temporary(name) {
                let meta = fs::symlink_metadata(&path)?;
                if !meta.is_file() || meta.len() > 4096 {
                    return Err(io::Error::other("invalid interrupted reservation write"));
                }
                removals.push(path);
                continue;
            }
            let id = if root == &records {
                name.strip_suffix(".json")
                    .ok_or_else(|| io::Error::other("unknown reservation file"))?
            } else {
                name
            };
            if id.len() != 32 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err(io::Error::other("invalid reservation identity"));
            }
            if root == &records {
                let meta = fs::symlink_metadata(&path)?;
                if !meta.is_file() || meta.len() > 4096 {
                    return Err(io::Error::other("invalid reservation record"));
                }
                let value: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
                if value["version"] != 1
                    || value["bytes"].as_u64().is_none()
                    || value["quarantine"].as_str().map(Path::new)
                        != Some(handoff.join(id).as_path())
                {
                    return Err(io::Error::other("reservation ownership mismatch"));
                }
            } else {
                // Validate every entry before deleting any reservation. These inputs were
                // never mounted, and the startup execution owner excludes a surviving writer.
                super::inventory::capture(&path, u64::MAX)?;
            }
            removals.push(path);
        }
    }
    // Remove quarantine bytes before releasing their accounting journals. A failed deletion
    // leaves the reservation present and startup fails rather than over-admitting capacity.
    removals.sort_by_key(|p| (!p.starts_with(&handoff), p.clone()));
    for path in removals {
        if path.starts_with(&handoff) {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
        fs::File::open(
            path.parent()
                .ok_or_else(|| io::Error::other("orphan has no parent"))?,
        )?
        .sync_all()?;
    }
    Ok(())
}

fn atomic_reservation_temporary(name: &str) -> bool {
    let Some(name) = name.strip_prefix('.').and_then(|n| n.strip_suffix(".tmp")) else {
        return false;
    };
    let Some((id, suffix)) = name.split_once(".json.") else {
        return false;
    };
    let Some((pid, counter)) = suffix.split_once('-') else {
        return false;
    };
    id.len() == 32
        && id.bytes().all(|b| b.is_ascii_hexdigit())
        && !pid.is_empty()
        && pid.bytes().all(|b| b.is_ascii_digit())
        && !counter.is_empty()
        && counter.bytes().all(|b| b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restart_reclaims_orphan_reservations_only_after_owned_quarantine_validation() {
        let cache = tempfile::tempdir().unwrap();
        let first = Reservation::acquire(cache.path(), 8, 16).unwrap();
        fs::write(first.root.join("partial"), b"12345678").unwrap();
        std::mem::forget(first);
        assert!(Reservation::acquire(cache.path(), 1, 16).is_err());
        let _owner =
            enrichment_store::state::exclusive(cache.path(), ".execution-owner.lock").unwrap();
        recover_orphans(cache.path()).unwrap();
        assert!(Reservation::acquire(cache.path(), 16, 16).is_ok());
        let next = Reservation::acquire(cache.path(), 8, 16).unwrap();
        std::os::unix::fs::symlink("/tmp", next.root.join("escape")).unwrap();
        let record = next.record.clone();
        std::mem::forget(next);
        assert!(recover_orphans(cache.path()).is_err());
        assert!(record.exists());
    }
    #[test]
    fn interrupted_atomic_reservation_writes_are_recovered_but_unknown_files_are_not() {
        let cache = tempfile::tempdir().unwrap();
        let records = cache.path().join("capsule-reservations");
        fs::create_dir(&records).unwrap();
        let path = records.join(format!(".{}.json.1234-0.tmp", "a".repeat(32)));
        fs::write(&path, b"{partial").unwrap();
        let unknown = records.join("operator-note.txt");
        fs::write(&unknown, b"keep").unwrap();
        assert!(recover_orphans(cache.path()).is_err());
        assert!(path.exists());
        assert!(unknown.exists());
        fs::remove_file(unknown).unwrap();
        recover_orphans(cache.path()).unwrap();
        assert!(!path.exists());
    }
    #[test]
    fn concurrent_and_crashed_allocations_cannot_overcommit_capacity() {
        let cache = tempfile::tempdir().unwrap();
        let first = Reservation::acquire(cache.path(), 8, 16).unwrap();
        let second = Reservation::acquire(cache.path(), 8, 16).unwrap();
        assert!(Reservation::acquire(cache.path(), 1, 16).is_err());
        drop(second);
        fs::write(first.root.join("occupied"), b"12345678").unwrap();
        assert!(Reservation::acquire(cache.path(), 1, 16).is_err());
        // Simulate a crashed process's disk footprint without releasing its journal.
        let record = first.record.clone();
        let root = first.root.clone();
        std::mem::forget(first);
        assert!(Reservation::acquire(cache.path(), 1, 16).is_err());
        // Only actual explicit deletion makes the space reusable.
        fs::remove_dir_all(root).unwrap();
        fs::remove_file(record).unwrap();
        assert!(Reservation::acquire(cache.path(), 16, 16).is_ok());
    }
    #[test]
    fn preparation_checks_before_writing_and_refuses_unknown_files() {
        let cache = tempfile::tempdir().unwrap();
        let root = cache.path().join("capsules/generation");
        fs::create_dir_all(&root).unwrap();
        let storage = Preparation::acquire(cache.path(), &root, 8, 32).unwrap();
        storage.write(&root.join("input"), b"12345678").unwrap();
        assert!(storage.write(&root.join("extra"), b"x").is_err());
        assert!(!root.join("extra").exists());
        assert_eq!(storage.archive_policy().unwrap().max_total_bytes, 0);
        storage.write(&root.join("input"), b"short").unwrap();
        assert_eq!(storage.archive_policy().unwrap().max_total_bytes, 3);
        std::os::unix::fs::symlink("input", root.join("link")).unwrap();
        assert!(storage.archive_policy().is_err());
    }
}
