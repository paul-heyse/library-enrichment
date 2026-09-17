use std::sync::Arc;

use crate::evidence::relational::{
    ApiObservation, ApiPayload, Definition, FactSource, Locator, PublicBinding, SubjectRef,
};
use crate::identity::Ecosystem;
use arrow::array::{ArrayRef, BooleanArray};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::cells::{batch, column, list, optional, text};

/// Definition rows, shared by public aliases.
///
/// # Errors
/// Inconsistent Arrow column shapes cannot be published.
pub fn definitions(rows: &[Definition]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(ArrowError::InvalidArgumentError)?;
    }
    batch(
        "definitions",
        vec![
            column(
                "definition_id",
                text(rows.iter().map(|r| r.definition_id.as_str())),
                false,
                "key:definition",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:symbol-kind/1",
            ),
            column(
                "definition_path",
                text(rows.iter().map(|r| r.definition_path.as_str())),
                false,
                "definition-path",
            ),
            column(
                "defined_in_package",
                text(rows.iter().map(|r| r.defined_in_package.as_str())),
                false,
                "package-name",
            ),
            column(
                "qualifier",
                optional(rows.iter().map(|r| r.qualifier.as_deref())),
                true,
                "definition-qualifier",
            ),
        ],
    )
}

/// Public binding rows. Display text is separate from exact typed component identity.
///
/// # Errors
/// Inconsistent Arrow column shapes cannot be published.
pub fn bindings(rows: &[PublicBinding]) -> Result<RecordBatch, ArrowError> {
    let path_ids: Vec<_> = rows.iter().map(|r| r.path.id()).collect();
    let displays: Vec<_> = rows.iter().map(|r| r.path.display()).collect();
    let parents: Vec<_> = rows
        .iter()
        .map(|r| r.path.parent().map(|p| p.id()))
        .collect();
    batch(
        "symbols",
        vec![
            column(
                "symbol_id",
                text(rows.iter().map(|r| r.symbol_id.as_str())),
                false,
                "key:symbol",
            ),
            column(
                "definition_id",
                text(rows.iter().map(|r| r.definition_id.as_str())),
                false,
                "ref:definition",
            ),
            column(
                "path_id",
                text(path_ids.iter().map(String::as_str)),
                false,
                "ref:path",
            ),
            column(
                "ecosystem",
                text(rows.iter().map(|r| match r.path.ecosystem() {
                    Ecosystem::Rust => "rust",
                    Ecosystem::Python => "python",
                })),
                false,
                "vocabulary:ecosystem/1",
            ),
            column(
                "components",
                list(rows.iter().map(|r| r.path.components())),
                false,
                "ordered:path-components",
            ),
            column(
                "path",
                text(displays.iter().map(String::as_str)),
                false,
                "display:public-path",
            ),
            column(
                "parent_path_id",
                optional(parents.iter().map(|p| p.as_deref())),
                true,
                "ref:path",
            ),
            column(
                "name",
                text(rows.iter().map(|r| r.name.as_str())),
                false,
                "public-name",
            ),
            column(
                "is_reexport",
                Arc::new(BooleanArray::from_iter(
                    rows.iter().map(|r| Some(r.is_reexport)),
                )),
                false,
                "reexport",
            ),
            column(
                "qualifier",
                optional(rows.iter().map(|r| r.qualifier.as_deref())),
                true,
                "binding-qualifier",
            ),
        ],
    )
}

pub fn subject(rows: &[&SubjectRef]) -> Result<ArrayRef, ArrowError> {
    crate::native_union::NativeUnion::encode(rows)
}

pub fn locator(rows: &[&Locator]) -> Result<ArrayRef, ArrowError> {
    crate::native_union::NativeUnion::encode(rows)
}

pub fn source(rows: &[&FactSource]) -> Result<ArrayRef, ArrowError> {
    crate::native_union::NativeStruct::encode(
        &rows.iter().map(|row| Some(*row)).collect::<Vec<_>>(),
    )
}

fn payload(rows: &[ApiObservation]) -> Result<ArrayRef, ArrowError> {
    ApiPayload::encode_body(&rows.iter().map(|row| &row.payload).collect::<Vec<_>>())
}

/// Independently qualified API observations, preserving nulls and source/stub alternatives.
///
/// # Errors
/// Invalid observations and inconsistent Arrow shapes are refused.
pub fn observations(rows: &[ApiObservation]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(ArrowError::InvalidArgumentError)?;
    }
    observations_fields(rows)
}

pub(crate) fn observations_fields(rows: &[ApiObservation]) -> Result<RecordBatch, ArrowError> {
    let mut columns = vec![
        column(
            "observation_id",
            text(rows.iter().map(|r| r.observation_id.as_str())),
            false,
            "key:observation",
        ),
        column(
            "subject",
            subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
            false,
            "typed-subject",
        ),
        column(
            "origin",
            text(rows.iter().map(|r| r.origin.as_str())),
            false,
            "vocabulary:api-origin/1",
        ),
        column(
            "environment_id",
            text(rows.iter().map(|r| r.environment_id.as_str())),
            false,
            "ref:environment",
        ),
        column("payload", payload(rows)?, false, "api-payload"),
    ];
    columns.extend(ApiPayload::separate_columns(
        &rows.iter().map(|row| &row.payload).collect::<Vec<_>>(),
    )?);
    columns.push(column(
        "source",
        source(&rows.iter().map(|r| &r.source).collect::<Vec<_>>())?,
        false,
        "fact-provenance",
    ));
    batch("api_observations", columns)
}
