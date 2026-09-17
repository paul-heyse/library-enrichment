use crate::evidence::{
    path::PublicPath,
    relational::{
        ApiObservation, ApiOrigin, ApiPayload, Definition, FactSource, PublicBinding, SubjectRef,
    },
};
use crate::identity::Ecosystem;
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::cells::{Row, RowSet, invalid};

/// Decode a projected definition batch. Invalid enums remain errors.
///
/// # Errors
/// Missing, null or malformed columns are rejected.
pub fn definitions(batch: &RecordBatch) -> Result<Vec<Definition>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let definition = <Definition as crate::native_union::NativeStruct>::decode(r)?;
            definition.validate().map_err(invalid)?;
            Ok(definition)
        })
        .collect()
}

/// Decode public bindings using exact components, verifying their derived display/index data.
///
/// # Errors
/// Corrupt path identities, nulls or unsupported column shapes are rejected.
pub fn bindings(batch: &RecordBatch) -> Result<Vec<PublicBinding>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let ecosystem = match r.text("ecosystem")? {
                "rust" => Ecosystem::Rust,
                "python" => Ecosystem::Python,
                _ => return Err(invalid("unknown ecosystem")),
            };
            let path = PublicPath::new(ecosystem, r.list("components")?).map_err(invalid)?;
            if path.id() != r.text("path_id")?
                || path.display() != r.text("path")?
                || path.parent().map(|p| p.id()).as_deref() != r.optional_text("parent_path_id")?
            {
                return Err(invalid("path projection disagrees with components"));
            }
            Ok(PublicBinding {
                symbol_id: r.text("symbol_id")?.into(),
                definition_id: r.text("definition_id")?.into(),
                path,
                name: r.text("name")?.into(),
                is_reexport: r.boolean("is_reexport")?,
                qualifier: r.owned("qualifier")?,
            })
        })
        .collect()
}

pub fn subject(r: Row<'_>) -> Result<SubjectRef, ArrowError> {
    let value: SubjectRef = crate::native_union::NativeUnion::decode(r)?;
    value.validate().map_err(invalid)?;
    Ok(value)
}

pub fn source(r: Row<'_>) -> Result<FactSource, ArrowError> {
    crate::native_union::NativeStruct::decode(r)
}

/// Recombine the declared compact record and its separately stored documentation.
pub fn payload(r: Row<'_>, docs: Option<String>) -> Result<ApiPayload, ArrowError> {
    ApiPayload::decode_body(r, docs)
}

/// Decode observations and recompute semantic IDs. Row order and string encoding are immaterial.
///
/// # Errors
/// Malformed variants, identities or nested values are rejected instead of silently omitted.
pub fn observations(batch: &RecordBatch) -> Result<Vec<ApiObservation>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let observation = ApiObservation {
                observation_id: r.text("observation_id")?.into(),
                subject: subject(r.structure("subject")?)?,
                origin: match r.text("origin")? {
                    "rustdoc" => ApiOrigin::Rustdoc,
                    "source" => ApiOrigin::Source,
                    "stub" => ApiOrigin::Stub,
                    _ => return Err(invalid("unknown API origin")),
                },
                environment_id:
                    <crate::identity::EnvironmentId as crate::native_union::Cell>::decode(
                        r,
                        "environment_id",
                    )?,
                payload: payload(r.structure("payload")?, r.owned("docs")?)?,
                source: source(r.structure("source")?)?,
            };
            observation.validate().map_err(invalid)?;
            Ok(observation)
        })
        .collect()
}

/// Decode native requested-domain subjects without JSON projection.
pub fn subjects(batch: &RecordBatch) -> Result<Vec<SubjectRef>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| subject(rows.row(i).structure("subject")?))
        .collect()
}
