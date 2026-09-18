//! Shared native execution provenance, scope admission and fact identity construction.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{common::Result, functions::core::expr_ext::FieldAccessor, prelude::*};
use enrichment_core::{
    evidence::{
        Artifact,
        arrow_model::expressions::{literal, record},
        execution::{ExecutionObservation, ExecutionPayload},
        relational::{FactSource, Locator, SubjectRef},
    },
    identity::EnvironmentId,
    native_key::Key,
    native_union::{Cell, NativeStruct, Rule},
    producer::ProducerRun,
    wire::{EvidenceClass, SourceVersionMatch},
};

enrichment_core::native_struct! { pub struct Observed {
    subject:SubjectRef => Rule::Text,
    payload:ExecutionPayload => Rule::Text,
    evidence_class:EvidenceClass => Rule::Text,
    artifact:Artifact => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Producer {
    environment_id:EnvironmentId => Rule::Text,
    image_id:String => Rule::NonEmpty,
    containment_identity:String => Rule::Sha256,
    run:ProducerRun => Rule::Text,
} }

pub(crate) fn content_digest(
    payload: datafusion::logical_expr::Expr,
) -> datafusion::logical_expr::Expr {
    let canonical = enrichment_core::native_identity::canonical_bytes(
        "enrichment/execution-payload/3",
        vec![std::sync::Arc::new(arrow::datatypes::Field::new(
            "value",
            ExecutionPayload::data_type(),
            false,
        ))]
        .into(),
    )
    .call(vec![payload]);
    datafusion::functions::encoding::expr_fn::encode(
        datafusion::functions::crypto::expr_fn::sha256(canonical),
        lit("hex"),
    )
}

/// Physical producers record bounded observations and canonical artifact receipts. This
/// relation supplies qualified provenance and IDs; no handler selects a semantic class or
/// silently accepts a result whose retained bytes describe another observation.
pub async fn lower(
    runtime: &QueryRuntime,
    producer: &Producer,
    observations: &[Observed],
) -> Result<Vec<ExecutionObservation>> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "execution_producer",
        Producer::batch(std::slice::from_ref(producer))?,
    )?;
    native_catalog::input(
        &session,
        "execution_observed",
        Observed::batch(observations)?,
    )?;
    runtime.require_empty(session.sql("SELECT 'execution_observation_bound' AS witness FROM execution_observed HAVING count(*)=0 OR count(*)>64").await?,"execution_fact_bound","execution_lowering").await?;
    let frame=session.sql("SELECT o.*,p.environment_id,p.image_id,p.containment_identity,p.run FROM execution_observed o CROSS JOIN execution_producer p").await?;
    let frame = frame.with_column("expected_digest", content_digest(col("payload")))?;
    native_catalog::work(&session, "execution_values", frame.into_view())?;
    runtime.require_empty(session.sql(r#"
      SELECT 'execution_result_content' AS witness FROM execution_values WHERE artifact.sha256<>expected_digest OR artifact.artifact_id<>concat('art_',expected_digest)
      UNION ALL SELECT 'execution_profile' FROM execution_values WHERE run.profile<>CASE WHEN payload.kind='runtime_object' OR payload.usage_probe.mode='runtime' THEN 'runtime' ELSE 'build' END
    "#).await?,"execution_result_provenance","execution_lowering").await?;
    let frame = session.table("execution_values").await?;
    let binding = crate::producer_run_plan::binding(col("run"))?;
    let source = record(
        &FactSource::data_type(),
        &[
            ("producer_binding_id", binding),
            ("extractor", col("run").field("producer")),
            ("extractor_version", col("run").field("producer_version")),
            ("artifact_id", col("artifact").field("artifact_id")),
            ("source_uri", col("artifact").field("source_uri")),
            ("source_version_match", literal(&SourceVersionMatch::Exact)?),
            ("locator", literal(&Locator::Artifact)?),
            ("evidence_class", col("evidence_class")),
        ],
    )?;
    let frame = frame.select(vec![
        col("subject"),
        col("environment_id"),
        col("image_id"),
        col("containment_identity"),
        col("payload"),
        source.alias("source"),
    ])?;
    let frame = frame.with_column("observation_id", Key::ExecutionObservation.expression())?;
    let frame = crate::native_delta::project(
        frame,
        &arrow::datatypes::Schema::new(ExecutionObservation::fields()),
    )?;
    admit(runtime, &frame).await?;
    runtime
        .records(
            frame.sort(vec![col("observation_id").sort(true, false)])?,
            64,
        )
        .await
}

/// The same complete intrinsic and cross-field admission is used for newly lowered facts
/// and incoming publication relations. Artifact/producer reachability is joined by ingest.
pub(crate) async fn admit(
    runtime: &QueryRuntime,
    frame: &datafusion::dataframe::DataFrame,
) -> Result<()> {
    if let Some(invalid) = enrichment_core::native_schema::intrinsic_violations(
        frame.clone(),
        frame.schema().as_arrow(),
    )? {
        runtime
            .require_empty(
                invalid.select(vec![lit("execution_field").alias("witness")])?,
                "execution_field_contract",
                "execution_admission",
            )
            .await?;
    }
    let session = runtime.session();
    let expected = frame
        .clone()
        .with_column("expected_id", Key::ExecutionObservation.expression())?
        .with_column(
            "expected_artifact",
            datafusion::functions::string::expr_fn::concat(vec![
                lit("art_"),
                content_digest(col("payload")),
            ]),
        )?;
    native_catalog::work(&session, "execution_facts", expected.into_view())?;
    runtime.require_empty(session.sql(r#"
      WITH observed AS (
        SELECT *,payload.semantic_query AS q,payload.runtime_object AS r,payload.usage_probe AS u,
          coalesce(payload.semantic_query.outcome,payload.runtime_object.outcome) AS outcome,
          coalesce(payload.semantic_query.limitations,payload.runtime_object.limitations) AS limitations,
          CASE WHEN payload.kind='semantic_query' THEN payload.semantic_query.hover IS NOT NULL OR cardinality(payload.semantic_query.locations)>0 OR cardinality(payload.semantic_query.diagnostics)>0
            WHEN payload.kind='runtime_object' THEN payload.runtime_object.type_name IS NOT NULL OR payload.runtime_object.signature IS NOT NULL OR payload.runtime_object.docstring IS NOT NULL OR cardinality(payload.runtime_object.attributes)>0 ELSE false END AS has_results
        FROM execution_facts
      )
      SELECT observation_id AS witness FROM observed WHERE observation_id<>expected_id OR source.artifact_id<>expected_artifact
      UNION ALL SELECT observation_id FROM observed WHERE NOT regexp_like(image_id,'^sha256:[a-f0-9]{64}$') OR NOT regexp_like(containment_identity,'^[a-f0-9]{64}$')
      UNION ALL SELECT observation_id FROM observed WHERE NOT coalesce(CASE
        WHEN payload.kind='runtime_object' OR payload.usage_probe.mode='runtime' THEN source.evidence_class='runtime_observed'
        WHEN payload.usage_probe.mode='compile' THEN source.evidence_class='compiler_derived'
        WHEN payload.usage_probe.mode='typecheck' THEN source.evidence_class='typechecker_observed'
        WHEN payload.kind='semantic_query' THEN source.evidence_class IN ('compiler_derived','typechecker_observed') ELSE false END,false)
      UNION ALL SELECT observation_id FROM observed WHERE payload.kind='semantic_query' AND
        ((subject.document.artifact_id IS DISTINCT FROM q.document_artifact_id) OR (q.method<>'diagnostics' AND q.position IS NULL)
          OR (q.method<>'hover' AND q.hover IS NOT NULL) OR (q.method<>'diagnostics' AND cardinality(q.diagnostics)>0)
          OR (q.method IN ('hover','diagnostics') AND cardinality(q.locations)>0) OR cardinality(q.locations)>1024 OR cardinality(q.diagnostics)>1024)
      UNION ALL SELECT observation_id FROM observed WHERE payload.kind='runtime_object' AND
        (subject.kind NOT IN ('symbol','document') OR NOT regexp_like(r.module,'^[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*(\.[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*)*$') OR cardinality(r.selection)>32 OR cardinality(r.attributes)>1024
          OR array_any_match(r.selection,part -> NOT regexp_like(part,'^[\p{Alphabetic}_][\p{Alphabetic}\p{N}_]*$')))
      UNION ALL SELECT observation_id FROM observed WHERE payload.kind='usage_probe' AND
        ((subject.document.artifact_id IS DISTINCT FROM u.snippet_artifact_id) OR (u.end='exited' AND u.exit_code IS NULL))
      UNION ALL SELECT observation_id FROM observed WHERE payload.kind IN ('semantic_query','runtime_object') AND
        ((outcome='results' AND NOT has_results) OR (outcome IN ('empty','unsupported','unresolved','cancelled') AND has_results)
          OR (outcome NOT IN ('results','empty') AND cardinality(limitations)=0) OR cardinality(limitations)>128)
      UNION ALL SELECT observation_id FROM (SELECT observation_id,unnest(payload.semantic_query.diagnostics) AS diagnostic FROM execution_facts) d
        WHERE diagnostic.severity IS NOT NULL AND diagnostic.severity NOT BETWEEN 1 AND 4
    "#).await?,"execution_payload_scope","execution_admission").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{
            ArtifactKind,
            execution::{
                ExecutionOutcome, RuntimeObject, SemanticMethod, SemanticQuery, UsageProbe,
                Utf8Position,
            },
        },
        execution::{ProbeMode, ProcessEnd},
        native_time::{AcquisitionTime, ObservationTime},
        policy::ExecutionProfile,
        producer::RunOutcome,
    };

    fn observation(
        payload: ExecutionPayload,
        subject: SubjectRef,
        evidence_class: EvidenceClass,
    ) -> Observed {
        let artifact = Artifact::describe(
            &payload.canonical_bytes().unwrap(),
            ArtifactKind::Other,
            "application/vnd.library-enrichment.canonical-arrow",
            "producer-result://unit",
            AcquisitionTime::from_micros(1).unwrap(),
        );
        Observed {
            subject,
            payload,
            evidence_class,
            artifact,
        }
    }

    #[tokio::test]
    async fn native_execution_facts_preserve_exact_payloads_and_refuse_false_provenance()
    -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let mut producer = Producer {
            environment_id: format!("env_{}", "1".repeat(64)).try_into().unwrap(),
            image_id: format!("sha256:{}", "2".repeat(64)),
            containment_identity: "3".repeat(64),
            run: ProducerRun {
                attempt_id: enrichment_core::identity::AttemptId::new(),
                producer: "unit".into(),
                producer_version: "1".into(),
                config_digest: "4".repeat(64),
                inputs: Default::default(),
                profile: ExecutionProfile::Runtime,
                started_at: ObservationTime::from_micros(1)?,
                finished_at: ObservationTime::from_micros(2)?,
                outcome: RunOutcome::Succeeded,
                gaps: vec![],
                log: None,
            },
        };
        let document = format!("art_{}", "5".repeat(64));
        let subject = SubjectRef::Document {
            artifact_id: document.clone(),
            heading: "consumer".into(),
        };
        let cases = [
            (
                observation(
                    ExecutionPayload::RuntimeObject(RuntimeObject {
                        module: "library".into(),
                        selection: vec!["Type".into()],
                        outcome: ExecutionOutcome::Results,
                        type_name: Some("library.Type".into()),
                        signature: None,
                        docstring: None,
                        attributes: vec![],
                        limitations: vec![],
                    }),
                    SubjectRef::Symbol {
                        symbol_id: format!("symbol_{}", "6".repeat(64)),
                    },
                    EvidenceClass::RuntimeObserved,
                ),
                ExecutionProfile::Runtime,
            ),
            (
                observation(
                    ExecutionPayload::SemanticQuery(SemanticQuery {
                        method: SemanticMethod::Hover,
                        document_artifact_id: document.clone(),
                        position: Some(Utf8Position { line: 0, byte: 0 }),
                        anchor_symbol_id: None,
                        server: "rust-analyzer".into(),
                        outcome: ExecutionOutcome::Results,
                        hover: Some("fn example()".into()),
                        locations: vec![],
                        diagnostics: vec![],
                        limitations: vec![],
                    }),
                    subject.clone(),
                    EvidenceClass::CompilerDerived,
                ),
                ExecutionProfile::Build,
            ),
            (
                observation(
                    ExecutionPayload::UsageProbe(UsageProbe {
                        mode: ProbeMode::Typecheck,
                        snippet_artifact_id: document,
                        end: ProcessEnd::Exited,
                        exit_code: Some(0),
                        stdout: "".into(),
                        stderr: "".into(),
                    }),
                    subject,
                    EvidenceClass::TypecheckerObserved,
                ),
                ExecutionProfile::Build,
            ),
        ];
        for (observed, profile) in cases {
            producer.run.profile = profile;
            let output = lower(&runtime, &producer, std::slice::from_ref(&observed)).await?;
            assert_eq!(output.len(), 1);
            let fact = &output[0];
            assert_eq!(fact.subject, observed.subject);
            assert_eq!(fact.payload, observed.payload);
            assert_eq!(fact.source.artifact_id, observed.artifact.artifact_id);
            assert_eq!(
                fact.source.producer_binding_id,
                producer.run.semantic_binding_id()
            );
            fact.validate().unwrap();
            let mut changed = observed.clone();
            changed.artifact = Artifact::describe(
                b"different result",
                ArtifactKind::Other,
                "application/octet-stream",
                "source://wrong",
                AcquisitionTime::from_micros(1)?,
            );
            assert!(lower(&runtime, &producer, &[changed]).await.is_err());
            let mut changed = observed;
            changed.evidence_class = EvidenceClass::StaticallyExtracted;
            assert!(lower(&runtime, &producer, &[changed]).await.is_err());
            let mut changed = fact.clone();
            changed.observation_id = "wrong".into();
            let frame = crate::native_catalog::batch(
                &runtime.session(),
                "execution_fact_plan",
                ExecutionObservation::batch(&[changed])?,
            )?;
            assert!(admit(&runtime, &frame).await.is_err());
        }
        assert!(lower(&runtime, &producer, &[]).await.is_err());
        runtime.close_diagnostics().await
    }
}
