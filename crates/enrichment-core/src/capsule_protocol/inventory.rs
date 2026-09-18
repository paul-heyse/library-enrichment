//! Exact retained-input inventories; unreadable or special files are errors, never zero bytes.
use crate::canonical;
use std::collections::BTreeMap;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

crate::native_union! {
    pub enum Entry {
        Directory = "directory" { mode: u32 => crate::native_union::Rule::Text },
        File = "file" { mode: u32 => crate::native_union::Rule::Text, bytes: u64 => crate::native_union::Rule::Text, sha256: String => crate::native_union::Rule::Sha256 },
    }
}

pub type Inventory = BTreeMap<String, Entry>;

/// The executor and daemon bind the same typed inventory, including file modes and sizes.
pub fn digest(entries: &Inventory) -> io::Result<String> {
    crate::native_key::Key::ProcessInventory
        .hex_digest(&crate::operation::identities::ProcessInventory {
            entries: entries.clone(),
        })
        .map_err(io::Error::other)
}

impl Entry {
    pub fn metadata_bytes(&self, path: &str) -> io::Result<usize> {
        Ok(serde_json::to_vec(&(path, self))?.len())
    }
    pub fn mode(&self) -> u32 {
        match self {
            Self::Directory { mode } | Self::File { mode, .. } => *mode,
        }
    }
    pub fn bytes(&self) -> u64 {
        match self {
            Self::Directory { .. } => 0,
            Self::File { bytes, .. } => *bytes,
        }
    }
}

/// Only explicitly selected outputs, inventoried after target quiescence.
pub fn selected(
    root: &Path,
    outputs: &BTreeMap<String, super::OutputKind>,
    byte_limit: u64,
) -> io::Result<Inventory> {
    let mut result = Inventory::new();
    let mut remaining = byte_limit;
    for (name, kind) in outputs {
        super::validate_path(name)?;
        let mut ancestor = root.to_owned();
        for component in Path::new(name)
            .parent()
            .into_iter()
            .flat_map(Path::components)
        {
            ancestor.push(component);
            if !std::fs::symlink_metadata(&ancestor)?.file_type().is_dir() {
                return Err(io::Error::other("output parent is a link or special file"));
            }
        }
        let path = root.join(name);
        let metadata = std::fs::symlink_metadata(&path)?;
        match kind {
            super::OutputKind::File => {
                let entry = file_entry(&path, &metadata, remaining)?;
                remaining -= entry.bytes();
                result.insert(name.clone(), entry);
            }
            super::OutputKind::Directory => {
                if !metadata.file_type().is_dir() {
                    return Err(io::Error::other("output directory has wrong kind"));
                }
                result.insert(
                    name.clone(),
                    Entry::Directory {
                        mode: metadata.mode() & 0o777,
                    },
                );
                for (relative, entry) in capture(&path, remaining)? {
                    let full = format!("{name}/{relative}");
                    super::validate_path(&full)?;
                    remaining -= entry.bytes();
                    result.insert(full, entry);
                }
            }
        }
        if result.len() > super::ENTRY_LIMIT {
            return Err(io::Error::other("output count exceeds bound"));
        }
        let metadata_bytes = result.iter().try_fold(0usize, |sum, (path, entry)| {
            sum.checked_add(entry.metadata_bytes(path)?)
                .ok_or_else(|| io::Error::other("inventory metadata overflow"))
        })?;
        if metadata_bytes > super::INVENTORY_METADATA_LIMIT {
            return Err(io::Error::other("inventory metadata exceeds bound"));
        }
    }
    Ok(result)
}

fn file_entry(path: &Path, metadata: &std::fs::Metadata, limit: u64) -> io::Result<Entry> {
    if !metadata.file_type().is_file() || metadata.nlink() != 1 {
        return Err(io::Error::other("capsule file is a link or special file"));
    }
    let (sha256, bytes) = canonical::sha256_reader(std::fs::File::open(path)?, limit)?;
    Ok(Entry::File {
        mode: metadata.mode() & 0o777,
        bytes,
        sha256,
    })
}

/// Capture every regular file and directory under this root, following no links.
pub fn capture(root: &Path, byte_limit: u64) -> io::Result<Inventory> {
    capture_bounded(root, byte_limit, super::ENTRY_LIMIT)
}

/// Source extractors and contained producers share one physical inventory reader, with
/// the entry ceiling of their declared input class.
pub fn capture_bounded(root: &Path, byte_limit: u64, entry_limit: usize) -> io::Result<Inventory> {
    if entry_limit == 0 {
        return Err(io::Error::other(
            "input inventory requires a positive entry bound",
        ));
    }
    if !std::fs::symlink_metadata(root)?.file_type().is_dir() {
        return Err(io::Error::other("retained input root is not a directory"));
    }
    let mut pending = vec![root.to_owned()];
    let mut result = Inventory::new();
    let mut used = 0u64;
    let mut metadata_bytes = 0usize;
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(root).map_err(io::Error::other)?;
            let name = relative
                .to_str()
                .ok_or_else(|| io::Error::other("non-UTF8 retained input"))?;
            super::validate_path(name)?;
            if result.len() >= entry_limit {
                return Err(io::Error::other(
                    "retained input inventory exceeds its path/count bound",
                ));
            }
            let name = name.to_owned();
            let metadata = std::fs::symlink_metadata(&path)?;
            let mode = metadata.permissions().mode() & 0o777;
            let stamp = if metadata.file_type().is_dir() {
                pending.push(path);
                Entry::Directory { mode }
            } else if metadata.file_type().is_file() {
                let entry = file_entry(&path, &metadata, byte_limit.saturating_sub(used))?;
                let bytes = entry.bytes();
                used = used
                    .checked_add(bytes)
                    .ok_or_else(|| io::Error::other("inventory size overflow"))?;
                entry
            } else {
                return Err(io::Error::other(
                    "retained input contains a symlink or special file",
                ));
            };
            metadata_bytes = metadata_bytes
                .checked_add(stamp.metadata_bytes(&name)?)
                .ok_or_else(|| io::Error::other("inventory metadata overflow"))?;
            if metadata_bytes > super::INVENTORY_METADATA_LIMIT {
                return Err(io::Error::other("inventory metadata exceeds bound"));
            }
            result.insert(name, stamp);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_extra_missing_linked_and_oversized_inputs_are_not_reused() {
        let root = tempfile::tempdir().expect("inputs");
        let file = root.path().join("module.py");
        std::fs::write(&file, b"original").expect("write");
        let original = capture(root.path(), 16).expect("inventory");
        std::fs::write(&file, b"modified").expect("mutate same length");
        assert_ne!(original, capture(root.path(), 16).expect("changed"));
        std::fs::write(&file, b"original").expect("restore");
        std::fs::write(root.path().join("extra.py"), b"extra").expect("extra");
        assert_ne!(original, capture(root.path(), 16).expect("extra inventory"));
        assert!(capture(root.path(), 8).is_err());
        std::fs::remove_file(&file).expect("remove");
        assert_ne!(original, capture(root.path(), 16).expect("missing"));
        std::os::unix::fs::symlink(root.path().join("extra.py"), file).expect("link");
        assert!(capture(root.path(), 16).is_err());
        std::fs::remove_file(root.path().join("module.py")).expect("remove symlink");
        std::fs::hard_link(root.path().join("extra.py"), root.path().join("module.py"))
            .expect("hard link");
        assert!(capture(root.path(), 16).is_err());
    }
}
