//! Physical byte allocations retain their DataFusion reservation through the last owner.
use datafusion::execution::memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
    sync::Arc,
};

#[derive(Debug)]
pub struct OwnedBytes {
    bytes: Box<[u8]>,
    length: usize,
    _memory: MemoryReservation,
}
impl std::ops::Deref for OwnedBytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.bytes[..self.length]
    }
}
impl AsRef<[u8]> for OwnedBytes {
    fn as_ref(&self) -> &[u8] {
        self
    }
}
impl OwnedBytes {
    /// Capture a native-selected file range. Reserve before the exact-size buffer;
    /// `Bytes::from_owner` preserves its charge through zero-copy slices.
    pub fn read_range(
        file: &mut File,
        range: std::ops::Range<u64>,
        pool: &Arc<dyn MemoryPool>,
    ) -> io::Result<Self> {
        let size = range
            .end
            .checked_sub(range.start)
            .ok_or_else(|| io::Error::other("object range is reversed"))?;
        let metadata = file.metadata()?;
        if !metadata.is_file() || range.end > metadata.len() {
            return Err(io::Error::other("object range exceeds regular file"));
        }
        let size = usize::try_from(size).map_err(io::Error::other)?;
        let memory = MemoryConsumer::new("object-store-input").register(pool);
        memory.try_grow(size).map_err(io::Error::other)?;
        let mut bytes = vec![0; size].into_boxed_slice();
        file.seek(SeekFrom::Start(range.start))?;
        file.read_exact(&mut bytes)?;
        Ok(Self {
            bytes,
            length: size,
            _memory: memory,
        })
    }
    /// One bounded regular-file descriptor, charged before allocation. The caller owns
    /// path authority and physical lifetime; this type supplies neither admission nor a lease.
    pub fn read(
        mut file: File,
        limit: u64,
        pool: &Arc<dyn MemoryPool>,
        name: &str,
    ) -> io::Result<Self> {
        let metadata = file.metadata()?;
        if !metadata.is_file() || metadata.len() > limit {
            return Err(io::Error::other(
                "owned input is not a bounded regular file",
            ));
        }
        let size = usize::try_from(metadata.len()).map_err(io::Error::other)?;
        let memory = MemoryConsumer::new(name).register(pool);
        memory.try_grow(size).map_err(io::Error::other)?;
        let mut bytes = vec![0; size].into_boxed_slice();
        file.read_exact(&mut bytes)?;
        if file.read(&mut [0])? != 0 {
            return Err(io::Error::other("owned input grew during capture"));
        }
        Ok(Self {
            bytes,
            length: size,
            _memory: memory,
        })
    }

    pub fn read_file(
        path: &Path,
        limit: u64,
        pool: &Arc<dyn MemoryPool>,
        name: &str,
    ) -> io::Result<Self> {
        let mut options = File::options();
        options.read(true);
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(0x20000 | 0x800);
        }
        Self::read(options.open(path)?, limit, pool, name)
    }
}

/// Incremental physical input whose capacity, including replacement overlap, is paid before
/// allocation. The limit is the already admitted byte bound, not a new policy decision.
pub struct OwnedBuffer {
    value: OwnedBytes,
    limit: usize,
}
impl OwnedBuffer {
    pub fn new(pool: &Arc<dyn MemoryPool>, limit: u64, name: &str) -> io::Result<Self> {
        Ok(Self {
            value: OwnedBytes {
                bytes: Box::default(),
                length: 0,
                _memory: MemoryConsumer::new(name).register(pool),
            },
            limit: usize::try_from(limit).map_err(io::Error::other)?,
        })
    }

    pub fn len(&self) -> usize {
        self.value.length
    }

    pub fn is_empty(&self) -> bool {
        self.value.length == 0
    }

    /// Failure leaves the previous allocation and bytes intact. Expose no unpaid copy route.
    pub fn extend(&mut self, bytes: &[u8]) -> io::Result<()> {
        let length = self
            .value
            .length
            .checked_add(bytes.len())
            .filter(|length| *length <= self.limit)
            .ok_or_else(|| io::Error::other("owned input exceeds byte bound"))?;
        let capacity = self.value.bytes.len();
        if length > capacity {
            let next = length
                .max(capacity.saturating_mul(2))
                .max(65536)
                .min(self.limit);
            self.value
                ._memory
                .try_grow(next)
                .map_err(io::Error::other)?;
            let mut replacement = vec![0; next].into_boxed_slice();
            replacement[..self.value.length].copy_from_slice(&self.value);
            self.value.bytes = replacement;
            self.value._memory.shrink(capacity);
        }
        self.value.bytes[self.value.length..length].copy_from_slice(bytes);
        self.value.length = length;
        Ok(())
    }

    /// Cheap clones and slices retain the allocation's reservation to its last owner.
    pub fn finish(self) -> bytes::Bytes {
        bytes::Bytes::from_owner(self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn incremental_input_pays_replacement_and_keeps_charge_through_last_slice() -> io::Result<()> {
        use datafusion::execution::memory_pool::GreedyMemoryPool;
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(196608));
        let mut input = OwnedBuffer::new(&pool, 131072, "incremental-input")?;
        assert!(input.is_empty());
        assert_eq!(pool.reserved(), 0);
        input.extend(b"start")?;
        assert_eq!(pool.reserved(), 65536);
        let competitor = MemoryConsumer::new("competing-reader").register(&pool);
        competitor.try_grow(1).map_err(io::Error::other)?;
        let next = vec![7; 65536];
        assert!(input.extend(&next).is_err(), "old and replacement coexist");
        assert_eq!(input.len(), 5);
        assert_eq!(pool.reserved(), 65537);
        drop(competitor);
        input.extend(&next)?;
        assert_eq!(pool.reserved(), 131072);
        assert!(input.extend(&next).is_err(), "finite body bound");
        assert_eq!(input.len(), 65541);
        let bytes = input.finish();
        let held = bytes.clone();
        let prefix = held.slice(..5);
        assert_eq!(&prefix[..], b"start");
        assert_eq!(&bytes[5..], &next);
        drop(bytes);
        drop(held);
        assert_eq!(pool.reserved(), 131072);
        drop(prefix);
        assert_eq!(pool.reserved(), 0);
        let mut empty = OwnedBuffer::new(&pool, 0, "empty-input")?;
        empty.extend(&[])?;
        assert!(empty.extend(&[1]).is_err());
        assert!(empty.finish().is_empty());
        assert_eq!(pool.reserved(), 0);
        Ok(())
    }

    #[test]
    fn input_buffer_reservation_survives_shared_readers_and_refuses_before_allocation()
    -> io::Result<()> {
        use datafusion::execution::memory_pool::GreedyMemoryPool;
        let root = tempfile::tempdir()?;
        let path = root.path().join("input");
        std::fs::write(&path, [7; 128])?;
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(128));
        let bytes = Arc::new(OwnedBytes::read_file(&path, 128, &pool, "unit")?);
        assert_eq!(pool.reserved(), 128);
        let reader = bytes.clone();
        drop(bytes);
        assert_eq!(reader.as_ref().as_ref(), &[7; 128]);
        assert_eq!(pool.reserved(), 128);
        assert!(OwnedBytes::read_file(&path, 128, &pool, "exhausted").is_err());
        drop(reader);
        assert_eq!(pool.reserved(), 0);
        assert!(OwnedBytes::read_file(&path, 127, &pool, "oversized").is_err());
        assert_eq!(pool.reserved(), 0);
        assert!(OwnedBytes::read_file(root.path(), 128, &pool, "directory").is_err());
        #[cfg(target_os = "linux")]
        {
            let link = root.path().join("link");
            std::os::unix::fs::symlink(&path, &link)?;
            assert!(OwnedBytes::read_file(&link, 128, &pool, "link").is_err());
        }
        Ok(())
    }
}
