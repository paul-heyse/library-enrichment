//! Final delivery for the closed comparison value shapes. Selection, equality, ordering and
//! provenance stay native. Only a small inline value becomes an owned JSON tree.
use arrow::{
    array::{Array, AsArray},
    datatypes::DataType,
};
use enrichment_core::{
    compare::AlternativeValue,
    evidence::{Artifact, ArtifactKind, artifact_id_for},
    wire::ArtifactHandle,
};
use serde::{
    Serialize, Serializer,
    ser::{Error, SerializeMap, SerializeSeq},
};
use std::io::{self, Write};

const INLINE_BYTES: usize = 4096;

/// Borrow cells directly; serde_json streams string escaping to Write. Arrow 59.3's JSON
/// Writer buffers an entire row before flushing, so it cannot bound an indivisible value.
struct Cell<'a> {
    array: &'a dyn Array,
    row: usize,
}

impl Serialize for Cell<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Self { array, row } = *self;
        if array.is_null(row) {
            return serializer.serialize_none();
        }
        match array.data_type() {
            DataType::Null => serializer.serialize_none(),
            DataType::Boolean => serializer.serialize_bool(array.as_boolean().value(row)),
            DataType::UInt32 => serializer.serialize_u32(
                array
                    .as_primitive::<arrow::datatypes::UInt32Type>()
                    .value(row),
            ),
            DataType::UInt64 => serializer.serialize_u64(
                array
                    .as_primitive::<arrow::datatypes::UInt64Type>()
                    .value(row),
            ),
            DataType::Int64 => serializer.serialize_i64(
                array
                    .as_primitive::<arrow::datatypes::Int64Type>()
                    .value(row),
            ),
            DataType::Utf8 => serializer.serialize_str(array.as_string::<i32>().value(row)),
            DataType::LargeUtf8 => serializer.serialize_str(array.as_string::<i64>().value(row)),
            DataType::Utf8View => serializer.serialize_str(array.as_string_view().value(row)),
            DataType::Struct(fields) => {
                let values = array.as_struct();
                let mut order: Vec<_> = (0..fields.len()).collect();
                order.sort_by_key(|i| fields[*i].name());
                let mut object = serializer.serialize_map(Some(fields.len()))?;
                for index in order {
                    object.serialize_entry(
                        fields[index].name(),
                        &Self {
                            array: values.column(index).as_ref(),
                            row,
                        },
                    )?;
                }
                object.end()
            }
            DataType::List(_) | DataType::LargeList(_) => {
                let values = if matches!(array.data_type(), DataType::List(_)) {
                    array.as_list::<i32>().value(row)
                } else {
                    array.as_list::<i64>().value(row)
                };
                let mut list = serializer.serialize_seq(Some(values.len()))?;
                for row in 0..values.len() {
                    list.serialize_element(&Cell {
                        array: values.as_ref(),
                        row,
                    })?;
                }
                list.end()
            }
            other => Err(S::Error::custom(format!(
                "unsupported comparison value type {other}"
            ))),
        }
    }
}

struct Inline(Vec<u8>);
impl Write for Inline {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > INLINE_BYTES.saturating_sub(self.0.len()) {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "inline comparison value",
            ));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct Charged<'a>(&'a mut dyn Write);
impl Write for Charged<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        crate::runtime::charge_artifact(bytes.len()).map_err(io::Error::other)?;
        self.0.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

pub(crate) fn deliver(
    array: &dyn Array,
    row: usize,
    blobs: &crate::BlobStore,
    artifact_bytes: usize,
) -> io::Result<Option<AlternativeValue>> {
    let cell = Cell { array, row };
    let mut inline = Inline(Vec::new());
    match serde_json::to_writer(&mut inline, &cell) {
        Ok(()) => {
            crate::runtime::charge_result(inline.0.len()).map_err(io::Error::other)?;
            return Ok(Some(AlternativeValue::Inline {
                value: serde_json::from_slice(&inline.0)?,
            }));
        }
        Err(e) if e.io_error_kind() == Some(io::ErrorKind::OutOfMemory) => {}
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
    // Count escaped bytes from borrowed Arrow before writing. Stop a page without leaving a
    // partial artifact or charging the operation for a discarded value.
    let bytes =
        enrichment_core::canonical::serialized_size(&cell, crate::result::MAX_BYTES as usize)?;
    if bytes > artifact_bytes {
        return Ok(None);
    }
    let stored = blobs.put_stream(
        crate::result::MAX_BYTES,
        |writer| serde_json::to_writer(Charged(writer), &cell).map_err(io::Error::from),
        |digest, size_bytes| Artifact {
            artifact_id: artifact_id_for(digest),
            sha256: digest.into(),
            size_bytes,
            kind: ArtifactKind::Other,
            media_type: "application/json".into(),
            source_uri: "service:comparison-value/2".into(),
            retrieved_at: enrichment_core::clock::now_rfc3339(),
            final_url: None,
            etag: None,
            last_modified: None,
            compression: None,
        },
    )?;
    let artifact = stored.acquired;
    let handle = ArtifactHandle {
        uri: enrichment_core::wire::ArtifactUri::try_from(format!(
            "library-evidence://artifacts/{}",
            artifact.artifact_id
        ))
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        artifact_id: artifact.artifact_id,
        media_type: artifact.media_type,
        description: "Complete observed comparison value".into(),
    };
    Ok(Some(AlternativeValue::Artifact {
        artifact: handle,
        size_bytes: artifact.size_bytes,
        sha256: artifact.sha256,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::{
        array::{ArrayRef, BooleanArray, StringArray, StructArray},
        datatypes::{Field, Fields},
    };
    use std::sync::Arc;

    #[test]
    fn individual_large_values_are_complete_readable_and_content_addressed() {
        let directory = tempfile::tempdir().unwrap();
        let blobs = crate::BlobStore::open(directory.path()).unwrap();
        // Escaped output exceeds the removed 16 MiB render ceiling while the Arrow scalar
        // remains small enough for the native batch allowance. Include explicit nested nulls.
        let text = format!("é😀{}", "\u{0001}".repeat(3 * 1024 * 1024));
        let fields = Fields::from(vec![
            Field::new("text", DataType::Utf8, false),
            Field::new("optional", DataType::Boolean, true),
        ]);
        let values = StructArray::new(
            fields,
            vec![
                Arc::new(StringArray::from(vec!["small", &text])) as ArrayRef,
                Arc::new(BooleanArray::from(vec![Some(true), None])) as ArrayRef,
            ],
            None,
        );
        assert_eq!(
            deliver(&values, 0, &blobs, crate::result::MAX_BYTES as usize)
                .unwrap()
                .unwrap(),
            AlternativeValue::Inline {
                value: serde_json::json!({"text": "small", "optional": true})
            }
        );
        let value = deliver(&values, 1, &blobs, crate::result::MAX_BYTES as usize)
            .unwrap()
            .unwrap();
        let AlternativeValue::Artifact {
            artifact,
            size_bytes,
            sha256,
        } = &value
        else {
            panic!("artifact delivery")
        };
        assert!(*size_bytes > 16 * 1024 * 1024);
        assert!(
            deliver(&values, 1, &blobs, 1024).unwrap().is_none(),
            "a value beyond this page's remaining capacity is deferred before writing"
        );
        let stored = blobs.find(&artifact.artifact_id).unwrap().unwrap();
        assert_eq!(*sha256, stored.sha256);
        assert_eq!(*size_bytes, stored.size_bytes);
        let recovered: serde_json::Value =
            blobs.read_json(&stored, crate::result::MAX_BYTES).unwrap();
        assert_eq!(
            recovered,
            serde_json::json!({"optional": null, "text": text})
        );
        assert_eq!(enrichment_core::canonical::digest_hex(&recovered), *sha256);
        assert_eq!(
            deliver(&values, 1, &blobs, crate::result::MAX_BYTES as usize)
                .unwrap()
                .unwrap(),
            value
        );
        assert_eq!(
            std::fs::read_dir(blobs.root().join(".staging"))
                .unwrap()
                .count(),
            0
        );
    }
}
