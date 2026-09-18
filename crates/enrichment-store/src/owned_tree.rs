//! Bounded physical inventory for a native-selected, exclusively owned directory.
//! Callers supply the lifetime fence; this driver neither selects nor grants removal.
use datafusion::common::{DataFusionError, Result};
use futures::{StreamExt, TryStreamExt};
use object_store::ObjectStore;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

const MAX_ENTRIES: usize = 8192;

struct Entry {
    path: PathBuf,
    identity: Identity,
}

struct Directory {
    path: PathBuf,
    device: u64,
    inode: u64,
}
impl Directory {
    fn read(path: PathBuf) -> std::io::Result<Self> {
        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::symlink_metadata(&path)?;
        if !metadata.is_dir() {
            return Err(std::io::Error::other("owned directory replaced"));
        }
        Ok(Self {
            path,
            device: metadata.dev(),
            inode: metadata.ino(),
        })
    }
    fn verify(&self) -> std::io::Result<()> {
        let current = Self::read(self.path.clone())?;
        if (self.device, self.inode) != (current.device, current.inode) {
            return Err(std::io::Error::other("owned directory identity changed"));
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
struct Identity {
    device: u64,
    inode: u64,
    length: u64,
    modified: std::time::SystemTime,
}
impl Identity {
    fn read(path: &Path) -> std::io::Result<Self> {
        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::symlink_metadata(path)?;
        if !metadata.is_file() || metadata.nlink() != 1 {
            return Err(std::io::Error::other("owned file is not a regular file"));
        }
        Ok(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            length: metadata.len(),
            modified: metadata.modified()?,
        })
    }
}

pub(crate) struct Inventory {
    files: Vec<Entry>,
    directories: Vec<Directory>,
}
impl Inventory {
    pub(crate) fn read(root: &Path) -> std::io::Result<Self> {
        Self::read_bounded(root, MAX_ENTRIES)
    }
    pub(crate) fn read_bounded(root: &Path, maximum: usize) -> std::io::Result<Self> {
        if maximum == 0 {
            return Err(std::io::Error::other(
                "owned tree needs a positive entry bound",
            ));
        }
        let mut inventory = Self {
            files: vec![],
            directories: vec![],
        };
        let metadata = match std::fs::symlink_metadata(root) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(inventory),
            Err(error) => return Err(error),
        };
        if !metadata.is_dir() {
            return Err(std::io::Error::other("owned tree is not a directory"));
        }
        inventory.directories.push(Directory::read(root.into())?);
        let mut index = 0;
        while index < inventory.directories.len() {
            for entry in std::fs::read_dir(&inventory.directories[index].path)? {
                if inventory.files.len() + inventory.directories.len() >= maximum {
                    return Err(std::io::Error::other("owned tree entry bound"));
                }
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    inventory.directories.push(Directory::read(entry.path())?);
                } else {
                    inventory.files.push(Entry {
                        identity: Identity::read(&entry.path())?,
                        path: entry.path(),
                    });
                }
            }
            index += 1;
        }
        Ok(inventory)
    }
    /// Remove only a fully inventoried tree. A late file prevents directory removal;
    /// failed deletion never becomes an acknowledged successful cleanup. Exact paths
    /// pass through the runtime's durable store and its metadata invalidation owner.
    pub(crate) async fn remove(self, store: Arc<dyn ObjectStore>) -> Result<()> {
        for directory in &self.directories {
            directory.verify()?;
        }
        for entry in self.files {
            if Identity::read(&entry.path)? != entry.identity {
                return Err(DataFusionError::Execution(
                    "owned file changed during removal".into(),
                ));
            }
            let location = object_store::path::Path::from_filesystem_path(&entry.path)
                .map_err(|error| DataFusionError::External(Box::new(error)))?;
            store
                .delete_stream(futures::stream::iter([Ok(location)]).boxed())
                .try_for_each(|_| async { Ok(()) })
                .await
                .map_err(|error| DataFusionError::External(Box::new(error)))?;
        }
        for directory in self.directories.into_iter().rev() {
            directory.verify()?;
            std::fs::remove_dir(&directory.path)?;
            File::open(
                directory
                    .path
                    .parent()
                    .ok_or_else(|| std::io::Error::other("owned directory has no parent"))?,
            )?
            .sync_all()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan19_owned_tree_inventory_refuses_links_and_observes_files() -> std::io::Result<()> {
        let parent = tempfile::tempdir()?;
        let root = parent.path().join("candidate");
        assert!(Inventory::read(&root)?.files.is_empty());
        std::fs::create_dir(&root)?;
        std::fs::create_dir(root.join("_delta_log"))?;
        std::fs::write(root.join("part.parquet"), b"incomplete")?;
        let inventory = Inventory::read(&root)?;
        assert_eq!(inventory.files.len(), 1);
        assert_eq!(inventory.directories.len(), 2);
        assert_eq!(inventory.files[0].identity.length, 10);
        std::os::unix::fs::symlink(parent.path(), root.join("escape"))?;
        assert!(Inventory::read(&root).is_err());
        assert!(Inventory::read(&root.join("escape")).is_err());
        std::fs::remove_file(root.join("escape"))?;
        assert!(Inventory::read_bounded(&root, 2).is_err());
        std::fs::rename(root.join("_delta_log"), root.join("old"))?;
        std::fs::create_dir(root.join("_delta_log"))?;
        assert!(inventory.directories[1].verify().is_err());
        Ok(())
    }
}
