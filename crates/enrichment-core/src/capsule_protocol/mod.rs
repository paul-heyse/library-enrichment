//! Internal executor protocol. The daemon owns requests and validates every returned byte.
//! This is not an MCP command surface and does not confer execution permission.
pub mod inventory;
use crate::{canonical, execution::ProcessEnd};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self, Write},
    path::{Component, Path},
};

pub const VERSION: u32 = 2;
pub const DATA_LIMIT: u64 = 64 * 1024 * 1024 * 1024;
pub const HEADER_LIMIT: usize = 16 * 1024 * 1024;
pub const ENTRY_LIMIT: usize = 100_000;
pub const INVENTORY_METADATA_LIMIT: usize = HEADER_LIMIT / 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Command,
    LanguageServer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputKind {
    File,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub version: u32,
    pub mode: Mode,
    pub argv: Vec<String>,
    pub inputs: inventory::Inventory,
    pub outputs: BTreeMap<String, OutputKind>,
    pub data_bytes: u64,
    pub output_bytes: usize,
    pub deadline_millis: u64,
    /// Host-selected image, helper and containment description digest.
    pub binding: String,
}

impl Operation {
    pub fn id(&self) -> String {
        canonical::digest_hex(&serde_json::json!(self))
    }

    pub fn validate(&self) -> io::Result<()> {
        if self.version != VERSION
            || self.argv.is_empty()
            || !self.argv[0].starts_with('/')
            || self.argv.len() > 1024
            || self
                .argv
                .iter()
                .try_fold(0usize, |sum, s| sum.checked_add(s.len()))
                .is_none_or(|n| n > 1024 * 1024)
            || self
                .argv
                .iter()
                .any(|s| s.len() > 1024 * 1024 || s.contains('\0'))
            || self.inputs.len() > ENTRY_LIMIT
            || self.outputs.len() > 32
            || self.data_bytes == 0
            || self.data_bytes > DATA_LIMIT
            || !(1024..=1024 * 1024).contains(&self.output_bytes)
            || !(1..=600_000).contains(&self.deadline_millis)
            || (self.mode == Mode::LanguageServer && !self.outputs.is_empty())
        {
            return Err(io::Error::other(
                "invalid or unsupported executor operation",
            ));
        }
        for path in self.inputs.keys().chain(self.outputs.keys()) {
            validate_path(path)?;
        }
        let outputs: Vec<_> = self.outputs.keys().collect();
        for (index, path) in outputs.iter().enumerate() {
            if outputs
                .iter()
                .skip(index + 1)
                .any(|other| other.starts_with(&format!("{path}/")))
            {
                return Err(io::Error::other("overlapping output roots"));
            }
        }
        Ok(())
    }

    pub fn permits(&self, path: &str, entry: &inventory::Entry) -> bool {
        self.outputs.iter().any(|(root, kind)| {
            if path == root {
                return matches!(
                    (kind, entry),
                    (OutputKind::File, inventory::Entry::File { .. })
                        | (OutputKind::Directory, inventory::Entry::Directory { .. })
                );
            }
            *kind == OutputKind::Directory && path.starts_with(&format!("{root}/"))
        })
    }
}

/// Paths have one canonical spelling, with no platform-dependent separators or components.
pub fn validate_path(path: &str) -> io::Result<()> {
    if path.is_empty()
        || path.len() > 4096
        || path.contains(['\\', '\0', '\n', '\r'])
        || path.starts_with('/')
        || path.ends_with('/')
        || path.split('/').count() > 64
        || path.split('/').any(|s| matches!(s, "" | "." | ".."))
        || Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(io::Error::other("invalid capsule relative path"));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "frame", rename_all = "snake_case", deny_unknown_fields)]
pub enum Frame {
    Start {
        version: u32,
        operation_id: String,
    },
    Entry {
        path: String,
        entry: inventory::Entry,
    },
    Complete {
        operation_id: String,
        inventory_digest: String,
        exit_code: Option<i32>,
        end: ProcessEnd,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    },
}

pub fn write_frame(sink: &mut impl Write, frame: &Frame) -> io::Result<()> {
    let bytes = serde_json::to_vec(frame)?;
    if bytes.len() > HEADER_LIMIT {
        return Err(io::Error::other("executor header exceeds limit"));
    }
    let length = u32::try_from(bytes.len()).map_err(io::Error::other)?;
    sink.write_all(&length.to_be_bytes())?;
    sink.write_all(&bytes)?;
    sink.flush()
}
