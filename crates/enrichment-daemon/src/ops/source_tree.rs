//! Private source trees reconstructed from verified retained archives, never cache authority.
use enrichment_core::{archive, canonical, policy::ArchivePolicy};
use std::{
    fs,
    io::{self, Read, Seek},
    path::{Path, PathBuf},
};

pub(super) struct SourceTree {
    scratch: tempfile::TempDir,
    root: PathBuf,
}
impl std::ops::Deref for SourceTree {
    type Target = Path;
    fn deref(&self) -> &Path {
        // Keep the private directory owner alive for every member read.
        debug_assert!(self.root.starts_with(self.scratch.path()));
        &self.root
    }
}

/// Verify the same open archive that will be extracted, then retain the private directory
/// owner until the caller finishes reading. Old extracted cache contents are never opened.
pub(super) fn open(root: &Path, digest: &str, source: impl Read) -> io::Result<SourceTree> {
    fs::create_dir_all(root)?;
    if fs::symlink_metadata(root)?.file_type().is_symlink() {
        return Err(io::Error::other("source scratch root is a link"));
    }
    let scratch = tempfile::Builder::new()
        .prefix(".source-read-")
        .tempdir_in(root)?;
    // Verify private captured bytes, not a mutable file checked and reopened later.
    let mut captured = tempfile::tempfile_in(scratch.path())?;
    let limit = 256 * 1024 * 1024;
    if io::copy(&mut source.take(limit + 1), &mut captured)? > limit {
        return Err(io::Error::other("source archive exceeds byte bound"));
    }
    captured.rewind()?;
    let (actual, _) = canonical::sha256_reader(&mut captured, limit)?;
    if actual != digest {
        return Err(io::Error::other(
            "source archive disagrees with retained digest",
        ));
    }
    captured.rewind()?;
    let destination = scratch.path().join("content");
    let extracted = archive::extract_tar_gz(captured, &destination, &ArchivePolicy::default())
        .map_err(io::Error::other)?;
    let top = extracted
        .top_level
        .ok_or_else(|| io::Error::other("archive needs one top-level directory"))?;
    let root = destination.join(top);
    if !fs::symlink_metadata(&root)?.is_dir() {
        return Err(io::Error::other("archive root is not a directory"));
    }
    Ok(SourceTree { scratch, root })
}

#[cfg(test)]
mod tests {
    use super::*;
    const ARCHIVE: &[u8] =
        include_bytes!("../../../../tests/fixtures/upstream/static/enr-fixture-0.1.0.crate");

    #[test]
    fn old_cache_bytes_never_supply_retained_source() {
        let temp = tempfile::tempdir().unwrap();
        let digest = canonical::sha256_hex(ARCHIVE);
        let old = temp.path().join(&digest).join("enr-fixture-0.1.0/src");
        fs::create_dir_all(&old).unwrap();
        fs::write(
            old.parent().unwrap().parent().unwrap().join(".complete"),
            format!("retained-source-cache/1\n{digest}\n"),
        )
        .unwrap();
        fs::write(old.join("lib.rs"), "poison").unwrap();
        let tree = open(temp.path(), &digest, ARCHIVE).unwrap();
        let expected = fs::read(tree.join("src/lib.rs")).unwrap();
        assert_ne!(expected, b"poison");
        let root = tree.root.clone();
        drop(tree);
        assert!(!root.exists());
        #[cfg(unix)]
        {
            fs::remove_file(old.join("lib.rs")).unwrap();
            let foreign = temp.path().join("foreign");
            fs::write(&foreign, vec![b'x'; expected.len()]).unwrap();
            std::os::unix::fs::symlink(&foreign, old.join("lib.rs")).unwrap();
            let tree = open(temp.path(), &digest, ARCHIVE).unwrap();
            assert_eq!(fs::read(tree.join("src/lib.rs")).unwrap(), expected);
            assert_eq!(fs::read(foreign).unwrap(), vec![b'x'; expected.len()]);
        }
        assert!(open(temp.path(), &digest, &b"changed archive"[..]).is_err());
        assert!(fs::read_dir(temp.path()).unwrap().all(|p| {
            !p.unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".source-read-")
        }));
    }
}
