//! Bounded wheel/ZIP extraction and distribution inventories, never package execution.
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Cursor, Read};
use std::path::{Component, Path};

use crate::policy::ArchivePolicy;

/// Extract a ZIP into an empty service-owned staging directory.
///
/// # Errors
/// Refuses duplicate or escaping names, links/devices, encryption, ZIP64, and resource excess.
pub fn extract_zip(bytes: &[u8], destination: &Path, policy: &ArchivePolicy) -> Result<(), String> {
    let lower = bytes.len().saturating_sub(65_557);
    let end = (lower..bytes.len().saturating_sub(21))
        .rev()
        .find(|&i| {
            bytes.get(i..i + 4) == Some(b"PK\x05\x06")
                && i + 22 + usize::from(u16::from_le_bytes([bytes[i + 20], bytes[i + 21]]))
                    == bytes.len()
        })
        .ok_or("ZIP end record missing")?;
    let n = usize::from(u16::from_le_bytes([bytes[end + 10], bytes[end + 11]]));
    if n == 65_535 || n > policy.max_entries || bytes[end + 4..end + 8] != [0, 0, 0, 0] {
        return Err("ZIP64, multi-disk or too many entries unsupported".into());
    }
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|e| e.to_string())?;
    if archive.len() != n {
        return Err("duplicate ZIP names or inconsistent entry count".into());
    }
    fs::create_dir_all(destination).map_err(|e| e.to_string())?;
    if fs::read_dir(destination)
        .map_err(|e| e.to_string())?
        .next()
        .is_some()
    {
        return Err("ZIP destination is not empty".into());
    }
    let mut seen = BTreeSet::new();
    let mut physical_paths = BTreeSet::new();
    let mut total = 0u64;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let raw = file.name().to_owned();
        if raw.contains(['\\', ':', '\0'])
            || Path::new(&raw).components().any(|c| {
                matches!(
                    c,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(format!("unsafe ZIP path {raw}"));
        }
        let relative = file
            .enclosed_name()
            .ok_or_else(|| format!("unsafe ZIP path {raw}"))?;
        if !seen.insert(relative.clone()) {
            return Err(format!("duplicate ZIP path {raw}"));
        }
        let kind = file.unix_mode().unwrap_or(0) & 0o170000;
        if file.encrypted() || file.is_symlink() || !matches!(kind, 0 | 0o100000 | 0o040000) {
            return Err(format!("non-regular or encrypted ZIP entry {raw}"));
        }
        total = total.checked_add(file.size()).ok_or("ZIP size overflow")?;
        if total > policy.max_total_bytes || file.size() > policy.max_entry_bytes {
            return Err("ZIP exceeds decompressed byte bound".into());
        }
        for path in relative
            .ancestors()
            .filter(|path| !path.as_os_str().is_empty())
        {
            physical_paths.insert(path.to_owned());
            if physical_paths.len() > policy.max_entries {
                return Err("ZIP physical entries exceed bound".into());
            }
        }
        let path = destination.join(relative);
        if file.is_dir() {
            fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            continue;
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let size = file.size();
        let mut output = fs::File::create(path).map_err(|e| e.to_string())?;
        let copied = io::copy(&mut file.by_ref().take(size.saturating_add(1)), &mut output)
            .map_err(|e| e.to_string())?;
        if copied != size {
            return Err("ZIP entry length mismatch".into());
        }
    }
    Ok(())
}

/// Inventory regular files without following links.
///
/// # Errors
/// A symlink, special file, invalid relative name or I/O error fails inventory.
pub fn files(root: &Path) -> Result<Vec<String>, String> {
    let mut pending = vec![root.to_owned()];
    let mut files = Vec::new();
    let mut entries = 0usize;
    let mut path_bytes = 0usize;
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            entries += 1;
            path_bytes = path_bytes
                .checked_add(entry.path().as_os_str().len())
                .ok_or("source path byte overflow")?;
            if entries > 20_000 || path_bytes > 16 * 1024 * 1024 {
                return Err("source inventory exceeds entry/path budget".into());
            }
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            if kind.is_dir() {
                pending.push(entry.path());
            } else if kind.is_file() {
                files.push(
                    entry
                        .path()
                        .strip_prefix(root)
                        .map_err(|e| e.to_string())?
                        .to_str()
                        .ok_or("non-UTF8 archive path")?
                        .to_owned(),
                );
            } else {
                return Err("links and special files cannot enter a worker view".into());
            }
        }
    }
    files.sort();
    Ok(files)
}

/// A scalar is absent or exactly one nonempty parsed header; malformed presence is not absence.
/// # Errors
/// Duplicates, empty vectors and blank values are invalid even when duplicate values agree.
pub fn optional_scalar<'a>(
    headers: &'a std::collections::BTreeMap<String, Vec<String>>,
    key: &str,
) -> Result<Option<&'a str>, String> {
    match headers.get(key).map(Vec::as_slice) {
        None => Ok(None),
        Some([value]) if !value.trim().is_empty() => Ok(Some(value)),
        _ => Err(format!(
            "metadata requires exactly one nonempty {key} when present"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn zip(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, bytes) in files {
            writer
                .start_file(*name, zip::write::SimpleFileOptions::default())
                .expect("file");
            writer.write_all(bytes).expect("bytes");
        }
        writer.finish().expect("finish").into_inner()
    }
    #[test]
    fn implicit_zip_directories_are_bounded_before_creation() {
        let bytes = zip(&[("root/deep/member.txt", b"bounded")]);
        let dir = tempfile::tempdir().unwrap();
        let policy = ArchivePolicy {
            max_entries: 2,
            ..ArchivePolicy::default()
        };
        assert!(extract_zip(&bytes, dir.path(), &policy).is_err());
        assert!(fs::read_dir(dir.path()).unwrap().next().is_none());
        let policy = ArchivePolicy {
            max_entries: 3,
            ..policy
        };
        extract_zip(&bytes, dir.path(), &policy).unwrap();
        assert_eq!(
            fs::read(dir.path().join("root/deep/member.txt")).unwrap(),
            b"bounded"
        );
    }

    #[test]
    fn wheel_extraction_rejects_escape_and_enforces_actual_bounds() {
        let dir = tempfile::tempdir().expect("dir");
        assert!(
            extract_zip(
                &zip(&[("../outside", b"bad")]),
                &dir.path().join("bad"),
                &ArchivePolicy::default()
            )
            .is_err()
        );
        assert!(!dir.path().join("outside").exists());
        let policy = ArchivePolicy {
            max_entry_bytes: 2,
            ..ArchivePolicy::default()
        };
        assert!(
            extract_zip(
                &zip(&[("large", b"long")]),
                &dir.path().join("large"),
                &policy
            )
            .is_err()
        );
        let safe = zip(&[("pkg/__init__.py", b"pass\n")]);
        extract_zip(&safe, &dir.path().join("safe"), &ArchivePolicy::default()).expect("safe");
        assert_eq!(
            fs::read(dir.path().join("safe/pkg/__init__.py")).expect("read"),
            b"pass\n"
        );
    }
}
