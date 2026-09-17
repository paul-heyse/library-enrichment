//! One current retained format: an Arrow IPC index/outcome followed by bounded JSON sections.
//!
//! Index ranges are relative to the result object. Large values are serialized from borrowed
//! JSON, never copied into a second complete document. A section read parses only the index.

use crate::BlobStore;
use enrichment_core::{
    canonical,
    evidence::{Artifact, ArtifactKind, artifact_id_for},
    native_union::{Cell, NativeStruct, Rule},
    operation::results::ResultRecord,
    wire::Envelope,
};
use std::{
    collections::BTreeMap,
    io::{self, Read, Seek, Write},
};

pub use enrichment_core::operation::results::MAX_BYTES;
const INDEX_BYTES: u64 = MAX_BYTES;
const MAGIC: &[u8; 8] = b"LERES004";
pub use enrichment_core::operation::results::{JOB_URI, MEDIA_TYPE};

enrichment_core::native_struct! {
 #[derive(Copy)]
 pub struct Window {
    start: u64 => Rule::Coordinate(enrichment_core::native_union::Unit::ByteOffset),
    end: u64 => Rule::RangeEnd { unit: enrichment_core::native_union::Unit::ByteOffset, start: "start".into() },
 }
}
enrichment_core::native_struct! {
 pub struct Index {
    body_bytes: u64 => Rule::UnsignedRange { min: 1, max: MAX_BYTES },
    codec_revision: String => Rule::NonEmpty,
    record: ResultRecord => Rule::Text,
    sections: BTreeMap<String, Window> => Rule::Map,
    references: Vec<Reference> => Rule::SequenceBounds { min: 0, max: 1024 },
 }
}
enrichment_core::native_struct! {
 pub struct Reference { receipt: Artifact => Rule::Text }
}

struct Recorder<W> {
    writer: W,
    bytes: u64,
}
impl<W: Write> Write for Recorder<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        crate::runtime::charge_artifact(0).map_err(io::Error::other)?;
        let next = self
            .bytes
            .checked_add(bytes.len() as u64)
            .filter(|n| *n <= MAX_BYTES)
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::OutOfMemory, "result body exceeds byte bound")
            })?;
        self.writer.write_all(bytes)?;
        self.bytes = next;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

fn member<W: Write, T: Cell>(
    writer: &mut Recorder<W>,
    name: &str,
    value: &T,
    sections: &mut BTreeMap<String, Window>,
) -> io::Result<()> {
    if writer.bytes > 1 {
        writer.write_all(b",")?;
    }
    serde_json::to_writer(&mut *writer, name)?;
    writer.write_all(b":")?;
    let start = writer.bytes;
    let field = std::sync::Arc::new(enrichment_core::native_union::field::<T>(name, Rule::Text));
    let array = T::encode(&[Some(value)]).map_err(io::Error::other)?;
    enrichment_core::native_json::write_value(
        &mut *writer,
        MAX_BYTES as usize,
        &field,
        array.as_ref(),
        0,
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
    let (payload_field, payload) = result.data.selected_payload().map_err(io::Error::other)?;
    let arrow::datatypes::DataType::Struct(fields) = payload_field.data_type() else {
        return Err(io::Error::other("tool result requires a native record"));
    };
    let payload = payload
        .as_any()
        .downcast_ref::<arrow::array::StructArray>()
        .ok_or_else(|| io::Error::other("tool result record representation"))?;
    for (i, field) in fields.iter().enumerate() {
        if i != 0 {
            out.write_all(b",")?;
        }
        serde_json::to_writer(&mut out, field.name())?;
        out.write_all(b":")?;
        let start = out.bytes;
        enrichment_core::native_json::write_value(
            &mut out,
            MAX_BYTES as usize,
            field,
            payload.column(i).as_ref(),
            0,
        )?;
        let window = Window {
            start,
            end: out.bytes,
        };
        sections.insert(format!("data.{}", field.name()), window);
        if let Some(section) = field.metadata().get("enrichment.section")
            && sections.insert(section.clone(), window).is_some()
        {
            return Err(io::Error::other("duplicate declared result section"));
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
    member(&mut out, "error", &result.error().cloned(), &mut sections)?;
    member(&mut out, "evidence", &result.evidence, &mut sections)?;
    member(&mut out, "freshness", &result.freshness, &mut sections)?;
    member(&mut out, "job", &result.job().cloned(), &mut sections)?;
    member(
        &mut out,
        "request_id",
        &"req_retained".to_owned(),
        &mut sections,
    )?;
    member(
        &mut out,
        "schema_version",
        &enrichment_core::SCHEMA_VERSION.to_owned(),
        &mut sections,
    )?;
    member(&mut out, "snapshot_id", &result.snapshot_id, &mut sections)?;
    member(&mut out, "status", &result.status(), &mut sections)?;
    member(&mut out, "summary", &result.summary, &mut sections)?;
    out.write_all(b"}")?;
    out.flush()?;
    sections.insert(
        "envelope".into(),
        Window {
            start: 0,
            end: out.bytes,
        },
    );
    Ok(Index {
        body_bytes: out.bytes,
        codec_revision: enrichment_core::native_json::REVISION.into(),
        record: ResultRecord::from_envelope(result).map_err(io::Error::other)?,
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
    let mut metadata = Vec::new();
    let batch = Index::batch(std::slice::from_ref(&index)).map_err(io::Error::other)?;
    {
        let bounded =
            enrichment_core::native_json::BoundedWriter::new(&mut metadata, INDEX_BYTES as usize);
        let mut writer =
            arrow::ipc::writer::StreamWriter::try_new(bounded, batch.schema().as_ref())
                .map_err(io::Error::other)?;
        writer.write(&batch).map_err(io::Error::other)?;
        writer.finish().map_err(io::Error::other)?;
    }
    let mut prefix = MAGIC.to_vec();
    prefix.extend_from_slice(&(metadata.len() as u64).to_le_bytes());
    let encoded = index
        .body_bytes
        .checked_add(prefix.len() as u64 + metadata.len() as u64)
        .filter(|bytes| *bytes <= MAX_BYTES)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::OutOfMemory,
                "indexed result exceeds byte bound",
            )
        })?;
    crate::runtime::charge_artifact(encoded as usize).map_err(io::Error::other)?;
    let retrieved_at =
        enrichment_core::native_time::AcquisitionTime::now().map_err(io::Error::other)?;
    let artifact = blobs.put_stream(
        MAX_BYTES,
        |writer| {
            writer.write_all(&prefix)?;
            writer.write_all(&metadata)?;
            if body(&mut *writer, result)? != index {
                return Err(io::Error::other(
                    "result changed between indexing and writing",
                ));
            }
            Ok(())
        },
        |digest, bytes| Artifact {
            artifact_id: artifact_id_for(digest),
            sha256: digest.into(),
            size_bytes: bytes,
            kind: ArtifactKind::Other,
            media_type: MEDIA_TYPE.into(),
            source_uri: uri.into(),
            retrieved_at,
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

/// Read the bounded native IPC index and validate all ranges against the file length.
/// The caller captures/hash-checks the immutable file before using these positions.
pub fn index(file: &mut (impl Read + Seek), total: u64) -> io::Result<(Index, u64)> {
    if total > MAX_BYTES {
        return Err(io::Error::other(
            "native result exceeds complete byte bound",
        ));
    }
    file.rewind()?;
    let mut framing = [0_u8; 16];
    file.read_exact(&mut framing)?;
    if &framing[..8] != MAGIC {
        return Err(io::Error::other("invalid native result framing"));
    }
    let length = u64::from_le_bytes(framing[8..].try_into().expect("fixed length"));
    let base = length
        .checked_add(16)
        .filter(|base| *base < total && *base <= INDEX_BYTES)
        .ok_or_else(|| io::Error::other("native result index exceeds byte bound"))?;
    let mut metadata = vec![0; length as usize];
    file.read_exact(&mut metadata)?;
    let mut reader = arrow::ipc::reader::StreamReader::try_new(io::Cursor::new(metadata), None)
        .map_err(io::Error::other)?;
    enrichment_core::native_schema::check_input(
        reader.schema().as_ref(),
        &arrow::datatypes::Schema::new(Index::fields()),
    )
    .map_err(io::Error::other)?;
    let batch = reader
        .next()
        .transpose()
        .map_err(io::Error::other)?
        .ok_or_else(|| io::Error::other("missing native result metadata"))?;
    if batch.num_rows() != 1 || reader.next().is_some() {
        return Err(io::Error::other("native result metadata cardinality"));
    }
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch)
        .map_err(io::Error::other)?;
    let index = <Index as NativeStruct>::decode(rows.row(0)).map_err(io::Error::other)?;
    if index.codec_revision != enrichment_core::native_json::REVISION
        || index.body_bytes.checked_add(base) != Some(total)
        || index.sections.len() > 128
        || index
            .sections
            .iter()
            .any(|(name, w)| name.len() > 128 || w.start >= w.end || w.end > index.body_bytes)
        || index.sections.get("envelope")
            != Some(&Window {
                start: 0,
                end: index.body_bytes,
            })
    {
        return Err(io::Error::other("invalid native result section contract"));
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
