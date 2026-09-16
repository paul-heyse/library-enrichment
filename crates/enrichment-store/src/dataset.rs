//! Bounded Arrow ingress for typed external producer records.
//!
//! Producer IPC writes use finite byte budgets independent of execution batch layout.

use std::fs::File;
use std::io::{self, Write};

/// Conservative input/output limits independent of the query memory pool.
#[derive(Debug, Clone)]
pub struct WriteLimits {
    pub record_bytes: usize,
    pub batch_rows: usize,
    pub batch_bytes: usize,
    pub file_bytes: u64,
    pub table_rows: usize,
}

impl Default for WriteLimits {
    fn default() -> Self {
        Self {
            record_bytes: 1024 * 1024,
            batch_rows: 1024,
            batch_bytes: 16 * 1024 * 1024,
            file_bytes: 256 * 1024 * 1024,
            table_rows: 1_000_000,
        }
    }
}

pub(crate) fn validate_limits(limits: &WriteLimits) -> io::Result<()> {
    if limits.record_bytes == 0
        || limits.batch_rows == 0
        || limits.batch_bytes < limits.record_bytes
        || limits.file_bytes == 0
        || limits.table_rows == 0
    {
        return Err(io::Error::other("invalid evidence write budgets"));
    }
    Ok(())
}

pub(crate) struct BoundedFile {
    pub(crate) file: File,
    pub(crate) bytes: u64,
    pub(crate) limit: u64,
}

impl Write for BoundedFile {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.bytes);
        if bytes.len() as u64 > remaining {
            return Err(io::Error::other("Arrow input exceeds byte budget"));
        }
        let written = self.file.write(bytes)?;
        self.bytes += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
