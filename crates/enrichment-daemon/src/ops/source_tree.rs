//! Private source trees reconstructed from verified retained archives, never cache authority.
use enrichment_core::{archive, canonical, policy::ArchivePolicy};
use std::{
    fs,
    io::{self, Read, Seek},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone)]
pub(super) struct SourceTree {
    owner: Arc<enrichment_store::PrivateDirectory>,
    root: PathBuf,
}
impl SourceTree {
    pub(super) fn owner(&self) -> Arc<enrichment_store::PrivateDirectory> {
        self.owner.clone()
    }
    pub(super) fn owned(
        owner: Arc<enrichment_store::PrivateDirectory>,
        root: PathBuf,
    ) -> io::Result<Self> {
        if !root.starts_with(owner.path())
            || root
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(io::Error::other("source tree escaped its private owner"));
        }
        Ok(Self { owner, root })
    }
}
impl std::ops::Deref for SourceTree {
    type Target = Path;
    fn deref(&self) -> &Path {
        debug_assert!(self.root.starts_with(self.owner.path()));
        &self.root
    }
}

/// Verify the same open archive that will be extracted, then retain the private directory
/// owner in the caller until physical reads finish. The destination has a durable obligation.
pub(super) fn open(
    owner: Arc<enrichment_store::PrivateDirectory>,
    digest: &str,
    source: impl Read,
) -> io::Result<SourceTree> {
    let root = extract(owner.path(), digest, source)?;
    SourceTree::owned(owner, root)
}

fn extract(root: &Path, digest: &str, source: impl Read) -> io::Result<PathBuf> {
    if !fs::symlink_metadata(root)?.is_dir() || fs::read_dir(root)?.next().is_some() {
        return Err(io::Error::other(
            "source scratch is not an empty owned directory",
        ));
    }
    // Verify private captured bytes, not a mutable file checked and reopened later.
    let mut captured = tempfile::tempfile_in(root)?;
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
    let destination = root.join("content");
    let extracted = archive::extract_tar_gz(captured, &destination, &ArchivePolicy::default())
        .map_err(io::Error::other)?;
    let top = extracted
        .top_level
        .ok_or_else(|| io::Error::other("archive needs one top-level directory"))?;
    let root = destination.join(top);
    if !fs::symlink_metadata(&root)?.is_dir() {
        return Err(io::Error::other("archive root is not a directory"));
    }
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    const ARCHIVE: &[u8] =
        include_bytes!("../../../../tests/fixtures/upstream/static/enr-fixture-0.1.0.crate");

    #[test]
    fn private_extraction_requires_empty_root_and_verified_bytes() {
        let temp = tempfile::tempdir().unwrap();
        let digest = canonical::sha256_hex(ARCHIVE);
        let tree = extract(temp.path(), &digest, ARCHIVE).unwrap();
        let expected = fs::read(tree.join("src/lib.rs")).unwrap();
        assert!(!expected.is_empty());
        assert!(extract(temp.path(), &digest, ARCHIVE).is_err());
        let extracted = tree.clone();
        drop(temp);
        assert!(!extracted.exists());
        let invalid = tempfile::tempdir().unwrap();
        assert!(extract(invalid.path(), &digest, &b"changed archive"[..]).is_err());
        assert!(fs::read_dir(invalid.path()).unwrap().next().is_none());
        #[cfg(unix)]
        {
            let parent = tempfile::tempdir().unwrap();
            let link = parent.path().join("linked-root");
            std::os::unix::fs::symlink(invalid.path(), &link).unwrap();
            assert!(extract(&link, &digest, ARCHIVE).is_err());
        }
    }
}
