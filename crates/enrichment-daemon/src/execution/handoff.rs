//! Host-owned framing, bounds and quarantine validation. Never trusts helper exit status as EOF.
use enrichment_core::{
    canonical,
    capsule_protocol::{
        self as protocol, Frame, Operation,
        inventory::{Entry, Inventory},
    },
    execution::ProcessEnd,
};
use std::{
    fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
};
use tokio::io::{AsyncRead, AsyncReadExt};

#[derive(Debug)]
pub struct Captured {
    pub entries: Inventory,
    pub exit_code: Option<i32>,
    pub end: ProcessEnd,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

async fn frame(source: &mut (impl AsyncRead + Unpin)) -> io::Result<Frame> {
    let length = source.read_u32().await? as usize;
    if length == 0 || length > protocol::HEADER_LIMIT {
        return Err(io::Error::other("invalid executor frame length"));
    }
    let mut bytes = vec![0; length];
    source.read_exact(&mut bytes).await?;
    serde_json::from_slice(&bytes).map_err(io::Error::other)
}

/// Caller reserves `operation.launch.resources.scratch_bytes` before creating this unmounted private quarantine.
/// The returned content is still unadmitted until whole-container absence is confirmed.
pub async fn receive(
    source: &mut (impl AsyncRead + Unpin),
    operation: &Operation,
    quarantine: &Path,
) -> io::Result<Captured> {
    operation.validate()?;
    if !fs::read_dir(quarantine)?.next().is_none() {
        return Err(io::Error::other("quarantine is not empty"));
    }
    match frame(source).await? {
        Frame::Start {
            version,
            operation_id,
        } if version == protocol::VERSION && operation_id == operation.id() => {}
        _ => return Err(io::Error::other("executor start does not match operation")),
    }
    let mut entries = Inventory::new();
    let mut total = 0u64;
    let mut metadata_bytes = 0usize;
    // Only parents of exact host-selected output roots may be implicit. Every directory
    // supplied by the executor must have its own counted entry before any child is written.
    let mut implicit = std::collections::BTreeSet::new();
    for output in operation.outputs.keys() {
        let mut parent = Path::new(output).parent();
        while let Some(path) = parent.filter(|p| !p.as_os_str().is_empty()) {
            implicit.insert(path.to_string_lossy().into_owned());
            parent = path.parent();
        }
    }
    for parent in &implicit {
        fs::create_dir_all(quarantine.join(parent))?;
    }
    loop {
        match frame(source).await? {
            Frame::Entry { path, entry } => {
                protocol::validate_path(&path)?;
                metadata_bytes = metadata_bytes
                    .checked_add(entry.metadata_bytes(&path)?)
                    .ok_or_else(|| io::Error::other("output metadata overflow"))?;
                if metadata_bytes > protocol::INVENTORY_METADATA_LIMIT {
                    return Err(io::Error::other("output metadata exceeds bound"));
                }
                if entries.len().saturating_add(implicit.len()) >= protocol::ENTRY_LIMIT
                    || entries.contains_key(&path)
                    || entries
                        .last_key_value()
                        .is_some_and(|(last, _)| last >= &path)
                    || !operation.permits(&path, &entry)
                    || entry.mode() > 0o777
                {
                    return Err(io::Error::other(
                        "unexpected, duplicate, unordered or invalid output entry",
                    ));
                }
                total = total
                    .checked_add(entry.bytes())
                    .ok_or_else(|| io::Error::other("output size overflow"))?;
                if total > operation.launch.resources.scratch_bytes {
                    return Err(io::Error::other("output exceeds reserved byte bound"));
                }
                let destination = quarantine.join(&path);
                if let Some(parent) = Path::new(&path)
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                {
                    let parent = parent
                        .to_str()
                        .ok_or_else(|| io::Error::other("invalid output parent"))?;
                    if !implicit.contains(parent)
                        && !matches!(entries.get(parent), Some(Entry::Directory { .. }))
                    {
                        return Err(io::Error::other(
                            "output parent lacks a counted directory entry",
                        ));
                    }
                }
                match &entry {
                    Entry::Directory { .. } => fs::create_dir(&destination)?,
                    Entry::File { bytes, sha256, .. } => {
                        let mut file = fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&destination)?;
                        let mut remaining = *bytes;
                        // Keep the streaming buffer off nested producer futures' stack frames.
                        let mut buffer = vec![0u8; 64 * 1024];
                        while remaining > 0 {
                            let read = usize::try_from(remaining.min(buffer.len() as u64))
                                .map_err(io::Error::other)?;
                            source.read_exact(&mut buffer[..read]).await?;
                            file.write_all(&buffer[..read])?;
                            remaining -= read as u64;
                        }
                        file.sync_all()?;
                        let (actual, size) =
                            canonical::sha256_reader(fs::File::open(&destination)?, *bytes)?;
                        if actual != *sha256 || size != *bytes {
                            return Err(io::Error::other("output content digest mismatch"));
                        }
                    }
                }
                entries.insert(path, entry);
            }
            Frame::Complete {
                operation_id,
                inventory_digest,
                exit_code,
                end,
                stdout,
                stderr,
            } => {
                if operation_id != operation.id()
                    || inventory_digest
                        != enrichment_core::capsule_protocol::inventory::digest(&entries)?
                    || stdout.len() > operation.launch.output_bytes
                    || stderr.len() > operation.launch.output_bytes
                    || end == ProcessEnd::Cancelled
                    || (end != ProcessEnd::Exited && exit_code.is_some())
                {
                    return Err(io::Error::other("invalid executor completion"));
                }
                let success = end == ProcessEnd::Exited && exit_code == Some(0);
                if (!success && !entries.is_empty())
                    || (success
                        && operation
                            .outputs
                            .keys()
                            .any(|root| !entries.contains_key(root)))
                {
                    return Err(io::Error::other(
                        "missing required output or output from failed execution",
                    ));
                }
                // Apply directory modes last so a read-only directory cannot disrupt parsing.
                for (path, entry) in entries.iter().rev() {
                    fs::set_permissions(
                        quarantine.join(path),
                        fs::Permissions::from_mode(entry.mode()),
                    )?;
                }
                return Ok(Captured {
                    entries,
                    exit_code,
                    end,
                    stdout,
                    stderr,
                });
            }
            Frame::Start { .. } => return Err(io::Error::other("duplicate executor start")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mode, OutputKind};
    fn operation() -> Operation {
        Operation {
            version: protocol::VERSION,
            invocation: None,
            prepared: None,
            mode: Mode::Command,
            argv: vec!["/bin/true".into()],
            inputs: Inventory::new(),
            outputs: [("result".into(), OutputKind::File)].into(),
            launch: enrichment_core::capsule_protocol::Launch::for_execution(
                &enrichment_core::config::Execution::default(),
                &format!("sha256:{}", "a".repeat(64)),
                "test-request",
                false,
            )
            .unwrap(),
        }
    }
    fn start(operation: &Operation) -> Vec<u8> {
        let mut stream = Vec::new();
        protocol::write_frame(
            &mut stream,
            &Frame::Start {
                version: protocol::VERSION,
                operation_id: operation.id(),
            },
        )
        .unwrap();
        stream
    }
    fn entry(stream: &mut Vec<u8>, path: &str, data: &[u8]) -> Entry {
        let value = Entry::File {
            mode: 0o600,
            bytes: data.len() as u64,
            sha256: canonical::sha256_hex(data),
        };
        protocol::write_frame(
            stream,
            &Frame::Entry {
                path: path.into(),
                entry: value.clone(),
            },
        )
        .unwrap();
        stream.extend_from_slice(data);
        value
    }
    fn complete(stream: &mut Vec<u8>, operation: &Operation, entries: &Inventory) {
        protocol::write_frame(
            stream,
            &Frame::Complete {
                operation_id: operation.id(),
                inventory_digest: enrichment_core::capsule_protocol::inventory::digest(entries)
                    .unwrap(),
                exit_code: Some(0),
                end: ProcessEnd::Exited,
                stdout: b"{\"frame\":\"complete\"}".to_vec(),
                stderr: Vec::new(),
            },
        )
        .unwrap();
    }
    #[tokio::test]
    async fn admitted_file_requires_exact_bytes_and_complete_host_bound_inventory() {
        let operation = operation();
        let mut stream = start(&operation);
        let value = entry(&mut stream, "result", b"proof");
        complete(&mut stream, &operation, &[("result".into(), value)].into());
        let quarantine = tempfile::tempdir().unwrap();
        let captured = receive(&mut stream.as_slice(), &operation, quarantine.path())
            .await
            .unwrap();
        assert_eq!(captured.stdout, b"{\"frame\":\"complete\"}");
        assert_eq!(
            fs::read(quarantine.path().join("result")).unwrap(),
            b"proof"
        );
    }
    #[tokio::test]
    async fn container_selected_parents_need_entries_but_host_selected_parents_are_bounded() {
        let mut operation = operation();
        operation.outputs = [("result".into(), OutputKind::Directory)].into();
        let mut stream = start(&operation);
        protocol::write_frame(
            &mut stream,
            &Frame::Entry {
                path: "result".into(),
                entry: Entry::Directory { mode: 0o700 },
            },
        )
        .unwrap();
        entry(&mut stream, "result/unreported/deep/file", b"bad");
        let quarantine = tempfile::tempdir().unwrap();
        assert!(
            receive(&mut stream.as_slice(), &operation, quarantine.path())
                .await
                .is_err()
        );
        assert!(!quarantine.path().join("result/unreported").exists());

        operation.outputs = [("target/doc/result.json".into(), OutputKind::File)].into();
        let mut stream = start(&operation);
        let value = entry(&mut stream, "target/doc/result.json", b"proof");
        complete(
            &mut stream,
            &operation,
            &[("target/doc/result.json".into(), value)].into(),
        );
        let quarantine = tempfile::tempdir().unwrap();
        receive(&mut stream.as_slice(), &operation, quarantine.path())
            .await
            .unwrap();
        assert_eq!(
            fs::read(quarantine.path().join("target/doc/result.json")).unwrap(),
            b"proof"
        );
    }

    #[tokio::test]
    async fn traversal_unknown_duplicate_oversized_truncated_and_digest_corrupt_streams_fail() {
        let operation = operation();
        let mut cases = vec![Vec::new(), vec![255, 255, 255, 255]];
        for path in [
            "../escape",
            "/absolute",
            "result/../escape",
            "unknown",
            "result\\escape",
        ] {
            let mut stream = start(&operation);
            entry(&mut stream, path, b"bad");
            cases.push(stream);
        }
        let mut duplicate = start(&operation);
        entry(&mut duplicate, "result", b"first");
        entry(&mut duplicate, "result", b"second");
        cases.push(duplicate);
        let mut oversized = start(&operation);
        protocol::write_frame(
            &mut oversized,
            &Frame::Entry {
                path: "result".into(),
                entry: Entry::File {
                    mode: 0o600,
                    bytes: u64::MAX,
                    sha256: "forged".into(),
                },
            },
        )
        .unwrap();
        cases.push(oversized);
        let mut corrupt = start(&operation);
        let value = entry(&mut corrupt, "result", b"proof");
        let last = corrupt.len() - 1;
        corrupt[last] = b'X';
        complete(&mut corrupt, &operation, &[("result".into(), value)].into());
        cases.push(corrupt);
        let mut incomplete = start(&operation);
        entry(&mut incomplete, "result", b"proof");
        cases.push(incomplete);
        let mut missing = start(&operation);
        complete(&mut missing, &operation, &Inventory::new());
        cases.push(missing);
        for stream in cases {
            let quarantine = tempfile::tempdir().unwrap();
            assert!(
                receive(&mut stream.as_slice(), &operation, quarantine.path())
                    .await
                    .is_err()
            );
        }
    }
}
