//! A sealed export has no service mutation authority. Exact table vectors still govern reads.
//! The permanent seal is part of the bundle checksum inventory; opening never changes bytes.
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

const MARKER: &str = ".evidence-immutable";
const CONTRACT: &[u8] = b"library-enrichment-immutable-root/1\n";

/// The normal service APIs cannot create, append to, maintain or repair a sealed root.
pub(crate) fn require_mutable(root: &Path) -> io::Result<()> {
    match fs::symlink_metadata(root.join(MARKER)) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
        Ok(_) => Err(io::Error::other("immutable evidence root refuses mutation")),
    }
}

#[derive(Debug)]
pub struct ImmutableRoot {
    root: PathBuf,
    seal: File,
    lease: Arc<File>,
}

impl ImmutableRoot {
    /// Seal only after every writer and retained reader has drained. The successful
    /// export includes this inode in its physical manifest before destination publication.
    pub(crate) fn seal(root: &Path) -> io::Result<()> {
        let _exclusive = crate::leases::exclusive(root)?;
        require_mutable(root)?;
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(root.join(MARKER))?;
        file.write_all(CONTRACT)?;
        file.sync_all()?;
        File::open(root)?.sync_all()
    }

    /// Require the explicit seal, a stable physical root and its permanent coordination
    /// inode. Checksum/catalog admission is performed separately by the bundle reader.
    pub fn open(root: &Path) -> io::Result<Arc<Self>> {
        if !fs::symlink_metadata(root)?.is_dir() {
            return Err(io::Error::other("immutable root is not a directory"));
        }
        let root = root.canonicalize()?;
        let lease = crate::leases::shared(&root)?;
        let value = Arc::new(Self {
            seal: read_seal(&root)?,
            root,
            lease,
        });
        value.validate()?;
        Ok(value)
    }
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
    pub(crate) fn lease(&self) -> Arc<File> {
        self.lease.clone()
    }
    pub(crate) fn validate(&self) -> io::Result<()> {
        use std::os::unix::fs::MetadataExt;
        let actual = read_seal(&self.root)?.metadata()?;
        let held = self.seal.metadata()?;
        if (
            actual.dev(),
            actual.ino(),
            actual.len(),
            actual.mtime(),
            actual.mtime_nsec(),
            actual.ctime(),
            actual.ctime_nsec(),
        ) != (
            held.dev(),
            held.ino(),
            held.len(),
            held.mtime(),
            held.mtime_nsec(),
            held.ctime(),
            held.ctime_nsec(),
        ) {
            return Err(io::Error::other("immutable root seal was replaced"));
        }
        Ok(())
    }
}

fn read_seal(root: &Path) -> io::Result<File> {
    let mut options = File::options();
    options.read(true);
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(0x20000 | 0x800);
    }
    let mut file = options.open(root.join(MARKER))?;
    if !file.metadata()?.is_file() || file.metadata()?.len() != CONTRACT.len() as u64 {
        return Err(io::Error::other("invalid immutable root seal"));
    }
    let mut bytes = [0; CONTRACT.len()];
    file.read_exact(&mut bytes)?;
    if bytes != CONTRACT {
        return Err(io::Error::other("unsupported immutable root contract"));
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn immutable_root_seal_requires_physical_quiescence_and_refuses_mutation() -> io::Result<()> {
        let root = tempfile::tempdir()?;
        crate::leases::initialize(root.path())?;
        let existing = crate::BlobStore::open(root.path())?;
        assert!(ImmutableRoot::open(root.path()).is_err());
        let reader = crate::leases::shared(root.path())?;
        assert!(ImmutableRoot::seal(root.path()).is_err());
        drop(reader);
        ImmutableRoot::seal(root.path())?;
        let held = ImmutableRoot::open(root.path())?;
        assert!(require_mutable(root.path()).is_err());
        assert!(crate::BlobStore::open(root.path()).is_err());
        assert!(
            existing
                .put_stream(
                    1,
                    |_| panic!("sealed write reached driver"),
                    |_, _| panic!("sealed write produced receipt")
                )
                .is_err()
        );
        assert!(crate::leases::exclusive(root.path()).is_err());
        fs::rename(root.path().join(MARKER), root.path().join("old-seal"))?;
        fs::write(root.path().join(MARKER), CONTRACT)?;
        assert!(held.validate().is_err());
        Ok(())
    }
}
