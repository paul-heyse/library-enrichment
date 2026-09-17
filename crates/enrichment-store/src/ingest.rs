//! Bounded document decoding to Arrow; native provenance, qualification and evidence plans.
use crate::{
    admission::Relation,
    dataset::WriteLimits,
    projection,
    record_writer::RelationBuffer,
    repository::{Attempts, EvidencePlans},
    runtime::QueryRuntime,
};
use arrow::{
    array::{ArrayRef, BooleanArray},
    record_batch::RecordBatch,
};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions::core::expr_fn::{coalesce, get_field},
    logical_expr::when,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    evidence::{
        Artifact,
        arrow_model::{
            cells,
            expressions::{record, variant},
        },
        document,
        ingest::{DocumentSource, IngestBudget, IngestContext},
        metadata::ReleaseMetadata,
    },
    native_key::Key,
};
use std::{path::Path, sync::Arc};

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
fn keyed(frame: DataFrame, key: Key, id: &str, relation: Relation) -> Result<DataFrame> {
    let frame = crate::native_delta::project(frame, key.schema().as_ref())?;
    crate::native_delta::project(
        frame.with_column(id, key.expression())?,
        relation.schema()?.as_ref(),
    )
}
async fn view(session: &SessionContext, name: &str, sql: &str) -> Result<()> {
    session.register_table(name, session.sql(sql).await?.into_view())?;
    Ok(())
}
fn field(relation: Relation, name: &str) -> Result<arrow::datatypes::DataType> {
    Ok(relation
        .schema()?
        .field_with_name(name)?
        .data_type()
        .clone())
}
/// Format decoding owns only a bounded fact file. Returned native scans retain that directory.
/// Attempts are decoded only at the bounded control-publication boundary, never re-encoded as
/// evidence input relations. Document/coverage/input IDs are computed in the plans below.
pub async fn prepare(
    context: IngestContext,
    input: impl DocumentSource + 'static,
    metadata: Option<ReleaseMetadata>,
    root: &Path,
    runtime: &QueryRuntime,
    limits: &WriteLimits,
) -> Result<(EvidencePlans, Attempts)> {
    if context.producer_runs.len() > 1024 || context.artifacts.len() > 8192 {
        return Err(invalid("producer provenance count exceeds bound"));
    }
    enrichment_core::canonical::serialized_size(&context.producer_runs, 16 * 1024 * 1024)?;
    enrichment_core::canonical::serialized_size(&context.artifacts, 64 * 1024 * 1024)?;
    std::fs::create_dir_all(root)?;
    let directory = tempfile::Builder::new()
        .prefix("document-facts-")
        .tempdir_in(root)?;
    let limits_owned = limits.clone();
    let operation = crate::runtime::capture_operation();
    let (directory, path) = tokio::task::spawn_blocking(move || {
        operation.run(|| {
            let mut buffer = RelationBuffer::staging(
                directory.path(),
                "documents",
                &limits_owned,
                document::encode,
            )?;
            let mut budget = IngestBudget::default();
            let mut ordinal = 0_u64;
            input
                .decode(&mut |fact| {
                    budget.record(&fact)?;
                    buffer
                        .push(document::Input { ordinal, fact })
                        .map_err(|e| e.to_string())?;
                    ordinal += 1;
                    Ok(())
                })
                .map_err(invalid)?;
            let (path, _, _, _) = buffer.finish_staging()?;
            std::fs::File::open(directory.path())?.sync_all()?;
            Ok::<_, DataFusionError>((Arc::new(directory), path))
        })
    })
    .await
    .map_err(|e| DataFusionError::External(Box::new(e)))??;
    let session = runtime.session();
    let provider =
        crate::arrow_input::provider(runtime, &path, document::encode(&[])?.schema()).await?;
    let provider = session.read_table(provider)?.into_view();
    session.register_table(
        "documents",
        crate::leases::staged_view(&provider, &session, directory)?,
    )?;
    let producers = session
        .read_batch(projection::provenance::producer_fields(
            &context.producer_runs,
        )?)?
        .with_column("producer_binding_id", Key::ProducerBinding.expression())?;
    let producers =
        crate::native_delta::project(producers, Relation::ProducerRuns.schema()?.as_ref())?;
    session.register_table("producers", producers.clone().into_view())?;
    runtime.require_empty(session.sql("SELECT attempt_id AS witness_id FROM producers GROUP BY attempt_id HAVING count(*)<>1").await?, "producer_attempt_key", "document_ingress").await?;
    session.register_table(
        "producing",
        producers
            .clone()
            .filter(col("attempt_id").eq(lit(&context.producing_attempt)))?
            .into_view(),
    )?;
    runtime
        .require_empty(
            session
                .sql("SELECT 'producing' AS witness_id FROM producing HAVING count(*)<>1")
                .await?,
            "producing_attempt",
            "document_ingress",
        )
        .await?;
    session.register_table(
        "acquisitions",
        session
            .read_batch(projection::staging::artifacts(&context.artifacts)?)?
            .distinct()?
            .into_view(),
    )?;
    view(
        &session,
        "declarations",
        "SELECT attempt_id, producer_binding_id, unnest(native_map_entries(inputs)) AS input FROM producers",
    )
    .await?;
    runtime.require_empty(session.sql("SELECT d.attempt_id AS witness_id FROM declarations d LEFT ANTI JOIN acquisitions a ON d.input.value=a.artifact.sha256").await?, "declared_input_acquisition", "document_ingress").await?;
    view(&session, "bound_inputs", "SELECT DISTINCT d.attempt_id,d.producer_binding_id,d.input.key AS role,a.artifact FROM declarations d JOIN acquisitions a ON d.input.value=a.artifact.sha256").await?;
    let inputs = keyed(session.sql("SELECT producer_binding_id,role,artifact.artifact_id AS artifact_id,artifact.sha256 AS sha256,artifact.media_type AS media_type,artifact.kind AS kind,artifact.size_bytes AS size_bytes,artifact.source_uri AS source_uri FROM bound_inputs").await?, Key::InputArtifact, "input_id", Relation::InputArtifacts)?;
    view(&session, "document_candidates", r#"SELECT d.*, i.producer_binding_id, i.artifact.sha256 AS digest, i.artifact.source_uri AS qualified_uri,
        CASE WHEN d.source_uri IS NULL AND coalesce(d.locator.lines.file, d.locator.archive_member.path, d.locator.python_declaration.file, d.locator.manifest_key.file, d.locator.markdown_section.file, d.locator.source_start.file) IS NOT NULL AND ends_with(i.artifact.source_uri, concat('#', coalesce(d.locator.lines.file, d.locator.archive_member.path, d.locator.python_declaration.file, d.locator.manifest_key.file, d.locator.markdown_section.file, d.locator.source_start.file))) THEN 1 ELSE 0 END AS member_match
        FROM documents d JOIN bound_inputs i ON d.artifact_id=i.artifact.artifact_id
        JOIN producing p ON i.attempt_id=p.attempt_id
        WHERE d.source_uri IS NULL OR d.source_uri=i.artifact.source_uri"#).await?;
    view(&session, "document_sources", "SELECT * FROM (SELECT *, max(member_match) OVER (PARTITION BY ordinal) AS best FROM document_candidates) WHERE member_match=best").await?;
    runtime.require_empty(session.sql("SELECT CAST(d.ordinal AS VARCHAR) AS witness_id FROM documents d LEFT ANTI JOIN document_sources s ON d.ordinal=s.ordinal").await?, "fragment_input_closure", "document_ingress").await?;
    runtime.require_empty(session.sql("SELECT CAST(ordinal AS VARCHAR) AS witness_id FROM document_sources GROUP BY ordinal HAVING count(DISTINCT qualified_uri)<>1 OR count(DISTINCT digest)<>1").await?, "fragment_source_ambiguity", "document_ingress").await?;
    let source_type = field(Relation::Fragments, "source")?;
    let source_version = match context.source_version_match {
        enrichment_core::wire::SourceVersionMatch::Exact => "exact",
        enrichment_core::wire::SourceVersionMatch::CompatibleClaimed => "compatible_claimed",
        enrichment_core::wire::SourceVersionMatch::Mismatched => "mismatched",
        enrichment_core::wire::SourceVersionMatch::Unknown => "unknown",
    };
    let source = record(
        &source_type,
        &[
            ("producer_binding_id", col("producer_binding_id")),
            ("extractor", col("producer")),
            ("extractor_version", col("producer_version")),
            ("artifact_id", col("artifact_id")),
            ("source_uri", col("qualified_uri")),
            (
                "source_version_match",
                coalesce(vec![col("source_version_match"), lit(source_version)]),
            ),
            ("evidence_class", col("evidence_class")),
            ("locator", col("locator")),
        ],
    )?;
    let subject_type = field(Relation::Fragments, "subject")?;
    let subject = when(
        col("kind").eq(lit("feature_definition")),
        variant(&subject_type, "feature", &[("name", col("label"))])?,
    )
    .when(
        col("kind").eq(lit("example")),
        variant(
            &subject_type,
            "example",
            &[
                ("artifact_id", col("artifact_id")),
                (
                    "path",
                    get_field(get_field(col("locator"), "archive_member"), "path"),
                ),
            ],
        )?,
    )
    .otherwise(variant(
        &subject_type,
        "document",
        &[
            ("artifact_id", col("artifact_id")),
            ("heading", col("label")),
        ],
    )?)?;
    let fragments = keyed(
        session
            .table("document_sources")
            .await?
            .select(vec![
                col("kind"),
                subject.alias("subject"),
                col("label").alias("display_subject"),
                col("text"),
                source.alias("source"),
            ])?
            .distinct()?,
        Key::TextFragment,
        "fragment_id",
        Relation::Fragments,
    )?;
    let coverage = coverage(&session, runtime, &context).await?;
    let mut plans = EvidencePlans::new();
    for relation in Relation::ALL {
        plans.insert(
            relation,
            session.read_batch(RecordBatch::new_empty(relation.schema()?))?,
        );
    }
    plans.insert(Relation::Fragments, fragments);
    plans.insert(Relation::InputArtifacts, inputs);
    plans.insert(Relation::ProducerRuns, producers);
    plans.insert(Relation::Coverage, coverage);
    if let Some(metadata) = metadata {
        let frame = session.read_batch(projection::metadata::fields(&[metadata])?)?;
        plans.insert(
            Relation::ReleaseMetadata,
            keyed(
                frame,
                Key::ReleaseMetadata,
                "metadata_id",
                Relation::ReleaseMetadata,
            )?,
        );
    }
    // A single bounded mechanical decode remains for the control envelope; evidence plans above
    // stream directly to Delta and do not read their rows back into domain objects.
    let acquisitions = session.sql("SELECT DISTINCT p.attempt_id,a.artifact FROM producers p JOIN acquisitions a ON p.log=a.artifact.artifact_id UNION SELECT DISTINCT attempt_id,artifact FROM bound_inputs").await?;
    let mut attempts: Attempts = context
        .producer_runs
        .into_iter()
        .map(|run| (run, Vec::<Artifact>::new()))
        .collect();
    let mut bytes = 0_usize;
    runtime
        .visit(acquisitions, limits.table_rows, |batch| {
            for (attempt, artifact) in projection::staging::attempt_artifacts(batch)? {
                bytes = bytes
                    .checked_add(enrichment_core::canonical::serialized_size(
                        &artifact,
                        limits.record_bytes,
                    )?)
                    .filter(|n| *n <= 64 * 1024 * 1024)
                    .ok_or_else(|| invalid("attempt acquisition byte budget exceeded"))?;
                attempts
                    .iter_mut()
                    .find(|(r, _)| r.attempt_id == attempt)
                    .ok_or_else(|| invalid("unknown attempt"))?
                    .1
                    .push(artifact);
            }
            Ok(())
        })
        .await?;
    Ok((plans, attempts))
}
/// Native execution contribution: the producer supplies bounded observations, exact input
/// acquisitions and its actual run. Joins enforce the closure; expressions derive coverage.
pub async fn execution(
    repository: &crate::repository::EvidenceRepository,
    metadata: &enrichment_core::evidence::snapshot::SnapshotMetadata,
    observations: Vec<enrichment_core::evidence::execution::ExecutionObservation>,
    run: enrichment_core::producer::ProducerRun,
    artifacts: Vec<Artifact>,
) -> Result<(EvidencePlans, Attempts)> {
    use datafusion::functions::core::expr_ext::FieldAccessor;
    use enrichment_core::evidence::ingest::{DocumentBatch, IngestContext};
    if observations.is_empty() || observations.len() > 64 {
        return Err(invalid("execution result count exceeds its bound"));
    }
    enrichment_core::canonical::serialized_size(&observations, 16 * 1024 * 1024)?;
    let context = IngestContext {
        ecosystem: metadata.release.key.ecosystem,
        symbol_package: metadata.symbol_package.clone(),
        release_id: metadata.release.release_id.to_string(),
        environment_id: metadata.environment.environment_id.to_string(),
        source_version_match: enrichment_core::wire::SourceVersionMatch::Exact,
        producing_attempt: run.attempt_id.clone(),
        producer_runs: vec![run],
        artifacts: artifacts.clone(),
        indexed: vec![],
        missing: vec![],
        gaps: vec![],
    };
    let (mut plans, attempts) = repository
        .prepare_documents(context, DocumentBatch::default(), None)
        .await?;
    let runtime = &repository.runtime;
    let session = runtime.session();
    for (name, relation) in [
        ("producers", Relation::ProducerRuns),
        ("inputs", Relation::InputArtifacts),
    ] {
        session.register_table(name, plans[&relation].clone().into_view())?;
    }
    session.register_table(
        "observations",
        session
            .read_batch(projection::execution::encode(&observations)?)?
            .into_view(),
    )?;
    session.register_table(
        "acquisitions",
        session
            .read_batch(projection::staging::artifacts(&artifacts)?)?
            .distinct()?
            .into_view(),
    )?;
    for (rule, sql) in [
        (
            "execution_result_key",
            "SELECT observation_id AS witness_id FROM observations GROUP BY observation_id HAVING count(*)<>1",
        ),
        (
            "execution_producing_run",
            "SELECT observation_id AS witness_id FROM observations o LEFT ANTI JOIN producers p ON o.source.producer_binding_id=p.producer_binding_id",
        ),
        (
            "execution_input_acquisition",
            "SELECT sha256 AS witness_id FROM inputs GROUP BY role,sha256 HAVING count(*)<>1",
        ),
        (
            "execution_exact_log",
            "SELECT p.attempt_id AS witness_id FROM producers p LEFT JOIN acquisitions a ON p.log=a.artifact.artifact_id GROUP BY p.attempt_id HAVING count(a.artifact)<>1",
        ),
        (
            "execution_acquisition_closure",
            "SELECT a.artifact.artifact_id AS witness_id FROM acquisitions a LEFT ANTI JOIN (SELECT artifact_id FROM inputs UNION SELECT log AS artifact_id FROM producers) i ON a.artifact.artifact_id=i.artifact_id",
        ),
    ] {
        runtime
            .require_empty(session.sql(sql).await?, rule, "execution_ingress")
            .await?;
    }
    let frame = session.sql("SELECT o.subject,o.payload.kind AS operation,p.producer_binding_id,p.gaps FROM observations o JOIN producers p ON o.source.producer_binding_id=p.producer_binding_id").await?;
    let kind = when(
        col("operation").eq(lit("semantic_query")),
        lit("semantic_queries"),
    )
    .when(
        col("operation").eq(lit("runtime_object")),
        lit("runtime_api"),
    )
    .otherwise(lit("usage_probes"))?;
    let coverage = keyed(
        frame
            .select(vec![
                col("producer_binding_id"),
                col("subject"),
                kind.alias("kind"),
                when(
                    datafusion::functions_nested::expr_fn::array_empty(col("gaps")),
                    lit("indexed"),
                )
                .otherwise(lit("partial"))?
                .alias("outcome"),
                col("gaps"),
            ])?
            .distinct()?,
        Key::Coverage,
        "coverage_id",
        Relation::Coverage,
    )?;
    // Payload/source IDs are verified by native admission before publication.
    let observations = session.table("observations").await?;
    runtime
        .require_empty(
            observations
                .clone()
                .filter(
                    col("environment_id").not_eq(lit(metadata.environment.environment_id.as_str())),
                )?
                .select(vec![col("source").field("artifact_id").alias("witness_id")])?,
            "execution_environment",
            "execution_ingress",
        )
        .await?;
    plans.insert(Relation::ExecutionObservations, observations);
    plans.insert(Relation::Coverage, coverage);
    Ok((plans, attempts))
}

/// The selected terminal result must name exactly one observation of the declared operation
/// from the actual producing attempt. This runs against the incoming native contribution.
pub(crate) async fn completion(
    runtime: &QueryRuntime,
    plans: &EvidencePlans,
    completion: &crate::repository::JobCompletion,
) -> Result<()> {
    use enrichment_core::evidence::catalog::PublishedJobKind;
    if completion.kind == PublishedJobKind::Resolve {
        return Ok(());
    }
    if completion.result_artifact_ids.is_empty() || completion.result_artifact_ids.len() > 64 {
        return Err(invalid("execution completion count exceeds bound"));
    }
    let session = runtime.session();
    for (name, relation) in [
        ("producers", Relation::ProducerRuns),
        ("observations", Relation::ExecutionObservations),
    ] {
        session.register_table(
            name,
            plans
                .get(&relation)
                .ok_or_else(|| invalid("incomplete execution contribution"))?
                .clone()
                .into_view(),
        )?;
    }
    session.register_table(
        "results",
        session
            .read_batch(cells::batch(
                "terminal_results",
                vec![cells::column(
                    "artifact_id",
                    cells::text(completion.result_artifact_ids.iter().map(String::as_str)),
                    false,
                    "artifact-id",
                )],
            )?)?
            .into_view(),
    )?;
    session.register_table(
        "producing",
        session
            .table("producers")
            .await?
            .filter(col("attempt_id").eq(lit(&completion.attempt_id)))?
            .into_view(),
    )?;
    runtime.require_empty(session.sql("SELECT artifact_id AS witness_id FROM results GROUP BY artifact_id HAVING count(*)<>1").await?, "terminal_result_key", "publication").await?;
    let violations = session.sql("SELECT r.artifact_id AS witness_id FROM results r LEFT JOIN observations o ON r.artifact_id=o.source.artifact_id LEFT JOIN producing p ON p.producer_binding_id=o.source.producer_binding_id GROUP BY r.artifact_id HAVING count(o.observation_id)<>1 OR count(p.attempt_id)<>1").await?;
    runtime
        .require_empty(violations, "terminal_producing_observation", "publication")
        .await?;
    let kind = if completion.kind == PublishedJobKind::Verify {
        "<>"
    } else {
        "="
    };
    runtime.require_empty(session.sql(&format!("SELECT o.observation_id AS witness_id FROM observations o JOIN results r ON o.source.artifact_id=r.artifact_id WHERE o.payload.kind {kind} 'usage_probe'")).await?,"terminal_operation_kind","publication").await
}

async fn coverage(
    session: &SessionContext,
    runtime: &QueryRuntime,
    context: &IngestContext,
) -> Result<DataFrame> {
    let kinds = context
        .indexed
        .iter()
        .map(|k| (k, true))
        .chain(context.missing.iter().map(|k| (k, false)))
        .collect::<Vec<_>>();
    session.register_table(
        "coverage_declared",
        session
            .read_batch(cells::batch(
                "coverage_declared",
                vec![
                    cells::column(
                        "kind",
                        cells::text(kinds.iter().map(|(k, _)| k.as_str())),
                        false,
                        "vocabulary:evidence-kind/1",
                    ),
                    cells::column(
                        "available",
                        Arc::new(BooleanArray::from_iter(kinds.iter().map(|(_, v)| Some(*v))))
                            as ArrayRef,
                        false,
                        "producer-declaration",
                    ),
                ],
            )?)?
            .distinct()?
            .into_view(),
    )?;
    runtime.require_empty(session.sql("SELECT kind AS witness_id FROM coverage_declared GROUP BY kind HAVING count(*)<>1").await?,"coverage_declaration_conflict","document_ingress").await?;
    let gaps = projection::provenance::gaps(&[&context.gaps])?;
    session.register_table(
        "coverage_gap_lists",
        session
            .read_batch(RecordBatch::try_from_iter([("gap", gaps)])?)?
            .into_view(),
    )?;
    view(
        session,
        "coverage_gaps",
        "SELECT unnest(gap) AS gap FROM coverage_gap_lists",
    )
    .await?;
    view(session,"coverage_grouped","SELECT gap.kind AS kind,array_agg(DISTINCT gap ORDER BY gap) AS gaps FROM coverage_gaps GROUP BY gap.kind").await?;
    let frame = session.sql("SELECT d.kind,d.available,g.gaps,p.producer_binding_id FROM coverage_declared d LEFT JOIN coverage_grouped g ON d.kind=g.kind CROSS JOIN producing p").await?;
    let gaps_type = field(Relation::Coverage, "gaps")?;
    let arrow::datatypes::DataType::List(gap_type) = &gaps_type else {
        return Err(invalid("coverage gap schema"));
    };
    let default_gap = record(
        gap_type.data_type(),
        &[
            ("kind", col("kind")),
            ("reason", lit("not_attempted")),
            ("detail", lit("not acquired within this producer scope")),
        ],
    )?;
    let outcome = when(!col("available"), lit("missing"))
        .when(col("gaps").is_null(), lit("indexed"))
        .otherwise(lit("partial"))?;
    let gaps = coalesce(vec![
        col("gaps"),
        when(
            col("available"),
            datafusion::logical_expr::cast(
                datafusion::functions_nested::expr_fn::make_array(vec![]),
                arrow::datatypes::DataType::new_list(gap_type.data_type().clone(), true),
            ),
        )
        .otherwise(datafusion::functions_nested::expr_fn::make_array(vec![
            default_gap,
        ]))?,
    ]);
    let subject = variant(
        &field(Relation::Coverage, "subject")?,
        "library",
        &[("release_id", lit(&context.release_id))],
    )?;
    keyed(
        frame.select(vec![
            col("producer_binding_id"),
            subject.alias("subject"),
            col("kind"),
            outcome.alias("outcome"),
            gaps.alias("gaps"),
        ])?,
        Key::Coverage,
        "coverage_id",
        Relation::Coverage,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::{
            ArtifactKind, EvidenceKind,
            document::DocumentFact,
            ingest::DocumentBatch,
            relational::{CoverageOutcome, Locator, SubjectRef},
        },
        identity::Ecosystem,
        policy::ExecutionProfile,
        producer::{ProducerRun, RunOutcome},
        wire::{EvidenceClass, SourceVersionMatch},
    };
    fn context(artifact: Artifact) -> IngestContext {
        IngestContext {
            ecosystem: Ecosystem::Rust,
            symbol_package: "example".into(),
            release_id: "rel_example".into(),
            environment_id: "env_example".into(),
            source_version_match: SourceVersionMatch::Exact,
            producing_attempt: "attempt_documents".into(),
            producer_runs: vec![ProducerRun {
                attempt_id: "attempt_documents".into(),
                producer: "documents".into(),
                producer_version: "3".into(),
                config_digest: "a".repeat(64),
                inputs: [("readme".into(), artifact.sha256.clone())].into(),
                profile: ExecutionProfile::Static,
                started_at: enrichment_core::native_time::ObservationTime::try_from(
                    "2026-09-16T00:00:00.000000Z".to_owned(),
                )
                .unwrap(),
                finished_at: enrichment_core::native_time::ObservationTime::try_from(
                    "2026-09-16T00:00:01.000000Z".to_owned(),
                )
                .unwrap(),
                outcome: RunOutcome::Succeeded,
                gaps: vec![],
                log: None,
            }],
            artifacts: vec![artifact],
            indexed: vec![EvidenceKind::Documentation],
            missing: vec![EvidenceKind::Examples],
            gaps: vec![],
        }
    }
    fn document(artifact: &Artifact) -> DocumentFact {
        DocumentFact::new(
            enrichment_core::evidence::FragmentKind::ReadmeSection,
            "Overview",
            &artifact.artifact_id,
            Locator::MarkdownSection {
                file: "README.md".into(),
                heading: "Overview".into(),
                line: 1,
            },
            "Native Arrow evidence 🌎".into(),
            EvidenceClass::Declared,
            "source",
            "3",
        )
        .unwrap()
    }
    #[tokio::test]
    async fn native_documents_qualify_sources_coverage_and_direct_delta_outputs() {
        let root = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default()).unwrap();
        let artifact = Artifact::describe(
            b"readme",
            ArtifactKind::Readme,
            "text/markdown",
            "https://example.org/pkg#README.md",
            enrichment_core::native_time::AcquisitionTime::try_from(
                "2026-09-16T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
        );
        let input = DocumentBatch {
            fragments: vec![document(&artifact), document(&artifact)],
        };
        let (plans, attempts) = prepare(
            context(artifact.clone()),
            input,
            None,
            root.path(),
            &runtime,
            &WriteLimits::default(),
        )
        .await
        .unwrap();
        assert_eq!(attempts.len(), 1);
        assert_eq!(attempts[0].1, vec![artifact.clone()]);
        let delta =
            crate::native_delta::DeltaStore::new(&root.path().join("delta"), runtime.clone())
                .unwrap();
        for (relation, plan) in plans {
            if ![
                Relation::Fragments,
                Relation::InputArtifacts,
                Relation::ProducerRuns,
                Relation::Coverage,
            ]
            .contains(&relation)
            {
                assert!(
                    runtime
                        .execute(plan)
                        .await
                        .unwrap()
                        .batches
                        .iter()
                        .all(|b| b.num_rows() == 0)
                );
                continue;
            }
            let contract =
                crate::native_delta::StorageContract::new(relation.schema().unwrap()).unwrap();
            let table = delta
                .create(relation.name(), &contract, true)
                .await
                .unwrap();
            let table = delta
                .append(table, &contract, plan, vec![])
                .await
                .unwrap_or_else(|e| panic!("{}: {e}", relation.name()));
            let result = runtime
                .execute(
                    runtime
                        .session()
                        .read_table(delta.provider(&table, &contract).await.unwrap())
                        .unwrap(),
                )
                .await
                .unwrap();
            for batch in &result.batches {
                match relation {
                    Relation::Fragments => {
                        let rows = projection::fragments_from_batch(batch).unwrap();
                        assert_eq!(rows.len(), 1);
                        assert_eq!(
                            rows[0].source.source_uri.as_deref(),
                            Some(artifact.source_uri.as_str())
                        );
                        assert_eq!(rows[0].source.evidence_class, EvidenceClass::Declared);
                        assert!(
                            matches!(&rows[0].subject,SubjectRef::Document{heading,..} if heading=="Overview")
                        );
                        assert_eq!(rows[0].text, "Native Arrow evidence 🌎");
                    }
                    Relation::Coverage => {
                        let rows = projection::coverage_from_batch(batch).unwrap();
                        assert_eq!(rows.len(), 2);
                        let readme = rows
                            .iter()
                            .find(|r| r.kind == EvidenceKind::Documentation)
                            .unwrap();
                        assert_eq!(readme.outcome, CoverageOutcome::Indexed);
                        assert!(readme.gaps.is_empty());
                        let examples = rows
                            .iter()
                            .find(|r| r.kind == EvidenceKind::Examples)
                            .unwrap();
                        assert_eq!(examples.outcome, CoverageOutcome::Missing);
                        assert_eq!(examples.gaps.len(), 1);
                    }
                    Relation::InputArtifacts => assert_eq!(
                        projection::input_artifacts_from_batch(batch).unwrap().len(),
                        1
                    ),
                    Relation::ProducerRuns => assert_eq!(
                        projection::producer_runs_from_batch(batch).unwrap().len(),
                        1
                    ),
                    _ => unreachable!(),
                }
            }
        }
        let mut conflicting = context(artifact.clone());
        conflicting.missing.push(EvidenceKind::Documentation);
        assert!(
            prepare(
                conflicting,
                DocumentBatch::default(),
                None,
                root.path(),
                &runtime,
                &WriteLimits::default()
            )
            .await
            .is_err()
        );
        let mut ambiguous = context(artifact.clone());
        let mut other = artifact.clone();
        other.source_uri = "https://elsewhere.example/README.md".into();
        ambiguous.artifacts.push(other);
        let mut fact = document(&artifact);
        fact.locator = Locator::Artifact;
        assert!(
            prepare(
                ambiguous,
                DocumentBatch {
                    fragments: vec![fact]
                },
                None,
                root.path(),
                &runtime,
                &WriteLimits::default()
            )
            .await
            .is_err()
        );
    }
}
