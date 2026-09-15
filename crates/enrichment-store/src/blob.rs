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
    ///
    /// Identity, and the record a later reader finds by digest. Do **not** hand this to a caller
    /// as the provenance of the current call — see [`StoredBlob::acquired`].
    pub artifact: Artifact,
    /// The same bytes, described by *this* retrieval.
    ///
    /// Identical to `artifact` for a blob this call wrote. For one already present it carries
    /// this call's `source_uri`, `retrieved_at` and validators instead of the first retrieval's.
    ///
    /// The distinction is not pedantry. Every per-file artifact's `source_uri` is version- or
    /// commit-qualified (`…/serde/1.0.0#README.md`), and a file that is unchanged between two
    /// releases hashes identically — so reporting the stored record would cite release 1.0.0 as
    /// the source of evidence gathered from 1.0.1. The digest is shared; the locator is not.
    pub acquired: Artifact,
    /// The blob's path on disk.
    pub path: PathBuf,
    /// Whether this call wrote the blob, or found it already present.
    pub newly_written: bool,
}

static STAGING_COUNTER: AtomicU64 = AtomicU64::new(0);

impl BlobStore {
    /// Open existing bytes without creating or repairing any files or directories.
    /// # Errors
    /// Missing content storage is an error.
    pub fn read_only(data_root: &Path) -> io::Result<Self> {
        let root = data_root.join("blobs");
        if !fs::symlink_metadata(root.join("sha256"))?.is_dir() {
            return Err(io::Error::other("blob root is not a directory"));
        }
        Ok(Self { root })
    }
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

    /// Store bytes, describing them with `describe(sha256_hex)`.
    ///
    /// The closure receives the digest so the caller can build the [`Artifact`] record with
    /// its real provenance (source URL, validators, retrieval time) without hashing twice. It is
    /// called on every path, not only when the blob is new: the stored record keeps the first
    /// retrieval's provenance, but the caller still needs this retrieval's — see
    /// [`StoredBlob::acquired`].
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
        let acquired = describe(&sha256);
        if acquired.sha256 != sha256
            || acquired.artifact_id != artifact_id_for(&sha256)
            || acquired.size_bytes != bytes.len() as u64
        {
            return Err(io::Error::other(
                "artifact record does not describe the supplied bytes",
            ));
        }
        if path.is_file() {
            let (stored_digest, stored_bytes) =
                canonical::sha256_reader(fs::File::open(&path)?, bytes.len() as u64)?;
            if stored_digest != sha256 || stored_bytes != bytes.len() as u64 {
                return Err(io::Error::other(
                    "stored blob disagrees with its content identity",
                ));
            }
            let artifact = match self.artifact(&sha256)? {
                Some(artifact) => artifact,
                None => {
                    crate::atomic::write_atomic(
                        &self.meta_path(&sha256),
                        &serde_json::to_vec_pretty(&acquired)?,
                    )?;
                    acquired.clone()
                }
            };
            return Ok(StoredBlob {
                artifact,
                acquired,
                path,
                newly_written: false,
            });
        }

        let artifact = acquired.clone();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let staging = self.staging_path();
        fs::write(&staging, bytes)?;
        fs::File::open(&staging)?.sync_all()?;
        let meta_staging = self.staging_path();
        fs::write(&meta_staging, serde_json::to_vec_pretty(&artifact)?)?;
        fs::File::open(&meta_staging)?.sync_all()?;

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
                    acquired,
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
        if let Some(parent) = path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }

        Ok(StoredBlob {
            artifact,
            acquired,
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

    /// Capture a verified, bounded stream for range/section reads. The private descriptor
    /// prevents a check-then-reopen race or later mutation of the retained file supplying
    /// different bytes under the verified identity. No whole-blob memory allocation occurs.
    pub fn capture(&self, artifact: &Artifact, limit: u64) -> io::Result<fs::File> {
        self.capture_content(
            &artifact.artifact_id,
            &artifact.sha256,
            artifact.size_bytes,
            limit,
        )
    }

    /// Capture the exact bytes of an already admitted typed producer input. Receipt clocks and
    /// transport validators are unnecessary for this retained-content check.
    /// # Errors
    /// Invalid identities, unsafe paths, changed bytes and size excess are refused.
    pub fn capture_input(
        &self,
        input: &enrichment_core::evidence::relational::InputArtifact,
        limit: u64,
    ) -> io::Result<fs::File> {
        self.capture_content(&input.artifact_id, &input.sha256, input.size_bytes, limit)
    }

    fn capture_content(
        &self,
        artifact_id: &str,
        digest: &str,
        bytes: u64,
        limit: u64,
    ) -> io::Result<fs::File> {
        use std::io::{Read, Seek};
        let source = self.open_content(artifact_id, digest, bytes, limit)?;
        let staging = self.root.join(".staging");
        if !fs::symlink_metadata(&staging)?.is_dir() {
            return Err(io::Error::other(
                "artifact scratch is not a physical directory",
            ));
        }
        let mut captured = tempfile::tempfile_in(staging)?;
        let copied = io::copy(&mut source.take(bytes + 1), &mut captured)?;
        if copied != bytes {
            return Err(io::Error::other("artifact length changed during capture"));
        }
        captured.rewind()?;
        let (actual, _) = canonical::sha256_reader(&mut captured, limit)?;
        if actual != digest {
            return Err(io::Error::other(
                "artifact bytes differ from retained identity",
            ));
        }
        captured.rewind()?;
        Ok(captured)
    }

    /// Decode a bounded immutable JSON artifact without writing scratch or reopening its path.
    /// # Errors
    /// Invalid descriptors, JSON, digests and byte lengths are refused before returning a value.
    pub fn read_json<T: serde::de::DeserializeOwned>(
        &self,
        artifact: &Artifact,
        limit: u64,
    ) -> io::Result<T> {
        let file = self.open_input(artifact, limit)?;
        canonical::verified_json(
            io::BufReader::new(file),
            &artifact.sha256,
            artifact.size_bytes,
        )
    }

    /// Read a complete delivery with transport identity injected while hashing only the original
    /// immutable document. The core envelope deserializer enforces every field and outcome rule.
    /// # Errors
    /// Noncanonical framing, duplicate request identity, malformed envelopes and changed bytes fail.
    pub fn read_delivery(
        &self,
        artifact: &Artifact,
        request_id: &str,
    ) -> io::Result<enrichment_core::wire::Envelope> {
        use std::io::Read;
        let file = self.open_input(artifact, 32 * 1024 * 1024)?;
        canonical::verified_read(file, &artifact.sha256, artifact.size_bytes, |reader| {
            let mut first = [0u8];
            reader.read_exact(&mut first)?;
            if first != *b"{" {
                return Err(io::Error::other("delivery must be a canonical JSON object"));
            }
            let prefix = format!("{{\"request_id\":{},", serde_json::to_string(request_id)?);
            serde_json::from_reader(std::io::Cursor::new(prefix.as_bytes()).chain(reader))
                .map_err(Into::into)
        })
    }

    fn open_input(&self, artifact: &Artifact, limit: u64) -> io::Result<fs::File> {
        self.open_content(
            &artifact.artifact_id,
            &artifact.sha256,
            artifact.size_bytes,
            limit,
        )
    }

    fn open_content(
        &self,
        artifact_id: &str,
        digest: &str,
        bytes: u64,
        limit: u64,
    ) -> io::Result<fs::File> {
        if digest.len() != 64
            || !digest.bytes().all(|b| b.is_ascii_hexdigit())
            || artifact_id != artifact_id_for(digest)
            || bytes > limit
        {
            return Err(io::Error::other("invalid or oversized retained artifact"));
        }
        let path = self.path_for(digest);
        for directory in [
            self.root.clone(),
            self.root.join("sha256"),
            path.parent()
                .ok_or_else(|| io::Error::other("missing shard"))?
                .to_owned(),
        ] {
            if !fs::symlink_metadata(directory)?.is_dir() {
                return Err(io::Error::other(
                    "artifact ancestor is not a physical directory",
                ));
            }
        }
        if !fs::symlink_metadata(&path)?.is_file() {
            return Err(io::Error::other("artifact is not a regular file"));
        }
        let mut options = fs::OpenOptions::new();
        options.read(true);
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            // Linux O_NOFOLLOW | O_NONBLOCK: reject substituted links, never block on FIFO.
            options.custom_flags(0o400000 | 0o4000);
        }
        let source = options.open(path)?;
        if !source.metadata()?.is_file() || source.metadata()?.len() != bytes {
            return Err(io::Error::other(
                "artifact size/type differs from retained identity",
            ));
        }
        Ok(source)
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
    fn verified_json_reads_need_no_writable_scratch_and_reject_same_size_mutation() {
        let (dir, blobs) = store();
        let bytes = br#"{"value":"original"}"#;
        let stored = blobs
            .put(bytes, |_| {
                Artifact::describe(
                    bytes,
                    ArtifactKind::Other,
                    "application/json",
                    "fixture:json",
                    "2026-09-14T00:00:00Z",
                )
            })
            .unwrap();
        std::fs::remove_dir(blobs.root().join(".staging")).unwrap();
        let readonly = BlobStore::read_only(dir.path()).unwrap();
        let value: serde_json::Value = readonly.read_json(&stored.artifact, 1024).unwrap();
        assert_eq!(value["value"], "original");
        assert!(!blobs.root().join(".staging").exists());
        std::fs::write(stored.path, br#"{"value":"tampered"}"#).unwrap();
        assert!(
            readonly
                .read_json::<serde_json::Value>(&stored.artifact, 1024)
                .is_err()
        );
    }

    #[test]
    fn identical_bytes_from_two_releases_keep_one_blob_but_two_locators() {
        // The defect this guards: a README unchanged across a patch release hashes identically,
        // so the second resolve found the first blob and reported the FIRST release's
        // `source_uri` as the source of its own evidence. The digest is legitimately shared;
        // the locator is not.
        let (_dir, store) = store();
        let bytes = b"# demo\n\nUnchanged between 1.0.0 and 1.0.1.\n";
        let describe_from = |uri: &'static str, at: &'static str| {
            move |_: &str| Artifact::describe(bytes, ArtifactKind::Readme, "text/markdown", uri, at)
        };

        let first = store
            .put(
                bytes,
                describe_from(
                    "https://crates.io/demo/1.0.0#README.md",
                    "2026-09-13T00:00:00Z",
                ),
            )
            .expect("first put");
        assert!(first.newly_written);
        assert_eq!(first.artifact, first.acquired, "a new blob has one story");

        let second = store
            .put(
                bytes,
                describe_from(
                    "https://crates.io/demo/1.0.1#README.md",
                    "2026-09-14T00:00:00Z",
                ),
            )
            .expect("second put");
        assert!(!second.newly_written, "identical bytes are one blob");
        assert_eq!(
            second.artifact.artifact_id, first.artifact.artifact_id,
            "identity is the digest, and it is shared"
        );

        // The stored record is still the first retrieval's -- that is what a later reader finds
        // by digest, and it is deliberate.
        assert_eq!(
            second.artifact.source_uri,
            "https://crates.io/demo/1.0.0#README.md"
        );
        assert_eq!(second.artifact.retrieved_at, "2026-09-13T00:00:00Z");

        // But this call gets its own locator, so a 1.0.1 answer never cites 1.0.0.
        assert_eq!(
            second.acquired.source_uri,
            "https://crates.io/demo/1.0.1#README.md"
        );
        assert_eq!(second.acquired.retrieved_at, "2026-09-14T00:00:00Z");
        assert_eq!(
            second.acquired.artifact_id, second.artifact.artifact_id,
            "same bytes, same handle: only the provenance differs"
        );
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
