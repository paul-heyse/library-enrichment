use std::collections::BTreeMap;
use std::sync::Arc;

use arrow::array::{ArrayRef, BooleanArray, UInt64Array};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use enrichment_core::evidence::{
    EvidenceKind, Gap, GapReason, PlannedFallback,
    relational::{CoverageFact, CoverageOutcome, InputArtifact},
};
use enrichment_core::producer::{ProducerRun, RunOutcome};

use super::{
    cells::{Row, RowSet, batch, column, invalid, optional, record_list, structure, text},
    decode, encode,
};

fn gaps(rows: &[&[Gap]]) -> Result<ArrayRef, ArrowError> {
    let flat = rows.iter().flat_map(|r| r.iter()).collect::<Vec<_>>();
    let fallback = structure(
        vec![
            column(
                "producer",
                optional(
                    flat.iter()
                        .map(|g| g.planned_fallback.as_ref().map(|p| p.producer.as_str())),
                ),
                true,
                "producer-name",
            ),
            column(
                "profile",
                optional(
                    flat.iter()
                        .map(|g| g.planned_fallback.as_ref().map(|p| p.profile.as_str())),
                ),
                true,
                "vocabulary:execution-profile/1",
            ),
            column(
                "enabled",
                Arc::new(BooleanArray::from_iter(
                    flat.iter()
                        .map(|g| g.planned_fallback.as_ref().map(|p| p.enabled)),
                )),
                true,
                "profile-enabled",
            ),
            column(
                "next_action",
                optional(
                    flat.iter()
                        .map(|g| g.planned_fallback.as_ref().map(|p| p.next_action.as_str())),
                ),
                true,
                "operator-action",
            ),
        ],
        Some(flat.iter().map(|g| g.planned_fallback.is_some()).collect()),
    )?;
    let values = structure(
        vec![
            column(
                "kind",
                text(flat.iter().map(|g| g.kind.as_str())),
                false,
                "vocabulary:evidence-kind/1",
            ),
            column(
                "reason",
                text(flat.iter().map(|g| g.reason.as_str())),
                false,
                "vocabulary:gap-reason/1",
            ),
            column(
                "detail",
                text(flat.iter().map(|g| g.detail.as_str())),
                false,
                "gap-detail",
            ),
            column("planned_fallback", fallback, true, "planned-producer"),
        ],
        None,
    )?;
    record_list(rows.iter().map(|r| r.len()), values)
}

fn gap_from_row(r: Row<'_>) -> Result<Gap, ArrowError> {
    let planned_fallback = r
        .optional_struct("planned_fallback")?
        .map(|p| {
            let profile = p.text("profile")?;
            profile
                .parse::<enrichment_core::policy::ExecutionProfile>()
                .map_err(invalid)?;
            Ok::<_, ArrowError>(PlannedFallback {
                producer: p.text("producer")?.into(),
                profile: profile.into(),
                enabled: p.boolean("enabled")?,
                next_action: p.text("next_action")?.into(),
            })
        })
        .transpose()?;
    Ok(Gap {
        kind: EvidenceKind::parse(r.text("kind")?)
            .ok_or_else(|| invalid("unknown evidence kind"))?,
        reason: GapReason::parse(r.text("reason")?).ok_or_else(|| invalid("unknown gap reason"))?,
        detail: r.text("detail")?.into(),
        planned_fallback,
    })
}

/// Operational attempts retain their actual clocks/logs alongside the semantic producer binding.
///
/// # Errors
/// Invalid Arrow shapes cannot be encoded.
pub fn producer_runs(rows: &[ProducerRun]) -> Result<RecordBatch, ArrowError> {
    let bindings: Vec<_> = rows.iter().map(ProducerRun::semantic_binding_id).collect();
    let inputs: Vec<_> = rows.iter().flat_map(|r| r.inputs.iter()).collect();
    let inputs = record_list(
        rows.iter().map(|r| r.inputs.len()),
        structure(
            vec![
                column(
                    "role",
                    text(inputs.iter().map(|(role, _)| role.as_str())),
                    false,
                    "input-role",
                ),
                column(
                    "digest",
                    text(inputs.iter().map(|(_, digest)| digest.as_str())),
                    false,
                    "input-content-digest",
                ),
            ],
            None,
        )?,
    )?;
    batch(
        "producer_runs",
        vec![
            column(
                "attempt_id",
                text(rows.iter().map(|r| r.attempt_id.as_str())),
                false,
                "key:producer-attempt",
            ),
            column(
                "producer_binding_id",
                text(bindings.iter().map(String::as_str)),
                false,
                "semantic:producer-binding",
            ),
            column(
                "producer",
                text(rows.iter().map(|r| r.producer.as_str())),
                false,
                "producer-name",
            ),
            column(
                "producer_version",
                text(rows.iter().map(|r| r.producer_version.as_str())),
                false,
                "producer-version",
            ),
            column(
                "config_digest",
                text(rows.iter().map(|r| r.config_digest.as_str())),
                false,
                "producer-config-digest",
            ),
            column("inputs", inputs, false, "producer-inputs"),
            column(
                "profile",
                text(rows.iter().map(|r| r.profile.as_str())),
                false,
                "vocabulary:execution-profile/1",
            ),
            column(
                "started_at",
                text(rows.iter().map(|r| r.started_at.as_str())),
                false,
                "attempt-start-time",
            ),
            column(
                "finished_at",
                text(rows.iter().map(|r| r.finished_at.as_str())),
                false,
                "attempt-finish-time",
            ),
            column(
                "outcome",
                text(rows.iter().map(|r| r.outcome.as_str())),
                false,
                "vocabulary:run-outcome/1",
            ),
            column(
                "gaps",
                gaps(&rows.iter().map(|r| r.gaps.as_slice()).collect::<Vec<_>>())?,
                false,
                "producer-gaps",
            ),
            column(
                "log",
                optional(rows.iter().map(|r| r.log.as_deref())),
                true,
                "bounded-attempt-log",
            ),
        ],
    )
}

/// Decode attempts and verify the semantic binding from actual version/configuration/inputs.
///
/// # Errors
/// Duplicate input roles, unknown vocabulary and inconsistent semantic bindings are rejected.
pub fn producer_runs_from_batch(batch: &RecordBatch) -> Result<Vec<ProducerRun>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let mut inputs = BTreeMap::new();
            for input in r.records("inputs")? {
                if inputs
                    .insert(input.text("role")?.into(), input.text("digest")?.into())
                    .is_some()
                {
                    return Err(invalid("duplicate producer input role"));
                }
            }
            let run = ProducerRun {
                attempt_id: r.text("attempt_id")?.into(),
                producer: r.text("producer")?.into(),
                producer_version: r.text("producer_version")?.into(),
                config_digest: r.text("config_digest")?.into(),
                inputs,
                profile: r.text("profile")?.parse().map_err(invalid)?,
                started_at: r.text("started_at")?.into(),
                finished_at: r.text("finished_at")?.into(),
                outcome: RunOutcome::parse(r.text("outcome")?)
                    .ok_or_else(|| invalid("unknown producer outcome"))?,
                gaps: r
                    .records("gaps")?
                    .into_iter()
                    .map(gap_from_row)
                    .collect::<Result<_, _>>()?,
                log: r.owned("log")?,
            };
            if run.attempt_id.is_empty()
                || run.producer.is_empty()
                || run.producer_version.is_empty()
                || run.semantic_binding_id() != r.text("producer_binding_id")?
            {
                return Err(invalid("invalid producer binding or attempt"));
            }
            Ok(run)
        })
        .collect()
}

/// Content-addressed input descriptors with source-specific acquisition qualification.
///
/// # Errors
/// Invalid descriptor identities are refused.
pub fn input_artifacts(rows: &[InputArtifact]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    batch(
        "input_artifacts",
        vec![
            column(
                "input_id",
                text(rows.iter().map(|r| r.input_id.as_str())),
                false,
                "key:input",
            ),
            column(
                "producer_binding_id",
                text(rows.iter().map(|r| r.producer_binding_id.as_str())),
                false,
                "ref:producer-binding",
            ),
            column(
                "role",
                text(rows.iter().map(|r| r.role.as_str())),
                false,
                "input-role",
            ),
            column(
                "artifact_id",
                text(rows.iter().map(|r| r.artifact_id.as_str())),
                false,
                "ref:artifact",
            ),
            column(
                "sha256",
                text(rows.iter().map(|r| r.sha256.as_str())),
                false,
                "content-digest:sha256",
            ),
            column(
                "media_type",
                text(rows.iter().map(|r| r.media_type.as_str())),
                false,
                "media-type",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:artifact-kind/1",
            ),
            column(
                "size_bytes",
                Arc::new(UInt64Array::from_iter_values(
                    rows.iter().map(|r| r.size_bytes),
                )),
                false,
                "content-size",
            ),
            column(
                "source_uri",
                text(rows.iter().map(|r| r.source_uri.as_str())),
                false,
                "acquisition-uri",
            ),
        ],
    )
}

/// Decode and revalidate exact artifact handle/digest and qualified input identity.
///
/// # Errors
/// Invalid identities or required values cannot become incomplete provenance.
pub fn input_artifacts_from_batch(batch: &RecordBatch) -> Result<Vec<InputArtifact>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = InputArtifact {
                input_id: r.text("input_id")?.into(),
                producer_binding_id: r.text("producer_binding_id")?.into(),
                role: r.text("role")?.into(),
                artifact_id: r.text("artifact_id")?.into(),
                sha256: r.text("sha256")?.into(),
                media_type: r.text("media_type")?.into(),
                kind: enrichment_core::evidence::ArtifactKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown artifact kind"))?,
                size_bytes: r.number("size_bytes")?,
                source_uri: r.text("source_uri")?.into(),
            };
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}

/// Coverage rows encode successful scope independently of nonempty fact tables.
///
/// # Errors
/// Contradictory outcomes or malformed scope identities are refused.
pub fn coverage(rows: &[CoverageFact]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    batch(
        "coverage",
        vec![
            column(
                "coverage_id",
                text(rows.iter().map(|r| r.coverage_id.as_str())),
                false,
                "key:coverage",
            ),
            column(
                "producer_binding_id",
                text(rows.iter().map(|r| r.producer_binding_id.as_str())),
                false,
                "ref:producer-binding",
            ),
            column(
                "subject",
                encode::subject(&rows.iter().map(|r| &r.subject).collect::<Vec<_>>())?,
                false,
                "coverage-subject",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| r.kind.as_str())),
                false,
                "vocabulary:evidence-kind/1",
            ),
            column(
                "outcome",
                text(rows.iter().map(|r| match r.outcome {
                    CoverageOutcome::Indexed => "indexed",
                    CoverageOutcome::Partial => "partial",
                    CoverageOutcome::Missing => "missing",
                })),
                false,
                "vocabulary:coverage-outcome/1",
            ),
            column(
                "gaps",
                gaps(&rows.iter().map(|r| r.gaps.as_slice()).collect::<Vec<_>>())?,
                false,
                "coverage-gaps",
            ),
        ],
    )
}

/// Decode coverage without turning missing/partial scope into an empty success.
///
/// # Errors
/// Unknown outcomes and invalid identities or gap declarations are refused.
pub fn coverage_from_batch(batch: &RecordBatch) -> Result<Vec<CoverageFact>, ArrowError> {
    let columns = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = CoverageFact {
                coverage_id: r.text("coverage_id")?.into(),
                producer_binding_id: r.text("producer_binding_id")?.into(),
                subject: decode::subject(r.structure("subject")?)?,
                kind: EvidenceKind::parse(r.text("kind")?)
                    .ok_or_else(|| invalid("unknown evidence kind"))?,
                outcome: match r.text("outcome")? {
                    "indexed" => CoverageOutcome::Indexed,
                    "partial" => CoverageOutcome::Partial,
                    "missing" => CoverageOutcome::Missing,
                    _ => return Err(invalid("unknown coverage outcome")),
                },
                gaps: r
                    .records("gaps")?
                    .into_iter()
                    .map(gap_from_row)
                    .collect::<Result<_, _>>()?,
            };
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}
