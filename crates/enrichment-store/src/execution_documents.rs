//! Native retained-content closure and coordinate admission; bounded reads are physical drivers.
use crate::{BlobStore, native_catalog, repository::EvidenceRepository, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result},
    dataframe::DataFrame,
    prelude::*,
};
use enrichment_core::{
    evidence::{arrow_model::expressions::record, execution::Utf8Range},
    native_lsp::Utf8RangeInput,
    native_union::{Cell, Domain, NativeStruct, Rule},
};

const DOCUMENT_BYTES: u64 = 1024 * 1024;
const DOCUMENT_BATCH: usize = 16;
enrichment_core::native_struct! { struct Input {
    artifact_id:String=>Rule::ArtifactIdentity {digest:"sha256".into()},
    sha256:String=>Rule::Sha256,
    size_bytes:u64=>Rule::UnsignedRange {min:0,max:DOCUMENT_BYTES}
} }
enrichment_core::native_struct! { struct Text {
    artifact_id:String=>Rule::Reference(Domain::Artifact), document:String=>Rule::Text
} }
enrichment_core::native_struct! { struct Coordinates {
    artifact_id:String=>Rule::Reference(Domain::Artifact), range:Utf8Range=>Rule::Text
} }

struct Prepared {
    documents: Vec<Input>,
    coordinates: DataFrame,
}

// Field drop order keeps the charge until the cloned text allocation is gone,
// including cancellation and errors from native coordinate admission.
struct CapturedText {
    texts: Vec<Text>,
    _memory: datafusion::execution::memory_pool::MemoryReservation,
}

async fn prepare(
    runtime: &QueryRuntime,
    session: &SessionContext,
    max_rows: usize,
) -> Result<Prepared> {
    let input = session
        .sql("SELECT DISTINCT artifact_id,sha256,size_bytes FROM snapshot.evidence.input_artifacts")
        .await?;
    native_catalog::work(session, "execution_content", input.into_view())?;
    runtime.require_empty(session.sql("SELECT artifact_id AS witness FROM execution_content GROUP BY artifact_id HAVING count(*)<>1").await?,"execution_input_identity","execution_documents").await?;
    let payload = session
        .table("snapshot.evidence.execution_observations")
        .await?
        .with_column(
            "expected_digest",
            crate::execution_fact_plan::content_digest(col("payload")),
        )?;
    native_catalog::work(session, "execution_payload_content", payload.into_view())?;
    runtime.require_empty(session.sql("SELECT e.observation_id AS witness FROM execution_payload_content e LEFT ANTI JOIN execution_content i ON e.source.artifact_id=i.artifact_id AND e.expected_digest=i.sha256").await?,"execution_retained_payload","execution_documents").await?;
    let needed=session.sql("SELECT payload.semantic_query.document_artifact_id AS artifact_id FROM execution_payload_content WHERE payload.kind='semantic_query' UNION SELECT payload.usage_probe.snippet_artifact_id FROM execution_payload_content WHERE payload.kind='usage_probe' UNION SELECT target.artifact.artifact_id FROM (SELECT unnest(payload.semantic_query.locations) AS target FROM execution_payload_content) WHERE target.kind='artifact'").await?;
    native_catalog::work(session, "execution_document_refs", needed.into_view())?;
    runtime.require_empty(session.sql("SELECT r.artifact_id AS witness FROM execution_document_refs r LEFT ANTI JOIN execution_content i ON r.artifact_id=i.artifact_id").await?,"execution_document_closure","execution_documents").await?;
    let selected=session.sql("SELECT i.* FROM execution_content i JOIN execution_document_refs r ON i.artifact_id=r.artifact_id ORDER BY i.artifact_id").await?;
    let selected =
        crate::native_delta::project(selected, &arrow::datatypes::Schema::new(Input::fields()))?;
    if let Some(violations) = enrichment_core::native_schema::intrinsic_violations(
        selected.clone(),
        selected.schema().as_arrow(),
    )? {
        runtime
            .require_empty(
                violations.select(vec![lit("execution_document_bound").alias("witness")])?,
                "execution_document_bound",
                "execution_documents",
            )
            .await?;
    }
    let schema = arrow::datatypes::Schema::new(Coordinates::fields());
    let positions=session.sql("SELECT payload.semantic_query.document_artifact_id AS artifact_id,payload.semantic_query.position AS position FROM execution_payload_content WHERE payload.kind='semantic_query' AND payload.semantic_query.position IS NOT NULL").await?;
    let point = record(
        &Utf8Range::data_type(),
        &[("start", col("position")), ("end", col("position"))],
    )?;
    let mut coordinates = crate::native_delta::project(
        positions.select(vec![col("artifact_id"), point.alias("range")])?,
        &schema,
    )?;
    for sql in [
        "SELECT artifact_id,diagnostic.range AS range FROM (SELECT payload.semantic_query.document_artifact_id AS artifact_id,unnest(payload.semantic_query.diagnostics) AS diagnostic FROM execution_payload_content)",
        "SELECT target.artifact.artifact_id AS artifact_id,target.artifact.range AS range FROM (SELECT unnest(payload.semantic_query.locations) AS target FROM execution_payload_content) WHERE target.kind='artifact'",
    ] {
        coordinates = coordinates.union(crate::native_delta::project(
            session.sql(sql).await?,
            &schema,
        )?)?;
    }
    Ok(Prepared {
        documents: runtime.records(selected, max_rows).await?,
        coordinates,
    })
}

async fn validate_texts(
    runtime: &QueryRuntime,
    coordinates: DataFrame,
    texts: &[Text],
) -> Result<()> {
    let session = runtime.session();
    native_catalog::input(&session, "execution_text", Text::batch(texts)?)?;
    native_catalog::work(&session, "execution_coordinates", coordinates.into_view())?;
    let joined=session.sql("SELECT p.*,t.document FROM execution_coordinates p JOIN execution_text t ON p.artifact_id=t.artifact_id").await?;
    let input = record(
        &Utf8RangeInput::data_type(),
        &[("document", col("document")), ("range", col("range"))],
    )?;
    let checked = joined.with_column(
        "checked",
        enrichment_core::native_lsp::utf8_coordinates().call(vec![input]),
    )?;
    native_catalog::work(&session, "checked_coordinates", checked.into_view())?;
    runtime.require_empty(session.sql("SELECT artifact_id AS witness FROM checked_coordinates WHERE checked IS DISTINCT FROM range").await?,"execution_utf8_coordinates","execution_documents").await
}

pub async fn validate(
    repository: &EvidenceRepository,
    session: &SessionContext,
    blobs: BlobStore,
    max_rows: usize,
) -> Result<()> {
    let runtime = &repository.runtime;
    let prepared = prepare(runtime, session, max_rows).await?;
    // Physical capture is bounded independently from the number of semantic observations.
    // Each unique document is read once; no Rust observation decoder or ad hoc text cache.
    for chunk in prepared.documents.chunks(DOCUMENT_BATCH) {
        let protection = repository
            .protect_artifacts(
                chunk
                    .iter()
                    .map(|input| input.artifact_id.clone())
                    .collect(),
            )
            .await?;
        let pool = runtime.session().runtime_env().memory_pool.clone();
        let inputs = chunk.to_vec();
        let source = blobs.clone();
        let captured = runtime
            .blocking(move || -> Result<_> {
                let _protection = protection;
                let memory = datafusion::execution::memory_pool::MemoryConsumer::new(
                    "execution-document-text",
                )
                .register(&pool);
                let mut texts = Vec::with_capacity(inputs.len());
                for input in inputs {
                    let bytes = source.read_content_owned(
                        &input.artifact_id,
                        &input.sha256,
                        input.size_bytes,
                        DOCUMENT_BYTES,
                        &pool,
                    )?;
                    memory.try_grow(bytes.len())?;
                    let document = std::str::from_utf8(&bytes)
                        .map_err(|error| DataFusionError::Execution(error.to_string()))?
                        .to_owned();
                    texts.push(Text {
                        artifact_id: input.artifact_id,
                        document,
                    });
                }
                Ok(CapturedText {
                    texts,
                    _memory: memory,
                })
            })
            .await??;
        validate_texts(runtime, prepared.coordinates.clone(), &captured.texts).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::evidence::execution::Utf8Position;

    enrichment_core::native_struct! { struct Source {
        artifact_id:String=>Rule::Reference(Domain::Artifact)
    } }
    enrichment_core::native_struct! { struct Observation {
        observation_id:String=>Rule::NonEmpty,
        source:Source=>Rule::Text,
        payload:enrichment_core::evidence::execution::ExecutionPayload=>Rule::Text
    } }

    #[tokio::test]
    async fn retained_document_plan_checks_payload_closure_and_selects_coordinates() -> Result<()> {
        use enrichment_core::evidence::execution::{
            ExecutionOutcome, ExecutionPayload, ExecutionTarget, SemanticMethod, SemanticQuery,
        };
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let document = Input {
            artifact_id: format!("art_{}", "a".repeat(64)),
            sha256: "a".repeat(64),
            size_bytes: 10,
        };
        let target = Input {
            artifact_id: format!("art_{}", "b".repeat(64)),
            sha256: "b".repeat(64),
            size_bytes: 12,
        };
        let payload = ExecutionPayload::SemanticQuery(SemanticQuery {
            method: SemanticMethod::Definition,
            document_artifact_id: document.artifact_id.clone(),
            position: Some(Utf8Position { line: 0, byte: 2 }),
            anchor_symbol_id: None,
            server: "unit".into(),
            outcome: ExecutionOutcome::Results,
            hover: None,
            locations: vec![ExecutionTarget::Artifact {
                artifact_id: target.artifact_id.clone(),
                range: Utf8Range {
                    start: Utf8Position { line: 0, byte: 0 },
                    end: Utf8Position { line: 0, byte: 4 },
                },
            }],
            diagnostics: vec![],
            limitations: vec![],
        });
        let bytes = payload
            .canonical_bytes()
            .map_err(DataFusionError::Execution)?;
        let digest = enrichment_core::canonical::sha256_hex(&bytes);
        let retained = Input {
            artifact_id: format!("art_{digest}"),
            sha256: digest,
            size_bytes: bytes.len() as u64,
        };
        let observed = Observation {
            observation_id: "query".into(),
            source: Source {
                artifact_id: retained.artifact_id.clone(),
            },
            payload,
        };
        let bind = |inputs: &[Input]| -> Result<SessionContext> {
            let session = runtime.session();
            let tables = [
                ("input_artifacts", Input::batch(inputs)?),
                (
                    "execution_observations",
                    Observation::batch(std::slice::from_ref(&observed))?,
                ),
            ]
            .into_iter()
            .map(|(name, batch)| {
                Ok((
                    name.into(),
                    native_catalog::batch(&session, name, batch)?.into_view(),
                ))
            })
            .collect::<Result<_>>()?;
            runtime.bound_session(
                [(
                    "snapshot".into(),
                    std::sync::Arc::new(
                        native_catalog::BoundCatalog::default()
                            .with_schema(native_catalog::BindingKind::AdmittedEvidence, tables),
                    )
                        as std::sync::Arc<dyn datafusion::catalog::CatalogProvider>,
                )]
                .into_iter()
                .collect(),
            )
        };
        let inputs = vec![document.clone(), target, retained];
        let prepared = prepare(&runtime, &bind(&inputs)?, 16).await?;
        assert_eq!(prepared.documents.len(), 2);
        let coordinates = runtime
            .records::<Coordinates>(prepared.coordinates, 16)
            .await?;
        assert_eq!(coordinates.len(), 2);
        assert!(
            coordinates
                .iter()
                .any(|row| row.artifact_id == document.artifact_id
                    && row.range.start.byte == 2
                    && row.range.end.byte == 2)
        );
        let mut missing = inputs.clone();
        missing.remove(1);
        assert!(prepare(&runtime, &bind(&missing)?, 16).await.is_err());
        let mut altered = inputs.clone();
        altered[2].sha256 = "c".repeat(64);
        assert!(prepare(&runtime, &bind(&altered)?, 16).await.is_err());
        let mut oversized = inputs.clone();
        oversized[0].size_bytes = DOCUMENT_BYTES + 1;
        assert!(prepare(&runtime, &bind(&oversized)?, 16).await.is_err());
        let mut contradictory = inputs.clone();
        contradictory.push(document);
        contradictory[3].size_bytes = 11;
        assert!(prepare(&runtime, &bind(&contradictory)?, 16).await.is_err());
        runtime.close_diagnostics().await
    }
    #[tokio::test]
    async fn retained_execution_coordinates_use_exact_utf8_document_boundaries() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let texts = [Text {
            artifact_id: format!("art_{}", "a".repeat(64)),
            document: "α😀\r\nlast\n".into(),
        }];
        let check = async |range: Utf8Range| -> Result<()> {
            validate_texts(
                &runtime,
                crate::native_catalog::batch(
                    &runtime.session(),
                    "execution_documents",
                    Coordinates::batch(&[Coordinates {
                        artifact_id: texts[0].artifact_id.clone(),
                        range,
                    }])?,
                )?,
                &texts,
            )
            .await
        };
        check(Utf8Range {
            start: Utf8Position { line: 0, byte: 2 },
            end: Utf8Position { line: 0, byte: 6 },
        })
        .await?;
        check(Utf8Range {
            start: Utf8Position { line: 2, byte: 0 },
            end: Utf8Position { line: 2, byte: 0 },
        })
        .await?;
        for range in [
            Utf8Range {
                start: Utf8Position { line: 0, byte: 1 },
                end: Utf8Position { line: 0, byte: 2 },
            },
            Utf8Range {
                start: Utf8Position { line: 3, byte: 0 },
                end: Utf8Position { line: 3, byte: 0 },
            },
            Utf8Range {
                start: Utf8Position { line: 1, byte: 3 },
                end: Utf8Position { line: 1, byte: 1 },
            },
        ] {
            assert!(check(range).await.is_err());
        }
        runtime.close_diagnostics().await
    }
}
