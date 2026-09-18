//! Native producer input composition and shared run admission.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::Result, dataframe::DataFrame, functions::core::expr_ext::FieldAccessor, prelude::*,
};
pub use enrichment_core::producer::ProducerAttempt as Attempt;
use enrichment_core::{
    evidence::{
        Artifact, ArtifactKind,
        arrow_model::expressions::{literal, record},
        relational::{FactSource, Locator},
    },
    native_union::{Cell, NativeStruct, Rule},
    producer::ProducerRun,
    wire::{EvidenceClass, SourceVersionMatch},
};
enrichment_core::native_union! { pub enum Role {
    Input = "input",
    Dependency = "dependency",
    Result = "result",
    Named = "named" { name:String => Rule::NonEmpty },
} }
enrichment_core::native_struct! { pub struct ReceiptInput {
    role:Role => Rule::Text,
    artifact:Artifact => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Input {
    role:String => Rule::NonEmpty,
    sha256:String => Rule::Sha256,
} }
enrichment_core::native_struct! { struct Acquisition {
    artifact:Artifact => Rule::Text,
} }

/// Every fact path binds the same declared semantic producer fields. Attempt clocks and
/// acquisition timestamps remain qualification records, never part of the semantic binding.
pub(crate) fn binding(
    run: datafusion::logical_expr::Expr,
) -> Result<datafusion::logical_expr::Expr> {
    let key = enrichment_core::native_key::Key::ProducerBinding;
    key.bind(
        key.schema()
            .fields()
            .iter()
            .map(|field| run.clone().field(field.name()))
            .collect(),
    )
}

/// Select a single qualified source for a declared producer role. Equal bytes from distinct
/// URIs are separate evidence and cannot be resolved by the order of a Rust receipt vector.
pub async fn fact_source(
    runtime: &QueryRuntime,
    run: &ProducerRun,
    role: &str,
    artifacts: &[Artifact],
    source_version_match: SourceVersionMatch,
) -> Result<FactSource> {
    ingress_bound(artifacts.len())?;
    enrichment_core::native_struct! { struct Scope {
        run:ProducerRun => Rule::Text,
        role:String => Rule::NonEmpty,
        source_version_match:SourceVersionMatch => Rule::Text,
    } }
    let session = runtime.session();
    native_catalog::input(
        &session,
        "fact_producer",
        Scope::batch(&[Scope {
            run: run.clone(),
            role: role.into(),
            source_version_match,
        }])?,
    )?;
    native_catalog::input(&session, "fact_acquisitions", Artifact::batch(artifacts)?)?;
    let runs = session.table("fact_producer").await?.select(
        ProducerRun::fields()
            .iter()
            .map(|field| col("run").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    admit(runtime, &runs).await?;
    let selected=session.sql(r#"
      WITH inputs AS (SELECT role,unnest(native_map_entries(run.inputs)) AS input FROM fact_producer)
      SELECT a.*
      FROM inputs i JOIN fact_acquisitions a ON i.input.value=a.sha256 WHERE i.input.key=i.role
    "#).await?;
    qualified(runtime, &session, selected).await?;
    let frame=session.sql("SELECT p.run,p.source_version_match,a.artifact_id,a.source_uri FROM fact_producer p CROSS JOIN qualified_meaning a").await?;
    let source = record(
        &FactSource::data_type(),
        &[
            ("producer_binding_id", binding(col("run"))?),
            ("extractor", col("run").field("producer")),
            ("extractor_version", col("run").field("producer_version")),
            ("artifact_id", col("artifact_id")),
            ("source_uri", col("source_uri")),
            ("source_version_match", col("source_version_match")),
            ("locator", literal(&Locator::Artifact)?),
            (
                "evidence_class",
                literal(&EvidenceClass::StaticallyExtracted)?,
            ),
        ],
    )?;
    let frame = frame.select(vec![source.alias("source")])?.select(
        FactSource::fields()
            .iter()
            .map(|field| col("source").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("producer fact source missing"))
}

/// Physical syntax extraction uses the same qualified receipt consensus as fact provenance.
/// Timestamp ordering chooses a receipt only after every semantic source field agrees.
pub async fn input_artifact(
    runtime: &QueryRuntime,
    artifacts: &[Artifact],
    digest: &str,
    kind: ArtifactKind,
) -> Result<Artifact> {
    ingress_bound(artifacts.len())?;
    let session = runtime.session();
    native_catalog::input(&session, "input_acquisitions", Artifact::batch(artifacts)?)?;
    let candidates = session.table("input_acquisitions").await?.filter(
        col("sha256")
            .eq(lit(digest))
            .and(col("kind").eq(literal(&kind)?)),
    )?;
    qualified(runtime, &session, candidates).await?;
    let frame = session.sql("SELECT * FROM qualified_acquisitions ORDER BY clock_instant(retrieved_at),final_url,etag,last_modified,compression LIMIT 1").await?;
    runtime.records(frame, 1).await?.pop().ok_or_else(|| {
        datafusion::common::exec_datafusion_err!("producer input acquisition missing")
    })
}

async fn qualified(
    runtime: &QueryRuntime,
    session: &SessionContext,
    candidates: DataFrame,
) -> Result<()> {
    native_catalog::work(session, "qualified_acquisitions", candidates.into_view())?;
    let meaning = session.sql("SELECT DISTINCT artifact_id,sha256,size_bytes,media_type,kind,source_uri FROM qualified_acquisitions").await?;
    native_catalog::work(session, "qualified_meaning", meaning.into_view())?;
    runtime
        .require_empty(
            session
                .sql(
                    r#"
      SELECT 'fact_source_ambiguity' AS witness FROM qualified_meaning HAVING count(*)<>1
      UNION ALL SELECT 'fact_source_identity' FROM qualified_meaning
        WHERE artifact_id<>concat('art_',sha256) OR NOT regexp_like(sha256,'^[0-9a-f]{64}$')
    "#,
                )
                .await?,
            "producer_fact_source",
            "native_normalization",
        )
        .await
}

fn ingress_bound(count: usize) -> Result<()> {
    if count > 8192 {
        return datafusion::common::resources_err!("producer acquisition/input count exceeds 8192");
    }
    Ok(())
}

/// Exclude exact operational receipts, never every source that happens to share their bytes.
/// Receipt clocks qualify acquisition records but disappear from semantic input identity.
pub async fn semantic_inputs(
    runtime: &QueryRuntime,
    artifacts: &[Artifact],
    logs: &[Artifact],
) -> Result<Vec<Input>> {
    let session = runtime.session();
    for (name, receipts) in [("producer_acquired", artifacts), ("producer_logs", logs)] {
        ingress_bound(receipts.len())?;
        let frame = crate::native_catalog::batch(
            &session,
            "producer_run_plan",
            Acquisition::batch(
                &receipts
                    .iter()
                    .cloned()
                    .map(|artifact| Acquisition { artifact })
                    .collect::<Vec<_>>(),
            )?,
        )?;
        let key = enrichment_core::native_key::Key::ArtifactReceipt;
        let frame = frame.with_column("receipt_key", key.expression())?;
        native_catalog::work(&session, name, frame.into_view())?;
    }
    let frame=session.sql("SELECT DISTINCT a.artifact.artifact_id AS role,a.artifact.sha256 AS sha256 FROM producer_acquired a LEFT ANTI JOIN producer_logs l ON a.receipt_key=l.receipt_key").await?;
    input_rules(runtime, &frame).await?;
    runtime
        .records(frame.sort(vec![col("role").sort(true, false)])?, 8192)
        .await
}

/// Content is one semantic input per role, while qualified acquisitions remain separate
/// retained receipts. A duplicate role with different bytes refuses; no last-winner map.
pub async fn receipt_inputs(
    runtime: &QueryRuntime,
    receipts: &[ReceiptInput],
) -> Result<Vec<Input>> {
    ingress_bound(receipts.len())?;
    let session = runtime.session();
    native_catalog::input(
        &session,
        "producer_receipts",
        ReceiptInput::batch(receipts)?,
    )?;
    runtime.require_empty(session.sql("SELECT 'producer_receipt_identity' AS witness FROM producer_receipts WHERE artifact.artifact_id<>concat('art_',artifact.sha256)").await?,"producer_receipt_identity","producer_composition").await?;
    let inputs = session
        .sql(
            r#"
      SELECT DISTINCT CASE role.kind
        WHEN 'input' THEN concat('input:',artifact.artifact_id)
        WHEN 'result' THEN concat('result:',artifact.artifact_id)
        WHEN 'dependency' THEN concat('dependency:',artifact.sha256)
        WHEN 'named' THEN role.named.name END AS role, artifact.sha256 AS sha256
      FROM producer_receipts
    "#,
        )
        .await?;
    input_rules(runtime, &inputs).await?;
    runtime
        .records(inputs.sort(vec![col("role").sort(true, false)])?, 8192)
        .await
}

/// The physical boundary supplies observations and named inputs. Native aggregation chooses
/// the unique, ordered input map and admits the whole run before it can supply fact identity.
pub async fn compose(
    runtime: &QueryRuntime,
    attempt: Attempt,
    inputs: &[Input],
) -> Result<ProducerRun> {
    ingress_bound(inputs.len())?;
    let session = runtime.session();
    let inputs =
        crate::native_catalog::batch(&session, "producer_run_plan", Input::batch(inputs)?)?
            .distinct()?;
    input_rules(runtime, &inputs).await?;
    native_catalog::work(&session, "producer_inputs", inputs.into_view())?;
    native_catalog::input(&session, "producer_attempt", Attempt::batch(&[attempt])?)?;
    // Aggregate entries once so key/value pairing survives every partition and ordering.
    let frame = session
        .sql(
            r#"
      WITH entries AS (
        SELECT coalesce(array_agg(named_struct('key',role,'value',sha256) ORDER BY role),
          CAST([] AS STRUCT(key VARCHAR,value VARCHAR)[])) AS entries FROM producer_inputs
      )
      SELECT a.*,map(array_transform(e.entries, x -> get_field(x,'key')),
        array_transform(e.entries, x -> get_field(x,'value'))) AS inputs
      FROM producer_attempt a CROSS JOIN entries e
    "#,
        )
        .await?;
    let frame =
        crate::native_delta::project(frame, &arrow::datatypes::Schema::new(ProducerRun::fields()))?;
    admit(runtime, &frame).await?;
    runtime.records(frame, 1).await?.pop().ok_or_else(|| {
        datafusion::common::DataFusionError::Execution("producer run projection missing".into())
    })
}

async fn input_rules(runtime: &QueryRuntime, inputs: &DataFrame) -> Result<()> {
    let session = runtime.session();
    native_catalog::work(
        &session,
        "producer_input_admission",
        inputs.clone().into_view(),
    )?;
    runtime.require_empty(session.sql(r#"
      SELECT role AS witness FROM producer_input_admission
      WHERE role IS NULL OR role='' OR octet_length(role)>4096
        OR sha256 IS NULL OR NOT regexp_like(sha256,'^[0-9a-f]{64}$')
      UNION ALL SELECT role FROM producer_input_admission GROUP BY role HAVING count(DISTINCT sha256)<>1
      UNION ALL SELECT 'producer_input_count' FROM producer_input_admission HAVING count(*)>8192
    "#).await?, "producer_input_contract", "producer_composition").await
}

/// Shared by composition, corpus ingestion and durable attempts. Encoders are mechanical;
/// all paths apply these clock, profile, input and shape rules to the same native fields.
pub(crate) async fn admit(runtime: &QueryRuntime, runs: &DataFrame) -> Result<()> {
    if let Some(invalid) = enrichment_core::native_schema::intrinsic_violations(
        runs.clone(),
        runs.schema().as_arrow(),
    )? {
        runtime
            .require_empty(
                invalid.select(vec![lit("producer_field").alias("witness")])?,
                "producer_field_contract",
                "producer_admission",
            )
            .await?;
    }
    let session = runtime.session();
    native_catalog::work(&session, "producer_run_admission", runs.clone().into_view())?;
    runtime
        .require_empty(
            value_rules(&session, "producer_run_admission", "attempt_id").await?,
            "producer_run_contract",
            "producer_admission",
        )
        .await?;
    runtime.require_empty(session.sql("SELECT 'producer_attempt_duplicate' AS witness FROM producer_run_admission GROUP BY attempt_id HAVING count(*)<>1 UNION ALL SELECT 'producer_count' FROM producer_run_admission HAVING count(*)>1024").await?,
        "producer_attempt_keys","producer_admission").await
}

/// Durable catalog admission uses the same rules with its association key. One actual
/// attempt can occur in several snapshots; uniqueness belongs to the containing relation.
pub(crate) async fn value_rules(
    session: &SessionContext,
    table: &str,
    key: &str,
) -> Result<DataFrame> {
    // Identifiers come from compiled native relation contracts, never caller SQL.
    session
        .sql(&format!(
            r#"
      SELECT {key} AS witness FROM {table}
      WHERE finished_at<started_at OR cardinality(inputs)>8192
        OR array_any_match(native_map_entries(inputs), x -> get_field(x,'key')=''
          OR octet_length(get_field(x,'key'))>4096
          OR NOT regexp_like(get_field(x,'value'),'^[0-9a-f]{{64}}$'))
        OR (log IS NOT NULL AND NOT regexp_like(log,'^art_[0-9a-f]{{64}}$'))
        OR (profile<>'static' AND log IS NULL)
    "#
        ))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::ArtifactKind,
        native_time::{AcquisitionTime, ObservationTime},
        policy::ExecutionProfile,
        producer::RunOutcome,
    };

    #[tokio::test]
    async fn native_fact_source_requires_one_qualified_declared_input() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let artifact = Artifact::describe(
            b"facts",
            ArtifactKind::Other,
            "application/octet-stream",
            "source://facts",
            AcquisitionTime::from_micros(1)?,
        );
        let attempt = Attempt {
            attempt_id: enrichment_core::identity::AttemptId::new(),
            producer: "fixture".into(),
            producer_version: "1".into(),
            config_digest: "a".repeat(64),
            profile: ExecutionProfile::Static,
            started_at: ObservationTime::from_micros(1)?,
            finished_at: ObservationTime::from_micros(2)?,
            outcome: RunOutcome::Succeeded,
            gaps: vec![],
            log: None,
        };
        let run = compose(
            &runtime,
            attempt,
            &[Input {
                role: "facts".into(),
                sha256: artifact.sha256.clone(),
            }],
        )
        .await?;
        let mut later = artifact.clone();
        later.retrieved_at = AcquisitionTime::from_micros(10)?;
        assert_eq!(
            input_artifact(
                &runtime,
                &[later.clone(), artifact.clone()],
                &artifact.sha256,
                artifact.kind
            )
            .await?,
            artifact
        );
        assert!(
            input_artifact(
                &runtime,
                std::slice::from_ref(&artifact),
                &artifact.sha256,
                ArtifactKind::RustdocJson
            )
            .await
            .is_err()
        );
        let selected = fact_source(
            &runtime,
            &run,
            "facts",
            &[artifact.clone(), later.clone()],
            SourceVersionMatch::Mismatched,
        )
        .await?;
        assert_eq!(selected.producer_binding_id, run.semantic_binding_id());
        assert_eq!(selected.artifact_id, artifact.artifact_id);
        assert_eq!(selected.source_uri.as_deref(), Some("source://facts"));
        assert_eq!(
            selected.source_version_match,
            SourceVersionMatch::Mismatched
        );
        assert_eq!(selected.evidence_class, EvidenceClass::StaticallyExtracted);
        assert!(
            fact_source(
                &runtime,
                &run,
                "missing",
                std::slice::from_ref(&artifact),
                SourceVersionMatch::Exact
            )
            .await
            .is_err()
        );
        assert!(
            fact_source(&runtime, &run, "facts", &[], SourceVersionMatch::Exact)
                .await
                .is_err()
        );
        later.source_uri = "source://different".into();
        assert!(
            input_artifact(
                &runtime,
                &[artifact.clone(), later.clone()],
                &artifact.sha256,
                artifact.kind
            )
            .await
            .is_err()
        );
        assert!(
            fact_source(
                &runtime,
                &run,
                "facts",
                &[artifact.clone(), later],
                SourceVersionMatch::Exact
            )
            .await
            .is_err()
        );
        let mut damaged = artifact.clone();
        damaged.size_bytes += 1;
        assert!(
            fact_source(
                &runtime,
                &run,
                "facts",
                &[artifact.clone(), damaged],
                SourceVersionMatch::Exact
            )
            .await
            .is_err()
        );
        let mut damaged = artifact;
        damaged.artifact_id = format!("art_{}", "0".repeat(64));
        assert!(
            fact_source(
                &runtime,
                &run,
                "facts",
                &[damaged],
                SourceVersionMatch::Exact
            )
            .await
            .is_err()
        );
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn native_producer_runs_preserve_acquisitions_and_refuse_input_conflicts() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let artifact = |bytes: &[u8], uri| {
            Artifact::describe(
                bytes,
                ArtifactKind::Other,
                "application/octet-stream",
                uri,
                AcquisitionTime::from_micros(1).unwrap(),
            )
        };
        let first = artifact(b"input", "source://one");
        let second = artifact(b"input", "source://two");
        let log = artifact(b"log", "attempt://one");
        let other_log = artifact(b"log", "attempt://two");
        let source_with_log_bytes = artifact(b"log", "source://log-format-guide");
        let selected = semantic_inputs(
            &runtime,
            &[
                first.clone(),
                second.clone(),
                log.clone(),
                source_with_log_bytes.clone(),
            ],
            std::slice::from_ref(&log),
        )
        .await?;
        assert_eq!(selected.len(), 2);
        assert!(
            selected
                .iter()
                .any(|row| row.sha256 == source_with_log_bytes.sha256)
        );
        assert!(
            semantic_inputs(
                &runtime,
                std::slice::from_ref(&log),
                std::slice::from_ref(&log)
            )
            .await?
            .is_empty()
        );
        let named = |artifact| ReceiptInput {
            role: Role::Named {
                name: "source".into(),
            },
            artifact,
        };
        let inputs =
            receipt_inputs(&runtime, &[named(first.clone()), named(second.clone())]).await?;
        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].role, "source");
        let attempt = Attempt {
            attempt_id: enrichment_core::identity::AttemptId::new(),
            producer: "fixture".into(),
            producer_version: "1".into(),
            config_digest: "a".repeat(64),
            profile: ExecutionProfile::Build,
            started_at: ObservationTime::from_micros(1).unwrap(),
            finished_at: ObservationTime::from_micros(2).unwrap(),
            outcome: RunOutcome::Succeeded,
            gaps: vec![],
            log: Some(log.artifact_id.clone()),
        };
        let run = compose(&runtime, attempt.clone(), &inputs).await?;
        assert_eq!(run.inputs, [("source".into(), first.sha256.clone())].into());
        let attempts = crate::attempt_plan::AttemptPlan::input(
            &runtime,
            vec![(
                run.clone(),
                vec![first.clone(), second.clone(), log.clone(), other_log],
            )],
        )?;
        attempts.validate(&runtime).await?;
        assert_eq!(attempts.logs(&runtime).await?.len(), 2);
        // Acquisitions are qualified records, even though both pairs share physical bytes.
        assert_eq!(
            runtime
                .records::<ReceiptOnly>(attempts.receipts(&runtime).await?, 4)
                .await?
                .len(),
            4
        );
        assert!(
            receipt_inputs(
                &runtime,
                &[
                    named(first.clone()),
                    named(artifact(b"different", "source://one"))
                ]
            )
            .await
            .is_err()
        );
        let mut backwards = attempt.clone();
        backwards.finished_at = ObservationTime::from_micros(0).unwrap();
        assert!(compose(&runtime, backwards, &inputs).await.is_err());
        let mut unlogged = attempt.clone();
        unlogged.log = None;
        assert!(compose(&runtime, unlogged, &inputs).await.is_err());
        let mut empty = attempt.clone();
        empty.profile = ExecutionProfile::Static;
        empty.log = None;
        assert!(compose(&runtime, empty, &[]).await?.inputs.is_empty());
        let mut repeated = attempt;
        repeated.attempt_id = enrichment_core::identity::AttemptId::new();
        repeated.started_at = ObservationTime::from_micros(4).unwrap();
        repeated.finished_at = ObservationTime::from_micros(5).unwrap();
        assert_eq!(
            compose(&runtime, repeated, &inputs)
                .await?
                .semantic_binding_id(),
            run.semantic_binding_id()
        );
        for receipts in [
            vec![log.clone()],
            vec![first.clone()],
            vec![first, log, artifact(b"undeclared", "source://extra")],
        ] {
            assert!(
                crate::attempt_plan::AttemptPlan::input(&runtime, vec![(run.clone(), receipts)])?
                    .validate(&runtime)
                    .await
                    .is_err()
            );
        }
        runtime.close_diagnostics().await
    }
    enrichment_core::native_struct! { struct ReceiptOnly {
        artifact:Artifact => Rule::Text,
    } }
}
