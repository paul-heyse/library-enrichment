//! Durable conservative reservations for unmounted output quarantine.
use std::{
    fs, io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use enrichment_store::physical_ownership::{OwnershipStore, StorageReservation};
pub struct Reservation {
    pub root: PathBuf,
    id: String,
    store: OwnershipStore,
    closed: bool,
}

/// A host preparation allocation. All first-party extraction/writes use its remaining bound.
/// The reservation outlives this builder's work; on drop the actual capsule remains counted.
pub struct Preparation {
    root: PathBuf,
    limit: u64,
    _reservation: Reservation,
}
impl Preparation {
    pub async fn acquire(
        store: &OwnershipStore,
        root: &Path,
        limit: u64,
        budget: u64,
    ) -> io::Result<Self> {
        Ok(Self {
            root: root.to_owned(),
            limit,
            _reservation: Reservation::acquire(store, limit, budget).await?,
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
    pub async fn acquire(store: &OwnershipStore, requested: u64, budget: u64) -> io::Result<Self> {
        let cache = PathBuf::from(store.cache());
        let id = uuid::Uuid::new_v4().simple().to_string();
        let root = cache.join("handoff").join(&id);
        let capture = cache.clone();
        store
            .reserve_storage(
                StorageReservation {
                    reservation_id: id.clone(),
                    cache: store.cache().into(),
                    quarantine: super::path_text(&root)?,
                    bytes: requested,
                    released: false,
                    sequence: 0,
                },
                move || {
                    bytes(&capture.join("capsules"))?
                        .checked_add(bytes(&capture.join("handoff"))?)
                        .ok_or_else(|| io::Error::other("storage size overflow"))
                },
                budget,
            )
            .await
            .map_err(io::Error::other)?;
        // A failed creation leaves a native obligation that restart can reconcile.
        fs::create_dir_all(&root)?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        Ok(Self {
            root,
            id,
            store: store.clone(),
            closed: false,
        })
    }
    pub async fn close(mut self) -> io::Result<()> {
        self.remove()?;
        self.store
            .release_storage(&self.id)
            .await
            .map_err(io::Error::other)?;
        self.closed = true;
        Ok(())
    }
    fn remove(&self) -> io::Result<()> {
        match fs::remove_dir_all(&self.root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        if let Some(parent) = self.root.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        if self.closed {
            return;
        }
        match self.remove() {
            Ok(()) => self.store.release_after_removal(self.id.clone()),
            Err(error) => {
                eprintln!("library-enrichmentd: retained quarantine reservation: {error}")
            }
        }
    }
}

/// Called under the cache owner lock after all container creators have been reconciled.
pub(crate) async fn recover_orphans(store: &OwnershipStore) -> io::Result<()> {
    let handoff = PathBuf::from(store.cache()).join("handoff");
    let reservations = store.reservations().await.map_err(io::Error::other)?;
    let mut captured = Vec::new();
    if handoff.try_exists()? {
        if !fs::symlink_metadata(&handoff)?.is_dir() {
            return Err(io::Error::other("quarantine is not a physical directory"));
        }
        for entry in fs::read_dir(&handoff)? {
            if captured.len() >= 1024 {
                return Err(io::Error::other("quarantine capture exceeds bound"));
            }
            let entry = entry?;
            super::inventory::capture(&entry.path(), u64::MAX)?;
            captured.push(super::path_text(&entry.path())?);
        }
    }
    for root in store
        .recovery_quarantines(captured)
        .await
        .map_err(io::Error::other)?
    {
        match fs::remove_dir_all(root) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    if handoff.try_exists()? {
        fs::File::open(&handoff)?.sync_all()?;
    }
    for reservation in reservations {
        store
            .release_storage(&reservation.reservation_id)
            .await
            .map_err(io::Error::other)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn native_capacity_survives_dropped_owner_and_recovers_only_after_bytes_removed() {
        let cache = tempfile::tempdir().unwrap();
        let store = crate::execution::test_ownership(cache.path());
        let first = Reservation::acquire(&store, 8, 16).await.unwrap();
        fs::write(first.root.join("payload"), [1; 8]).unwrap();
        assert!(Reservation::acquire(&store, 1, 16).await.is_err());
        std::mem::forget(first);
        recover_orphans(&store).await.unwrap();
        let next = Reservation::acquire(&store, 16, 16).await.unwrap();
        next.close().await.unwrap();
    }
    #[tokio::test]
    async fn preparation_refuses_writes_beyond_its_reserved_bound() {
        let cache = tempfile::tempdir().unwrap();
        let store = crate::execution::test_ownership(cache.path());
        let root = cache.path().join("capsules/test");
        fs::create_dir_all(&root).unwrap();
        let storage = Preparation::acquire(&store, &root, 8, 32).await.unwrap();
        storage.write(&root.join("file"), [0; 8]).unwrap();
        assert!(storage.write(&root.join("file"), [0; 9]).is_err());
    }
}
