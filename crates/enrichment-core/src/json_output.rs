//! Accounted final JSON bytes. No semantic Value tree is reconstructed for transport.
use datafusion::execution::memory_pool::{MemoryConsumer, MemoryPool, MemoryReservation};
use serde::Serialize;
use serde_json::value::RawValue;
use std::{io, sync::Arc};

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct JsonOutput {
    value: Box<RawValue>,
    #[serde(skip)]
    _memory: MemoryReservation,
}
impl JsonOutput {
    pub fn as_str(&self) -> &str {
        self.value.get()
    }

    /// Charge the requested buffer before allocation. Any allocator overcapacity
    /// is charged too; boxing may copy that buffer, so its overlap is also reserved.
    pub fn write(
        pool: &Arc<dyn MemoryPool>,
        bytes: usize,
        write: impl FnOnce(&mut dyn io::Write) -> io::Result<()>,
    ) -> io::Result<Self> {
        let memory = MemoryConsumer::new("native_json_output").register(pool);
        memory.try_grow(bytes).map_err(io::Error::other)?;
        let mut output = Vec::new();
        output.try_reserve_exact(bytes).map_err(io::Error::other)?;
        memory
            .try_resize(output.capacity())
            .map_err(io::Error::other)?;
        let mut writer = crate::native_json::BoundedWriter::new(&mut output, bytes);
        write(&mut writer)?;
        if writer.written() != bytes {
            return Err(io::Error::other("JSON measurement changed"));
        }
        if output.capacity() != bytes {
            memory.try_grow(bytes).map_err(io::Error::other)?;
        }
        let text = String::from_utf8(output).map_err(io::Error::other)?;
        let value = RawValue::from_string(text).map_err(io::Error::other)?;
        memory.try_resize(bytes).map_err(io::Error::other)?;
        Ok(Self {
            value,
            _memory: memory,
        })
    }

    pub fn serialize(pool: &Arc<dyn MemoryPool>, value: &impl Serialize) -> io::Result<Self> {
        Self::write(pool, measure(value)?, |writer| {
            serde_json::to_writer(writer, value).map_err(io::Error::other)
        })
    }
}

/// Count without allocating a byte buffer or converting into an owned JSON tree.
pub fn measure(value: &impl Serialize) -> io::Result<usize> {
    let mut count = crate::native_json::BoundedWriter::new(io::sink(), usize::MAX);
    serde_json::to_writer(&mut count, value).map_err(io::Error::other)?;
    Ok(count.written())
}

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::execution::memory_pool::GreedyMemoryPool;

    #[test]
    fn output_owns_exact_buffer_until_last_transport_drop() -> io::Result<()> {
        let value = serde_json::json!({"text": "é\n🦀", "large": u64::MAX});
        let expected = serde_json::to_string(&value)?;
        let pool = Arc::new(GreedyMemoryPool::new(expected.len() * 3)) as Arc<dyn MemoryPool>;
        let encoded = JsonOutput::serialize(&pool, &value)?;
        assert_eq!(pool.reserved(), expected.len());
        assert_eq!(encoded.as_str(), expected);
        let outer = JsonOutput::serialize(&pool, &encoded)?;
        assert_eq!(outer.as_str(), expected);
        assert_eq!(pool.reserved(), expected.len() * 2);
        drop(encoded);
        assert_eq!(pool.reserved(), expected.len());
        drop(outer);
        assert_eq!(pool.reserved(), 0);
        let tiny = Arc::new(GreedyMemoryPool::new(expected.len() - 1)) as Arc<dyn MemoryPool>;
        assert!(JsonOutput::serialize(&tiny, &value).is_err());
        assert_eq!(tiny.reserved(), 0);
        Ok(())
    }

    #[test]
    fn mismatched_measurement_and_invalid_json_release_reservation() {
        let pool = Arc::new(GreedyMemoryPool::new(64)) as Arc<dyn MemoryPool>;
        for (length, text) in [(1, b"null".as_slice()), (5, b"null"), (4, b"nope")] {
            assert!(JsonOutput::write(&pool, length, |w| w.write_all(text)).is_err());
            assert_eq!(pool.reserved(), 0);
        }
    }
}
