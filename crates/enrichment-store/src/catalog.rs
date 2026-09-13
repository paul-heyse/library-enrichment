//! The catalog: which releases, environments and contexts exist, and which snapshot is current.
//!
//! Small JSON records under `<data_root>/contexts/` and `<data_root>/releases/`, each written
//! atomically (temp file in the same directory, then `rename`). A context's `current` pointer
//! is one file whose content is a snapshot identity; swapping it is a single rename, which is
//! what lets a reader never see a half-published state (blueprint §8.2).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use enrichment_core::identity::{
    Context, ContextId, Ecosystem, Environment, Release, ReleaseId, SnapshotId,
};
use serde::{Serialize, de::DeserializeOwned};

/// Catalog rooted at a data directory.
#[derive(Debug, Clone)]
pub struct Catalog {
    root: PathBuf,
}

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
    fs::write(&temp, bytes)?;
    if let Err(err) = fs::rename(&temp, path) {
        let _ = fs::remove_file(&temp);
        return Err(err);
    }
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    write_atomic(path, &bytes)
}

fn read_json<T: DeserializeOwned>(path: &Path) -> io::Result<Option<T>> {
    if !path.is_file() {
        return Ok(None);
    }
    let bytes = fs::read(path)?;
    Ok(Some(serde_json::from_slice(&bytes)?))
}

fn ecosystem_dir(ecosystem: Ecosystem) -> &'static str {
    match ecosystem {
        Ecosystem::Rust => "rust",
        Ecosystem::Python => "python",
    }
}

impl Catalog {
    /// Open (creating if needed) the catalog under a data root.
    ///
    /// # Errors
    ///
    /// Fails if the directories cannot be created.
    pub fn open(data_root: &Path) -> io::Result<Self> {
        fs::create_dir_all(data_root.join("contexts"))?;
        fs::create_dir_all(data_root.join("releases").join("by-name"))?;
        Ok(Self {
            root: data_root.to_path_buf(),
        })
    }

    fn release_path(&self, id: &ReleaseId) -> PathBuf {
        self.root
            .join("releases")
            .join(format!("{}.json", id.as_str()))
    }

    fn by_name_path(&self, ecosystem: Ecosystem, name: &str, version: &str) -> PathBuf {
        self.root
            .join("releases")
            .join("by-name")
            .join(ecosystem_dir(ecosystem))
            .join(name.to_ascii_lowercase())
            .join(format!("{version}.json"))
    }

    fn context_dir(&self, id: &ContextId) -> PathBuf {
        self.root.join("contexts").join(id.as_str())
    }

    /// Record a release, indexed by identity and by `(ecosystem, name, version)`.
    ///
    /// # Errors
    ///
    /// Fails on I/O error.
    pub fn record_release(&self, release: &Release) -> io::Result<()> {
        write_json(&self.release_path(&release.release_id), release)?;
        write_json(
            &self.by_name_path(
                release.key.ecosystem,
                &release.key.package,
                &release.key.version,
            ),
            &serde_json::json!({ "release_id": release.release_id }),
        )
    }

    /// Look a release up by identity.
    ///
    /// # Errors
    ///
    /// Fails on I/O error or a malformed record.
    pub fn release(&self, id: &ReleaseId) -> io::Result<Option<Release>> {
        read_json(&self.release_path(id))
    }

    /// Look a release up by `(ecosystem, name, version)`.
    ///
    /// # Errors
    ///
    /// Fails on I/O error or a malformed record.
    pub fn find_release(
        &self,
        ecosystem: Ecosystem,
        name: &str,
        version: &str,
    ) -> io::Result<Option<Release>> {
        let pointer: Option<serde_json::Value> =
            read_json(&self.by_name_path(ecosystem, name, version))?;
        let Some(id) = pointer.and_then(|p| {
            p.get("release_id")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        }) else {
            return Ok(None);
        };
        let Ok(id) = ReleaseId::try_from(id) else {
            return Ok(None);
        };
        self.release(&id)
    }

    /// Record a context and the environment it binds.
    ///
    /// # Errors
    ///
    /// Fails on I/O error.
    pub fn record_context(&self, context: &Context, environment: &Environment) -> io::Result<()> {
        let dir = self.context_dir(&context.context_id);
        write_json(&dir.join("context.json"), context)?;
        write_json(&dir.join("environment.json"), environment)
    }

    /// Read a context and its environment.
    ///
    /// # Errors
    ///
    /// Fails on I/O error or a malformed record.
    pub fn context(&self, id: &ContextId) -> io::Result<Option<(Context, Environment)>> {
        let dir = self.context_dir(id);
        let Some(context) = read_json::<Context>(&dir.join("context.json"))? else {
            return Ok(None);
        };
        let Some(environment) = read_json::<Environment>(&dir.join("environment.json"))? else {
            return Ok(None);
        };
        Ok(Some((context, environment)))
    }

    /// Record an arbitrary named JSON document under a context (e.g. the last resolution).
    ///
    /// # Errors
    ///
    /// Fails on I/O error.
    pub fn record_document<T: Serialize>(
        &self,
        id: &ContextId,
        name: &str,
        value: &T,
    ) -> io::Result<()> {
        write_json(&self.context_dir(id).join(format!("{name}.json")), value)
    }

    /// Read a named JSON document recorded under a context.
    ///
    /// # Errors
    ///
    /// Fails on I/O error or a malformed record.
    pub fn document<T: DeserializeOwned>(
        &self,
        id: &ContextId,
        name: &str,
    ) -> io::Result<Option<T>> {
        read_json(&self.context_dir(id).join(format!("{name}.json")))
    }

    /// Point a context at a published snapshot. One rename; readers see old or new, never
    /// neither.
    ///
    /// # Errors
    ///
    /// Fails on I/O error.
    pub fn set_current_snapshot(&self, id: &ContextId, snapshot: &SnapshotId) -> io::Result<()> {
        write_atomic(
            &self.context_dir(id).join("current"),
            format!("{}\n", snapshot.as_str()).as_bytes(),
        )
    }

    /// The snapshot a context currently points at, if any.
    ///
    /// # Errors
    ///
    /// Fails on I/O error.
    pub fn current_snapshot(&self, id: &ContextId) -> io::Result<Option<SnapshotId>> {
        let path = self.context_dir(id).join("current");
        if !path.is_file() {
            return Ok(None);
        }
        let text = fs::read_to_string(path)?;
        Ok(SnapshotId::try_from(text.trim().to_owned()).ok())
    }
}

#[cfg(test)]
mod tests {
    use enrichment_core::identity::{ReleaseKey, ResearchMode};

    use super::*;

    fn release() -> Release {
        Release::new(ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "enr-fixture".into(),
            version: "0.1.0".into(),
            artifact_digest: Some("ab".repeat(32)),
        })
    }

    #[test]
    fn releases_are_found_by_identity_and_by_name() {
        let dir = tempfile::tempdir().expect("dir");
        let catalog = Catalog::open(dir.path()).expect("open");
        let release = release();
        catalog.record_release(&release).expect("record");
        assert_eq!(
            catalog.release(&release.release_id).expect("read"),
            Some(release.clone())
        );
        assert_eq!(
            catalog
                .find_release(Ecosystem::Rust, "ENR-fixture", "0.1.0")
                .expect("read"),
            Some(release)
        );
        assert_eq!(
            catalog
                .find_release(Ecosystem::Rust, "enr-fixture", "9.9.9")
                .expect("read"),
            None
        );
    }

    #[test]
    fn a_context_round_trips_with_its_environment_and_pointer() {
        let dir = tempfile::tempdir().expect("dir");
        let catalog = Catalog::open(dir.path()).expect("open");
        let environment = Environment::unspecified();
        let context = Context::new(
            release().release_id,
            environment.environment_id.clone(),
            ResearchMode::Project,
        );
        catalog
            .record_context(&context, &environment)
            .expect("record");
        assert_eq!(
            catalog.context(&context.context_id).expect("read"),
            Some((context.clone(), environment))
        );
        assert_eq!(
            catalog.current_snapshot(&context.context_id).expect("read"),
            None
        );
        let snapshot = SnapshotId::try_from("snap_0123456789abcdef".to_owned()).expect("id");
        catalog
            .set_current_snapshot(&context.context_id, &snapshot)
            .expect("point");
        assert_eq!(
            catalog.current_snapshot(&context.context_id).expect("read"),
            Some(snapshot)
        );
        catalog
            .record_document(&context.context_id, "note", &serde_json::json!({"k": 1}))
            .expect("doc");
        let back: Option<serde_json::Value> =
            catalog.document(&context.context_id, "note").expect("read");
        assert_eq!(back, Some(serde_json::json!({"k": 1})));
        assert!(
            fs::read_dir(catalog.context_dir(&context.context_id))
                .expect("dir")
                .all(|e| !e
                    .expect("entry")
                    .file_name()
                    .to_string_lossy()
                    .ends_with(".tmp")),
            "no temp files survive an atomic write"
        );
    }
}
