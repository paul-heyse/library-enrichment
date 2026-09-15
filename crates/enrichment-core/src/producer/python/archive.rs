//! Bounded wheel/ZIP extraction and distribution inventories, never package execution.
use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Cursor, Read};
use std::path::{Component, Path};

use super::{Distribution, ObservationOrigin, WorkerFile, metadata_headers};
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

fn metadata_text(path: &Path) -> Result<String, String> {
    String::from_utf8(
        super::super::source::read_file_limited(path, 1024 * 1024).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
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

/// Read distribution files and metadata independently of Griffe.
///
/// # Errors
/// Metadata or selected roots cannot be read safely.
pub fn inventory(
    root: &Path,
    filename: &str,
    sha256: &str,
    source_root: &str,
) -> Result<Distribution, String> {
    let all = files(root)?;
    let mut result = Distribution {
        filename: filename.into(),
        sha256: sha256.into(),
        source_root: source_root.into(),
        ..Distribution::default()
    };
    let mut roots = BTreeSet::new();
    let mut metadata_file = None;
    let wheel = filename.ends_with(".whl");
    for file in &all {
        if file.ends_with(".dist-info/METADATA") || (!wheel && file == "PKG-INFO") {
            if metadata_file.replace(file).is_some() {
                return Err("multiple distribution metadata authorities in one archive".into());
            }
            result.metadata = metadata_headers(&metadata_text(&root.join(file))?);
        }
        if file.ends_with(".dist-info/entry_points.txt") {
            result.entry_points = Some(metadata_text(&root.join(file))?);
        }
        if file.ends_with("py.typed") {
            result.typed_markers.push(file.clone());
        }
        let mut import_path = file.as_str();
        if let Some((_, rest)) = file
            .split_once(".data/purelib/")
            .or_else(|| file.split_once(".data/platlib/"))
        {
            import_path = rest;
        } else if file.contains(".data/")
            || file.contains(".dist-info/")
            || file.contains(".egg-info/")
        {
            continue;
        }
        if !wheel && let Some(rest) = import_path.strip_prefix("src/") {
            import_path = rest;
        }
        let origin = if import_path.ends_with(".pyi") {
            Some(ObservationOrigin::Stub)
        } else if import_path.ends_with(".py") {
            Some(ObservationOrigin::Source)
        } else {
            None
        };
        if let Some(origin) = origin {
            let stem = import_path.rsplit_once('.').map_or(import_path, |(s, _)| s);
            let mut parts: Vec<String> = stem
                .split('/')
                .map(|p| p.strip_suffix("-stubs").unwrap_or(p).to_owned())
                .collect();
            if parts.last().is_some_and(|p| p == "__init__") {
                parts.pop();
            }
            if parts.is_empty()
                || parts
                    .iter()
                    .any(|p| p.is_empty() || !p.chars().all(|c| c.is_alphanumeric() || c == '_'))
            {
                continue;
            }
            roots.insert(parts[0].clone());
            result.files.push(WorkerFile {
                file: file.clone(),
                module: parts.join("."),
                origin,
            });
        } else if import_path.ends_with(".so") || import_path.ends_with(".pyd") {
            result.native_files.push(file.clone());
            if let Some(first) = import_path.split('/').next() {
                roots.insert(first.split('.').next().unwrap_or(first).into());
            }
        }
    }
    result.import_roots = roots.into_iter().collect();
    result.name = optional_scalar(&result.metadata, "name")?.map(str::to_owned);
    result.version = optional_scalar(&result.metadata, "version")?.map(str::to_owned);
    Ok(result)
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

/// Corroborate selected filename/index claims against the actual archived metadata.
///
/// # Errors
/// Rejects ambiguous identities, unsupported wheel versions, inconsistent tags or interpreter constraints.
pub fn validate_metadata(
    root: &Path,
    distribution: &Distribution,
    request: &crate::request::ResolveRequest,
    version: &str,
) -> Result<Vec<String>, String> {
    use pep440_rs::{Version, VersionSpecifiers};
    fn scalar<'a>(
        headers: &'a std::collections::BTreeMap<String, Vec<String>>,
        key: &str,
    ) -> Result<&'a str, String> {
        optional_scalar(headers, key)?
            .ok_or_else(|| format!("metadata requires one nonempty {key}"))
    }
    let name = scalar(&distribution.metadata, "name")?;
    let recorded: Version = scalar(&distribution.metadata, "version")?
        .parse()
        .map_err(|e| format!("invalid metadata version: {e}"))?;
    let selected: Version = version
        .parse()
        .map_err(|e| format!("invalid selected version: {e}"))?;
    if super::normalize_name(name) != request.name || recorded != selected {
        return Err("distribution metadata identity disagrees with selected release".into());
    }
    let mut warnings = Vec::new();
    if distribution.metadata.contains_key("requires-python") {
        let requirement: VersionSpecifiers = scalar(&distribution.metadata, "requires-python")?
            .parse()
            .map_err(|e| format!("invalid Requires-Python: {e}"))?;
        if let Some(interpreter) = &request.python_version {
            let interpreter: Version = interpreter
                .parse()
                .map_err(|e| format!("invalid interpreter: {e}"))?;
            if !requirement.contains(&interpreter) {
                return Err("internal Requires-Python excludes the declared interpreter".into());
            }
        } else {
            warnings
                .push("Requires-Python has not been checked against a declared interpreter".into());
        }
    }
    if !distribution.filename.ends_with(".whl") {
        return Ok(warnings);
    }
    let paths = files(root)?;
    let authorities: Vec<_> = paths
        .iter()
        .filter(|p| p.ends_with(".dist-info/METADATA"))
        .collect();
    let [metadata] = authorities.as_slice() else {
        return Err("wheel requires exactly one metadata authority".into());
    };
    let directory = metadata.strip_suffix("/METADATA").ok_or("metadata path")?;
    if directory.contains('/') {
        return Err("wheel metadata must be top-level".into());
    }
    let stem = distribution
        .filename
        .strip_suffix(".whl")
        .ok_or("wheel filename")?;
    let parts: Vec<_> = stem.split('-').collect();
    if !matches!(parts.len(), 5 | 6) {
        return Err("invalid wheel filename fields".into());
    }
    let filename_version: Version = parts[1]
        .parse()
        .map_err(|e| format!("filename version: {e}"))?;
    let (dir_name, dir_version) = directory
        .strip_suffix(".dist-info")
        .and_then(|p| p.rsplit_once('-'))
        .ok_or("dist-info identity")?;
    let dir_version: Version = dir_version
        .parse()
        .map_err(|e| format!("dist-info version: {e}"))?;
    if super::normalize_name(parts[0]) != request.name
        || filename_version != selected
        || super::normalize_name(dir_name) != request.name
        || dir_version != selected
    {
        return Err("wheel filename/dist-info/metadata identity mismatch".into());
    }
    let wheel = metadata_headers(
        &metadata_text(&root.join(directory).join("WHEEL")).map_err(|e| format!("WHEEL: {e}"))?,
    );
    let (major, minor) = scalar(&wheel, "wheel-version")?
        .split_once('.')
        .ok_or("invalid Wheel-Version")?;
    let major: u32 = major.parse().map_err(|_| "invalid wheel major version")?;
    let minor: u32 = minor.parse().map_err(|_| "invalid wheel minor version")?;
    if major != 1 {
        return Err("unsupported wheel major version".into());
    }
    if minor > 0 {
        warnings.push(format!(
            "Wheel-Version 1.{minor} is newer than supported 1.0; unknown fields ignored"
        ));
    }
    if !matches!(scalar(&wheel, "root-is-purelib")?, "true" | "false") {
        return Err("invalid Root-Is-Purelib".into());
    }
    let mut expected = BTreeSet::new();
    for py in parts[parts.len() - 3].split('.') {
        for abi in parts[parts.len() - 2].split('.') {
            for platform in parts[parts.len() - 1].split('.') {
                expected.insert(format!("{py}-{abi}-{platform}"));
            }
        }
    }
    let observed: BTreeSet<_> = wheel
        .get("tag")
        .ok_or("WHEEL Tag missing")?
        .iter()
        .cloned()
        .collect();
    if observed != expected {
        return Err("WHEEL tags disagree with expanded filename tags".into());
    }
    Ok(warnings)
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
    #[test]
    fn inventory_includes_namespace_stub_and_data_roots_without_scripts() {
        let dir = tempfile::tempdir().expect("dir");
        let data = zip(&[
            ("ns/part.py", b""),
            ("foo-stubs/__init__.pyi", b""),
            ("x.data/purelib/mapped.py", b""),
            ("x.data/scripts/ignored.py", b""),
            ("foo-stubs/py.typed", b"partial\n"),
        ]);
        extract_zip(&data, dir.path(), &ArchivePolicy::default()).expect("extract");
        let got = inventory(dir.path(), "x.whl", "digest", "").expect("inventory");
        assert_eq!(got.import_roots, vec!["foo", "mapped", "ns"]);
        assert_eq!(got.files.len(), 3);
        assert_eq!(got.typed_markers, vec!["foo-stubs/py.typed"]);
    }
    #[test]
    fn wheel_src_package_is_not_sdist_layout_and_metadata_is_unambiguous() {
        let dir = tempfile::tempdir().expect("dir");
        let data = zip(&[("src/__init__.py", b""), ("src/api.py", b"")]);
        extract_zip(&data, dir.path(), &ArchivePolicy::default()).expect("extract");
        let wheel = inventory(dir.path(), "x.whl", "digest", "").expect("wheel");
        assert_eq!(wheel.import_roots, vec!["src"]);
        assert_eq!(wheel.files[1].module, "src.api");
        let source = inventory(dir.path(), "x.tar.gz", "digest", "").expect("source");
        assert_eq!(source.import_roots, vec!["api"]);
        for name in ["first", "second"] {
            let metadata = dir.path().join(format!("{name}.dist-info"));
            fs::create_dir(&metadata).expect("metadata dir");
            fs::write(metadata.join("METADATA"), "Name: x\nVersion: 1\n").expect("metadata");
        }
        assert!(
            inventory(dir.path(), "x.whl", "digest", "")
                .expect_err("ambiguous")
                .contains("multiple")
        );
    }
    #[test]
    fn internal_metadata_corroborates_identity_tags_and_python_requirements() {
        let dir = tempfile::tempdir().expect("dir");
        let metadata = dir.path().join("pkg-1.0.dist-info");
        fs::create_dir(&metadata).expect("dir");
        fs::write(
            metadata.join("METADATA"),
            "Name: pkg\nVersion: 1.0.0\nRequires-Python: >=3.10\n",
        )
        .expect("metadata");
        let wheel =
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py2-none-any\nTag: py3-none-any\n";
        fs::write(metadata.join("WHEEL"), wheel).expect("wheel");
        let distribution =
            inventory(dir.path(), "pkg-1.0-py2.py3-none-any.whl", "digest", "").expect("inventory");
        let mut request = crate::request::ResolveRequest {
            name: "pkg".into(),
            python_version: Some("3.14".into()),
            ..Default::default()
        };
        assert!(
            validate_metadata(dir.path(), &distribution, &request, "1.0")
                .expect("valid")
                .is_empty()
        );
        request.python_version = Some("3.9".into());
        assert!(
            validate_metadata(dir.path(), &distribution, &request, "1.0")
                .expect_err("internal constraint")
                .contains("Requires-Python")
        );
        request.python_version = Some("3.14".into());
        fs::write(
            metadata.join("WHEEL"),
            wheel.replace("Tag: py2-none-any\n", ""),
        )
        .expect("wheel");
        assert!(
            validate_metadata(dir.path(), &distribution, &request, "1.0")
                .expect_err("tags")
                .contains("tags disagree")
        );
        fs::write(metadata.join("WHEEL"), wheel.replace("1.0", "2.0")).expect("wheel");
        assert!(
            validate_metadata(dir.path(), &distribution, &request, "1.0")
                .expect_err("major")
                .contains("major version")
        );
        fs::write(metadata.join("WHEEL"), wheel.replace("1.0", "1.1")).expect("wheel");
        assert_eq!(
            validate_metadata(dir.path(), &distribution, &request, "1.0")
                .expect("minor warning")
                .len(),
            1
        );
    }
}
