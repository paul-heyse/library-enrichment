//! Native Arrow structs/lists for the closed retained execution vocabulary.
use super::{
    cells::{Row, RowSet, batch, column, invalid, list, optional, record_list, structure, text},
    decode, encode,
};
use crate::{
    evidence::execution::*,
    execution::{ProbeMode, ProcessEnd},
};
use arrow::{
    array::{ArrayRef, Int64Array, UInt32Array},
    error::ArrowError,
    record_batch::RecordBatch,
};
use std::sync::Arc;

fn u32s(values: impl IntoIterator<Item = Option<u32>>) -> ArrayRef {
    Arc::new(UInt32Array::from_iter(values))
}
fn i64s(values: impl IntoIterator<Item = Option<i64>>) -> ArrayRef {
    Arc::new(Int64Array::from_iter(values))
}
// Parquet 59.3 selective padding below a list/optional struct can expose child nulls.
// Physical coordinate descendants are nullable for native UNNEST/take; position()/range()
// still require every field when its parent exists. See compatibility matrix 2026-09-14.
fn positions(rows: &[Option<Utf8Position>]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "line",
                u32s(rows.iter().map(|r| Some(r.map_or(0, |r| r.line)))),
                true,
                "zero-based-line",
            ),
            column(
                "byte",
                u32s(rows.iter().map(|r| Some(r.map_or(0, |r| r.byte)))),
                true,
                "utf8-byte-boundary",
            ),
        ],
        Some(rows.iter().map(Option::is_some).collect()),
    )
}
fn ranges(rows: &[Option<Utf8Range>]) -> Result<ArrayRef, ArrowError> {
    let empty = Utf8Position { line: 0, byte: 0 };
    structure(
        vec![
            column(
                "start",
                positions(
                    &rows
                        .iter()
                        .map(|r| Some(r.map_or(empty, |r| r.start)))
                        .collect::<Vec<_>>(),
                )?,
                true,
                "range-start",
            ),
            column(
                "end",
                positions(
                    &rows
                        .iter()
                        .map(|r| Some(r.map_or(empty, |r| r.end)))
                        .collect::<Vec<_>>(),
                )?,
                true,
                "range-end",
            ),
        ],
        Some(rows.iter().map(Option::is_some).collect()),
    )
}
fn position(r: Row<'_>) -> Result<Utf8Position, ArrowError> {
    Ok(Utf8Position {
        line: u32::try_from(r.number("line")?).map_err(|e| invalid(e.to_string()))?,
        byte: u32::try_from(r.number("byte")?).map_err(|e| invalid(e.to_string()))?,
    })
}
fn range(r: Row<'_>) -> Result<Utf8Range, ArrowError> {
    Ok(Utf8Range {
        start: position(r.structure("start")?)?,
        end: position(r.structure("end")?)?,
    })
}
fn targets(rows: &[&ExecutionTarget]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "kind",
                text(rows.iter().map(|r| match r {
                    ExecutionTarget::Artifact { .. } => "artifact",
                    ExecutionTarget::External { .. } => "external",
                    ExecutionTarget::Unresolved { .. } => "unresolved",
                })),
                false,
                "vocabulary:execution-target/1",
            ),
            column(
                "artifact_id",
                optional(rows.iter().map(|r| match r {
                    ExecutionTarget::Artifact { artifact_id, .. } => Some(artifact_id.as_str()),
                    _ => None,
                })),
                true,
                "ref:artifact",
            ),
            column(
                "range",
                ranges(
                    &rows
                        .iter()
                        .map(|r| match r {
                            ExecutionTarget::Artifact { range, .. } => Some(*range),
                            _ => None,
                        })
                        .collect::<Vec<_>>(),
                )?,
                true,
                "utf8-range",
            ),
            column(
                "scope",
                optional(rows.iter().map(|r| match r {
                    ExecutionTarget::External { scope, .. } => Some(scope.as_str()),
                    _ => None,
                })),
                true,
                "qualified-external-source",
            ),
            column(
                "path",
                optional(rows.iter().map(|r| match r {
                    ExecutionTarget::External { path, .. } => Some(path.as_str()),
                    _ => None,
                })),
                true,
                "external-relative-path",
            ),
            column(
                "limitation",
                optional(rows.iter().map(|r| match r {
                    ExecutionTarget::External { limitation, .. }
                    | ExecutionTarget::Unresolved { limitation } => Some(limitation.as_str()),
                    _ => None,
                })),
                true,
                "scope-limitation",
            ),
        ],
        None,
    )
}
fn target(r: Row<'_>) -> Result<ExecutionTarget, ArrowError> {
    Ok(match r.text("kind")? {
        "artifact" => {
            r.variant(&["artifact_id", "range"])?;
            ExecutionTarget::Artifact {
                artifact_id: r.text("artifact_id")?.into(),
                range: range(r.structure("range")?)?,
            }
        }
        "external" => {
            r.variant(&["scope", "path", "limitation"])?;
            ExecutionTarget::External {
                scope: r.text("scope")?.into(),
                path: r.text("path")?.into(),
                limitation: r.text("limitation")?.into(),
            }
        }
        "unresolved" => {
            r.variant(&["limitation"])?;
            ExecutionTarget::Unresolved {
                limitation: r.text("limitation")?.into(),
            }
        }
        _ => return Err(invalid("unknown execution target")),
    })
}
fn diagnostics(rows: &[&ExecutionDiagnostic]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "range",
                ranges(&rows.iter().map(|r| Some(r.range)).collect::<Vec<_>>())?,
                false,
                "utf8-range",
            ),
            column(
                "severity",
                u32s(rows.iter().map(|r| r.severity)),
                true,
                "lsp-severity",
            ),
            column(
                "code",
                optional(rows.iter().map(|r| r.code.as_deref())),
                true,
                "diagnostic-code",
            ),
            column(
                "source",
                optional(rows.iter().map(|r| r.source.as_deref())),
                true,
                "diagnostic-source",
            ),
            column(
                "message",
                text(rows.iter().map(|r| r.message.as_str())),
                false,
                "diagnostic-message",
            ),
        ],
        None,
    )
}
fn diagnostic(r: Row<'_>) -> Result<ExecutionDiagnostic, ArrowError> {
    Ok(ExecutionDiagnostic {
        range: range(r.structure("range")?)?,
        severity: r
            .optional_number("severity")?
            .map(u32::try_from)
            .transpose()
            .map_err(|e| invalid(e.to_string()))?,
        code: r.owned("code")?,
        source: r.owned("source")?,
        message: r.text("message")?.into(),
    })
}

fn semantics(rows: &[Option<&SemanticQuery>]) -> Result<ArrayRef, ArrowError> {
    let locations: Vec<_> = rows.iter().flatten().flat_map(|r| &r.locations).collect();
    let diagnostic_rows: Vec<_> = rows.iter().flatten().flat_map(|r| &r.diagnostics).collect();
    structure(
        vec![
            column(
                "method",
                text(rows.iter().map(|r| r.map_or("", |r| r.method.as_str()))),
                false,
                "vocabulary:semantic-method/1",
            ),
            column(
                "document_artifact_id",
                text(
                    rows.iter()
                        .map(|r| r.map_or("", |r| r.document_artifact_id.as_str())),
                ),
                false,
                "ref:artifact",
            ),
            column(
                "position",
                positions(
                    &rows
                        .iter()
                        .map(|r| r.and_then(|r| r.position))
                        .collect::<Vec<_>>(),
                )?,
                true,
                "utf8-position",
            ),
            column(
                "anchor_symbol_id",
                optional(
                    rows.iter()
                        .map(|r| r.and_then(|r| r.anchor_symbol_id.as_deref())),
                ),
                true,
                "ref:symbol",
            ),
            column(
                "server",
                text(rows.iter().map(|r| r.map_or("", |r| r.server.as_str()))),
                false,
                "server-identity",
            ),
            column(
                "outcome",
                text(rows.iter().map(|r| r.map_or("", |r| r.outcome.as_str()))),
                false,
                "vocabulary:execution-outcome/1",
            ),
            column(
                "hover",
                optional(rows.iter().map(|r| r.and_then(|r| r.hover.as_deref()))),
                true,
                "observed-hover",
            ),
            column(
                "locations",
                record_list(
                    rows.iter().map(|r| r.map_or(0, |r| r.locations.len())),
                    targets(&locations)?,
                )?,
                false,
                "scoped-locations",
            ),
            column(
                "diagnostics",
                record_list(
                    rows.iter().map(|r| r.map_or(0, |r| r.diagnostics.len())),
                    diagnostics(&diagnostic_rows)?,
                )?,
                false,
                "document-diagnostics",
            ),
            column(
                "limitations",
                list(
                    rows.iter()
                        .map(|r| r.map_or(&[][..], |r| r.limitations.as_slice())),
                ),
                false,
                "scope-limitations",
            ),
        ],
        Some(rows.iter().map(Option::is_some).collect()),
    )
}
fn objects(rows: &[Option<&RuntimeObject>]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "module",
                text(rows.iter().map(|r| r.map_or("", |r| r.module.as_str()))),
                false,
                "python-import",
            ),
            column(
                "selection",
                list(
                    rows.iter()
                        .map(|r| r.map_or(&[][..], |r| r.selection.as_slice())),
                ),
                false,
                "ordered-attribute-selection",
            ),
            column(
                "outcome",
                text(rows.iter().map(|r| r.map_or("", |r| r.outcome.as_str()))),
                false,
                "vocabulary:execution-outcome/1",
            ),
            column(
                "type_name",
                optional(rows.iter().map(|r| r.and_then(|r| r.type_name.as_deref()))),
                true,
                "runtime-type-name",
            ),
            column(
                "signature",
                optional(rows.iter().map(|r| r.and_then(|r| r.signature.as_deref()))),
                true,
                "runtime-signature",
            ),
            column(
                "docstring",
                optional(rows.iter().map(|r| r.and_then(|r| r.docstring.as_deref()))),
                true,
                "runtime-docstring",
            ),
            column(
                "attributes",
                list(
                    rows.iter()
                        .map(|r| r.map_or(&[][..], |r| r.attributes.as_slice())),
                ),
                false,
                "observed-attributes",
            ),
            column(
                "limitations",
                list(
                    rows.iter()
                        .map(|r| r.map_or(&[][..], |r| r.limitations.as_slice())),
                ),
                false,
                "scope-limitations",
            ),
        ],
        Some(rows.iter().map(Option::is_some).collect()),
    )
}
fn probes(rows: &[Option<&UsageProbe>]) -> Result<ArrayRef, ArrowError> {
    structure(
        vec![
            column(
                "mode",
                text(rows.iter().map(|r| {
                    r.map_or("", |r| match r.mode {
                        ProbeMode::Compile => "compile",
                        ProbeMode::Typecheck => "typecheck",
                        ProbeMode::Runtime => "runtime",
                    })
                })),
                false,
                "vocabulary:probe-mode/1",
            ),
            column(
                "snippet_artifact_id",
                text(
                    rows.iter()
                        .map(|r| r.map_or("", |r| r.snippet_artifact_id.as_str())),
                ),
                false,
                "ref:artifact",
            ),
            column(
                "end",
                text(rows.iter().map(|r| {
                    r.map_or("", |r| match r.end {
                        ProcessEnd::Exited => "exited",
                        ProcessEnd::Deadline => "deadline",
                        ProcessEnd::OutputLimit => "output_limit",
                        ProcessEnd::Cancelled => "cancelled",
                    })
                })),
                false,
                "vocabulary:process-end/1",
            ),
            column(
                "exit_code",
                i64s(
                    rows.iter()
                        .map(|r| r.and_then(|r| r.exit_code.map(i64::from))),
                ),
                true,
                "process-exit-code",
            ),
            column(
                "stdout",
                text(rows.iter().map(|r| r.map_or("", |r| r.stdout.as_str()))),
                false,
                "literal-process-output",
            ),
            column(
                "stderr",
                text(rows.iter().map(|r| r.map_or("", |r| r.stderr.as_str()))),
                false,
                "literal-process-error",
            ),
        ],
        Some(rows.iter().map(Option::is_some).collect()),
    )
}

/// Encode the sole authoritative execution relation with native conditional structs.
/// # Errors
/// Invalid identities, source qualification and variant payloads are rejected.
pub fn encode(rows: &[ExecutionObservation]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    fields(rows)
}

pub(crate) fn fields(rows: &[ExecutionObservation]) -> Result<RecordBatch, ArrowError> {
    batch(
        "execution_observations",
        vec![
            column(
                "observation_id",
                text(rows.iter().map(|r| r.observation_id.as_str())),
                false,
                "key:execution-observation",
            ),
            column(
                "subject",
                encode::subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "typed-subject",
            ),
            column(
                "environment_id",
                text(rows.iter().map(|r| r.environment_id.as_str())),
                false,
                "ref:environment",
            ),
            column(
                "image_id",
                text(rows.iter().map(|r| r.image_id.as_str())),
                false,
                "producer-image",
            ),
            column(
                "containment_identity",
                text(rows.iter().map(|r| r.containment_identity.as_str())),
                false,
                "containment-identity",
            ),
            column(
                "payload",
                structure(
                    vec![
                        column(
                            "kind",
                            text(rows.iter().map(|r| r.payload.kind())),
                            false,
                            "vocabulary:execution-payload/1",
                        ),
                        column(
                            "semantic_query",
                            semantics(
                                &rows
                                    .iter()
                                    .map(|r| match &r.payload {
                                        ExecutionPayload::SemanticQuery(q) => Some(q),
                                        _ => None,
                                    })
                                    .collect::<Vec<_>>(),
                            )?,
                            true,
                            "semantic-query",
                        ),
                        column(
                            "runtime_object",
                            objects(
                                &rows
                                    .iter()
                                    .map(|r| match &r.payload {
                                        ExecutionPayload::RuntimeObject(q) => Some(q),
                                        _ => None,
                                    })
                                    .collect::<Vec<_>>(),
                            )?,
                            true,
                            "runtime-object",
                        ),
                        column(
                            "usage_probe",
                            probes(
                                &rows
                                    .iter()
                                    .map(|r| match &r.payload {
                                        ExecutionPayload::UsageProbe(q) => Some(q),
                                        _ => None,
                                    })
                                    .collect::<Vec<_>>(),
                            )?,
                            true,
                            "usage-probe",
                        ),
                    ],
                    None,
                )?,
                false,
                "typed-execution-payload",
            ),
            column(
                "source",
                encode::source(&rows.iter().map(|r| &r.source).collect::<Vec<_>>())?,
                false,
                "fact-provenance",
            ),
        ],
    )
}

fn payload(r: Row<'_>) -> Result<ExecutionPayload, ArrowError> {
    let kind = r.text("kind")?;
    r.variant(&[kind])?;
    let r = r.structure(kind)?;
    let outcome = |r: Row<'_>| {
        ExecutionOutcome::parse(r.text("outcome")?)
            .ok_or_else(|| invalid("unknown execution outcome"))
    };
    Ok(match kind {
        "semantic_query" => ExecutionPayload::SemanticQuery(SemanticQuery {
            method: SemanticMethod::parse(r.text("method")?)
                .ok_or_else(|| invalid("unknown semantic method"))?,
            document_artifact_id: r.text("document_artifact_id")?.into(),
            position: r.optional_struct("position")?.map(position).transpose()?,
            anchor_symbol_id: r.owned("anchor_symbol_id")?,
            server: r.text("server")?.into(),
            outcome: outcome(r)?,
            hover: r.owned("hover")?,
            locations: r
                .records("locations")?
                .into_iter()
                .map(target)
                .collect::<Result<_, _>>()?,
            diagnostics: r
                .records("diagnostics")?
                .into_iter()
                .map(diagnostic)
                .collect::<Result<_, _>>()?,
            limitations: r.list("limitations")?,
        }),
        "runtime_object" => ExecutionPayload::RuntimeObject(RuntimeObject {
            module: r.text("module")?.into(),
            selection: r.list("selection")?,
            outcome: outcome(r)?,
            type_name: r.owned("type_name")?,
            signature: r.owned("signature")?,
            docstring: r.owned("docstring")?,
            attributes: r.list("attributes")?,
            limitations: r.list("limitations")?,
        }),
        "usage_probe" => ExecutionPayload::UsageProbe(UsageProbe {
            mode: match r.text("mode")? {
                "compile" => ProbeMode::Compile,
                "typecheck" => ProbeMode::Typecheck,
                "runtime" => ProbeMode::Runtime,
                _ => return Err(invalid("unknown probe mode")),
            },
            snippet_artifact_id: r.text("snippet_artifact_id")?.into(),
            end: match r.text("end")? {
                "exited" => ProcessEnd::Exited,
                "deadline" => ProcessEnd::Deadline,
                "output_limit" => ProcessEnd::OutputLimit,
                "cancelled" => ProcessEnd::Cancelled,
                _ => return Err(invalid("unknown process end")),
            },
            exit_code: r
                .optional_signed("exit_code")?
                .map(i32::try_from)
                .transpose()
                .map_err(|e| invalid(e.to_string()))?,
            stdout: r.text("stdout")?.into(),
            stderr: r.text("stderr")?.into(),
        }),
        _ => return Err(invalid("unknown execution payload")),
    })
}

/// Decode a bounded batch and recompute every execution fact identity.
/// # Errors
/// Extra/inconsistent variants and corrupted identities are rejected.
pub fn decode(batch: &RecordBatch) -> Result<Vec<ExecutionObservation>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = rows.row(i);
            let result = ExecutionObservation {
                observation_id: r.text("observation_id")?.into(),
                subject: decode::subject(r.structure("subject")?)?,
                environment_id: r.text("environment_id")?.into(),
                image_id: r.text("image_id")?.into(),
                containment_identity: r.text("containment_identity")?.into(),
                payload: payload(r.structure("payload")?)?,
                source: decode::source(r.structure("source")?)?,
            };
            result.validate().map_err(invalid)?;
            Ok(result)
        })
        .collect()
}
