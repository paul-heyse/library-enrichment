//! Operator-only, locked cleanup. Evidence is removed only by an explicit evidence/reset action.
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use enrichment_core::{
    config::Config,
    native_key::Key,
    operation::identities::{PhysicalFile, PhysicalInventory},
};
use enrichment_store::{StatePaths, leases, state};
use serde::Serialize;

const CACHE_PAYLOADS: &[&str] = &[
    "capsules",
    "handoff",
    "downloads",
    "unpacked",
    "http",
    "workers",
    "query-spill",
];
const EVIDENCE_PAYLOADS: &[&str] = &["blobs", "delta", "staging", "diagnostics"];
const MAX_ENTRIES: usize = 100_000;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Cache,
    Evidence,
    Development,
}

#[derive(Debug, Serialize)]
pub struct Candidate {
    pub path: PathBuf,
    pub files: usize,
    pub directories: usize,
    /// Logical bytes of removed files, not an estimate of filesystem blocks reclaimed.
    pub file_bytes: u64,
    pub inventory_digest: String,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub scope: Scope,
    pub applied: bool,
    pub candidates: Vec<Candidate>,
    pub removed: Vec<PathBuf>,
    pub removed_file_bytes: u64,
    pub removed_files: usize,
    pub preserved: Vec<PathBuf>,
    pub error: Option<String>,
}

struct Guards {
    _data: fs::File,
    _cache: fs::File,
    _storage: fs::File,
}
fn guards(paths: &StatePaths) -> io::Result<Guards> {
    let data = state::exclusive(&paths.data_root, ".daemon.lock")?;
    let cache = state::exclusive(&paths.cache_root, ".execution-owner.lock")?;
    leases::initialize(&paths.data_root)?;
    let storage = state::exclusive(&paths.cache_root, "capsule-storage.lock")?;
    Ok(Guards {
        _data: data,
        _cache: cache,
        _storage: storage,
    })
}

/// Preview by default, and apply only while holding daemon, cache, evidence and storage locks.
/// Returns partial deletion counts and the first error; it never reports a failed removal as freed.
/// # Errors
/// Unowned roots, active readers/workers, unknown file kinds and unavailable recovery fail closed.
pub fn cleanup(
    config: &Config,
    paths: &StatePaths,
    scope: Scope,
    apply: bool,
) -> io::Result<Report> {
    state::verify(paths)?;
    if scope == Scope::Development {
        return Err(io::Error::other(
            "use the explicit development reset entry point",
        ));
    }
    run(config, paths, scope, apply)
}

/// Hard cutover of only the known service payloads under this checkout's `.dev-state`.
/// Logs, credentials, execution images, unrelated fixtures and source files remain outside its inventory.
/// # Errors
/// A different root, link, missing repository identity or active owner refuses the reset.
pub fn reset_development(config: &Config, root: &Path, apply: bool) -> io::Result<Report> {
    let canonical = root.canonicalize()?;
    if canonical != root || root.file_name().is_none_or(|n| n != ".dev-state") {
        return Err(io::Error::other(
            "development reset requires a physical absolute .dev-state directory",
        ));
    }
    let checkout = root
        .parent()
        .ok_or_else(|| io::Error::other("missing development checkout"))?;
    let instructions = fs::read_to_string(checkout.join("AGENTS.md"))?;
    let package = fs::read_to_string(checkout.join("Cargo.toml"))?;
    if !instructions.contains("library-enrichment agent instructions")
        || !package.contains("crates/enrichment-daemon")
    {
        return Err(io::Error::other(
            "development reset root is not this service's checkout",
        ));
    }
    let paths = StatePaths::explicit(root.join("cache"), root.join("data"));
    state::validate_paths(&paths)?;
    state::verify_existing_markers(&paths)?;
    // Existing operator roots are required: a typo cannot create and mark a new ownership claim.
    for path in [&paths.cache_root, &paths.data_root] {
        fs::read_dir(path)?;
    }
    run(config, &paths, Scope::Development, apply)
}

fn run(config: &Config, paths: &StatePaths, scope: Scope, apply: bool) -> io::Result<Report> {
    let _guards = guards(paths)?;
    let mut targets = Vec::new();
    if matches!(scope, Scope::Cache | Scope::Development) {
        targets.extend(CACHE_PAYLOADS.iter().map(|p| paths.cache_root.join(p)));
    }
    if matches!(scope, Scope::Evidence | Scope::Development) {
        targets.extend(EVIDENCE_PAYLOADS.iter().map(|p| paths.data_root.join(p)));
    }
    targets.sort();
    let mut report = Report {
        scope,
        applied: apply,
        candidates: Vec::new(),
        removed: Vec::new(),
        removed_file_bytes: 0,
        removed_files: 0,
        preserved: Vec::new(),
        error: None,
    };
    // Recovery changes native ownership records and can enqueue final release writes.
    // Complete it and join the native runtime before observing bytes for deletion. No
    // runtime/provider may retain a shared evidence lock or write after this boundary.
    if let Err(error) = prepare(config, paths, apply) {
        report.error = Some(error.to_string());
        return Ok(report);
    }
    let _retention = leases::exclusive(&paths.data_root)?;
    for root in [&paths.data_root, &paths.cache_root] {
        for entry in fs::read_dir(root)? {
            let path = entry?.path();
            if !targets.contains(&path) {
                report.preserved.push(path);
            }
        }
    }
    report.preserved.sort();
    for path in targets {
        match fs::symlink_metadata(&path) {
            Ok(_) => report.candidates.push(candidate(&path)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    if !apply {
        return Ok(report);
    }
    let operation = (|| -> io::Result<()> {
        for expected in &report.candidates {
            let actual = candidate(&expected.path)?;
            if actual.inventory_digest != expected.inventory_digest {
                return Err(io::Error::other(format!(
                    "cleanup candidate changed: {}",
                    expected.path.display()
                )));
            }
            remove_counted(
                &expected.path,
                &mut report.removed_file_bytes,
                &mut report.removed_files,
            )?;
            report.removed.push(expected.path.clone());
            fs::File::open(
                expected
                    .path
                    .parent()
                    .ok_or_else(|| io::Error::other("cleanup candidate has no parent"))?,
            )?
            .sync_all()?;
        }
        if scope == Scope::Development {
            state::mark_reset(paths)?;
        }
        Ok(())
    })();
    if let Err(error) = operation {
        report.error = Some(error.to_string());
    }
    Ok(report)
}

fn prepare(config: &Config, paths: &StatePaths, apply: bool) -> io::Result<()> {
    let runtime = enrichment_store::runtime::QueryRuntime::new(
        &paths.cache_root.join("maintenance-spill"),
        (&config.arrow).into(),
    )
    .map_err(io::Error::other)?;
    let result = (|| -> io::Result<()> {
        let control = if apply {
            enrichment_store::control::ControlStore::open(&paths.data_root, runtime.clone())?
        } else {
            enrichment_store::control::ControlStore::read_only(&paths.data_root, runtime.clone())?
        };
        let ownership = enrichment_store::physical_ownership::OwnershipStore::new(
            control,
            runtime.clone(),
            &paths.cache_root,
        )?;
        let paths = paths.clone();
        let execution = config.execution.clone();
        runtime
            .bootstrap(async move {
                crate::execution::ownership::validate_for_state(&paths, &execution, &ownership)
                    .await?;
                if apply {
                    // Confirm every recorded container is absent before removing its mounts.
                    // Image storage stays outside this inventory.
                    let supervisor = crate::execution::cleanup::Supervisor::new(
                        Arc::new(tokio::sync::Semaphore::new(1)),
                        execution.cleanup_deadline_seconds,
                        1,
                    );
                    let runner = crate::execution::Runner::new(
                        &execution,
                        &paths.cache_root,
                        supervisor,
                        ownership.clone(),
                    )?;
                    runner.recover_owned().await?;
                    crate::execution::budget::recover_orphans(&ownership).await?;
                }
                Ok(())
            })
            .map_err(io::Error::other)?
    })();
    let closing = runtime.clone();
    let closed = runtime
        .bootstrap(async move { closing.close_diagnostics().await })
        .and_then(std::convert::identity)
        .map_err(io::Error::other);
    match (result, closed) {
        (Ok(()), result) | (result, Ok(())) => result,
        (Err(error), Err(close)) => Err(io::Error::other(format!(
            "{error}; native maintenance drain also failed: {close}"
        ))),
    }
}

fn remove_counted(path: &Path, bytes: &mut u64, files: &mut usize) -> io::Result<()> {
    let mut pending = vec![(path.to_owned(), false)];
    let mut seen = 0;
    while let Some((path, visited)) = pending.pop() {
        seen += 1;
        if seen > MAX_ENTRIES * 2 {
            return Err(io::Error::other("cleanup deletion bound exceeded"));
        }
        let meta = fs::symlink_metadata(&path)?;
        if meta.is_dir() {
            if visited {
                fs::remove_dir(&path)?;
            } else {
                pending.push((path.clone(), true));
                for child in fs::read_dir(path)? {
                    pending.push((child?.path(), false));
                }
            }
        } else if meta.is_file() {
            let next = bytes
                .checked_add(meta.len())
                .ok_or_else(|| io::Error::other("cleanup byte total overflow"))?;
            fs::remove_file(&path)?;
            *bytes = next;
            *files += 1;
        } else {
            return Err(io::Error::other(
                "cleanup candidate changed to a link or special file",
            ));
        }
    }
    Ok(())
}

fn candidate(path: &Path) -> io::Result<Candidate> {
    use std::os::unix::fs::MetadataExt;
    let mut pending = vec![(path.to_owned(), 0)];
    let mut rows = Vec::new();
    let (mut files, mut directories, mut file_bytes) = (0, 0, 0u64);
    while let Some((entry, depth)) = pending.pop() {
        if rows.len() >= MAX_ENTRIES || depth > 64 {
            return Err(io::Error::other("cleanup inventory bound exceeded"));
        }
        let meta = fs::symlink_metadata(&entry)?;
        if meta.is_dir() {
            directories += 1;
            for child in fs::read_dir(&entry)? {
                if pending.len() + rows.len() >= MAX_ENTRIES {
                    return Err(io::Error::other("cleanup inventory bound exceeded"));
                }
                pending.push((child?.path(), depth + 1));
            }
        } else if meta.is_file() && meta.nlink() == 1 {
            files += 1;
            file_bytes = file_bytes
                .checked_add(meta.len())
                .ok_or_else(|| io::Error::other("cleanup byte total overflow"))?;
        } else {
            return Err(io::Error::other(format!(
                "cleanup refuses a link or special file: {}",
                entry.display()
            )));
        }
        rows.push(PhysicalFile {
            path: entry
                .strip_prefix(path)
                .map_err(io::Error::other)?
                .to_path_buf(),
            device: meta.dev(),
            inode: meta.ino(),
            mode: meta.mode(),
            bytes: meta.len(),
            modified_seconds: meta.mtime(),
            modified_nanoseconds: meta.mtime_nsec(),
            changed_seconds: meta.ctime(),
            changed_nanoseconds: meta.ctime_nsec(),
        });
    }
    Ok(Candidate {
        path: path.to_owned(),
        files,
        directories,
        file_bytes,
        inventory_digest: Key::PhysicalInventory
            .hex_digest(&PhysicalInventory { files: rows })
            .map_err(io::Error::other)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn close(runtime: enrichment_store::runtime::QueryRuntime) {
        let closing = runtime.clone();
        runtime
            .bootstrap(async move { closing.close_diagnostics().await })
            .unwrap()
            .unwrap();
    }
    fn fixture() -> (tempfile::TempDir, StatePaths) {
        let temp = tempfile::tempdir().unwrap();
        let paths = StatePaths::explicit(temp.path().join("cache"), temp.path().join("data"));
        state::initialize(&paths).unwrap();
        let runtime = enrichment_store::runtime::QueryRuntime::new(
            &temp.path().join("spill"),
            Default::default(),
        )
        .unwrap();
        let control =
            enrichment_store::control::ControlStore::open(&paths.data_root, runtime.clone())
                .unwrap();
        runtime
            .bootstrap(async move { control.pin().await })
            .unwrap()
            .unwrap();
        close(runtime);
        fs::create_dir(paths.data_root.join("blobs")).unwrap();
        fs::write(paths.data_root.join("blobs/evidence"), b"facts").unwrap();
        fs::create_dir(paths.cache_root.join("capsules")).unwrap();
        fs::write(paths.cache_root.join("capsules/scratch"), b"scratch").unwrap();
        (temp, paths)
    }
    #[test]
    fn preview_cache_cleanup_and_explicit_evidence_removal_are_distinct() {
        let (_temp, paths) = fixture();
        let config = Config::default();
        let preview = cleanup(&config, &paths, Scope::Cache, false).unwrap();
        assert!(preview.error.is_none(), "{preview:?}");
        assert!(!preview.applied);
        assert!(preview.removed.is_empty());
        assert_eq!(preview.candidates[0].file_bytes, 7);
        assert!(paths.cache_root.join("capsules/scratch").exists());
        let applied = cleanup(&config, &paths, Scope::Cache, true).unwrap();
        assert!(applied.error.is_none(), "{applied:?}");
        assert_eq!(applied.removed_file_bytes, 7);
        assert!(paths.data_root.join("blobs/evidence").exists());
        let evidence = cleanup(&config, &paths, Scope::Evidence, true).unwrap();
        assert!(evidence.error.is_none(), "{evidence:?}");
        assert!(evidence.removed_file_bytes >= 5);
        assert!(!paths.data_root.join("blobs").exists());
        state::verify(&paths).unwrap();
        leases::exclusive(&paths.data_root).unwrap();
    }
    #[test]
    fn live_owners_readers_and_linked_candidates_prevent_deletion() {
        let (_temp, paths) = fixture();
        let config = Config::default();
        let owner = state::exclusive(&paths.cache_root, ".execution-owner.lock").unwrap();
        assert!(cleanup(&config, &paths, Scope::Cache, true).is_err());
        drop(owner);
        leases::initialize(&paths.data_root).unwrap();
        let reader = leases::shared(&paths.data_root).unwrap();
        assert!(cleanup(&config, &paths, Scope::Evidence, true).is_err());
        drop(reader);
        std::os::unix::fs::symlink(
            paths.data_root.join("blobs"),
            paths.cache_root.join("capsules/link"),
        )
        .unwrap();
        assert!(cleanup(&config, &paths, Scope::Cache, true).is_err());
        assert!(paths.data_root.join("blobs/evidence").exists());
    }
    #[test]
    fn switching_execution_roots_cannot_hide_an_unfinished_container() {
        let (temp, paths) = fixture();
        let mut config = Config::default();
        config.execution.storage_root = Some(temp.path().join("engine-a"));
        let supervisor = crate::execution::cleanup::Supervisor::new(
            Arc::new(tokio::sync::Semaphore::new(1)),
            5,
            1,
        );
        let runtime = enrichment_store::runtime::QueryRuntime::new(
            &temp.path().join("spill"),
            Default::default(),
        )
        .unwrap();
        let control =
            enrichment_store::control::ControlStore::open(&paths.data_root, runtime.clone())
                .unwrap();
        let ownership = enrichment_store::physical_ownership::OwnershipStore::new(
            control,
            runtime.clone(),
            &paths.cache_root,
        )
        .unwrap();
        let _runner = crate::execution::Runner::new(
            &config.execution,
            &paths.cache_root,
            supervisor,
            ownership.clone(),
        )
        .unwrap();
        let root = temp.path().join("engine-a").to_str().unwrap().to_owned();
        runtime
            .bootstrap(async move {
                use enrichment_store::physical_ownership::*;
                let cache = ownership.cache().to_owned();
                ownership
                    .register_root(ExecutionRoot {
                        root: root.clone(),
                        cache: cache.clone(),
                        broker: "/usr/bin/podman".into(),
                    })
                    .await?;
                let name = format!("libenr-{}", "a".repeat(32));
                ownership
                    .reserve_owner(PhysicalOwner {
                        name: name.clone(),
                        root,
                        cache,
                        capsule: "/fixture".into(),
                        image: "fixture".into(),
                        operation_id: "fixture".into(),
                        authority: enrichment_core::execution::ProcessAuthority::Qualification {
                            definition_id: "fixture".into(),
                        },
                        created_at: enrichment_core::native_time::ObservationTime::now().unwrap(),
                        state: PhysicalState::Reserved,
                        creator_boot_id: None,
                        creator_pid: None,
                        sequence: 0,
                    })
                    .await?;
                ownership
                    .observe(
                        &name,
                        OwnershipObservation::Creating {
                            boot_id: fs::read_to_string("/proc/sys/kernel/random/boot_id")?
                                .trim()
                                .into(),
                        },
                    )
                    .await
            })
            .unwrap()
            .unwrap();
        drop(_runner);
        close(runtime);
        config.execution.storage_root = Some(temp.path().join("engine-b"));
        let result = cleanup(&config, &paths, Scope::Cache, true).unwrap();
        assert!(result.error.as_deref().unwrap().contains("creator"));
        assert!(result.removed.is_empty());
        assert!(paths.cache_root.join("capsules/scratch").exists());
    }
    #[tokio::test]
    async fn development_preview_preserves_startability_and_foreign_markers_block_before_deletion()
    {
        let temp = tempfile::tempdir().unwrap();
        fs::write(
            temp.path().join("AGENTS.md"),
            "library-enrichment agent instructions",
        )
        .unwrap();
        fs::write(temp.path().join("Cargo.toml"), "crates/enrichment-daemon").unwrap();
        let root = temp.path().join(".dev-state");
        let paths = StatePaths::explicit(root.join("cache"), root.join("data"));
        fs::create_dir_all(&paths.data_root).unwrap();
        fs::create_dir_all(&paths.cache_root).unwrap();
        reset_development(&Config::default(), &root, false).unwrap();
        let service = crate::service::Service::open(Config::default(), paths.clone()).unwrap();
        drop(service);
        let marker = paths.data_root.join(state::MARKER);
        let mut identity: serde_json::Value =
            serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
        identity["role"] = serde_json::json!("foreign");
        fs::write(&marker, serde_json::to_vec(&identity).unwrap()).unwrap();
        fs::write(paths.data_root.join("blobs/keep"), b"facts").unwrap();
        assert!(reset_development(&Config::default(), &root, true).is_err());
        assert!(paths.data_root.join("blobs/keep").exists());
    }
}
