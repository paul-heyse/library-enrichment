//! The content-addressed blob store: `blobs/sha256/<ab>/<digest>` plus a metadata sidecar.
//!
//! Immutable raw artifacts are keyed by digest (blueprint §8.4) and kept as retrieved so
//! normalization can improve without refetching (§6.3). A blob is written to a staging file
//! and renamed into place, so a reader never sees a partial blob; a second `put` of identical
//! bytes finds the existing blob and keeps its original provenance.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use enrichment_core::canonical;
use enrichment_core::evidence::{Artifact, ArtifactKind, artifact_id_for};

/// One blob store rooted at `<data_root>/blobs`.
#[derive(Debug, Clone)]
pub struct BlobStore {
    root: PathBuf,
}

/// A stored blob and its record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredBlob {
    /// The artifact record, as stored (the first retrieval's provenance).
    pub artifact: Artifact,
    /// The blob's path on disk.
    pub path: PathBuf,
    /// Whether this call wrote the blob, or found it already present.
    pub newly_written: bool,
}

static STAGING_COUNTER: AtomicU64 = AtomicU64::new(0);

impl BlobStore {
    /// Open (creating if needed) the blob store under a data root.
    ///
    /// # Errors
    ///
    /// Fails if the directories cannot be created.
    pub fn open(data_root: &Path) -> io::Result<Self> {
        let root = data_root.join("blobs");
        fs::create_dir_all(root.join("sha256"))?;
        fs::create_dir_all(root.join(".staging"))?;
        Ok(Self { root })
    }

    /// The directory this store writes under.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Store bytes, describing them with `describe(sha256_hex)` if they are new.
    ///
    /// The closure receives the digest so the caller can build the [`Artifact`] record with
    /// its real provenance (source URL, validators, retrieval time) without hashing twice.
    ///
    /// # Errors
    ///
    /// Fails on I/O error. A blob already present is not an error.
    pub fn put(
        &self,
        bytes: &[u8],
        describe: impl FnOnce(&str) -> Artifact,
    ) -> io::Result<StoredBlob> {
        let sha256 = canonical::sha256_hex(bytes);
        let path = self.path_for(&sha256);
        if path.is_file() {
            let artifact = self.artifact(&sha256)?.unwrap_or_else(|| describe(&sha256));
            return Ok(StoredBlob {
                artifact,
                path,
                newly_written: false,
            });
        }

        let artifact = describe(&sha256);
        debug_assert_eq!(
            artifact.sha256, sha256,
            "the record must describe these bytes"
        );
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let staging = self.staging_path();
        fs::write(&staging, bytes)?;
        let meta_staging = self.staging_path();
        fs::write(&meta_staging, serde_json::to_vec_pretty(&artifact)?)?;

        // Data first, then metadata: a crash between the two leaves a blob without a record,
        // which `artifact()` reports as absent and the next `put` repairs.
        if let Err(err) = fs::rename(&staging, &path) {
            let _ = fs::remove_file(&staging);
            let _ = fs::remove_file(&meta_staging);
            if path.is_file() {
                // Lost a race to an identical blob; ours is redundant and the winner's record
                // is the one kept.
                let artifact = self.artifact(&sha256)?.unwrap_or(artifact);
                return Ok(StoredBlob {
                    artifact,
                    path,
                    newly_written: false,
                });
            }
            return Err(err);
        }
        let meta_path = self.meta_path(&sha256);
        if meta_path.is_file() {
            let _ = fs::remove_file(&meta_staging);
        } else {
            fs::rename(&meta_staging, &meta_path)?;
        }

        Ok(StoredBlob {
            artifact,
            path,
            newly_written: true,
        })
    }

    /// Whether a blob with this digest is stored.
    #[must_use]
    pub fn contains(&self, sha256_hex: &str) -> bool {
        self.path_for(sha256_hex).is_file()
    }

    /// The path a digest maps to, whether or not it exists.
    #[must_use]
    pub fn path_for(&self, sha256_hex: &str) -> PathBuf {
        let shard = sha256_hex.get(..2).unwrap_or("__");
        self.root.join("sha256").join(shard).join(sha256_hex)
    }

    fn meta_path(&self, sha256_hex: &str) -> PathBuf {
        let mut path = self.path_for(sha256_hex).into_os_string();
        path.push(".meta.json");
        PathBuf::from(path)
    }

    /// Read a blob's bytes.
    ///
    /// # Errors
    ///
    /// Fails if the blob is absent or unreadable.
    pub fn read(&self, sha256_hex: &str) -> io::Result<Vec<u8>> {
        fs::read(self.path_for(sha256_hex))
    }

    /// Read a blob's record, or `None` if the blob or its record is absent.
    ///
    /// # Errors
    ///
    /// Fails if the record exists but is unreadable or malformed.
    pub fn artifact(&self, sha256_hex: &str) -> io::Result<Option<Artifact>> {
        let meta_path = self.meta_path(sha256_hex);
        if !meta_path.is_file() {
            return Ok(None);
        }
        let text = fs::read(meta_path)?;
        let artifact: Artifact = serde_json::from_slice(&text)?;
        Ok(Some(artifact))
    }

    /// Find a blob by its artifact handle (`art_<32 hex>`).
    ///
    /// The handle is a digest prefix, so the lookup is a directory scan of one shard. Only
    /// service-issued handles reach this; a path or an arbitrary string never resolves.
    ///
    /// # Errors
    ///
    /// Fails on I/O error.
    pub fn find(&self, artifact_id: &str) -> io::Result<Option<Artifact>> {
        let Some(prefix) = artifact_id.strip_prefix("art_") else {
            return Ok(None);
        };
        if prefix.len() < 2 || !prefix.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Ok(None);
        }
        let shard = self.root.join("sha256").join(&prefix[..2]);
        if !shard.is_dir() {
            return Ok(None);
        }
        for entry in fs::read_dir(shard)? {
            let entry = entry?;
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            if name.len() == 64 && name.starts_with(prefix) {
                return self.artifact(name);
            }
        }
        Ok(None)
    }

    fn staging_path(&self) -> PathBuf {
        let n = STAGING_COUNTER.fetch_add(1, Ordering::Relaxed);
        self.root
            .join(".staging")
            .join(format!("{}-{n}.tmp", std::process::id()))
    }
}

/// Describe bytes about to be stored, for callers that have no HTTP provenance to add.
#[must_use]
pub fn describe_local(
    bytes: &[u8],
    kind: ArtifactKind,
    media_type: &str,
    source_uri: &str,
    retrieved_at: &str,
) -> Artifact {
    let mut artifact = Artifact::describe(bytes, kind, media_type, source_uri, retrieved_at);
    artifact.artifact_id = artifact_id_for(&artifact.sha256);
    artifact
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (tempfile::TempDir, BlobStore) {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = BlobStore::open(dir.path()).expect("open");
        (dir, store)
    }

    fn describe(kind: ArtifactKind) -> impl FnOnce(&str) -> Artifact {
        move |_| Artifact::describe(b"", kind, "text/plain", "x://y", "2026-09-13T00:00:00Z")
    }

    #[test]
    fn a_blob_round_trips_and_is_sharded_by_digest() {
        let (_dir, store) = store();
        let stored = store
            .put(b"hello", |_| {
                Artifact::describe(b"hello", ArtifactKind::Other, "text/plain", "x://y", "t")
            })
            .expect("put");
        assert!(stored.newly_written);
        assert!(
            stored
                .path
                .starts_with(store.root().join("sha256").join("2c"))
        );
        assert_eq!(store.read(&stored.artifact.sha256).expect("read"), b"hello");
        assert_eq!(
            store.artifact(&stored.artifact.sha256).expect("meta"),
            Some(stored.artifact.clone())
        );
        assert_eq!(
            store.find(&stored.artifact.artifact_id).expect("find"),
            Some(stored.artifact)
        );
    }

    #[test]
    fn identical_bytes_are_stored_once_and_keep_the_first_provenance() {
        let (_dir, store) = store();
        let first = store
            .put(b"same", |_| {
                Artifact::describe(
                    b"same",
                    ArtifactKind::Readme,
                    "text/markdown",
                    "x://first",
                    "t1",
                )
            })
            .expect("first");
        let second = store
            .put(b"same", |_| {
                Artifact::describe(
                    b"same",
                    ArtifactKind::Readme,
                    "text/markdown",
                    "x://second",
                    "t2",
                )
            })
            .expect("second");
        assert!(first.newly_written);
        assert!(!second.newly_written);
        assert_eq!(second.artifact.source_uri, "x://first");
        assert!(
            fs::read_dir(store.root().join(".staging"))
                .expect("dir")
                .next()
                .is_none()
        );
    }

    #[test]
    fn a_handle_that_is_not_service_issued_never_resolves() {
        let (_dir, store) = store();
        store.put(b"", describe(ArtifactKind::Other)).expect("put");
        for bad in ["/etc/passwd", "art_", "art_zz", "sha256/e3/x", "../../x"] {
            assert_eq!(store.find(bad).expect("find"), None, "{bad}");
        }
    }

    #[test]
    fn a_missing_blob_is_absent_not_an_error() {
        let (_dir, store) = store();
        assert!(!store.contains(&"0".repeat(64)));
        assert_eq!(store.artifact(&"0".repeat(64)).expect("ok"), None);
        assert!(store.read(&"0".repeat(64)).is_err());
    }
}
