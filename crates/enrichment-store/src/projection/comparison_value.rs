//! Final delivery for the closed comparison value shapes. Selection, equality, ordering and
//! provenance stay native. Inline values decode through the declared native union; artifacts use the same wire codec.
use arrow::{array::Array, datatypes::FieldRef};
use enrichment_core::{
    compare::AlternativeValue,
    evidence::{Artifact, ArtifactKind, artifact_id_for},
    native_json,
    wire::ArtifactHandle,
};
use std::io::{self, Write};

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
    inline: bool,
    bytes: usize,
) -> io::Result<AlternativeValue> {
    if inline {
        let actual = native_json::write_value(io::sink(), bytes, field, array, row)?;
        if actual != bytes {
            return Err(io::Error::other("native comparison byte witness changed"));
        }
        crate::runtime::charge_result(bytes).map_err(io::Error::other)?;
        let value = array
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .ok_or_else(|| io::Error::other("comparison value requires declared native variant"))?;
        let batch = arrow::record_batch::RecordBatch::from(value.clone());
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch)
            .map_err(io::Error::other)?;
        use enrichment_core::native_union::NativeUnion;
        return Ok(AlternativeValue::Inline {
            value: <enrichment_core::compare::ComparisonValue as NativeUnion>::decode(
                rows.row(row),
            )
            .map_err(io::Error::other)?,
        });
    }
    let retrieved_at =
        enrichment_core::native_time::AcquisitionTime::now().map_err(io::Error::other)?;
    let stored = blobs.put_stream(
        bytes as u64,
        |writer| {
            let actual = native_json::write_value(Charged(writer), bytes, field, array, row)?;
            if actual != bytes {
                return Err(io::Error::other("native comparison byte witness changed"));
            }
            Ok(())
        },
        |digest, size_bytes| Artifact {
            artifact_id: artifact_id_for(digest),
            sha256: digest.into(),
            size_bytes,
            kind: ArtifactKind::Other,
            media_type: "application/json".into(),
            source_uri: "service:comparison-value/4".into(),
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
    Ok(AlternativeValue::Artifact {
        artifact: handle,
        size_bytes: artifact.size_bytes,
        sha256: artifact.sha256,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{compare::ComparisonValue, native_union::Cell};
    use std::sync::Arc;

    #[test]
    fn individual_large_values_are_complete_readable_and_content_addressed() {
        let directory = tempfile::tempdir().unwrap();
        let blobs = crate::BlobStore::open(directory.path()).unwrap();
        // Escaped output exceeds the removed 16 MiB render ceiling while the Arrow scalar
        // remains small enough for the native batch allowance. Include explicit nested nulls.
        let text = format!("é😀{}", "\u{0001}".repeat(3 * 1024 * 1024));
        let short = ComparisonValue::Fragment {
            kind: enrichment_core::evidence::FragmentKind::DocText,
            text: "small".into(),
            evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
        };
        let long = ComparisonValue::Fragment {
            kind: enrichment_core::evidence::FragmentKind::DocText,
            text: text.clone(),
            evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
        };
        let values = ComparisonValue::encode(&[Some(&short), Some(&long)]).unwrap();
        let field = Arc::new(enrichment_core::native_union::field::<ComparisonValue>(
            "value",
            enrichment_core::native_union::Rule::Text,
        ));
        let small_bytes = native_json::write_value(
            io::sink(),
            crate::result::MAX_BYTES as usize,
            &field,
            values.as_ref(),
            0,
        )
        .unwrap();
        let large_bytes = native_json::write_value(
            io::sink(),
            crate::result::MAX_BYTES as usize,
            &field,
            values.as_ref(),
            1,
        )
        .unwrap();
        assert_eq!(
            deliver(&field, values.as_ref(), 0, &blobs, true, small_bytes).unwrap(),
            AlternativeValue::Inline { value: short }
        );
        let value = deliver(&field, values.as_ref(), 1, &blobs, false, large_bytes).unwrap();
        let AlternativeValue::Artifact {
            artifact,
            size_bytes,
            sha256,
        } = &value
        else {
            panic!("artifact delivery")
        };
        assert!(*size_bytes > 16 * 1024 * 1024);
        let stored = artifact.receipt.clone();
        assert_eq!(*sha256, stored.sha256);
        assert_eq!(*size_bytes, stored.size_bytes);
        let recovered: serde_json::Value =
            blobs.read_json(&stored, crate::result::MAX_BYTES).unwrap();
        assert_eq!(recovered, serde_json::to_value(&long).unwrap());
        let repeated = deliver(&field, values.as_ref(), 1, &blobs, false, large_bytes).unwrap();
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
