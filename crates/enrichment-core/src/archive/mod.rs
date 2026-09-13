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
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

use flate2::read::GzDecoder;
use tar::EntryType;

use crate::policy::ArchivePolicy;

/// What extraction produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extracted {
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
    fs::create_dir_all(destination)?;
    if fs::read_dir(destination)?.next().is_some() {
        return Err(ArchiveError::DestinationNotEmpty(destination.to_path_buf()));
    }

    let mut archive = tar::Archive::new(GzDecoder::new(reader));
    archive.set_preserve_permissions(false);
    archive.set_preserve_mtime(false);
    archive.set_unpack_xattrs(false);

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
        let kind_name = match kind {
            EntryType::Regular | EntryType::Directory => None,
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

        let relative = safe_relative_path(&raw_path, &shown)?;
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
        root: destination.to_path_buf(),
        entries: written_entries,
        bytes: written_bytes,
        top_level: top_level.flatten(),
    })
}

/// Validate an entry path: relative, no `..`, no root or prefix components.
fn safe_relative_path(raw: &Path, shown: &str) -> Result<PathBuf, ArchiveError> {
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
