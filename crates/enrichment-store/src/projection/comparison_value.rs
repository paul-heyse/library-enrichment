//! Final delivery for the closed comparison value shapes. Selection, equality, ordering and
//! provenance stay native. Only a small inline value becomes an owned JSON tree.
use arrow::{array::Array, datatypes::FieldRef};
use enrichment_core::{
    compare::AlternativeValue,
    evidence::{Artifact, ArtifactKind, artifact_id_for},
    native_json,
    wire::ArtifactHandle,
};
use std::io::{self, Write};

const INLINE_BYTES: usize = 4096;

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
    field: &FieldRef,
    array: &dyn Array,
    row: usize,
    blobs: &crate::BlobStore,
    artifact_bytes: usize,
) -> io::Result<Option<AlternativeValue>> {
    let mut inline = Vec::new();
    match native_json::write_value(&mut inline, INLINE_BYTES, field, array, row) {
        Ok(_) => {
            crate::runtime::charge_result(inline.len()).map_err(io::Error::other)?;
            return Ok(Some(AlternativeValue::Inline {
                value: serde_json::from_slice(&inline)?,
            }));
        }
        Err(e) if e.kind() == io::ErrorKind::OutOfMemory => {}
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    }
    // Count escaped bytes from borrowed Arrow before writing. Stop a page without leaving a
    // partial artifact or charging the operation for a discarded value.
    let bytes = native_json::write_value(
        io::sink(),
        crate::result::MAX_BYTES as usize,
        field,
        array,
        row,
    )?;
    if bytes > artifact_bytes {
        return Ok(None);
    }
    let retrieved_at =
        enrichment_core::native_time::AcquisitionTime::now().map_err(io::Error::other)?;
    let stored = blobs.put_stream(
        crate::result::MAX_BYTES,
        |writer| {
            native_json::write_value(
                Charged(writer),
                crate::result::MAX_BYTES as usize,
                field,
                array,
                row,
            )
            .map(|_| ())
        },
        |digest, size_bytes| Artifact {
            artifact_id: artifact_id_for(digest),
            sha256: digest.into(),
            size_bytes,
            kind: ArtifactKind::Other,
            media_type: "application/json".into(),
            source_uri: "service:comparison-value/3".into(),
            retrieved_at,
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
        receipt: artifact.clone(),
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
        datatypes::{DataType, Field, Fields},
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
        let field = native_json::record_field(values.fields().clone());
        assert_eq!(
            deliver(
                &field,
                &values,
                0,
                &blobs,
                crate::result::MAX_BYTES as usize
            )
            .unwrap()
            .unwrap(),
            AlternativeValue::Inline {
                value: serde_json::json!({"text": "small", "optional": true})
            }
        );
        let value = deliver(
            &field,
            &values,
            1,
            &blobs,
            crate::result::MAX_BYTES as usize,
        )
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
            deliver(&field, &values, 1, &blobs, 1024).unwrap().is_none(),
            "a value beyond this page's remaining capacity is deferred before writing"
        );
        let stored = artifact.receipt.clone();
        assert_eq!(*sha256, stored.sha256);
        assert_eq!(*size_bytes, stored.size_bytes);
        let recovered: serde_json::Value =
            blobs.read_json(&stored, crate::result::MAX_BYTES).unwrap();
        assert_eq!(
            recovered,
            serde_json::json!({"optional": null, "text": text})
        );
        let repeated = deliver(
            &field,
            &values,
            1,
            &blobs,
            crate::result::MAX_BYTES as usize,
        )
        .unwrap()
        .unwrap();
        let AlternativeValue::Artifact {
            artifact: repeated,
            sha256: repeated_digest,
            size_bytes: repeated_size,
        } = repeated
        else {
            panic!("complete repeated artifact")
        };
        assert_eq!(repeated_digest, *sha256);
        assert_eq!(repeated_size, *size_bytes);
        assert_eq!(repeated.receipt.artifact_id, artifact.receipt.artifact_id);
        assert_eq!(
            std::fs::read_dir(blobs.root().join(".staging"))
                .unwrap()
                .count(),
            0
        );
    }
}
