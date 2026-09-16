//! One immutable JSON result with a bounded leading section index (research-result/3).
//!
//! Index ranges are relative to the result object. Large values are serialized from borrowed
//! JSON, never copied into a second complete document. A section read parses only the index.

use crate::BlobStore;
use enrichment_core::{
    canonical,
    evidence::{Artifact, ArtifactKind, artifact_id_for},
    wire::Envelope,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self, BufRead, Read, Seek, Write},
};

pub const MAX_BYTES: u64 = 32 * 1024 * 1024;
const INDEX_BYTES: u64 = 16 * 1024;
pub const JOB_URI: &str = "service:job-delivery/3";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Format {
    #[serde(rename = "research-result/3")]
    V3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Window {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Index {
    body_bytes: u64,
    format: Format,
    pub sections: BTreeMap<String, Window>,
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub receipt: Artifact,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document<T> {
    pub index: Index,
    pub result: T,
}

struct Recorder<W> {
    writer: W,
    bytes: u64,
}
impl<W: Write> Write for Recorder<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        crate::runtime::charge_artifact(0).map_err(io::Error::other)?;
        self.writer.write_all(bytes)?;
        self.bytes = self
            .bytes
            .checked_add(bytes.len() as u64)
            .filter(|n| *n <= MAX_BYTES)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::OutOfMemory, "result body exceeds byte bound")
            })?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

fn member<W: Write>(
    writer: &mut Recorder<W>,
    name: &str,
    value: &impl Serialize,
    sections: &mut BTreeMap<String, Window>,
) -> io::Result<()> {
    if writer.bytes > 1 {
        writer.write_all(b",")?;
    }
    serde_json::to_writer(&mut *writer, name)?;
    writer.write_all(b":")?;
    let start = writer.bytes;
    // Metadata is bounded independently of the large data payload. Canonical field ordering
    // is independent of serde_json/preserve_order feature unification.
    serde_json::to_writer(
        &mut *writer,
        &canonical::BorrowedValue(&serde_json::to_value(value)?),
    )?;
    sections.insert(
        name.into(),
        Window {
            start,
            end: writer.bytes,
        },
    );
    Ok(())
}

fn body<W: Write>(writer: W, result: &Envelope) -> io::Result<Index> {
    let mut out = Recorder { writer, bytes: 0 };
    let mut sections = BTreeMap::new();
    out.write_all(b"{")?;
    member(&mut out, "artifacts", &result.artifacts, &mut sections)?;
    member(&mut out, "context_id", &result.context_id, &mut sections)?;
    member(&mut out, "coverage", &result.coverage, &mut sections)?;
    out.write_all(b",\"data\":")?;
    let start = out.bytes;
    out.write_all(b"{")?;
    let mut values: Vec<_> = result.data.iter().collect();
    values.sort_by_key(|(key, _)| *key);
    for (i, (key, value)) in values.into_iter().enumerate() {
        if i != 0 {
            out.write_all(b",")?;
        }
        serde_json::to_writer(&mut out, key)?;
        out.write_all(b":")?;
        let start = out.bytes;
        serde_json::to_writer(&mut out, &canonical::BorrowedValue(value))?;
        let window = Window {
            start,
            end: out.bytes,
        };
        sections.insert(format!("data.{key}"), window);
        let alias = match key.as_str() {
            "observations" => Some("signature"),
            "changes" => Some("changes"),
            "aspect_outcomes" => Some("aspects"),
            _ => None,
        };
        if let Some(alias) = alias {
            sections.insert(alias.into(), window);
        }
    }
    out.write_all(b"}")?;
    sections.insert(
        "data".into(),
        Window {
            start,
            end: out.bytes,
        },
    );
    member(&mut out, "delivery", &result.delivery, &mut sections)?;
    member(&mut out, "error", &result.error(), &mut sections)?;
    member(&mut out, "evidence", &result.evidence, &mut sections)?;
    member(&mut out, "freshness", &result.freshness, &mut sections)?;
    member(&mut out, "job", &result.job(), &mut sections)?;
    member(&mut out, "request_id", &"req_retained", &mut sections)?;
    member(
        &mut out,
        "schema_version",
        &enrichment_core::SCHEMA_VERSION,
        &mut sections,
    )?;
    member(&mut out, "snapshot_id", &result.snapshot_id, &mut sections)?;
    member(&mut out, "status", &result.status(), &mut sections)?;
    member(&mut out, "summary", &result.summary, &mut sections)?;
    out.write_all(b"}")?;
    out.flush()?;
    Ok(Index {
        body_bytes: out.bytes,
        format: Format::V3,
        sections,
        references: result
            .artifacts
            .iter()
            .map(|handle| Reference {
                receipt: handle.receipt.clone(),
            })
            .collect(),
    })
}

/// Index and body become one content-addressed artifact; publication/export cannot omit either.
/// # Errors
/// Invalid index size, capacity or writes leave no successful result descriptor.
pub fn store(blobs: &BlobStore, result: &Envelope, uri: &str) -> io::Result<(Artifact, Index)> {
    let index = body(io::sink(), result)?;
    let mut prefix = b"{\"index\":".to_vec();
    serde_json::to_writer(&mut prefix, &index)?;
    prefix.extend_from_slice(b",\"result\":\n");
    if prefix.len() as u64 > INDEX_BYTES {
        return Err(io::Error::other("result index exceeds bound"));
    }
    let encoded = index
        .body_bytes
        .checked_add(prefix.len() as u64 + 1)
        .filter(|bytes| *bytes <= MAX_BYTES)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::OutOfMemory,
                "indexed result exceeds byte bound",
            )
        })?;
    crate::runtime::charge_artifact(encoded as usize).map_err(io::Error::other)?;
    let artifact = blobs.put_stream(
        MAX_BYTES,
        |writer| {
            writer.write_all(&prefix)?;
            if body(&mut *writer, result)? != index {
                return Err(io::Error::other(
                    "result changed between indexing and writing",
                ));
            }
            writer.write_all(b"}")
        },
        |digest, bytes| Artifact {
            artifact_id: artifact_id_for(digest),
            sha256: digest.into(),
            size_bytes: bytes,
            kind: ArtifactKind::Other,
            media_type: "application/json".into(),
            source_uri: uri.into(),
            retrieved_at: enrichment_core::clock::now_rfc3339(),
            final_url: None,
            etag: None,
            last_modified: None,
            compression: None,
        },
    )?;
    Ok((artifact.acquired, index))
}

impl Index {
    /// Validate every index entry against the actual typed result, once at admission.
    pub fn validate_result(&self, result: &Envelope) -> io::Result<()> {
        if *self != body(io::sink(), result)? {
            return Err(io::Error::other(
                "result section index disagrees with its body",
            ));
        }
        Ok(())
    }
}

/// Read only the bounded first line and validate all relative ranges against the file length.
/// The caller captures/hash-checks the immutable file before using these positions.
pub fn index(file: &mut (impl Read + Seek), total: u64) -> io::Result<(Index, u64)> {
    file.rewind()?;
    let mut prefix = Vec::new();
    io::BufReader::new(file.take(INDEX_BYTES + 1)).read_until(b'\n', &mut prefix)?;
    let base = prefix.len() as u64;
    if base > INDEX_BYTES
        || !prefix.ends_with(b",\"result\":\n")
        || !prefix.starts_with(b"{\"index\":")
    {
        return Err(io::Error::other("invalid research-result/3 framing"));
    }
    let index: Index = serde_json::from_slice(&prefix[9..prefix.len() - 11])?;
    if index
        .body_bytes
        .checked_add(base)
        .and_then(|n| n.checked_add(1))
        != Some(total)
        || index.sections.len() > 128
        || index
            .sections
            .iter()
            .any(|(name, w)| name.len() > 128 || w.start >= w.end || w.end > index.body_bytes)
    {
        return Err(io::Error::other("invalid research result section ranges"));
    }
    Ok((index, base))
}

/// Capture once and decode one bounded section, without parsing unrelated result payloads.
pub fn read_section<T: serde::de::DeserializeOwned>(
    file: &mut std::fs::File,
    index: &Index,
    base: u64,
    section: &str,
    limit: u64,
) -> io::Result<T> {
    let range = index
        .sections
        .get(section)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "unknown result section"))?;
    if range.end - range.start > limit {
        return Err(io::Error::other("result section exceeds bound"));
    }
    file.seek(io::SeekFrom::Start(base + range.start))?;
    serde_json::from_reader(file.take(range.end - range.start)).map_err(Into::into)
}

/// Verify the entire immutable byte stream while retaining only requested indexed sections.
/// This path needs no scratch directory and never parses the unrelated data payload.
pub(crate) fn verified_sections(
    file: &mut std::fs::File,
    artifact: &Artifact,
    names: &[&str],
    limit: u64,
) -> io::Result<(Index, BTreeMap<String, serde_json::Value>)> {
    let (expected, base) = index(file, artifact.size_bytes)?;
    let mut ranges = vec![(
        Window {
            start: 0,
            end: base,
        },
        Vec::new(),
    )];
    let mut retained = base;
    for name in names {
        let window = expected
            .sections
            .get(*name)
            .ok_or_else(|| io::Error::other("missing header section"))?;
        retained = retained
            .checked_add(window.end - window.start)
            .filter(|n| *n <= limit)
            .ok_or_else(|| io::Error::other("result header exceeds bound"))?;
        ranges.push((
            Window {
                start: base + window.start,
                end: base + window.end,
            },
            Vec::new(),
        ));
    }
    struct Selected<'a> {
        file: &'a mut std::fs::File,
        offset: u64,
        ranges: Vec<(Window, Vec<u8>)>,
    }
    impl Read for Selected<'_> {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            let n = self.file.read(bytes)?;
            let end = self.offset + n as u64;
            for (range, selected) in &mut self.ranges {
                let start = self.offset.max(range.start);
                let stop = end.min(range.end);
                if start < stop {
                    selected.extend_from_slice(
                        &bytes[(start - self.offset) as usize..(stop - self.offset) as usize],
                    );
                }
            }
            self.offset = end;
            Ok(n)
        }
    }
    file.rewind()?;
    let mut selected = Selected {
        file,
        offset: 0,
        ranges,
    };
    let (digest, bytes) = canonical::sha256_reader(&mut selected, MAX_BYTES)?;
    if digest != artifact.sha256 || bytes != artifact.size_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "retained result digest or size changed",
        ));
    }
    let mut ranges = selected.ranges.into_iter();
    let (_, prefix) = ranges
        .next()
        .ok_or_else(|| io::Error::other("missing result prefix"))?;
    // Verify that the index used to select ranges was itself part of the hashed stream.
    let (actual, actual_base) = index(&mut io::Cursor::new(prefix), artifact.size_bytes)?;
    if actual != expected || actual_base != base {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "result index changed during read",
        ));
    }
    let values = names
        .iter()
        .zip(ranges)
        .map(|(name, (_, bytes))| Ok(((*name).to_owned(), serde_json::from_slice(&bytes)?)))
        .collect::<io::Result<_>>()?;
    Ok((actual, values))
}
