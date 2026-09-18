use std::sync::Arc;

use crate::evidence::relational::{
    ApiObservation, ApiPayload, Definition, FactSource, Locator, PublicBinding, SubjectRef,
};
use arrow::array::{ArrayRef, BooleanArray};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;

use super::cells::{batch, column, list, native_column, optional, ruled_column, text};

/// Definition rows, shared by public aliases.
///
/// # Errors
/// Inconsistent Arrow column shapes cannot be published.
pub fn definitions(rows: &[Definition]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(ArrowError::InvalidArgumentError)?;
    }
    <Definition as crate::native_union::NativeStruct>::batch(rows)
}

/// Public binding rows. Display text is separate from exact typed component identity.
///
/// # Errors
/// Inconsistent Arrow column shapes cannot be published.
pub fn bindings(rows: &[PublicBinding]) -> Result<RecordBatch, ArrowError> {
    let ecosystems: Vec<_> = rows.iter().map(|row| row.path.ecosystem()).collect();
    let path_ids: Vec<_> = rows.iter().map(|r| r.path.id()).collect();
    let displays: Vec<_> = rows.iter().map(|r| r.path.display()).collect();
    let parents: Vec<_> = rows
        .iter()
        .map(|r| r.path.parent().map(|p| p.id()))
        .collect();
    batch(
        "symbols",
        vec![
            ruled_column(
                "symbol_id",
                text(rows.iter().map(|r| r.symbol_id.as_str())),
                false,
                "key:symbol",
                crate::native_union::Rule::NonEmpty,
            ),
            ruled_column(
                "definition_id",
                text(rows.iter().map(|r| r.definition_id.as_str())),
                false,
                "ref:definition",
                crate::native_union::Rule::Reference(crate::native_union::Domain::Definition),
            ),
            ruled_column(
                "path_id",
                text(path_ids.iter().map(String::as_str)),
                false,
                "ref:path",
                crate::native_union::Rule::NonEmpty,
            ),
            native_column(
                "ecosystem",
                &ecosystems.iter().map(Some).collect::<Vec<_>>(),
                crate::native_union::Rule::Text,
            )?,
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
            ruled_column(
                "parent_path_id",
                optional(parents.iter().map(|p| p.as_deref())),
                true,
                "ref:path",
                crate::native_union::Rule::NonEmpty,
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
        ruled_column(
            "observation_id",
            text(rows.iter().map(|r| r.observation_id.as_str())),
            false,
            "key:observation",
            crate::native_union::Rule::NonEmpty,
        ),
        column(
            "subject",
            subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
            false,
            "typed-subject",
        ),
        native_column(
            "origin",
            &rows.iter().map(|row| Some(&row.origin)).collect::<Vec<_>>(),
            crate::native_union::Rule::Text,
        )?,
        (
            crate::native_union::field::<crate::identity::EnvironmentId>(
                "environment_id",
                crate::native_union::Rule::Text,
            ),
            <crate::identity::EnvironmentId as crate::native_union::Cell>::encode(
                &rows
                    .iter()
                    .map(|row| Some(&row.environment_id))
                    .collect::<Vec<_>>(),
            )?,
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
