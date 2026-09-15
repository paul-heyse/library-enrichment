//! Explicit ownership and format identity for operator-managed service roots.
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::StatePaths;

pub const MARKER: &str = ".library-enrichment-state.json";
/// Operator-owned sidecars select this generation explicitly; older roots stay inactive.
pub const GENERATION: u32 = 6;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Identity {
    format: String,
    role: String,
    root: std::path::PathBuf,
}

/// Validate absolute, nonoverlapping, physical roots before creating service directories.
/// # Errors
/// Relative paths, root directories, links and overlapping roots are refused.
pub fn validate_paths(paths: &StatePaths) -> io::Result<()> {
    for root in [&paths.data_root, &paths.cache_root] {
        if !root.is_absolute() || root.parent().is_none() {
            return Err(io::Error::other(
                "state root must be an absolute dedicated directory",
            ));
        }
        for ancestor in root.ancestors() {
            match fs::symlink_metadata(ancestor) {
                Ok(meta) if !meta.is_dir() => {
                    return Err(io::Error::other(
                        "state root has a link or non-directory ancestor",
                    ));
                }
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                Err(e) => return Err(e),
            }
        }
        if root.components().any(|c| {
            !matches!(
                c,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        }) {
            return Err(io::Error::other("state root must be normalized"));
        }
    }
    if paths.data_root.starts_with(&paths.cache_root)
        || paths.cache_root.starts_with(&paths.data_root)
    {
        return Err(io::Error::other(
            "cache and evidence roots must not overlap",
        ));
    }
    Ok(())
}

/// Initialize only new empty roots, or verify the existing target identity.
/// Existing unmarked evidence is never silently adopted as the target format.
/// # Errors
/// An incompatible or unmarked nonempty root needs the explicit development reset.
pub fn initialize(paths: &StatePaths) -> io::Result<()> {
    validate_paths(paths)?;
    for (root, role) in [(&paths.data_root, "evidence"), (&paths.cache_root, "cache")] {
        fs::create_dir_all(root)?;
        if fs::symlink_metadata(root.join(MARKER)).is_ok() {
            verify_root(root, role)?;
        } else {
            if !only_coordination_files(root, role)? {
                return Err(io::Error::other(format!(
                    "unmarked nonempty {role} root {}; explicitly reset development state before using the target format",
                    root.display()
                )));
            }
            mark_root(root, role)?;
        }
    }
    Ok(())
}

fn only_coordination_files(root: &Path, role: &str) -> io::Result<bool> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let allowed: &[&str] = if role == "evidence" {
            &[".daemon.lock", crate::leases::LOCK_FILE]
        } else {
            &[".execution-owner.lock", "capsule-storage.lock"]
        };
        let metadata = fs::symlink_metadata(entry.path())?;
        if !entry
            .file_name()
            .to_str()
            .is_some_and(|s| allowed.contains(&s))
            || !metadata.is_file()
            || metadata.len() != 0
        {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Development cutover may encounter no marker; any marker that does exist must already
/// prove ownership of this exact root before the first deletion.
pub fn verify_existing_markers(paths: &StatePaths) -> io::Result<()> {
    validate_paths(paths)?;
    for (root, role) in [(&paths.data_root, "evidence"), (&paths.cache_root, "cache")] {
        match fs::symlink_metadata(root.join(MARKER)) {
            Ok(_) => verify_root(root, role)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Verify both existing ownership markers without changing files.
/// # Errors
/// Missing or altered markers, links and unsupported formats fail closed.
pub fn verify(paths: &StatePaths) -> io::Result<()> {
    validate_paths(paths)?;
    verify_root(&paths.data_root, "evidence")?;
    verify_root(&paths.cache_root, "cache")
}

fn identity(root: &Path, role: &str) -> io::Result<Identity> {
    Ok(Identity {
        format: format!("library-enrichment-state/{GENERATION}"),
        role: role.into(),
        root: root.canonicalize()?,
    })
}

fn verify_root(root: &Path, role: &str) -> io::Result<()> {
    let path = root.join(MARKER);
    let meta = fs::symlink_metadata(&path)?;
    if !meta.is_file() || meta.len() > 4096 {
        return Err(io::Error::other("invalid state ownership marker"));
    }
    let actual: Identity = serde_json::from_slice(&fs::read(path)?)?;
    if actual != identity(root, role)? {
        return Err(io::Error::other(
            "state ownership or format identity mismatch",
        ));
    }
    Ok(())
}

/// Mark roots only after the explicit development cutover has removed old service payloads.
/// The caller holds all ownership locks and preserves unrelated files.
/// # Errors
/// Existing incompatible markers and persistence failures are refused.
pub fn mark_reset(paths: &StatePaths) -> io::Result<()> {
    mark_root(&paths.data_root, "evidence")?;
    mark_root(&paths.cache_root, "cache")
}

fn mark_root(root: &Path, role: &str) -> io::Result<()> {
    let marker = root.join(MARKER);
    if fs::symlink_metadata(&marker).is_ok() {
        return verify_root(root, role);
    }
    crate::atomic::write_atomic(&marker, &serde_json::to_vec(&identity(root, role)?)?)?;
    fs::File::open(root)?.sync_all()
}

/// Open a permanent empty lock inode. Never unlink these files during cleanup.
/// # Errors
/// Existing links, replacement races and active ownership are refused immediately.
pub fn exclusive(root: &Path, name: &str) -> io::Result<fs::File> {
    if !matches!(
        name,
        ".daemon.lock" | ".execution-owner.lock" | "capsule-storage.lock"
    ) {
        return Err(io::Error::other("unknown state coordination lock"));
    }
    let path = root.join(name);
    match fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
    {
        Ok(file) => {
            file.sync_all()?;
            fs::File::open(root)?.sync_all()?;
        }
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
        Err(e) => return Err(e),
    }
    let before = fs::symlink_metadata(&path)?;
    if !before.is_file() || before.len() != 0 {
        return Err(io::Error::other("invalid state coordination lock"));
    }
    let file = fs::File::open(&path)?;
    file.try_lock().map_err(io::Error::from)?;
    use std::os::unix::fs::MetadataExt;
    let after = fs::symlink_metadata(path)?;
    let actual = file.metadata()?;
    if !after.is_file()
        || actual.len() != 0
        || (before.dev(), before.ino()) != (actual.dev(), actual.ino())
        || (after.dev(), after.ino()) != (actual.dev(), actual.ino())
    {
        return Err(io::Error::other("state coordination lock was replaced"));
    }
    Ok(file)
}

/// Remove only bounded unpublished snapshot scratch after the daemon owns its writer lock.
/// Completed snapshot directories and catalog files are outside this cleanup authority.
/// # Errors
/// Unknown paths, links, excessive inventories or I/O errors require operator inspection.
pub fn recover_staging(paths: &crate::StatePaths) -> std::io::Result<usize> {
    use std::{fs, io, os::unix::fs::MetadataExt};
    let root = paths.staging();
    match fs::symlink_metadata(&root) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(e),
        Ok(m) if !m.is_dir() => {
            return Err(io::Error::other("snapshot staging root is not a directory"));
        }
        Ok(_) => {}
    }
    let mut candidates = Vec::new();
    for entry in fs::read_dir(&root)? {
        if candidates.len() >= 1024 {
            return Err(io::Error::other(
                "unpublished snapshot directory bound exceeded",
            ));
        }
        let entry = entry?;
        let name = entry.file_name();
        let name = name
            .to_str()
            .ok_or_else(|| io::Error::other("non-UTF8 snapshot scratch name"))?;
        let suffix = [
            "evidence-contribution-",
            "evidence-environment-",
            "evidence-union-",
            "evidence-",
        ]
        .iter()
        .find_map(|prefix| name.strip_prefix(prefix));
        if !entry.file_type()?.is_dir()
            || suffix.is_none_or(|s| {
                !(6..=32).contains(&s.len()) || !s.bytes().all(|b| b.is_ascii_alphanumeric())
            })
        {
            return Err(io::Error::other(
                "unrecognized unpublished snapshot directory",
            ));
        }
        let mut files = Vec::new();
        let mut directories = Vec::new();
        for file in fs::read_dir(entry.path())? {
            if files.len() >= 32 {
                return Err(io::Error::other("unpublished snapshot file bound exceeded"));
            }
            let file = file?;
            let name = file.file_name();
            let name = name
                .to_str()
                .ok_or_else(|| io::Error::other("non-UTF8 snapshot scratch file"))?;
            if name
                .strip_prefix("producer-references-")
                .is_some_and(|suffix| {
                    (6..=32).contains(&suffix.len())
                        && suffix.bytes().all(|b| b.is_ascii_alphanumeric())
                })
                && file.file_type()?.is_dir()
            {
                if !directories.is_empty() {
                    return Err(io::Error::other("multiple producer staging directories"));
                }
                for child in fs::read_dir(file.path())? {
                    let child = child?;
                    let metadata = fs::symlink_metadata(child.path())?;
                    if !matches!(
                        child.file_name().to_str(),
                        Some(
                            "producer_bindings.parquet"
                                | "producer_relationships.parquet"
                                | "producer_fragments.parquet"
                                | "producer_inputs.parquet"
                                | "producer_artifacts.parquet"
                        )
                    ) || !metadata.is_file()
                        || metadata.nlink() != 1
                        || metadata.len() > 256 * 1024 * 1024
                        || files.len() >= 32
                    {
                        return Err(io::Error::other("unsafe or unknown producer staging child"));
                    }
                    files.push(child.path());
                }
                directories.push(file.path());
                continue;
            }
            let expected = name == "manifest.json"
                || crate::admission::Relation::ALL
                    .iter()
                    .any(|r| name == format!("{}.parquet", r.name()))
                || name
                    .strip_prefix(".manifest.json.")
                    .and_then(|s| s.strip_suffix(".tmp"))
                    .is_some_and(|s| {
                        s.split_once('-').is_some_and(|(pid, count)| {
                            !pid.is_empty()
                                && !count.is_empty()
                                && pid.bytes().chain(count.bytes()).all(|b| b.is_ascii_digit())
                        })
                    });
            let metadata = fs::symlink_metadata(file.path())?;
            if !expected
                || !metadata.is_file()
                || metadata.nlink() != 1
                || metadata.len() > 256 * 1024 * 1024
            {
                return Err(io::Error::other(
                    "unsafe or unrecognized unpublished snapshot file",
                ));
            }
            files.push(file.path());
        }
        candidates.push((entry.path(), files, directories));
    }
    let count = candidates.len();
    // Complete validation precedes deletion. The service writer lock excludes publication.
    for (directory, files, children) in candidates {
        for file in files {
            fs::remove_file(file)?;
        }
        for child in children {
            fs::remove_dir(child)?;
        }
        fs::remove_dir(directory)?;
    }
    fs::File::open(root)?.sync_all()?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn staging_recovery_validates_every_candidate_before_deleting_anything() {
        let temp = tempfile::tempdir().unwrap();
        let paths = StatePaths::explicit(temp.path().join("cache"), temp.path().join("data"));
        initialize(&paths).unwrap();
        let valid = paths.staging().join("evidence-abcdef");
        fs::create_dir_all(&valid).unwrap();
        fs::write(valid.join("manifest.json"), "incomplete output").unwrap();
        fs::create_dir(valid.join("producer-references-abcdef")).unwrap();
        fs::write(
            valid.join("producer-references-abcdef/producer_bindings.parquet"),
            "interrupted ingestion",
        )
        .unwrap();
        let bad = paths.staging().join("evidence-uvwxyz");
        fs::create_dir(&bad).unwrap();
        fs::write(bad.join("unknown"), "preserve").unwrap();
        assert!(recover_staging(&paths).is_err());
        assert!(valid.join("manifest.json").exists());
        fs::remove_file(bad.join("unknown")).unwrap();
        fs::hard_link(valid.join("manifest.json"), bad.join("manifest.json")).unwrap();
        assert!(recover_staging(&paths).is_err());
        fs::remove_file(bad.join("manifest.json")).unwrap();
        std::os::unix::fs::symlink(valid.join("manifest.json"), bad.join("manifest.json")).unwrap();
        assert!(recover_staging(&paths).is_err());
        fs::remove_file(bad.join("manifest.json")).unwrap();
        assert_eq!(recover_staging(&paths).unwrap(), 2);
        assert!(paths.staging().read_dir().unwrap().next().is_none());
    }

    #[test]
    fn state_identity_refuses_legacy_roots_links_and_overlap() {
        let temp = tempfile::tempdir().unwrap();
        let paths = StatePaths::explicit(temp.path().join("cache"), temp.path().join("data"));
        initialize(&paths).unwrap();
        verify(&paths).unwrap();
        assert!(
            validate_paths(&StatePaths::explicit(
                &paths.data_root,
                paths.data_root.join("nested")
            ))
            .is_err()
        );
        let other = temp.path().join("old");
        fs::create_dir(&other).unwrap();
        fs::write(other.join("evidence"), "old").unwrap();
        assert!(initialize(&StatePaths::explicit(&paths.cache_root, &other)).is_err());
        fs::remove_file(paths.data_root.join(MARKER)).unwrap();
        std::os::unix::fs::symlink(paths.cache_root.join(MARKER), paths.data_root.join(MARKER))
            .unwrap();
        assert!(verify(&paths).is_err());
    }

    #[test]
    fn the_previous_generation_is_rejected_without_adopting_or_removing_bytes() {
        let temp = tempfile::tempdir().unwrap();
        let paths = StatePaths::explicit(temp.path().join("cache"), temp.path().join("data"));
        initialize(&paths).unwrap();
        for root in [&paths.cache_root, &paths.data_root] {
            let marker = root.join(MARKER);
            let mut old: serde_json::Value =
                serde_json::from_slice(&fs::read(&marker).unwrap()).unwrap();
            old["format"] = "library-enrichment-state/5".into();
            fs::write(&marker, serde_json::to_vec(&old).unwrap()).unwrap();
            fs::write(
                root.join("old-retained-payload"),
                b"preserve old generation",
            )
            .unwrap();
        }
        let before: Vec<_> = [&paths.cache_root, &paths.data_root]
            .into_iter()
            .map(|root| {
                (
                    fs::read(root.join(MARKER)).unwrap(),
                    fs::read(root.join("old-retained-payload")).unwrap(),
                )
            })
            .collect();
        assert!(initialize(&paths).is_err());
        assert!(verify(&paths).is_err());
        let after: Vec<_> = [&paths.cache_root, &paths.data_root]
            .into_iter()
            .map(|root| {
                (
                    fs::read(root.join(MARKER)).unwrap(),
                    fs::read(root.join("old-retained-payload")).unwrap(),
                )
            })
            .collect();
        assert_eq!(before, after);
        let fresh = StatePaths::explicit(
            temp.path().join("fresh-cache"),
            temp.path().join("fresh-data"),
        );
        initialize(&fresh).unwrap();
        verify(&fresh).unwrap();
    }
}
