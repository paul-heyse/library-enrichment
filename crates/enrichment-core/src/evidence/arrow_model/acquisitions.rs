//! Attempt-specific acquisition receipts stay outside semantic fact identities.
use super::cells::{Row, column, invalid, optional, record_list, structure, text};
use crate::evidence::{Artifact, ArtifactKind, artifact_id_for};
use arrow::{
    array::{ArrayRef, UInt64Array},
    error::ArrowError,
};
use std::sync::Arc;

/// One authoritative descriptor layout for evidence receipts and durable result references.
pub fn data_type() -> arrow::datatypes::DataType {
    values(&[])
        .expect("declared artifact contract")
        .data_type()
        .clone()
}

pub fn array(rows: &[&[Artifact]]) -> Result<ArrayRef, ArrowError> {
    let flat: Vec<_> = rows.iter().flat_map(|r| r.iter()).collect();
    record_list(rows.iter().map(|r| r.len()), values(&flat)?)
}

pub fn values(flat: &[&Artifact]) -> Result<ArrayRef, ArrowError> {
    for a in flat {
        validate(a)?;
    }
    structure(
        vec![
            column(
                "artifact_id",
                text(flat.iter().map(|a| a.artifact_id.as_str())),
                false,
                "ref:artifact",
            ),
            column(
                "sha256",
                text(flat.iter().map(|a| a.sha256.as_str())),
                false,
                "sha256",
            ),
            column(
                "media_type",
                text(flat.iter().map(|a| a.media_type.as_str())),
                false,
                "media-type",
            ),
            column(
                "kind",
                text(flat.iter().map(|a| a.kind.as_str())),
                false,
                "vocabulary:artifact-kind/1",
            ),
            column(
                "size_bytes",
                Arc::new(UInt64Array::from_iter_values(
                    flat.iter().map(|a| a.size_bytes),
                )),
                false,
                "byte-count",
            ),
            column(
                "source_uri",
                text(flat.iter().map(|a| a.source_uri.as_str())),
                false,
                "source-uri",
            ),
            column(
                "final_url",
                optional(flat.iter().map(|a| a.final_url.as_deref())),
                true,
                "final-url",
            ),
            column(
                "retrieved_at",
                text(flat.iter().map(|a| a.retrieved_at.as_str())),
                false,
                "acquisition-clock",
            ),
            column(
                "etag",
                optional(flat.iter().map(|a| a.etag.as_deref())),
                true,
                "http-etag",
            ),
            column(
                "last_modified",
                optional(flat.iter().map(|a| a.last_modified.as_deref())),
                true,
                "http-last-modified",
            ),
            column(
                "compression",
                optional(flat.iter().map(|a| a.compression.as_deref())),
                true,
                "transport-compression",
            ),
        ],
        None,
    )
}

pub fn decode(row: Row<'_>) -> Result<Vec<Artifact>, ArrowError> {
    row.records("acquisitions")?
        .into_iter()
        .map(decode_one)
        .collect()
}

pub fn decode_one(r: Row<'_>) -> Result<Artifact, ArrowError> {
    let a = Artifact {
        artifact_id: r.text("artifact_id")?.into(),
        sha256: r.text("sha256")?.into(),
        media_type: r.text("media_type")?.into(),
        kind: ArtifactKind::parse(r.text("kind")?)
            .ok_or_else(|| invalid("unknown artifact kind"))?,
        size_bytes: r.number("size_bytes")?,
        source_uri: r.text("source_uri")?.into(),
        final_url: r.owned("final_url")?,
        retrieved_at: r.text("retrieved_at")?.into(),
        etag: r.owned("etag")?,
        last_modified: r.owned("last_modified")?,
        compression: r.owned("compression")?,
    };
    validate(&a)?;
    Ok(a)
}

fn validate(a: &Artifact) -> Result<(), ArrowError> {
    if a.sha256.len() != 64
        || !a
            .sha256
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        || a.artifact_id != artifact_id_for(&a.sha256)
        || a.source_uri.is_empty()
        || a.retrieved_at.is_empty()
    {
        return Err(invalid("invalid artifact acquisition descriptor"));
    }
    Ok(())
}
