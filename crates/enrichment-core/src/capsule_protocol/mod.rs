//! Internal executor protocol. The daemon owns requests and validates every returned byte.
//! This is not an MCP command surface and does not confer execution permission.
pub mod inventory;
use crate::execution::ProcessEnd;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self, Write},
    path::{Component, Path},
};

pub const VERSION: u32 = 3;
pub const DATA_LIMIT: u64 = 64 * 1024 * 1024 * 1024;
pub const HEADER_LIMIT: usize = 16 * 1024 * 1024;
pub const ENTRY_LIMIT: usize = 100_000;
pub const INVENTORY_METADATA_LIMIT: usize = HEADER_LIMIT / 2;

crate::native_vocabulary! {
pub enum Mode {
    Command = "command",
    LanguageServer = "language_server",
}
}

crate::native_vocabulary! {
pub enum OutputKind {
    File = "file",
    Directory = "directory",
}
}

crate::native_struct! {
pub struct Operation {
    version: u32 => crate::native_union::Rule::Text,
    mode: Mode => crate::native_union::Rule::Text,
    argv: Vec<String> => crate::native_union::Rule::Sequence,
    inputs: inventory::Inventory => crate::native_union::Rule::Map,
    outputs: BTreeMap<String, OutputKind> => crate::native_union::Rule::Map,
    data_bytes: u64 => crate::native_union::Rule::Text,
    output_bytes: usize => crate::native_union::Rule::Text,
    deadline_millis: u64 => crate::native_union::Rule::Text,
    /// Host-selected image, helper and containment description digest.
    binding: String => crate::native_union::Rule::Text,
}
}

impl Operation {
    pub fn binding(image: &str, containment: &str) -> datafusion::error::Result<String> {
        crate::native_key::Key::ProcessBinding.record(&crate::operation::jobs::ProcessBinding {
            image: image.into(),
            containment: containment.into(),
        })
    }

    pub fn id(&self) -> String {
        crate::native_key::Key::ProcessOperation
            .record(self)
            .expect("declared bounded executor Arrow contract")
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

#[cfg(test)]
mod native_identity_tests {
    use super::*;
    #[test]
    fn process_identity_binds_program_bytes_inputs_outputs_and_limits() {
        let original = Operation {
            version: VERSION,
            mode: Mode::Command,
            argv: vec!["/bin/tool".into(), "input".into()],
            inputs: [
                (
                    "input".into(),
                    inventory::Entry::File {
                        mode: 0o400,
                        bytes: 3,
                        sha256: "a".repeat(64),
                    },
                ),
                (
                    "directory".into(),
                    inventory::Entry::Directory { mode: 0o700 },
                ),
            ]
            .into(),
            outputs: [("result".into(), OutputKind::File)].into(),
            data_bytes: 1024,
            output_bytes: 1024,
            deadline_millis: 1000,
            binding: "exact-image-and-helper".into(),
        };
        original.validate().unwrap();
        let id = original.id();
        assert!(id.starts_with("process_"));
        for change in 0..8 {
            let mut changed = original.clone();
            match change {
                0 => changed.argv.push("--other".into()),
                1 => {
                    changed.inputs.insert(
                        "input".into(),
                        inventory::Entry::File {
                            mode: 0o400,
                            bytes: 3,
                            sha256: "b".repeat(64),
                        },
                    );
                }
                2 => {
                    changed.outputs.insert("another".into(), OutputKind::File);
                }
                3 => changed.data_bytes += 1,
                4 => changed.output_bytes += 1,
                5 => changed.deadline_millis += 1,
                6 => changed.binding.push('2'),
                _ => changed.mode = Mode::LanguageServer,
            }
            assert_ne!(id, changed.id(), "changed process field {change}");
        }
    }
}
