//! Safe extraction of `.crate` archives (gzip-compressed tar), blueprint §10.
//!
//! Every check happens on the entry header *before* anything is written, so a hostile archive
//! is rejected without a byte landing outside the destination (gate C11). Refused outright:
//! absolute paths, `..` components, symlinks, hard links, character and block devices, FIFOs,
//! more entries than the policy allows, and any entry or total larger than the policy allows.
//!
//! Extraction goes through this module's own loop rather than `tar::Archive::unpack`, so the
//! policy is applied to each header by code that can be read in one screen, and so the
//! destination is required to be empty: nothing here ever merges into an existing tree.

use std::fs;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Component, Path, PathBuf};

use flate2::read::GzDecoder;
use tar::EntryType;

use crate::policy::ArchivePolicy;

/// What extraction produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extracted {
    /// Revision-only omissions. Ordinary package archives still refuse all links.
    pub omissions: Vec<ArchiveOmission>,
    /// The destination directory.
    pub root: PathBuf,
    /// Number of file and directory entries written.
    pub entries: usize,
    /// Total decompressed bytes written.
    pub bytes: u64,
    /// The single top-level directory every entry sat under, if there was exactly one.
    /// A `.crate` archive is always `{name}-{version}/...`.
    pub top_level: Option<String>,
}

pub const REVISION_EXTRACTION_POLICY: &str = "revision-extraction/3";

crate::native_struct! {
pub struct ArchiveOmission {
    path: String => crate::native_union::Rule::NonEmpty,
    entry_kind: OmittedEntryKind => crate::native_union::Rule::Text,
    /// Bounded untrusted text; never interpreted as a filesystem instruction.
    target: String => crate::native_union::Rule::Text,
    reason: String => crate::native_union::Rule::NonEmpty,
}
}

crate::native_vocabulary! { pub enum OmittedEntryKind { Symlink = "symlink", HardLink = "hard_link" } }

/// Why extraction was refused or failed.
#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    /// The destination exists and is not empty.
    #[error("refusing to extract into non-empty directory {0}")]
    DestinationNotEmpty(PathBuf),
    /// More entries than the policy allows.
    #[error("archive has more than {limit} entries")]
    TooManyEntries {
        /// The configured limit.
        limit: usize,
    },
    /// One entry declares more bytes than the policy allows.
    #[error("entry `{path}` declares {size} bytes; the limit is {limit}")]
    EntryTooLarge {
        /// The entry path as written in the archive.
        path: String,
        /// Declared size.
        size: u64,
        /// The configured limit.
        limit: u64,
    },
    /// The archive as a whole declares more bytes than the policy allows.
    #[error("archive exceeds the {limit}-byte total decompressed limit")]
    TotalTooLarge {
        /// The configured limit.
        limit: u64,
    },
    /// An entry path is absolute.
    #[error("entry `{path}` has an absolute path")]
    AbsolutePath {
        /// The entry path as written.
        path: String,
    },
    /// An entry path would escape the destination.
    #[error("entry `{path}` would escape the destination")]
    PathTraversal {
        /// The entry path as written.
        path: String,
    },
    /// An entry is a link, device or other type that is never extracted.
    #[error("entry `{path}` is a {kind}, which is never extracted")]
    UnsupportedEntry {
        /// The entry path as written.
        path: String,
        /// Human-readable entry kind.
        kind: &'static str,
    },
    /// An entry's bytes did not match its declared size.
    #[error("entry `{path}` was truncated")]
    Truncated {
        /// The entry path as written.
        path: String,
    },
    /// An I/O or decompression failure.
    #[error("archive I/O failed: {0}")]
    Io(#[from] io::Error),
}

/// Extract a gzip-compressed tar archive under `destination`.
///
/// # Errors
///
/// Returns an [`ArchiveError`] for any policy violation, a non-empty destination, or an I/O
/// failure. A violation found after some entries were written leaves those entries in place
/// under `destination`; callers extract into a scratch directory they discard on error.
pub fn extract_tar_gz<R: Read>(
    reader: R,
    destination: &Path,
    policy: &ArchivePolicy,
) -> Result<Extracted, ArchiveError> {
    extract_tar_gz_inner(reader, destination, policy, None)
}

/// Extract a Git revision archive, allowing only a bounded global commit comment.
///
/// # Errors
/// Archive policy and mismatched or unsupported PAX metadata are rejected.
pub fn extract_revision_tar_gz<R: Read + Seek>(
    mut reader: R,
    destination: &Path,
    policy: &ArchivePolicy,
    commit: &str,
) -> Result<Extracted, ArchiveError> {
    let position = reader.stream_position()?;
    revision_preflight(&mut reader, policy, commit)?;
    reader.seek(SeekFrom::Start(position))?;
    extract_tar_gz_inner(reader, destination, policy, Some(commit))
}

/// Normal tar iteration resolves GNU/PAX paths but eagerly reads extension payloads. A raw
/// pass over the same immutable input proves their bounds before allowing that interpretation.
fn revision_preflight(
    reader: impl Read,
    policy: &ArchivePolicy,
    commit: &str,
) -> Result<(), ArchiveError> {
    let mut archive = tar::Archive::new(GzDecoder::new(reader));
    let mut total = 0u64;
    for (index, entry) in archive.entries()?.raw(true).enumerate() {
        if index >= policy.max_entries {
            return Err(ArchiveError::TooManyEntries {
                limit: policy.max_entries,
            });
        }
        let mut entry = entry?;
        let kind = entry.header().entry_type();
        let size = entry.header().size()?;
        let extension = matches!(
            kind,
            EntryType::GNULongName
                | EntryType::GNULongLink
                | EntryType::XHeader
                | EntryType::XGlobalHeader
        );
        let limit = if matches!(kind, EntryType::GNULongName | EntryType::GNULongLink) {
            4096
        } else if extension {
            16384
        } else {
            policy.max_entry_bytes
        };
        if size > limit {
            return Err(ArchiveError::EntryTooLarge {
                path: entry.path()?.display().to_string(),
                size,
                limit,
            });
        }
        total = total
            .checked_add(size)
            .filter(|n| *n <= policy.max_total_bytes)
            .ok_or(ArchiveError::TotalTooLarge {
                limit: policy.max_total_bytes,
            })?;
        if kind == EntryType::XGlobalHeader {
            let mut data = Vec::new();
            entry.by_ref().take(limit + 1).read_to_end(&mut data)?;
            validate_commit_pax(&data, commit)?;
        } else if kind == EntryType::XHeader {
            let mut data = Vec::new();
            entry.by_ref().take(limit + 1).read_to_end(&mut data)?;
            validate_local_pax(&data)?;
        } else {
            let read = io::copy(&mut entry, &mut io::sink())?;
            if read != size {
                return Err(ArchiveError::Truncated {
                    path: entry.path()?.display().to_string(),
                });
            }
        }
    }
    Ok(())
}

fn extract_tar_gz_inner<R: Read>(
    reader: R,
    destination: &Path,
    policy: &ArchivePolicy,
    commit: Option<&str>,
) -> Result<Extracted, ArchiveError> {
    fs::create_dir_all(destination)?;
    if fs::read_dir(destination)?.next().is_some() {
        return Err(ArchiveError::DestinationNotEmpty(destination.to_path_buf()));
    }

    let mut archive = tar::Archive::new(GzDecoder::new(reader));
    archive.set_preserve_permissions(false);
    archive.set_preserve_mtime(false);
    archive.set_unpack_xattrs(false);

    let mut seen = std::collections::BTreeMap::new();
    let mut omissions = Vec::new();
    let mut written_entries = 0usize;
    let mut written_bytes = 0u64;
    let mut top_level: Option<Option<String>> = None;

    for entry in archive.entries()? {
        let mut entry = entry?;
        let raw_path = entry.path()?.into_owned();
        let shown = raw_path.display().to_string();

        written_entries += 1;
        if written_entries > policy.max_entries {
            return Err(ArchiveError::TooManyEntries {
                limit: policy.max_entries,
            });
        }

        let kind = entry.header().entry_type();
        let relative = safe_relative_path(&raw_path, &shown)?;
        if seen.contains_key(&relative)
            || relative.ancestors().skip(1).any(|parent| {
                seen.get(parent)
                    .is_some_and(|kind| *kind != EntryType::Directory)
            })
            || (kind != EntryType::Directory
                && seen
                    .keys()
                    .any(|child: &PathBuf| child.starts_with(&relative)))
        {
            return Err(ArchiveError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("duplicate or conflicting archive path {shown}"),
            )));
        }
        seen.insert(relative.clone(), kind);
        if kind == EntryType::XGlobalHeader && commit.is_some() {
            let size = entry.header().size()?;
            if size > 16384 {
                return Err(ArchiveError::EntryTooLarge {
                    path: shown,
                    size,
                    limit: 16384,
                });
            }
            written_bytes = written_bytes.saturating_add(size);
            if written_bytes > policy.max_total_bytes {
                return Err(ArchiveError::TotalTooLarge {
                    limit: policy.max_total_bytes,
                });
            }
            let mut metadata = Vec::new();
            entry.by_ref().take(16385).read_to_end(&mut metadata)?;
            validate_commit_pax(&metadata, commit.unwrap_or_default())?;
            continue;
        }
        let kind_name = match kind {
            EntryType::Regular | EntryType::Directory => None,
            EntryType::Symlink if commit.is_some() => None,
            EntryType::Link if commit.is_some() => None,
            EntryType::Symlink => Some("symlink"),
            EntryType::Link => Some("hard link"),
            EntryType::Char => Some("character device"),
            EntryType::Block => Some("block device"),
            EntryType::Fifo => Some("fifo"),
            _ => Some("unsupported entry type"),
        };
        if let Some(kind) = kind_name {
            return Err(ArchiveError::UnsupportedEntry { path: shown, kind });
        }

        let first = relative
            .components()
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .map(str::to_owned);
        match &top_level {
            None => top_level = Some(first),
            Some(existing) if *existing != first => top_level = Some(None),
            Some(_) => {}
        }

        let size = entry.header().size()?;
        if size > policy.max_entry_bytes {
            return Err(ArchiveError::EntryTooLarge {
                path: shown,
                size,
                limit: policy.max_entry_bytes,
            });
        }
        written_bytes = written_bytes.saturating_add(size);
        if written_bytes > policy.max_total_bytes {
            return Err(ArchiveError::TotalTooLarge {
                limit: policy.max_total_bytes,
            });
        }

        if matches!(kind, EntryType::Symlink | EntryType::Link) {
            if size != 0 {
                return Err(ArchiveError::Io(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "link entry must not carry a payload",
                )));
            }
            let link = entry.link_name()?.ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "link target is missing")
            })?;
            let text = link
                .to_str()
                .filter(|text| {
                    !text.is_empty() && text.len() <= 4096 && !text.chars().any(char::is_control)
                })
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid or excessive link target",
                    )
                })?;
            let mut resolved = if kind == EntryType::Symlink {
                relative.parent().unwrap_or(Path::new("")).to_owned()
            } else {
                PathBuf::new()
            };
            for component in link.components() {
                match component {
                    Component::Normal(part) => resolved.push(part),
                    Component::CurDir => {}
                    Component::ParentDir if resolved.components().count() > 1 => {
                        resolved.pop();
                    }
                    Component::ParentDir => {
                        return Err(ArchiveError::PathTraversal { path: shown });
                    }
                    Component::RootDir | Component::Prefix(_) => {
                        return Err(ArchiveError::AbsolutePath { path: shown });
                    }
                }
            }
            if resolved.components().next() != relative.components().next() {
                return Err(ArchiveError::PathTraversal { path: shown });
            }
            omissions.push(ArchiveOmission {
                path: relative.to_string_lossy().into_owned(),
                entry_kind: if kind == EntryType::Symlink {
                    OmittedEntryKind::Symlink
                } else {
                    OmittedEntryKind::HardLink
                },
                target: text.into(),
                reason: "Links are recorded without being created or followed".into(),
            });
            continue;
        }

        let target = destination.join(&relative);
        if kind == EntryType::Directory {
            fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        // `create_new` so a duplicate entry cannot overwrite an earlier one, and so nothing is
        // ever written through a pre-existing path of any kind.
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&target)?;
        let copied = io::copy(&mut entry.by_ref().take(size), &mut file)?;
        if copied != size {
            return Err(ArchiveError::Truncated { path: shown });
        }
    }

    Ok(Extracted {
        omissions,
        root: destination.to_path_buf(),
        entries: written_entries,
        bytes: written_bytes,
        top_level: top_level.flatten(),
    })
}

fn validate_local_pax(mut bytes: &[u8]) -> Result<(), ArchiveError> {
    let invalid = || {
        ArchiveError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsupported, duplicate or malformed revision PAX override",
        ))
    };
    let mut keys = std::collections::BTreeSet::new();
    while !bytes.is_empty() {
        let space = bytes.iter().position(|b| *b == b' ').ok_or_else(invalid)?;
        let count: usize = std::str::from_utf8(&bytes[..space])
            .map_err(|_| invalid())?
            .parse()
            .map_err(|_| invalid())?;
        if count <= space + 1 || count > bytes.len() {
            return Err(invalid());
        }
        let record = std::str::from_utf8(&bytes[space + 1..count])
            .map_err(|_| invalid())?
            .strip_suffix('\n')
            .ok_or_else(invalid)?;
        let (key, value) = record.split_once('=').ok_or_else(invalid)?;
        if !matches!(key, "path" | "linkpath" | "mtime")
            || !keys.insert(key.to_owned())
            || value.is_empty()
            || value.chars().any(char::is_control)
        {
            return Err(invalid());
        }
        bytes = &bytes[count..];
    }
    Ok(())
}

fn validate_commit_pax(mut bytes: &[u8], commit: &str) -> Result<(), ArchiveError> {
    let invalid = || {
        ArchiveError::Io(io::Error::other(
            "unsupported or mismatched global revision PAX metadata",
        ))
    };
    while !bytes.is_empty() {
        let space = bytes.iter().position(|b| *b == b' ').ok_or_else(invalid)?;
        let count: usize = std::str::from_utf8(&bytes[..space])
            .map_err(|_| invalid())?
            .parse()
            .map_err(|_| invalid())?;
        if count <= space + 1 || count > bytes.len() {
            return Err(invalid());
        }
        let record = std::str::from_utf8(&bytes[space + 1..count]).map_err(|_| invalid())?;
        if record != format!("comment={commit}\n") {
            return Err(invalid());
        }
        bytes = &bytes[count..];
    }
    Ok(())
}

/// Validate an entry path: relative, no `..`, no root or prefix components.
fn safe_relative_path(raw: &Path, shown: &str) -> Result<PathBuf, ArchiveError> {
    if raw
        .to_str()
        .is_none_or(|path| path.len() > 4096 || path.chars().any(char::is_control))
    {
        return Err(ArchiveError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "archive path must be bounded UTF-8 without control characters",
        )));
    }
    let mut out = PathBuf::new();
    for component in raw.components() {
        match component {
            Component::Normal(part) => out.push(part),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(ArchiveError::PathTraversal {
                    path: shown.to_owned(),
                });
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(ArchiveError::AbsolutePath {
                    path: shown.to_owned(),
                });
            }
        }
    }
    if out.as_os_str().is_empty() {
        return Err(ArchiveError::PathTraversal {
            path: shown.to_owned(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::Compression;
    use flate2::write::GzEncoder;
    use tar::{Builder, Header};

    use super::*;

    /// Build a gzip tarball in memory from a closure that appends entries.
    fn tarball(build: impl FnOnce(&mut Builder<Vec<u8>>)) -> Vec<u8> {
        let mut builder = Builder::new(Vec::new());
        build(&mut builder);
        let tar = builder.into_inner().expect("tar finishes");
        let mut gz = GzEncoder::new(Vec::new(), Compression::fast());
        gz.write_all(&tar).expect("gzip writes");
        gz.finish().expect("gzip finishes")
    }

    fn file(builder: &mut Builder<Vec<u8>>, path: &str, body: &[u8]) {
        let mut header = Header::new_gnu();
        header.set_size(body.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, path, body)
            .expect("append regular file");
    }

    fn special(builder: &mut Builder<Vec<u8>>, path: &str, kind: EntryType, link: Option<&str>) {
        let mut header = Header::new_gnu();
        header.set_entry_type(kind);
        header.set_size(0);
        header.set_mode(0o644);
        header.set_cksum();
        match link {
            Some(target) => builder
                .append_link(&mut header, path, target)
                .expect("append link"),
            None => builder
                .append_data(&mut header, path, io::empty())
                .expect("append special"),
        }
    }

    fn dest() -> tempfile::TempDir {
        tempfile::tempdir().expect("temp dir")
    }

    #[test]
    fn revision_omits_unrelated_links_without_following() {
        let manifest = "[package]\nname='selected'\nversion='1.0.0'\n";
        let bytes = tarball(|b| {
            file(b, "repo/pkg/Cargo.toml", manifest.as_bytes());
            file(b, "repo/pkg/src/lib.rs", b"pub fn selected() {}\n");
            file(b, "repo/AGENTS.md", b"untrusted instructions");
            special(b, "repo/CLAUDE.md", EntryType::Symlink, Some("AGENTS.md"));
        });
        let dir = dest();
        let extracted = extract_revision_tar_gz(
            io::Cursor::new(&bytes),
            dir.path(),
            &ArchivePolicy::default(),
            "commit",
        )
        .expect("safe omissions");
        assert!(!dir.path().join("repo/CLAUDE.md").exists());
        assert_eq!(extracted.omissions.len(), 1);
        assert_eq!(extracted.omissions[0].target, "AGENTS.md");
        let receipt = crate::producer::revision::RevisionInputs::collect(
            crate::canonical::sha256_hex(&bytes),
            &extracted,
            "pkg",
            crate::identity::Ecosystem::Rust,
            manifest,
        )
        .expect("closure");
        assert!(receipt.missing_inputs.is_empty());
        assert_eq!(receipt.selected_package, "repo/pkg");
    }

    #[test]
    fn revision_required_link_is_recorded_as_missing() {
        let manifest = "[package]\nname='selected'\n[lib]\npath='src/linked.rs'\n";
        let bytes = tarball(|b| {
            file(b, "repo/pkg/Cargo.toml", manifest.as_bytes());
            special(
                b,
                "repo/pkg/src/linked.rs",
                EntryType::Symlink,
                Some("actual.rs"),
            );
            file(b, "repo/pkg/src/actual.rs", b"pub fn actual() {}\n");
        });
        let dir = dest();
        let extracted = extract_revision_tar_gz(
            io::Cursor::new(&bytes),
            dir.path(),
            &ArchivePolicy::default(),
            "commit",
        )
        .expect("omission recorded");
        let receipt = crate::producer::revision::RevisionInputs::collect(
            crate::canonical::sha256_hex(&bytes),
            &extracted,
            "pkg",
            crate::identity::Ecosystem::Rust,
            manifest,
        )
        .expect("closure");

        assert_eq!(receipt.missing_inputs, ["repo/pkg/src/linked.rs"]);
        assert_eq!(receipt.omissions[0].path, "repo/pkg/src/linked.rs");
    }

    #[test]
    fn revision_collects_inherited_transitive_inputs_with_confined_roots() {
        let selected = "[package]\nname='selected'\nworkspace='../workspace'\nreadme.workspace=true\nlicense-file.workspace=true\n[dependencies]\nshared.workspace=true\n[[bin]]\nname='cli'\npath='../cli/main.rs'\n";
        let bytes = tarball(|b| {
            file(b, "repo/pkg/Cargo.toml", selected.as_bytes());
            file(b, "repo/workspace/Cargo.toml", b"[workspace]\n[workspace.package]\nreadme='../README.md'\nlicense-file='../LICENSE'\n[workspace.dependencies]\nshared={path='../shared'}\nunused={path='../../escape'}\n");
            file(b, "repo/shared/Cargo.toml", b"[package]\nname='shared'\n[target.'cfg(unix)'.build-dependencies]\nleaf={path='../leaf'}\n[package.metadata.dependencies]\nignored={path='../../escape'}\n");
            file(
                b,
                "repo/leaf/Cargo.toml",
                b"[package]\nname='leaf'\n[dependencies]\ncycle={path='../pkg'}\n",
            );
            file(b, "repo/LICENSE", b"license");
            file(b, "repo/cli/main.rs", b"fn main() {}\n");
            special(
                b,
                "repo/shared/src/linked.rs",
                EntryType::Symlink,
                Some("actual.rs"),
            );
            special(b, "repo/README.md", EntryType::Symlink, Some("README.real"));
            special(b, "repo/CLAUDE.md", EntryType::Symlink, Some("AGENTS.md"));
        });
        let dir = dest();
        let extracted = extract_revision_tar_gz(
            io::Cursor::new(&bytes),
            dir.path(),
            &ArchivePolicy::default(),
            "commit",
        )
        .unwrap();
        let inputs = crate::producer::revision::RevisionInputs::collect(
            crate::canonical::sha256_hex(&bytes),
            &extracted,
            "pkg",
            crate::identity::Ecosystem::Rust,
            selected,
        )
        .unwrap();
        assert_eq!(
            inputs.source_roots,
            ["repo/leaf", "repo/pkg", "repo/shared"]
        );
        assert_eq!(inputs.missing_inputs, ["repo/README.md"]);
        assert!(
            inputs
                .declared_inputs
                .contains(&"repo/workspace/Cargo.toml".into())
        );
        assert!(inputs.declared_inputs.contains(&"repo/cli/main.rs".into()));
        assert!(inputs.declared_inputs.contains(&"repo/LICENSE".into()));
        assert!(!inputs.declared_inputs.contains(&"repo/CLAUDE.md".into()));

        assert!(
            crate::producer::revision::RevisionInputs::collect(
                crate::canonical::sha256_hex(&bytes),
                &extracted,
                "pkg",
                crate::identity::Ecosystem::Rust,
                "[package]\nname='selected'\n[dependencies]\nescaping={path='../../outside'}\n",
            )
            .unwrap_err()
            .contains("escapes archive wrapper")
        );
    }

    #[test]
    fn revision_rejects_hostile_targets_and_link_path_collisions() {
        for target in ["/etc/passwd", "../../outside"] {
            let bytes = tarball(|b| special(b, "repo/pkg/link", EntryType::Symlink, Some(target)));
            let dir = dest();
            assert!(
                extract_revision_tar_gz(
                    io::Cursor::new(&bytes),
                    dir.path(),
                    &ArchivePolicy::default(),
                    "commit"
                )
                .is_err()
            );
        }
        for first in [true, false] {
            let bytes = tarball(|b| {
                if first {
                    special(b, "repo/pkg/link", EntryType::Symlink, Some("target"));
                }
                file(b, "repo/pkg/link/child", b"must not pass through a link");
                if !first {
                    special(b, "repo/pkg/link", EntryType::Symlink, Some("target"));
                }
            });
            let dir = dest();
            assert!(
                extract_revision_tar_gz(
                    io::Cursor::new(&bytes),
                    dir.path(),
                    &ArchivePolicy::default(),
                    "commit"
                )
                .is_err()
            );
        }
    }

    #[test]
    fn a_well_formed_crate_archive_extracts_under_its_top_level_directory() {
        let bytes = tarball(|b| {
            file(
                b,
                "enr-fixture-0.1.0/Cargo.toml",
                b"[package]\nname = \"enr-fixture\"\n",
            );
            file(b, "enr-fixture-0.1.0/src/lib.rs", b"pub struct Widget;\n");
        });
        let dir = dest();
        let out = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default()).expect("ok");
        assert_eq!(out.entries, 2);
        assert_eq!(out.top_level.as_deref(), Some("enr-fixture-0.1.0"));
        assert_eq!(
            fs::read_to_string(dir.path().join("enr-fixture-0.1.0/src/lib.rs")).expect("read"),
            "pub struct Widget;\n"
        );
    }

    /// Write a header whose name bytes are set directly, bypassing `Header::set_path`, which
    /// refuses `..` and strips a leading `/` -- exactly the shapes a hostile archive carries.
    fn raw_named(builder: &mut Builder<Vec<u8>>, name: &str, body: &[u8]) {
        let mut header = Header::new_gnu();
        {
            let gnu = header.as_gnu_mut().expect("gnu header");
            gnu.name = [0u8; 100];
            gnu.name[..name.len()].copy_from_slice(name.as_bytes());
        }
        header.set_size(body.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append(&header, body).expect("append raw header");
    }

    #[test]
    fn a_parent_directory_component_is_refused_before_any_write() {
        let bytes = tarball(|b| raw_named(b, "ok/../../escape.txt", b"x"));
        let dir = dest();
        let err = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default())
            .expect_err("traversal");
        assert!(matches!(err, ArchiveError::PathTraversal { .. }), "{err}");
        assert!(
            !dir.path()
                .parent()
                .expect("parent")
                .join("escape.txt")
                .exists()
        );
        assert!(fs::read_dir(dir.path()).expect("dir").next().is_none());
    }

    #[test]
    fn an_absolute_path_is_refused() {
        let bytes = tarball(|b| raw_named(b, "/etc/evil", b"x"));
        let dir = dest();
        let err = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default())
            .expect_err("absolute");
        assert!(matches!(err, ArchiveError::AbsolutePath { .. }), "{err}");
        assert!(fs::read_dir(dir.path()).expect("dir").next().is_none());
        assert!(!Path::new("/etc/evil").exists());
    }

    #[test]
    fn symlinks_and_hard_links_are_refused() {
        for kind in [EntryType::Symlink, EntryType::Link] {
            let bytes = tarball(|b| special(b, "pkg/link", kind, Some("/etc/passwd")));
            let dir = dest();
            let err = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default())
                .expect_err("link refused");
            assert!(
                matches!(err, ArchiveError::UnsupportedEntry { .. }),
                "{err}"
            );
        }
    }

    #[test]
    fn device_files_and_fifos_are_refused() {
        for kind in [EntryType::Char, EntryType::Block, EntryType::Fifo] {
            let bytes = tarball(|b| special(b, "pkg/dev", kind, None));
            let dir = dest();
            let err = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default())
                .expect_err("device refused");
            assert!(
                matches!(err, ArchiveError::UnsupportedEntry { .. }),
                "{err}"
            );
        }
    }

    #[test]
    fn an_oversized_entry_is_refused_from_its_header() {
        let bytes = tarball(|b| file(b, "pkg/big.bin", &[0u8; 4096]));
        let policy = ArchivePolicy {
            max_entry_bytes: 1024,
            ..ArchivePolicy::default()
        };
        let dir = dest();
        let err = extract_tar_gz(&bytes[..], dir.path(), &policy).expect_err("too large");
        assert!(
            matches!(err, ArchiveError::EntryTooLarge { size: 4096, .. }),
            "{err}"
        );
        assert!(!dir.path().join("pkg/big.bin").exists());
    }

    #[test]
    fn the_total_and_entry_count_limits_hold() {
        let bytes = tarball(|b| {
            file(b, "pkg/a", &[0u8; 700]);
            file(b, "pkg/b", &[0u8; 700]);
        });
        let dir = dest();
        let err = extract_tar_gz(
            &bytes[..],
            dir.path(),
            &ArchivePolicy {
                max_total_bytes: 1000,
                ..ArchivePolicy::default()
            },
        )
        .expect_err("total");
        assert!(matches!(err, ArchiveError::TotalTooLarge { .. }), "{err}");

        let dir = dest();
        let err = extract_tar_gz(
            &bytes[..],
            dir.path(),
            &ArchivePolicy {
                max_entries: 1,
                ..ArchivePolicy::default()
            },
        )
        .expect_err("count");
        assert!(
            matches!(err, ArchiveError::TooManyEntries { limit: 1 }),
            "{err}"
        );
    }

    #[test]
    fn a_non_empty_destination_is_refused() {
        let bytes = tarball(|b| file(b, "pkg/a", b"x"));
        let dir = dest();
        fs::write(dir.path().join("existing"), b"").expect("write");
        let err = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default())
            .expect_err("non-empty");
        assert!(matches!(err, ArchiveError::DestinationNotEmpty(_)), "{err}");
    }

    #[test]
    fn a_duplicate_entry_cannot_overwrite_an_earlier_one() {
        let bytes = tarball(|b| {
            file(b, "pkg/a", b"first");
            file(b, "pkg/a", b"second");
        });
        let dir = dest();
        let err = extract_tar_gz(&bytes[..], dir.path(), &ArchivePolicy::default())
            .expect_err("duplicate");
        assert!(matches!(err, ArchiveError::Io(_)), "{err}");
        assert_eq!(
            fs::read_to_string(dir.path().join("pkg/a")).expect("read"),
            "first"
        );
    }
}
