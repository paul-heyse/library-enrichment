//! Native binding of a complete result to the exact publication candidate.
//! Rebase evaluates this plan against the new manifest; no callback interprets a transport DTO.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::*};
use enrichment_core::{
    evidence::{
        arrow_model::expressions::{child, derive_record, record},
        catalog::PublishedJobKind,
        snapshot::EvidenceManifest,
    },
    native_union::{Cell, NativeStruct, Rule},
    operation::results::{ResultHeader, ResultRecord},
    wire::{
        Coverage, Outcome,
        data::{SnapshotSummary, ToolData},
    },
};

enrichment_core::native_struct! {
    struct Binding {
        template: ResultRecord => Rule::Text,
        manifest: EvidenceManifest => Rule::Text,
        coverage: Coverage => Rule::Text,
        kind: PublishedJobKind => Rule::Text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        identity::{Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode},
        wire::{Freshness, SourceVersionMatch, data::ResolveData},
    };

    #[tokio::test]
    async fn cancelled_probe_is_not_a_failed_verdict()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        use enrichment_core::{
            execution::{
                ProbeMode, ProcessAuthority, ProcessEnd, ProcessObservation, VerificationData,
            },
            wire::{EvidenceClass, JobState},
        };
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let error: enrichment_core::wire::Envelope = serde_json::from_str(include_str!(
            "../../../tests/fixtures/wire/error.fixture.json"
        ))?;
        let mut result = ResultRecord::from_envelope(&error)?;
        let now = enrichment_core::native_time::ObservationTime::now()?;
        result.data = VerificationData {
            evidence_class: EvidenceClass::CompilerDerived,
            producer_runs: vec![],
            source_context_id: format!("ctx_{}", "1".repeat(64)).try_into().unwrap(),
            source_snapshot_id: format!("snap_{}", "1".repeat(64)).try_into().unwrap(),
            derived_context: None,
            derived_snapshot_id: None,
            environment: None,
            mode: ProbeMode::Compile,
            profile: enrichment_core::policy::ExecutionProfile::Build,
            snippet_origin: "agent".into(),
            test_intent: None,
            snippet_artifact_id: "snippet".into(),
            lock_artifact_id: None,
            result_artifact_id: "result".into(),
            limitations: vec![],
            observations: vec![ProcessObservation {
                operation_id: "operation".into(),
                authority: ProcessAuthority::Qualification {
                    definition_id: "fixture".into(),
                },
                image_id: "image".into(),
                command: vec!["probe".into()],
                started_at: now,
                finished_at: now,
                exit_code: None,
                end: ProcessEnd::Cancelled,
                stdout: String::new(),
                stderr: String::new(),
                cleanup_confirmed: true,
            }],
        }
        .into();
        for (end, expected) in [
            (ProcessEnd::Cancelled, JobState::Cancelled),
            (ProcessEnd::Deadline, JobState::Failed),
        ] {
            let ToolData::VerifyUsage(data) = &mut result.data else {
                unreachable!()
            };
            data.observations[0].end = end;
            let frame = runtime
                .session()
                .read_batch(BoundResult::batch(&[BoundResult {
                    result: result.clone(),
                    state: JobState::Failed,
                }])?)?
                .with_column("state", terminal_state(col("result"))?)?;
            assert_eq!(
                runtime.records::<BoundResult>(frame, 1).await?[0].state,
                expected
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn publication_binds_payload_scope_and_outcome_together()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let release = Release::new(ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "binding".into(),
            version: "1.0.0".into(),
            artifact_digest: None,
        });
        let environment = Environment::unspecified();
        let context = Context::new(
            release.release_id.clone(),
            environment.environment_id.clone(),
            ResearchMode::Upstream,
        );
        let metadata = enrichment_core::evidence::snapshot::SnapshotMetadata {
            context: context.clone(),
            release: release.clone(),
            environment: environment.clone(),
            symbol_package: "binding".into(),
            crate_name: "binding".into(),
            crate_version: Some("1.0.0".into()),
            normalizer_version: "native/6".into(),
            observed_configuration: None,
            producer_items: 0,
        }
        .descriptor();
        let components = [("fixture".into(), "revision".into())]
            .into_iter()
            .collect();
        let manifest = EvidenceManifest {
            snapshot_id: EvidenceManifest::derive_id(&metadata, &components)?,
            schema_version: enrichment_core::evidence::snapshot::FORMAT.into(),
            metadata,
            components,
            tables: vec![],
            counts: Default::default(),
            indexed: vec![],
            missing: vec![],
            published_at: enrichment_core::native_time::ObservationTime::now()?,
        };
        let input = ResultRecord {
            header: ResultHeader {
                summary: "unbound".into(),
                context_id: None,
                snapshot_id: None,
                coverage: Coverage::unassessed("template"),
                freshness: Freshness {
                    registry_checked_at: None,
                    source_version_match: SourceVersionMatch::Unknown,
                    latest_verified: false,
                },
                outcome: Outcome::Ok { job: None },
            },
            data: ResolveData {
                release,
                environment,
                context,
                upstream: None,
                observed_configuration: None,
                hosted_rustdoc_json: None,
                python: None,
                snapshot: None,
                artifacts: vec![],
                gaps: vec![],
                producer_runs: vec![],
                answered_from_cache: false,
            }
            .into(),
            evidence: vec![],
            artifacts: vec![],
            delivery: Default::default(),
        };
        let incomplete = bind(
            &runtime,
            input.clone(),
            &manifest,
            Coverage::unassessed("unknown"),
            PublishedJobKind::Resolve,
        )
        .await?;
        assert_eq!(incomplete.state, enrichment_core::wire::JobState::Partial);
        assert!(matches!(
            incomplete.result.header.outcome,
            Outcome::Partial { .. }
        ));
        assert_eq!(
            incomplete.result.header.snapshot_id.as_ref(),
            Some(&manifest.snapshot_id)
        );
        let ToolData::ResolveLibrary(payload) = &incomplete.result.data else {
            panic!("resolve payload")
        };
        assert_eq!(
            payload.snapshot.as_ref().unwrap().snapshot_id,
            manifest.snapshot_id
        );
        let mut coverage = Coverage::unassessed("qualified");
        coverage
            .assessments
            .push(enrichment_core::wire::evidence::ScopeAssessment {
                snapshot_id: manifest.snapshot_id.clone(),
                subject: enrichment_core::evidence::relational::SubjectRef::Library {
                    release_id: manifest.release_id.clone(),
                },
                kind: enrichment_core::evidence::EvidenceKind::PublicApi,
                state: enrichment_core::wire::evidence::ScopeState::Indexed,
                witness_id: Some("coverage-fixture".into()),
            });
        let complete = bind(
            &runtime,
            input,
            &manifest,
            coverage,
            PublishedJobKind::Resolve,
        )
        .await?;
        assert_eq!(complete.state, enrichment_core::wire::JobState::Succeeded);
        assert!(matches!(complete.result.header.outcome, Outcome::Ok { .. }));
        // Native final encoding must preserve the same generated JSON contract, including flatten.
        let request = enrichment_core::wire::RequestId::try_from("req_binding".to_owned()).unwrap();
        let envelope = complete.result.clone().into_envelope(request);
        let (field, array) = envelope.data.selected_payload()?;
        let mut bytes = Vec::new();
        enrichment_core::native_json::write_value(
            &mut bytes,
            32 * 1024 * 1024,
            &field,
            array.as_ref(),
            0,
        )?;
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes)?,
            serde_json::to_value(envelope.data)?
        );
        let delivery = crate::result::store(
            &crate::BlobStore::open(root.path())?,
            &complete
                .result
                .clone()
                .into_envelope("req_admission".to_owned().try_into()?),
            crate::result::JOB_URI,
        )?
        .0;
        let publication = enrichment_core::evidence::catalog::JobPublication {
            job_id: format!("job_{}", "1".repeat(32)),
            context_id: manifest.context_id.clone(),
            snapshot_id: manifest.snapshot_id.clone(),
            kind: PublishedJobKind::Resolve,
            state: complete.state,
            attempt_id: "attempt".into(),
            result_artifact_ids: vec![delivery.artifact_id.clone()],
            delivery,
        };
        admit_job(&runtime, &publication, complete.result.clone()).await?;
        let mut wrong = complete.result.clone();
        wrong.header.snapshot_id = None;
        assert!(admit_job(&runtime, &publication, wrong).await.is_err());
        let mut wrong = complete.result;
        let ToolData::ResolveLibrary(data) = &mut wrong.data else {
            unreachable!()
        };
        data.snapshot = None;
        assert!(admit_job(&runtime, &publication, wrong).await.is_err());
        Ok(())
    }
}

enrichment_core::native_struct! {
    pub struct BoundResult {
        result: ResultRecord => Rule::Text,
        state: enrichment_core::wire::JobState => Rule::Text,
    }
}

/// Select scope, outcome, summary and tool-specific snapshot references together.
pub async fn bind(
    runtime: &QueryRuntime,
    template: ResultRecord,
    manifest: &EvidenceManifest,
    coverage: Coverage,
    kind: PublishedJobKind,
) -> Result<BoundResult> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "result_binding",
        session
            .read_batch(Binding::batch(&[Binding {
                template,
                manifest: manifest.clone(),
                coverage,
                kind,
            }])?)?
            .into_view(),
    )?;
    let frame = session.sql("SELECT *, array_distinct(array_concat(coverage.limitations,template.header.coverage.limitations)) AS limitations, concat('Published ',manifest.snapshot_id,' for ',manifest.metadata.crate_name,' ',coalesce(manifest.metadata.crate_version,'selected revision'),'; indexed ',CAST(manifest.counts.definitions AS VARCHAR),' definitions and ',CAST(manifest.counts.fragments AS VARCHAR),' evidence fragments. See coverage and gaps for qualified limits.') AS publication_summary FROM result_binding").await?;
    let assessments = col("coverage").field("assessments");
    let member = Expr::LambdaVariable(datafusion::logical_expr::expr::LambdaVariable::new(
        "assessment".into(),
        Some(std::sync::Arc::new(enrichment_core::native_union::field::<
            enrichment_core::wire::evidence::ScopeAssessment,
        >("assessment", Rule::Text))),
    ));
    let incomplete = datafusion::functions_nested::expr_fn::array_any_match(
        assessments.clone(),
        datafusion::logical_expr::expr_fn::lambda(
            vec!["assessment"],
            member.field("state").not_eq(lit("indexed")),
        ),
    );
    let frame = frame.with_column(
        "complete",
        datafusion::functions_nested::expr_fn::array_length(assessments)
            .gt(lit(0u64))
            .and(incomplete.not()),
    )?;
    let header = col("template").field("header");
    let coverage = derive_record(
        col("coverage"),
        &Coverage::data_type(),
        &[("limitations", col("limitations"))],
    )?;
    let outcome_type = Outcome::data_type();
    let ok = record(
        &outcome_type,
        &[
            ("status", lit("ok")),
            ("ok", record(&child(&outcome_type, "ok")?, &[])?),
        ],
    )?;
    let partial = record(
        &outcome_type,
        &[
            ("status", lit("partial")),
            ("partial", record(&child(&outcome_type, "partial")?, &[])?),
        ],
    )?;
    let selected_outcome = datafusion::logical_expr::when(
        col("kind").eq(lit("resolve")),
        datafusion::logical_expr::when(col("complete"), ok).otherwise(partial.clone())?,
    )
    .when(
        header
            .clone()
            .field("outcome")
            .field("status")
            .eq(lit("ok"))
            .and(col("complete").not()),
        partial,
    )
    .otherwise(header.clone().field("outcome"))?;
    let summary =
        datafusion::logical_expr::when(col("kind").eq(lit("resolve")), col("publication_summary"))
            .otherwise(header.clone().field("summary"))?;
    let bound_header = derive_record(
        header,
        &ResultHeader::data_type(),
        &[
            (
                "context_id",
                col("manifest").field("metadata").field("context_id"),
            ),
            ("snapshot_id", col("manifest").field("snapshot_id")),
            ("summary", summary),
            ("coverage", coverage),
            ("outcome", selected_outcome),
        ],
    )?;
    let summary = record(
        &SnapshotSummary::data_type(),
        &[
            ("snapshot_id", col("manifest").field("snapshot_id")),
            (
                "normalizer_version",
                col("manifest")
                    .field("metadata")
                    .field("normalizer_version"),
            ),
            ("counts", col("manifest").field("counts")),
            ("published_at", col("manifest").field("published_at")),
        ],
    )?;
    let data = col("template").field("data");
    let data_type = ToolData::data_type();
    let resolve = derive_record(
        data.clone().field("resolve_library"),
        &child(&data_type, "resolve_library")?,
        &[("snapshot", summary)],
    )?;
    let verify = derive_record(
        data.clone().field("verify_usage"),
        &child(&data_type, "verify_usage")?,
        &[("derived_snapshot_id", col("manifest").field("snapshot_id"))],
    )?;
    let data = derive_record(
        data,
        &data_type,
        &[("resolve_library", resolve), ("verify_usage", verify)],
    )?;
    let selected = derive_record(
        col("template"),
        &ResultRecord::data_type(),
        &[("header", bound_header), ("data", data)],
    )?;
    let frame = frame.select(vec![selected.alias("result")])?;
    let state = terminal_state(col("result"))?;
    let frame = frame.select(vec![col("result"), state.alias("state")])?;
    runtime
        .records::<BoundResult>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Execution(
                "publication result selection is empty".into(),
            )
        })
}

/// A cancelled process supplied no verdict. Preserve that captured fact even though the
/// presentation outcome is an error; no caller can independently override this state.
fn terminal_state(result: Expr) -> Result<Expr> {
    let status = result
        .clone()
        .field("header")
        .field("outcome")
        .field("status");
    let data = result.field("data");
    let last = datafusion::functions_nested::expr_fn::array_element(
        data.clone().field("verify_usage").field("observations"),
        lit(-1i64),
    );
    let cancelled = data
        .field("tool")
        .eq(lit("verify_usage"))
        .and(last.field("end").eq(lit("cancelled")))
        .and(status.clone().eq(lit("error")));
    datafusion::logical_expr::when(cancelled, lit("cancelled"))
        .when(status.clone().eq(lit("ok")), lit("succeeded"))
        .when(status.clone().eq(lit("partial")), lit("partial"))
        .when(status.eq(lit("error")), lit("failed"))
        .otherwise(lit("invalid"))
}

enrichment_core::native_struct! { struct JobAdmission {
    publication: enrichment_core::evidence::catalog::JobPublication => Rule::Text,
    result: ResultRecord => Rule::Text,
} }
enrichment_core::native_struct! { struct ComparisonAdmission {
    publication: enrichment_core::evidence::catalog::ComparisonPublication => Rule::Text,
    result: ResultRecord => Rule::Text,
} }

pub async fn admit_job(
    runtime: &QueryRuntime,
    publication: &enrichment_core::evidence::catalog::JobPublication,
    result: ResultRecord,
) -> Result<()> {
    let session = runtime.session();
    let input = session
        .read_batch(JobAdmission::batch(&[JobAdmission {
            publication: publication.clone(),
            result,
        }])?)?
        .with_column("selected_state", terminal_state(col("result"))?)?;
    native_catalog::work(&session, "publication_input", input.into_view())?;
    runtime.require_empty(session.sql("SELECT publication.job_id AS witness FROM publication_input WHERE
        (result.header.context_id IS DISTINCT FROM publication.context_id)
        OR (result.header.snapshot_id IS DISTINCT FROM publication.snapshot_id)
        OR (selected_state IS DISTINCT FROM publication.state)
        OR (result.data.tool IS DISTINCT FROM CASE publication.kind WHEN 'resolve' THEN 'resolve_library' WHEN 'inspect' THEN 'inspect_symbol' WHEN 'verify' THEN 'verify_usage' END)
        OR (publication.kind='resolve' AND (result.data.resolve_library.snapshot.snapshot_id IS DISTINCT FROM publication.snapshot_id))
        OR (publication.kind='verify' AND (result.data.verify_usage.derived_snapshot_id IS DISTINCT FROM publication.snapshot_id))")
        .await?, "publication_result_scope", "publication").await
}

pub async fn admit_comparison(
    runtime: &QueryRuntime,
    publication: &enrichment_core::evidence::catalog::ComparisonPublication,
    result: ResultRecord,
) -> Result<()> {
    let session = runtime.session();
    let input = session
        .read_batch(ComparisonAdmission::batch(&[ComparisonAdmission {
            publication: publication.clone(),
            result,
        }])?)?
        .with_column("selected_state", terminal_state(col("result"))?)?;
    native_catalog::work(&session, "publication_input", input.into_view())?;
    runtime.require_empty(session.sql("SELECT publication.job_id AS witness FROM publication_input WHERE
        (result.data.tool IS DISTINCT FROM 'compare_releases')
        OR (result.header.context_id IS DISTINCT FROM publication.after_context_id)
        OR (result.header.snapshot_id IS DISTINCT FROM publication.after_snapshot_id)
        OR (selected_state IS DISTINCT FROM publication.state)
        OR (result.data.compare_releases.before.context_id IS DISTINCT FROM publication.before_context_id)
        OR (result.data.compare_releases.before.snapshot_id IS DISTINCT FROM publication.before_snapshot_id)
        OR (result.data.compare_releases.after.context_id IS DISTINCT FROM publication.after_context_id)
        OR (result.data.compare_releases.after.snapshot_id IS DISTINCT FROM publication.after_snapshot_id)")
        .await?, "comparison_result_scope", "publication").await
}
