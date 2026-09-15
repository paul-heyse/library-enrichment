//! Durable association of a cache owner with every execution root it has used.
use enrichment_core::{canonical, config::Execution};
use enrichment_store::StatePaths;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const RECORD: &str = ".execution-roots.json";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    pub path: PathBuf,
    pub broker: PathBuf,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Registry {
    version: u32,
    owner: String,
    roots: Vec<Root>,
}

fn owner(cache: &Path) -> io::Result<String> {
    Ok(canonical::sha256_hex(
        cache.canonicalize()?.to_string_lossy().as_bytes(),
    ))
}

pub fn read(cache: &Path) -> io::Result<Vec<Root>> {
    let path = cache.join(RECORD);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(value) => value,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    if !metadata.is_file() || metadata.len() > 1_048_576 {
        return Err(io::Error::other("invalid execution-root registry"));
    }
    let record: Registry = serde_json::from_slice(&fs::read(path)?)?;
    if record.version != 1 || record.owner != owner(cache)? || record.roots.len() > 64 {
        return Err(io::Error::other(
            "execution-root registry ownership or bound mismatch",
        ));
    }
    for root in &record.roots {
        validate_root(cache, root)?;
    }
    Ok(record.roots)
}

fn validate_root(cache: &Path, root: &Root) -> io::Result<()> {
    if !root.broker.is_absolute()
        || !root.path.is_absolute()
        || root.path.parent().is_none()
        || root.path == cache
        || cache.starts_with(&root.path)
        || (root.path.starts_with(cache) && !root.path.starts_with(cache.join("podman")))
    {
        return Err(io::Error::other(
            "execution root overlaps cache payloads or has an invalid broker path",
        ));
    }
    if root.path.canonicalize()? != root.path {
        return Err(io::Error::other(
            "execution root is no longer the recorded physical path",
        ));
    }
    Ok(())
}

/// Register before the first container can be created. The caller holds cache ownership.
pub fn register(cache: &Path, root: &Root) -> io::Result<()> {
    validate_root(cache, root)?;
    let mut roots = read(cache)?;
    if roots.contains(root) {
        return Ok(());
    }
    if roots.len() >= 64 {
        return Err(io::Error::other(
            "execution-root history bound exceeded; reconcile operator configuration",
        ));
    }
    roots.push(root.clone());
    roots.sort();
    enrichment_store::atomic::write_atomic(
        &cache.join(RECORD),
        &serde_json::to_vec(&Registry {
            version: 1,
            owner: owner(cache)?,
            roots,
        })?,
    )
}

/// Validate historical as well as current roots before startup or any cleanup candidate deletion.
pub fn validate_for_state(paths: &StatePaths, execution: &Execution) -> io::Result<()> {
    let mut roots = read(&paths.cache_root)?;
    roots.push(Root {
        path: super::admission::execution_root(execution, &paths.cache_root),
        broker: execution.broker(),
    });
    for root in roots {
        // The selected root may not exist yet, but it must already be disjoint from evidence
        // and all deletable cache payloads. Existing registered roots also pass physical checks.
        if root.path.starts_with(&paths.data_root)
            || paths.data_root.starts_with(&root.path)
            || root.path == paths.cache_root
            || paths.cache_root.starts_with(&root.path)
            || (root.path.starts_with(&paths.cache_root)
                && !root.path.starts_with(paths.cache_root.join("podman")))
        {
            return Err(io::Error::other(
                "execution storage overlaps evidence or deletable cache state",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn root_changes_keep_prior_owners_and_reject_payload_overlap() {
        let temp = tempfile::tempdir().unwrap();
        let paths = StatePaths::explicit(temp.path().join("cache"), temp.path().join("data"));
        enrichment_store::state::initialize(&paths).unwrap();
        for name in ["engine-a", "engine-b"] {
            let root = temp.path().join(name);
            fs::create_dir(&root).unwrap();
            register(
                &paths.cache_root,
                &Root {
                    path: root,
                    broker: "/usr/bin/podman".into(),
                },
            )
            .unwrap();
        }
        assert_eq!(read(&paths.cache_root).unwrap().len(), 2);
        let mut config = Execution {
            storage_root: Some(paths.cache_root.join("capsules")),
            ..Execution::default()
        };
        assert!(validate_for_state(&paths, &config).is_err());
        config.storage_root = Some(paths.data_root.join("engine"));
        assert!(validate_for_state(&paths, &config).is_err());
    }
}
