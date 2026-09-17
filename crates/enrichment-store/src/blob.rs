//! Immutable content-addressed bytes. Receipt, authorization and retention authority is Delta.
//! Writes are bounded, digest-verified and conditional; readers supply admitted descriptors.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use enrichment_core::canonical;
use enrichment_core::evidence::{Artifact, artifact_id_for};

/// One blob store rooted at `<data_root>/blobs`.
#[derive(Debug, Clone)]
pub struct BlobStore {
    root: PathBuf,
}

/// A stored blob and its record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredBlob {
    /// This physical acquisition's descriptor. Native catalog records own its retention.
    pub acquired: Artifact,
    /// The blob's path on disk.
    pub path: PathBuf,
    /// Whether this call wrote the blob, or found it already present.
    pub newly_written: bool,
}

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

    /// Stream bounded output to private staging, then admit its digest and metadata together.
    /// Failed writers leave no readable artifact or persistent staging file.
    /// # Errors
    /// I/O failures, excess bytes and inconsistent descriptors fail before publication.
    pub fn put_stream(
        &self,
        limit: u64,
        write: impl FnOnce(&mut dyn io::Write) -> io::Result<()>,
        describe: impl FnOnce(&str, u64) -> Artifact,
    ) -> io::Result<StoredBlob> {
        use io::{Seek, Write};
        struct Bounded<'a> {
            writer: io::BufWriter<&'a mut fs::File>,
            bytes: u64,
            limit: u64,
        }
        impl Write for Bounded<'_> {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                let total = self
                    .bytes
                    .checked_add(bytes.len() as u64)
                    .filter(|n| *n <= self.limit)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::OutOfMemory,
                            "streamed artifact exceeds byte bound",
                        )
                    })?;
                self.writer.write_all(bytes)?;
                self.bytes = total;
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                self.writer.flush()
            }
        }
        let mut staging = tempfile::NamedTempFile::new_in(self.root.join(".staging"))?;
        {
            let mut bounded = Bounded {
                writer: io::BufWriter::new(staging.as_file_mut()),
                bytes: 0,
                limit,
            };
            write(&mut bounded)?;
            bounded.flush()?;
        }
        staging.as_file().sync_all()?;
        staging.as_file_mut().rewind()?;
        let (digest, bytes) = canonical::sha256_reader(staging.as_file_mut(), limit)?;
        let acquired = describe(&digest, bytes);
        if acquired.sha256 != digest
            || acquired.size_bytes != bytes
            || acquired.artifact_id != artifact_id_for(&digest)
        {
            return Err(io::Error::other(
                "stream descriptor differs from retained bytes",
            ));
        }
        let path = self.path_for(&digest);
        let parent = path
            .parent()
            .ok_or_else(|| io::Error::other("missing blob shard"))?;
        fs::create_dir_all(parent)?;
        let newly_written = match staging.persist_noclobber(&path) {
            Ok(_) => true,
            Err(error) if error.error.kind() == io::ErrorKind::AlreadyExists => {
                let existing = canonical::sha256_reader(fs::File::open(&path)?, bytes)?;
                if existing != (digest.clone(), bytes) {
                    return Err(io::Error::other("existing artifact failed identity check"));
                }
                false
            }
            Err(error) => return Err(error.error),
        };
        fs::File::open(parent)?.sync_all()?;
        Ok(StoredBlob {
            acquired,
            path,
            newly_written,
        })
    }

    /// Store finite bytes through the same conditional immutable writer as streaming output.
    pub fn put(
        &self,
        bytes: &[u8],
        describe: impl FnOnce(&str) -> Artifact,
    ) -> io::Result<StoredBlob> {
        self.put_stream(
            bytes.len() as u64,
            |writer| writer.write_all(bytes),
            |digest, _| describe(digest),
        )
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

    /// Verify immutable content using one descriptor without creating scratch or reopening it.
    pub fn verify(&self, artifact: &Artifact, limit: u64) -> io::Result<()> {
        let actual = canonical::sha256_reader(self.open_input(artifact, limit)?, limit)?;
        if actual != (artifact.sha256.clone(), artifact.size_bytes) {
            return Err(io::Error::other(
                "artifact bytes differ from retained identity",
            ));
        }
        Ok(())
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
        let (index, mut sections) =
            self.read_result_sections(artifact, &["envelope"], crate::result::MAX_BYTES)?;
        let mut result: enrichment_core::wire::Envelope = serde_json::from_value(
            sections
                .remove("envelope")
                .ok_or_else(|| io::Error::other("missing native envelope section"))?,
        )?;
        index.validate_result(&result)?;
        result.request_id = request_id.to_owned().try_into().map_err(io::Error::other)?;
        Ok(result)
    }

    /// Read selected verified result sections without scratch writes or full JSON decoding.
    pub fn read_result_sections(
        &self,
        artifact: &Artifact,
        names: &[&str],
        limit: u64,
    ) -> io::Result<(
        crate::result::Index,
        std::collections::BTreeMap<String, serde_json::Value>,
    )> {
        crate::result::verified_sections(
            &mut self.open_input(artifact, crate::result::MAX_BYTES)?,
            artifact,
            names,
            limit,
        )
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::evidence::ArtifactKind;

    fn store() -> (tempfile::TempDir, BlobStore) {
        let dir = tempfile::tempdir().expect("temp dir");
        let store = BlobStore::open(dir.path()).expect("open");
        (dir, store)
    }

    #[test]
    fn failed_streams_do_not_publish_or_leak_staging() {
        let (_dir, blobs) = store();
        let result = blobs.put_stream(
            5,
            |writer| {
                writer.write_all(b"123")?;
                writer.write_all(b"456")
            },
            |_, _| panic!("failed write must not describe a result"),
        );
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::OutOfMemory);
        assert_eq!(
            fs::read_dir(blobs.root.join(".staging")).unwrap().count(),
            0
        );
        assert_eq!(fs::read_dir(blobs.root.join("sha256")).unwrap().count(), 0);
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
                    enrichment_core::native_time::AcquisitionTime::try_from(
                        "2026-09-14T00:00:00.000000Z".to_owned(),
                    )
                    .unwrap(),
                )
            })
            .unwrap();
        std::fs::remove_dir(blobs.root().join(".staging")).unwrap();
        let readonly = BlobStore::read_only(dir.path()).unwrap();
        let value: serde_json::Value = readonly.read_json(&stored.acquired, 1024).unwrap();
        assert_eq!(value["value"], "original");
        assert!(!blobs.root().join(".staging").exists());
        std::fs::write(stored.path, br#"{"value":"tampered"}"#).unwrap();
        assert!(
            readonly
                .read_json::<serde_json::Value>(&stored.acquired, 1024)
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
            move |_: &str| {
                Artifact::describe(
                    bytes,
                    ArtifactKind::Readme,
                    "text/markdown",
                    uri,
                    enrichment_core::native_time::AcquisitionTime::try_from(
                        at.replace("Z", ".000000Z"),
                    )
                    .unwrap(),
                )
            }
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
            second.acquired.artifact_id, first.acquired.artifact_id,
            "identity is the digest, and it is shared"
        );

        // But this call gets its own locator, so a 1.0.1 answer never cites 1.0.0.
        assert_eq!(
            second.acquired.source_uri,
            "https://crates.io/demo/1.0.1#README.md"
        );
        assert_eq!(
            String::from(second.acquired.retrieved_at),
            "2026-09-14T00:00:00.000000Z"
        );
    }

    #[test]
    fn a_blob_round_trips_and_is_sharded_by_digest() {
        let (_dir, store) = store();
        let stored = store
            .put(b"hello", |_| {
                Artifact::describe(
                    b"hello",
                    ArtifactKind::Other,
                    "text/plain",
                    "x://y",
                    enrichment_core::native_time::AcquisitionTime::from_micros(1).unwrap(),
                )
            })
            .expect("put");
        assert!(stored.newly_written);
        assert!(
            stored
                .path
                .starts_with(store.root().join("sha256").join("2c"))
        );
        assert_eq!(store.read(&stored.acquired.sha256).expect("read"), b"hello");
        let files = fs::read_dir(stored.path.parent().unwrap()).unwrap().count();
        assert_eq!(files, 1, "only immutable bytes, no metadata sidecar");
    }

    #[test]
    fn identical_bytes_are_stored_once_without_overwriting_acquisition_provenance() {
        let (_dir, store) = store();
        let first = store
            .put(b"same", |_| {
                Artifact::describe(
                    b"same",
                    ArtifactKind::Readme,
                    "text/markdown",
                    "x://first",
                    enrichment_core::native_time::AcquisitionTime::from_micros(1).unwrap(),
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
                    enrichment_core::native_time::AcquisitionTime::from_micros(2).unwrap(),
                )
            })
            .expect("second");
        assert!(first.newly_written);
        assert!(!second.newly_written);
        assert_eq!(second.acquired.source_uri, "x://second");
        assert!(
            fs::read_dir(store.root().join(".staging"))
                .expect("dir")
                .next()
                .is_none()
        );
    }

    #[test]
    fn a_missing_blob_is_absent_not_an_error() {
        let (_dir, store) = store();
        assert!(!store.contains(&"0".repeat(64)));
        assert!(store.read(&"0".repeat(64)).is_err());
    }
}
