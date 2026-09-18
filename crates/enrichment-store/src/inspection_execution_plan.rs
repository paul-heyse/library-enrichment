//! Native inspection producer completeness, gaps and retained result presentation.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{common::Result, prelude::*};
use enrichment_core::{
    evidence::{
        Artifact, Gap, SymbolHeader,
        arrow_model::expressions::{literal, record},
        execution::{ExecutionObservation, ExecutionPayload},
    },
    native_union::{Cell, NativeStruct, Rule},
    producer::{ProducerRun, RunOutcome},
    request::InspectRequest,
    wire::{ArtifactHandle, Coverage, JobState, data::InspectData},
};

enrichment_core::native_struct! { struct Payload { payload:ExecutionPayload => Rule::Text } }
enrichment_core::native_struct! { pub struct Summary {
    run_outcome:RunOutcome => Rule::Text,
    state:JobState => Rule::Text,
    gaps:Vec<Gap> => Rule::Set,
} }
enrichment_core::native_struct! { struct RenderInput {
    request:InspectRequest => Rule::Text,
    symbol:SymbolHeader => Rule::Text,
    run:ProducerRun => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Rendered {
    summary:String => Rule::Text,
    data:InspectData => Rule::Text,
    coverage:Coverage => Rule::Text,
    artifacts:Vec<ArtifactHandle> => Rule::Sequence,
    state:JobState => Rule::Text,
} }

async fn expected_methods(session: &SessionContext) -> Result<()> {
    native_catalog::work(
        session,
        "inspection_default_methods",
        session
            .read_empty()?
            .select(vec![
                literal(&enrichment_core::native_semantics::DEFAULT_METHODS.to_vec())?
                    .alias("methods"),
            ])?
            .into_view(),
    )?;
    let expected=session.sql("SELECT unnest(CASE WHEN coalesce(cardinality(r.execution.methods),0)=0 THEN d.methods ELSE r.execution.methods END) AS method FROM inspection_request r CROSS JOIN inspection_default_methods d WHERE r.execution.runtime IS NULL").await?;
    native_catalog::work(session, "inspection_expected_methods", expected.into_view())?;
    Ok(())
}

async fn classify(runtime: &QueryRuntime, session: &SessionContext) -> Result<()> {
    expected_methods(session).await?;
    runtime.require_empty(session.sql(r#"
      SELECT 'inspection_producer_empty' AS witness FROM inspection_payloads HAVING count(*)=0
      UNION ALL SELECT 'inspection_payload_scope' FROM inspection_payloads p CROSS JOIN inspection_request r
        WHERE p.payload.kind<>CASE WHEN r.execution.runtime IS NULL THEN 'semantic_query' ELSE 'runtime_object' END
      UNION ALL SELECT 'inspection_runtime_cardinality' FROM inspection_payloads WHERE payload.kind='runtime_object' HAVING count(*)>1
      UNION ALL SELECT 'inspection_runtime_selection' FROM inspection_payloads p CROSS JOIN inspection_request r WHERE p.payload.kind='runtime_object' AND
        ((p.payload.runtime_object.module IS DISTINCT FROM r.execution.runtime.module) OR (p.payload.runtime_object.selection IS DISTINCT FROM r.execution.runtime.attributes))
      UNION ALL SELECT 'inspection_duplicate_method' FROM inspection_payloads WHERE payload.kind='semantic_query' GROUP BY payload.semantic_query.method HAVING count(*)>1
      UNION ALL SELECT 'inspection_missing_method' FROM inspection_expected_methods e LEFT ANTI JOIN inspection_payloads p ON p.payload.semantic_query.method=e.method
      UNION ALL SELECT 'inspection_unrequested_method' FROM inspection_payloads p LEFT ANTI JOIN inspection_expected_methods e ON p.payload.semantic_query.method=e.method WHERE p.payload.kind='semantic_query'
    "#).await?,"inspection_producer_scope","inspection_publication").await?;
    let qualified=session.sql(r#"
      SELECT p.payload,d.evidence_kind AS kind,
        coalesce(p.payload.semantic_query.outcome,p.payload.runtime_object.outcome) AS outcome,
        coalesce(p.payload.semantic_query.limitations,p.payload.runtime_object.limitations) AS limitations,
        CASE WHEN p.payload.kind='semantic_query' THEN concat(p.payload.semantic_query.method,': ',array_to_string(p.payload.semantic_query.limitations,' '))
          ELSE array_to_string(p.payload.runtime_object.limitations,' ') END AS detail
      FROM inspection_payloads p JOIN operation.declarations.execution_payloads d ON p.payload.kind=d.kind
    "#).await?;
    native_catalog::work(session, "inspection_qualified", qualified.into_view())?;
    let gaps=session.sql("SELECT DISTINCT kind,CASE outcome WHEN 'failed' THEN 'extraction_failed' WHEN 'unresolved' THEN 'upstream_unavailable' ELSE 'not_attempted' END AS reason,detail FROM inspection_qualified WHERE outcome NOT IN ('results','empty')").await?;
    let gap = record(
        &Gap::data_type(),
        &Gap::fields()
            .iter()
            .filter(|f| f.name() != "planned_fallback")
            .map(|f| (f.name().as_str(), col(f.name())))
            .collect::<Vec<_>>(),
    )?;
    native_catalog::work(
        session,
        "inspection_gaps",
        gaps.select(vec![gap.alias("gap")])?.into_view(),
    )?;
    let frame=session.sql("WITH complete AS (SELECT bool_and(outcome IN ('results','empty')) AS complete FROM inspection_qualified) SELECT CASE WHEN complete THEN 'succeeded' ELSE 'partial' END AS run_outcome,CASE WHEN complete THEN 'succeeded' ELSE 'partial' END AS state,(SELECT array_agg(gap ORDER BY gap.kind,gap.reason,gap.detail) FROM inspection_gaps) AS gaps FROM complete").await?;
    let frame = frame.with_column(
        "gaps",
        datafusion::functions::core::expr_fn::coalesce(vec![
            col("gaps"),
            literal(&Vec::<Gap>::new())?,
        ]),
    )?;
    native_catalog::work(session, "inspection_summary", frame.into_view())?;
    Ok(())
}

enrichment_core::native_struct! { struct Sufficiency { sufficient:bool => Rule::Text } }

/// Reuse is qualified by the requested method or runtime selector. An explicit unsupported
/// answer is reusable evidence about this producer; a failed or incomplete attempt is not.
pub async fn sufficient(
    runtime: &QueryRuntime,
    request: &InspectRequest,
    facts: &[ExecutionObservation],
) -> Result<bool> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "inspection_request",
        InspectRequest::batch(std::slice::from_ref(request))?,
    )?;
    native_catalog::input(
        &session,
        "inspection_facts",
        ExecutionObservation::batch(facts)?,
    )?;
    expected_methods(&session).await?;
    let frame = session.sql(r#"
      WITH qualified AS (
        SELECT f.payload FROM inspection_facts f
        WHERE coalesce(f.payload.semantic_query.outcome,f.payload.runtime_object.outcome) IN ('results','empty','unsupported')
      ), missing AS (
        SELECT e.method FROM inspection_expected_methods e LEFT ANTI JOIN qualified q ON e.method=q.payload.semantic_query.method
      ), matching_runtime AS (
        SELECT q.payload FROM qualified q CROSS JOIN inspection_request r
        WHERE q.payload.kind='runtime_object' AND q.payload.runtime_object.module=r.execution.runtime.module
          AND q.payload.runtime_object.selection=r.execution.runtime.attributes
      ) SELECT CASE WHEN r.execution.runtime IS NULL THEN (SELECT count(*)=0 FROM missing)
          ELSE (SELECT count(*)>0 FROM matching_runtime) END AS sufficient FROM inspection_request r
    "#).await?;
    Ok(runtime
        .records::<Sufficiency>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("inspection sufficiency missing"))?
        .sufficient)
}

enrichment_core::native_struct! { pub struct RetainedCandidate {
    context_id:enrichment_core::identity::ContextId => Rule::Text,
    snapshot_id:enrichment_core::identity::SnapshotId => Rule::Text,
} }
enrichment_core::native_struct! { pub struct RetainedChoice {
    request:Option<InspectRequest> => Rule::Text,
    actions:Vec<enrichment_core::wire::RecoveryAction> => Rule::SequenceBounds { min:0,max:64 },
} }

enrichment_core::native_struct! { struct ExecutionScope {
    request:InspectRequest => Rule::Text,
    symbol:SymbolHeader => Rule::Text,
    ecosystem:enrichment_core::identity::Ecosystem => Rule::Text,
    snapshot_id:enrichment_core::identity::SnapshotId => Rule::Text,
} }
enrichment_core::native_struct! { pub struct AdmittedExecution {
    request:InspectRequest => Rule::Text,
    profile:enrichment_core::policy::ExecutionProfile => Rule::Text,
} }

/// Bind the durable request to the actual selected symbol and snapshot before readiness or
/// process admission. Runtime import paths must name this public binding exactly.
pub async fn admit_execution(
    runtime: &QueryRuntime,
    request: &InspectRequest,
    symbol: &SymbolHeader,
    ecosystem: enrichment_core::identity::Ecosystem,
    snapshot_id: &enrichment_core::identity::SnapshotId,
) -> Result<AdmittedExecution> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    use enrichment_core::evidence::arrow_model::expressions::derive_record;
    let session = runtime.session();
    native_catalog::input(
        &session,
        "inspection_scope",
        ExecutionScope::batch(&[ExecutionScope {
            request: request.clone(),
            symbol: symbol.clone(),
            ecosystem,
            snapshot_id: snapshot_id.clone(),
        }])?,
    )?;
    runtime.require_empty(session.sql(r#"
      SELECT 'runtime_requires_python' AS witness FROM inspection_scope WHERE request.execution.runtime IS NOT NULL AND ecosystem<>'python'
      UNION ALL SELECT 'runtime_public_binding' FROM inspection_scope WHERE request.execution.runtime IS NOT NULL AND
        array_to_string(array_concat([request.execution.runtime.module],request.execution.runtime.attributes),'.')<>symbol.path
      UNION ALL SELECT 'explicit_inspection_profile' FROM inspection_scope WHERE
        (request.execution.profile IS DISTINCT FROM CASE WHEN request.execution.runtime IS NULL THEN 'build' ELSE 'runtime' END)
        OR coalesce(request.execution.intent='retained',true)
    "#).await?,"inspection_execution_scope","inspection_execution").await?;
    let frame=session.sql("SELECT *,CASE WHEN request.execution.runtime IS NULL THEN 'build' ELSE 'runtime' END AS profile FROM inspection_scope").await?;
    let request = derive_record(
        col("request"),
        &InspectRequest::data_type(),
        &[
            ("snapshot_id", col("snapshot_id")),
            ("symbol_path", col("symbol").field("path")),
            ("definition_id", col("symbol").field("definition_id")),
        ],
    )?;
    runtime
        .records(
            frame.select(vec![request.alias("request"), col("profile")])?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| {
            datafusion::common::exec_datafusion_err!("inspection execution admission missing")
        })
}

/// The caller supplies only snapshots it has freshly admitted and compared. This plan
/// preserves ambiguity, orders exact recovery requests and forbids effect intent in reuse.
pub async fn retained_choice(
    runtime: &QueryRuntime,
    request: &InspectRequest,
    candidates: &[RetainedCandidate],
) -> Result<RetainedChoice> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    use enrichment_core::{
        evidence::arrow_model::expressions::{derive_record, variant},
        request::{InspectionOptions, ResearchRequest},
        wire::RecoveryAction,
    };
    let session = runtime.session();
    native_catalog::input(
        &session,
        "retained_candidates",
        RetainedCandidate::batch(candidates)?,
    )?;
    runtime.require_empty(session.sql("SELECT 'retained_candidate_bound' AS witness FROM retained_candidates HAVING count(*)>64 UNION ALL SELECT 'retained_candidate_duplicate' FROM retained_candidates GROUP BY context_id,snapshot_id HAVING count(*)>1").await?,"retained_inspection_candidates","inspection_reuse").await?;
    let frame = session
        .table("retained_candidates")
        .await?
        .with_column("original", literal(request)?)?;
    let execution = derive_record(
        col("original").field("execution"),
        &InspectionOptions::data_type(),
        &[("intent", lit("retained"))],
    )?;
    let request = derive_record(
        col("original"),
        &InspectRequest::data_type(),
        &[
            ("context_id", col("context_id")),
            ("snapshot_id", col("snapshot_id")),
            ("execution", execution),
        ],
    )?;
    let frame = frame.with_column("request", request)?;
    let tool = variant(
        &ResearchRequest::data_type(),
        "inspect_symbol",
        &InspectRequest::fields()
            .iter()
            .map(|f| (f.name().as_str(), col("request").field(f.name())))
            .collect::<Vec<_>>(),
    )?;
    let action = variant(
        &RecoveryAction::data_type(),
        "call_tool",
        &[("request", tool)],
    )?;
    native_catalog::work(
        &session,
        "retained_requests",
        frame.with_column("action", action)?.into_view(),
    )?;
    let frame=session.sql("SELECT CASE WHEN count(*)=1 THEN first_value(request) ELSE NULL END AS request,CASE WHEN count(*)>1 THEN array_agg(action ORDER BY context_id,snapshot_id) ELSE NULL END AS actions FROM retained_requests").await?.with_column("actions",datafusion::functions::core::expr_fn::coalesce(vec![col("actions"),literal(&Vec::<RecoveryAction>::new())?]))?;
    runtime.records(frame, 1).await?.pop().ok_or_else(|| {
        datafusion::common::exec_datafusion_err!("retained inspection selection missing")
    })
}

pub async fn summarize(
    runtime: &QueryRuntime,
    request: &InspectRequest,
    payloads: Vec<ExecutionPayload>,
) -> Result<Summary> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "inspection_request",
        InspectRequest::batch(std::slice::from_ref(request))?,
    )?;
    native_catalog::input(
        &session,
        "inspection_payloads",
        Payload::batch(
            &payloads
                .into_iter()
                .map(|payload| Payload { payload })
                .collect::<Vec<_>>(),
        )?,
    )?;
    classify(runtime, &session).await?;
    runtime
        .records(session.table("inspection_summary").await?, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("inspection summary missing"))
}

/// Artifact selection is an exact join to result sources and the actual attempt log. Raw
/// input artifacts remain in publication provenance without becoming unrelated output handles.
pub async fn render(
    runtime: &QueryRuntime,
    request: &InspectRequest,
    symbol: SymbolHeader,
    facts: Vec<ExecutionObservation>,
    run: ProducerRun,
    artifacts: Vec<Artifact>,
) -> Result<Rendered> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "inspection_request",
        InspectRequest::batch(std::slice::from_ref(request))?,
    )?;
    native_catalog::input(
        &session,
        "inspection_render",
        RenderInput::batch(&[RenderInput {
            request: request.clone(),
            symbol,
            run,
        }])?,
    )?;
    native_catalog::input(
        &session,
        "inspection_facts",
        ExecutionObservation::batch(&facts)?,
    )?;
    native_catalog::input(
        &session,
        "inspection_artifacts",
        Artifact::batch(&artifacts)?,
    )?;
    native_catalog::work(
        &session,
        "inspection_payloads",
        session
            .sql("SELECT payload FROM inspection_facts")
            .await?
            .into_view(),
    )?;
    classify(runtime, &session).await?;
    runtime.require_empty(session.sql(r#"
      SELECT 'producer_summary_changed' AS witness FROM inspection_render r CROSS JOIN inspection_summary s WHERE r.run.outcome<>s.run_outcome OR r.run.gaps IS DISTINCT FROM s.gaps
      UNION ALL SELECT 'duplicate_inspection_fact' FROM inspection_facts GROUP BY observation_id HAVING count(*)>1
      UNION ALL SELECT 'conflicting_artifact_receipts' FROM inspection_artifacts GROUP BY artifact_id HAVING count(DISTINCT named_struct('sha256',sha256,'media_type',media_type,'size_bytes',size_bytes))>1
      UNION ALL SELECT 'result_artifact_missing' FROM inspection_facts f LEFT ANTI JOIN inspection_artifacts a ON f.source.artifact_id=a.artifact_id
      UNION ALL SELECT 'attempt_log_missing' FROM inspection_render r LEFT ANTI JOIN inspection_artifacts a ON r.run.log=a.artifact_id WHERE r.run.log IS NOT NULL
    "#).await?,"inspection_result_closure","inspection_publication").await?;
    let fact = record(
        &ExecutionObservation::data_type(),
        &ExecutionObservation::fields()
            .iter()
            .map(|f| (f.name().as_str(), col(f.name())))
            .collect::<Vec<_>>(),
    )?;
    native_catalog::work(
        &session,
        "inspection_fact_values",
        session
            .table("inspection_facts")
            .await?
            .select(vec![fact.alias("fact")])?
            .into_view(),
    )?;
    let selected=session.sql("WITH selected AS (SELECT source.artifact_id AS artifact_id FROM inspection_facts UNION SELECT run.log AS artifact_id FROM inspection_render WHERE run.log IS NOT NULL) SELECT DISTINCT a.* FROM inspection_artifacts a LEFT SEMI JOIN selected s ON a.artifact_id=s.artifact_id").await?;
    let receipt = record(
        &Artifact::data_type(),
        &Artifact::fields()
            .iter()
            .map(|f| (f.name().as_str(), col(f.name())))
            .collect::<Vec<_>>(),
    )?;
    let handle = record(
        &ArtifactHandle::data_type(),
        &[
            ("receipt", receipt.clone()),
            (
                "uri",
                datafusion::functions::string::expr_fn::concat(vec![
                    lit(format!(
                        "{}artifacts/",
                        enrichment_core::wire::ids::ARTIFACT_URI_SCHEME
                    )),
                    col("artifact_id"),
                ]),
            ),
            (
                "description",
                lit("Retained execution result or actual attempt log"),
            ),
        ],
    )?;
    native_catalog::work(
        &session,
        "inspection_handles",
        selected
            .select(vec![
                handle.alias("handle"),
                enrichment_core::native_key::Key::ArtifactReceipt
                    .bind(vec![receipt])?
                    .alias("receipt_key"),
            ])?
            .into_view(),
    )?;
    let base=session.sql(r#"
      SELECT r.*,s.state,
        CASE WHEN r.request.execution.runtime IS NULL THEN 'semantics' ELSE 'runtime' END AS aspect,
        CASE WHEN r.request.execution.runtime IS NULL THEN 'semantic_queries' ELSE 'runtime_api' END AS kind,
        concat('Retained ',CASE WHEN r.request.execution.runtime IS NULL THEN 'semantic' ELSE 'runtime' END,' inspection of `',r.symbol.path,'` in its resolved environment.') AS summary,
        (SELECT array_agg(fact ORDER BY fact.observation_id) FROM inspection_fact_values) AS facts,
        (SELECT array_agg(handle ORDER BY handle.receipt.artifact_id,receipt_key) FROM inspection_handles) AS handles,
        (SELECT array_agg(note ORDER BY note) FROM (SELECT DISTINCT unnest(limitations) AS note FROM inspection_qualified)) AS limitations
      FROM inspection_render r CROSS JOIN inspection_summary s
    "#).await?;
    let data = record(
        &InspectData::data_type(),
        &[
            ("symbol", col("symbol")),
            ("execution_observations", col("facts")),
            (
                "producer_runs",
                datafusion::functions_nested::expr_fn::make_array(vec![col("run")]),
            ),
            (
                "aspects",
                datafusion::functions_nested::expr_fn::make_array(vec![col("aspect")]),
            ),
            ("docs_truncated", lit(false)),
            (
                "children",
                literal(&Vec::<enrichment_core::wire::data::InspectionCandidate>::new())?,
            ),
            (
                "members",
                literal(&Vec::<enrichment_core::wire::data::InspectionCandidate>::new())?,
            ),
            (
                "candidates",
                literal(&Vec::<enrichment_core::wire::data::InspectionCandidate>::new())?,
            ),
            (
                "aspect_outcomes",
                literal(&Vec::<enrichment_core::wire::AspectOutcome>::new())?,
            ),
            ("also_at", literal(&Vec::<String>::new())?),
            (
                "observations",
                literal(&Vec::<enrichment_core::wire::data::ApiObservationProjection>::new())?,
            ),
            (
                "relationships",
                literal(&Vec::<
                    enrichment_core::evidence::relational::RelationshipObservation,
                >::new())?,
            ),
            (
                "fragments",
                literal(&Vec::<enrichment_core::wire::data::FragmentProjection>::new())?,
            ),
        ],
    )?;
    let missing = datafusion::logical_expr::when(
        col("state").eq(lit("succeeded")),
        literal(&Vec::<String>::new())?,
    )
    .otherwise(datafusion::functions_nested::expr_fn::make_array(vec![
        col("kind"),
    ]))?;
    let coverage = record(
        &Coverage::data_type(),
        &[
            (
                "scope",
                lit("selected consumer queries; no complete-library execution claim"),
            ),
            (
                "indexed",
                datafusion::functions_nested::expr_fn::make_array(vec![col("kind")]),
            ),
            ("missing", missing),
            (
                "limitations",
                datafusion::functions::core::expr_fn::coalesce(vec![
                    col("limitations"),
                    literal(&Vec::<String>::new())?,
                ]),
            ),
            (
                "assessments",
                literal(&Vec::<enrichment_core::wire::evidence::ScopeAssessment>::new())?,
            ),
        ],
    )?;
    runtime
        .records(
            base.select(vec![
                col("summary"),
                data.alias("data"),
                coverage.alias("coverage"),
                datafusion::functions::core::expr_fn::coalesce(vec![
                    col("handles"),
                    literal(&Vec::<ArtifactHandle>::new())?,
                ])
                .alias("artifacts"),
                col("state"),
            ])?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("inspection rendering missing"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{
            ArtifactKind, GapReason, SymbolKind,
            execution::{ExecutionOutcome, RuntimeObject},
            relational::{FactSource, Locator, SubjectRef},
        },
        native_time::{AcquisitionTime, ObservationTime},
        policy::ExecutionProfile,
        request::{InspectionOptions, RuntimeSelection},
        wire::{EvidenceClass, ResearchSelection, SourceVersionMatch},
    };
    fn request() -> InspectRequest {
        InspectRequest {
            context_id: format!("ctx_{}", "1".repeat(64)).try_into().unwrap(),
            snapshot_id: None,
            symbol_path: "Type".into(),
            definition_id: None,
            selection: ResearchSelection::Default,
            max_bytes: None,
            execution: Some(InspectionOptions {
                runtime: Some(RuntimeSelection {
                    module: "example".into(),
                    attributes: vec!["Type".into()],
                }),
                ..Default::default()
            }),
        }
    }
    fn payload(outcome: ExecutionOutcome) -> ExecutionPayload {
        ExecutionPayload::RuntimeObject(RuntimeObject {
            module: "example".into(),
            selection: vec!["Type".into()],
            outcome,
            type_name: (outcome == ExecutionOutcome::Results).then(|| "example.Type".into()),
            signature: None,
            docstring: None,
            attributes: vec![],
            limitations: vec!["bounded observation".into()],
        })
    }

    fn observed(payload: ExecutionPayload, class: EvidenceClass) -> ExecutionObservation {
        let source = Artifact::describe(
            &payload.canonical_bytes().unwrap(),
            ArtifactKind::Other,
            "application/vnd.library-enrichment.canonical-arrow",
            "producer-result://fixture",
            AcquisitionTime::from_micros(1).unwrap(),
        );
        let subject = match &payload {
            ExecutionPayload::SemanticQuery(query) => SubjectRef::Document {
                artifact_id: query.document_artifact_id.clone(),
                heading: "fixture".into(),
            },
            _ => SubjectRef::Symbol {
                symbol_id: format!("symbol_{}", "2".repeat(64)),
            },
        };
        ExecutionObservation::new(
            subject,
            format!("env_{}", "4".repeat(64)).try_into().unwrap(),
            format!("sha256:{}", "5".repeat(64)),
            "6".repeat(64),
            payload,
            FactSource {
                producer_binding_id: format!("producer_{}", "7".repeat(64)),
                extractor: "fixture".into(),
                extractor_version: "1".into(),
                artifact_id: source.artifact_id,
                source_uri: Some(source.source_uri),
                source_version_match: SourceVersionMatch::Exact,
                locator: Locator::Artifact,
                evidence_class: class,
            },
        )
        .unwrap()
    }

    #[tokio::test]
    async fn native_reuse_requires_each_method_and_exact_runtime_selector() -> Result<()> {
        use enrichment_core::evidence::execution::{SemanticMethod, SemanticQuery};
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let mut request = request();
        assert!(!sufficient(&runtime, &request, &[]).await?);
        for outcome in [
            ExecutionOutcome::Results,
            ExecutionOutcome::Empty,
            ExecutionOutcome::Unsupported,
            ExecutionOutcome::Incomplete,
            ExecutionOutcome::Failed,
            ExecutionOutcome::Cancelled,
            ExecutionOutcome::Unresolved,
        ] {
            let fact = observed(payload(outcome), EvidenceClass::RuntimeObserved);
            assert_eq!(
                sufficient(&runtime, &request, std::slice::from_ref(&fact)).await?,
                matches!(
                    outcome,
                    ExecutionOutcome::Results
                        | ExecutionOutcome::Empty
                        | ExecutionOutcome::Unsupported
                )
            );
        }
        request
            .execution
            .as_mut()
            .unwrap()
            .runtime
            .as_mut()
            .unwrap()
            .attributes = vec!["Other".into()];
        assert!(
            !sufficient(
                &runtime,
                &request,
                &[observed(
                    payload(ExecutionOutcome::Empty),
                    EvidenceClass::RuntimeObserved
                )]
            )
            .await?
        );
        request.execution = Some(InspectionOptions::default());
        let mut facts = enrichment_core::native_semantics::DEFAULT_METHODS
            .iter()
            .map(|method| {
                observed(
                    ExecutionPayload::SemanticQuery(SemanticQuery {
                        method: *method,
                        document_artifact_id: format!("art_{}", "8".repeat(64)),
                        position: Some(enrichment_core::evidence::execution::Utf8Position {
                            line: 0,
                            byte: 0,
                        }),
                        anchor_symbol_id: None,
                        server: "fixture".into(),
                        outcome: ExecutionOutcome::Unsupported,
                        hover: None,
                        locations: vec![],
                        diagnostics: vec![],
                        limitations: vec!["unsupported in this producer".into()],
                    }),
                    EvidenceClass::CompilerDerived,
                )
            })
            .collect::<Vec<_>>();
        assert!(sufficient(&runtime, &request, &facts).await?);
        let removed = facts.pop().unwrap();
        assert!(!sufficient(&runtime, &request, &facts).await?);
        request.execution.as_mut().unwrap().methods = vec![SemanticMethod::Hover];
        assert!(sufficient(&runtime, &request, &facts).await?);
        assert!(!sufficient(&runtime, &request, &[removed]).await?);
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn native_retained_choice_preserves_ambiguity_and_exact_pinned_recovery() -> Result<()> {
        use enrichment_core::{
            request::{InspectionIntent, ResearchRequest},
            wire::RecoveryAction,
        };
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let mut request = request();
        request.execution.as_mut().unwrap().intent = InspectionIntent::ExecuteOnMiss;
        let a = RetainedCandidate {
            context_id: format!("ctx_{}", "a".repeat(64)).try_into().unwrap(),
            snapshot_id: format!("snap_{}", "b".repeat(64)).try_into().unwrap(),
        };
        let b = RetainedCandidate {
            context_id: format!("ctx_{}", "c".repeat(64)).try_into().unwrap(),
            snapshot_id: format!("snap_{}", "d".repeat(64)).try_into().unwrap(),
        };
        let none = retained_choice(&runtime, &request, &[]).await?;
        assert!(none.request.is_none() && none.actions.is_empty());
        let one = retained_choice(&runtime, &request, std::slice::from_ref(&a)).await?;
        assert!(one.actions.is_empty());
        let selected = one.request.unwrap();
        assert_eq!(selected.context_id, a.context_id);
        assert_eq!(selected.snapshot_id, Some(a.snapshot_id.clone()));
        assert_eq!(
            selected.execution.as_ref().unwrap().intent,
            InspectionIntent::Retained
        );
        assert_eq!(selected.selection, request.selection);
        let ambiguous = retained_choice(&runtime, &request, &[b.clone(), a.clone()]).await?;
        assert!(ambiguous.request.is_none());
        assert_eq!(ambiguous.actions.len(), 2);
        for (action, candidate) in ambiguous.actions.into_iter().zip([&a, &b]) {
            let RecoveryAction::CallTool { request } = action else {
                panic!("expected call-tool recovery")
            };
            let ResearchRequest::Inspect(request) = *request else {
                panic!("expected inspection")
            };
            assert_eq!(&request.context_id, &candidate.context_id);
            assert_eq!(request.snapshot_id.as_ref(), Some(&candidate.snapshot_id));
            assert_eq!(
                request.execution.unwrap().intent,
                InspectionIntent::Retained
            );
        }
        assert!(
            retained_choice(&runtime, &request, &[a.clone(), a])
                .await
                .is_err()
        );
        assert!(
            retained_choice(&runtime, &request, &vec![b; 65])
                .await
                .is_err()
        );
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn native_inspection_completeness_covers_every_outcome_and_refuses_wrong_scope()
    -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let request = request();
        for (outcome, reason) in [
            (ExecutionOutcome::Results, None),
            (ExecutionOutcome::Empty, None),
            (ExecutionOutcome::Failed, Some(GapReason::ExtractionFailed)),
            (
                ExecutionOutcome::Unresolved,
                Some(GapReason::UpstreamUnavailable),
            ),
            (ExecutionOutcome::Unsupported, Some(GapReason::NotAttempted)),
            (ExecutionOutcome::Incomplete, Some(GapReason::NotAttempted)),
            (ExecutionOutcome::Cancelled, Some(GapReason::NotAttempted)),
        ] {
            let summary = summarize(&runtime, &request, vec![payload(outcome)]).await?;
            assert_eq!(
                summary.state,
                if reason.is_none() {
                    JobState::Succeeded
                } else {
                    JobState::Partial
                }
            );
            assert_eq!(
                summary.run_outcome,
                if reason.is_none() {
                    RunOutcome::Succeeded
                } else {
                    RunOutcome::Partial
                }
            );
            assert_eq!(summary.gaps.first().map(|g| g.reason), reason);
        }
        assert!(summarize(&runtime, &request, vec![]).await.is_err());
        let wrong = InspectRequest {
            execution: None,
            ..request
        };
        assert!(
            summarize(&runtime, &wrong, vec![payload(ExecutionOutcome::Empty)])
                .await
                .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn native_inspection_render_preserves_qualification_and_exact_artifact_closure()
    -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let request = request();
        let payload = payload(ExecutionOutcome::Incomplete);
        let summary = summarize(&runtime, &request, vec![payload.clone()]).await?;
        let artifact = |bytes: &[u8], uri: &str| {
            Artifact::describe(
                bytes,
                ArtifactKind::Other,
                "application/octet-stream",
                uri,
                AcquisitionTime::from_micros(1).unwrap(),
            )
        };
        let result = artifact(
            &payload.canonical_bytes().unwrap(),
            "producer-result://inspection",
        );
        let log = artifact(b"attempt log", "service://attempt");
        let input = artifact(b"input bytes", "source://input");
        let run = ProducerRun {
            attempt_id: enrichment_core::identity::AttemptId::new(),
            producer: "runtime-object".into(),
            producer_version: "fixture".into(),
            config_digest: "a".repeat(64),
            inputs: Default::default(),
            profile: ExecutionProfile::Runtime,
            started_at: ObservationTime::from_micros(1)?,
            finished_at: ObservationTime::from_micros(2)?,
            outcome: summary.run_outcome,
            gaps: summary.gaps,
            log: Some(log.artifact_id.clone()),
        };
        let symbol = SymbolHeader {
            symbol_id: format!("symbol_{}", "2".repeat(64)),
            definition_id: format!("def_{}", "3".repeat(64)),
            path: "example.Type".into(),
            name: "Type".into(),
            kind: SymbolKind::Class,
            parent_path: Some("example".into()),
            is_reexport: false,
            definition_path: "example.Type".into(),
            defined_in_package: "example".into(),
            qualifier: None,
        };
        let snapshot = format!("snap_{}", "9".repeat(64)).try_into().unwrap();
        let mut executed = request.clone();
        executed.execution.as_mut().unwrap().intent =
            enrichment_core::request::InspectionIntent::ExecuteOnMiss;
        executed.execution.as_mut().unwrap().profile = Some(ExecutionProfile::Runtime);
        let admitted = admit_execution(
            &runtime,
            &executed,
            &symbol,
            enrichment_core::identity::Ecosystem::Python,
            &snapshot,
        )
        .await?;
        assert_eq!(admitted.profile, ExecutionProfile::Runtime);
        assert_eq!(admitted.request.snapshot_id, Some(snapshot.clone()));
        assert_eq!(admitted.request.symbol_path, symbol.path);
        assert_eq!(
            admitted.request.definition_id,
            Some(symbol.definition_id.clone())
        );
        assert!(
            admit_execution(
                &runtime,
                &executed,
                &symbol,
                enrichment_core::identity::Ecosystem::Rust,
                &snapshot
            )
            .await
            .is_err()
        );
        executed
            .execution
            .as_mut()
            .unwrap()
            .runtime
            .as_mut()
            .unwrap()
            .module = "other".into();
        assert!(
            admit_execution(
                &runtime,
                &executed,
                &symbol,
                enrichment_core::identity::Ecosystem::Python,
                &snapshot
            )
            .await
            .is_err()
        );
        executed.execution.as_mut().unwrap().runtime = None;
        assert!(
            admit_execution(
                &runtime,
                &executed,
                &symbol,
                enrichment_core::identity::Ecosystem::Python,
                &snapshot
            )
            .await
            .is_err()
        );
        executed.execution.as_mut().unwrap().profile = Some(ExecutionProfile::Build);
        assert_eq!(
            admit_execution(
                &runtime,
                &executed,
                &symbol,
                enrichment_core::identity::Ecosystem::Python,
                &snapshot
            )
            .await?
            .profile,
            ExecutionProfile::Build
        );
        let fact = ExecutionObservation::new(
            SubjectRef::Symbol {
                symbol_id: symbol.symbol_id.clone(),
            },
            format!("env_{}", "4".repeat(64)).try_into().unwrap(),
            format!("sha256:{}", "5".repeat(64)),
            "6".repeat(64),
            payload,
            FactSource {
                producer_binding_id: run.semantic_binding_id(),
                extractor: run.producer.clone(),
                extractor_version: run.producer_version.clone(),
                artifact_id: result.artifact_id.clone(),
                source_uri: Some(result.source_uri.clone()),
                source_version_match: SourceVersionMatch::Exact,
                locator: Locator::Artifact,
                evidence_class: EvidenceClass::RuntimeObserved,
            },
        )
        .unwrap();
        let artifacts = vec![input.clone(), result.clone(), log.clone()];
        let rendered = render(
            &runtime,
            &request,
            symbol.clone(),
            vec![fact.clone()],
            run.clone(),
            artifacts.clone(),
        )
        .await?;
        assert_eq!(rendered.state, JobState::Partial);
        assert_eq!(rendered.data.symbol, Some(symbol.clone()));
        assert_eq!(
            rendered.data.execution_observations,
            std::slice::from_ref(&fact)
        );
        assert_eq!(rendered.data.aspects, ["runtime"]);
        assert_eq!(
            rendered.coverage.missing,
            std::collections::BTreeSet::from(["runtime_api".into()])
        );
        assert_eq!(rendered.coverage.limitations, ["bounded observation"]);
        assert_eq!(rendered.artifacts.len(), 2);
        assert!(
            rendered
                .artifacts
                .iter()
                .all(|handle| handle.receipt.artifact_id != input.artifact_id)
        );
        let mut reversed = artifacts.clone();
        reversed.reverse();
        assert_eq!(
            render(
                &runtime,
                &request,
                symbol.clone(),
                vec![fact.clone()],
                run.clone(),
                reversed
            )
            .await?,
            rendered
        );
        assert!(
            render(
                &runtime,
                &request,
                symbol.clone(),
                vec![fact.clone()],
                run.clone(),
                vec![log]
            )
            .await
            .is_err()
        );
        let mut changed = run.clone();
        changed.outcome = RunOutcome::Succeeded;
        assert!(
            render(
                &runtime,
                &request,
                symbol,
                vec![fact.clone()],
                changed,
                artifacts
            )
            .await
            .is_err()
        );
        let coverage = crate::research_inspection::coverage(
            &runtime,
            Coverage::unassessed("runtime"),
            None,
            false,
            false,
            &[fact],
        )
        .await?;
        assert!(coverage.missing.contains("runtime_api"));
        assert!(
            coverage
                .limitations
                .iter()
                .any(|note| note == "bounded observation")
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
