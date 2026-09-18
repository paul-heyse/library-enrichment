//! Inspection coverage is composed from qualified facts and physical source observations.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::col,
};
use enrichment_core::{
    evidence::{execution::ExecutionObservation, model::AncillaryFacts},
    native_union::{NativeStruct, Rule},
    wire::{
        AspectOutcome, Coverage, Diagnostic, InspectionAspect, Page, research::AspectSelection,
    },
};

enrichment_core::native_struct! { pub struct Requirements {
    source:bool => Rule::Text,
    configuration:bool => Rule::Text,
    ancillary:bool => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Route {
    selection:AspectSelection => Rule::Text,
    fragment_kinds:Vec<enrichment_core::evidence::FragmentKind> => Rule::SequenceBounds { min:0,max:2 },
} }
enrichment_core::native_struct! { struct Selected {
    ordinal:usize => Rule::Coordinate(enrichment_core::native_union::Unit::Ordinal),
    selection:AspectSelection => Rule::Text,
} }
enrichment_core::native_struct! { pub struct PageObservation {
    aspect:InspectionAspect => Rule::Text,
    page:Page => Rule::Text,
} }
enrichment_core::native_vocabulary! { pub enum FailureStage {
    Page="page", Ancillary="ancillary", Availability="availability", Source="source",
} }
enrichment_core::native_struct! { pub struct Failure {
    aspect:InspectionAspect => Rule::Text,
    stage:FailureStage => Rule::Text,
    reason:String => Rule::NonEmpty,
    diagnostic:Diagnostic => Rule::Text,
} }

fn selected(
    runtime: &QueryRuntime,
    selection: &[AspectSelection],
) -> Result<datafusion::prelude::SessionContext> {
    let session = runtime.session();
    let rows = selection
        .iter()
        .cloned()
        .enumerate()
        .map(|(ordinal, selection)| Selected { ordinal, selection })
        .collect::<Vec<_>>();
    crate::native_catalog::input(&session, "inspection_selected", Selected::batch(&rows)?)?;
    Ok(session)
}

/// Physical callers dispatch these declared routes and capture observations. Requirements,
/// fragment policy and final outcome selection remain in the same native catalog.
pub async fn routes(
    runtime: &QueryRuntime,
    selection: &[AspectSelection],
) -> Result<(Vec<Route>, Requirements)> {
    let session = selected(runtime, selection)?;
    let routes=runtime.records(session.sql("SELECT s.selection,d.fragment_kinds FROM inspection_selected s JOIN operation.declarations.inspection_aspects d ON s.selection.aspect=d.aspect ORDER BY s.ordinal").await?,InspectionAspect::VALUES.len()).await?;
    let requirements=runtime.records(session.sql("SELECT count(*) FILTER (WHERE d.observation='source')>0 AS source,count(*) FILTER (WHERE d.observation='configuration')>0 AS configuration,count(*) FILTER (WHERE d.observation IN ('source','configuration'))>0 AS ancillary FROM inspection_selected s JOIN operation.declarations.inspection_aspects d ON s.selection.aspect=d.aspect").await?,1).await?.pop().ok_or_else(||DataFusionError::Internal("inspection requirements missing".into()))?;
    Ok((routes, requirements))
}

/// Validate observation scope and compose one outcome per requested aspect. Failed stages
/// retain every distinct reason; the diagnostic is selected in stable stage-label order.
/// No caller overwrites a prior failure or manufactures an available empty page.
pub async fn outcomes(
    runtime: &QueryRuntime,
    selection: &[AspectSelection],
    pages: &[PageObservation],
    failures: &[Failure],
) -> Result<Vec<AspectOutcome>> {
    let session = selected(runtime, selection)?;
    crate::native_catalog::input(&session, "inspection_pages", PageObservation::batch(pages)?)?;
    crate::native_catalog::input(&session, "inspection_failures", Failure::batch(failures)?)?;
    runtime.require_empty(session.sql(r#"
      SELECT 'duplicate_selected_aspect' AS witness FROM inspection_selected GROUP BY selection.aspect HAVING count(*)<>1
      UNION ALL SELECT 'duplicate_aspect_page' FROM inspection_pages GROUP BY aspect HAVING count(*)<>1
      UNION ALL SELECT 'duplicate_failure_stage' FROM inspection_failures GROUP BY aspect,stage HAVING count(*)<>1
      UNION ALL SELECT 'unrequested_page' FROM inspection_pages p LEFT ANTI JOIN inspection_selected s ON p.aspect=s.selection.aspect
      UNION ALL SELECT 'unrequested_failure' FROM inspection_failures f LEFT ANTI JOIN inspection_selected s ON f.aspect=s.selection.aspect
      UNION ALL SELECT 'missing_collection_observation' FROM inspection_selected s JOIN operation.declarations.inspection_aspects d ON s.selection.aspect=d.aspect
        LEFT ANTI JOIN inspection_pages p ON p.aspect=s.selection.aspect LEFT ANTI JOIN inspection_failures f ON f.aspect=s.selection.aspect WHERE d.observation IN ('none','execution')
      UNION ALL SELECT 'singleton_page' FROM inspection_pages p JOIN operation.declarations.inspection_aspects d ON p.aspect=d.aspect WHERE d.observation IN ('source','configuration')
      UNION ALL SELECT 'wrong_failure_stage' FROM inspection_failures f JOIN operation.declarations.inspection_aspects d ON f.aspect=d.aspect
        WHERE (f.stage='page' AND d.observation NOT IN ('none','execution'))
          OR (f.stage='ancillary' AND d.observation NOT IN ('source','configuration'))
          OR (f.stage='availability' AND d.observation<>'configuration')
          OR (f.stage='source' AND d.observation<>'source')
    "#).await?,"inspection_observation_scope","inspection_outcomes").await?;
    let plan=session.sql(r#"
      WITH failures AS (
        SELECT aspect,string_agg(reason,'; ' ORDER BY stage) AS reason,
          first_value(diagnostic ORDER BY stage) AS diagnostic FROM inspection_failures GROUP BY aspect
      ) SELECT s.selection.aspect AS aspect,
        CASE WHEN f.aspect IS NULL THEN 'available' ELSE 'failed' END AS state,
        f.reason,p.page,f.diagnostic FROM inspection_selected s
        LEFT JOIN inspection_pages p ON p.aspect=s.selection.aspect
        LEFT JOIN failures f ON f.aspect=s.selection.aspect ORDER BY s.ordinal
    "#).await?;
    runtime.records(plan, InspectionAspect::VALUES.len()).await
}

enrichment_core::native_struct! { pub struct TextRecovery {
    request:enrichment_core::request::InspectRequest => Rule::Text,
    symbol:enrichment_core::evidence::model::SymbolHeader => Rule::Text,
    snapshot:enrichment_core::identity::SnapshotId => Rule::Text,
    selection:AspectSelection => Rule::Text,
    witness:enrichment_core::operation::selections::SelectionWitness => Rule::Text,
    max_bytes:usize => Rule::UnsignedRange { min:1,max:u64::MAX },
    after:Option<String> => Rule::Text,
} }
enrichment_core::native_struct! { struct Recovery { action:enrichment_core::wire::RecoveryAction => Rule::Text } }

/// Retained-only full-text recovery binds the same exact symbol/snapshot and boundary, with
/// a freshly derived selection witness. The format kernel only encodes that selected cursor.
pub async fn text_recovery(
    runtime: &QueryRuntime,
    input: TextRecovery,
) -> Result<enrichment_core::wire::RecoveryAction> {
    use datafusion::{functions_nested::expr_fn::make_array, prelude::lit};
    use enrichment_core::{
        evidence::arrow_model::expressions::{derive_record, null, record, variant},
        native_key::Key,
        native_union::Cell,
        request::{InspectRequest, InspectionOptions, ResearchRequest},
        search::row_page::CursorInput,
        wire::{RecoveryAction, ResearchSelection},
    };
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "inspection_recovery",
        TextRecovery::batch(&[input])?,
    )?;
    runtime.require_empty(session.sql("SELECT 'non_fragment_recovery' AS witness FROM inspection_recovery r LEFT JOIN operation.declarations.inspection_aspects d ON r.selection.aspect=d.aspect WHERE d.aspect IS NULL OR cardinality(d.fragment_kinds)=0").await?,"inspection_recovery_scope","inspection_recovery").await?;
    let execution = derive_record(
        col("request").field("execution"),
        &InspectionOptions::data_type(),
        &[("intent", lit("retained"))],
    )?;
    let frame = session
        .table("inspection_recovery")
        .await?
        .with_column("retained_execution", execution)?;
    let digest = Key::InspectionSelection.bind(vec![
        col("witness"),
        col("symbol").field("symbol_id"),
        col("selection").field("aspect"),
        col("selection").field("max_items"),
        null(&<Option<usize>>::data_type())?,
        col("max_bytes"),
        col("retained_execution"),
    ])?;
    let cursor = enrichment_core::search::row_page::encoder().call(vec![record(
        &CursorInput::data_type(),
        &[
            ("snapshot", col("snapshot")),
            ("selection", digest),
            ("after", col("after")),
        ],
    )?]);
    let aspect = derive_record(
        col("selection"),
        &AspectSelection::data_type(),
        &[
            ("max_characters", null(&<Option<usize>>::data_type())?),
            ("cursor", cursor),
        ],
    )?;
    let selection = variant(
        &ResearchSelection::data_type(),
        "explicit",
        &[("aspects", make_array(vec![aspect]))],
    )?;
    let request = derive_record(
        col("request"),
        &InspectRequest::data_type(),
        &[
            ("snapshot_id", col("snapshot")),
            ("symbol_path", col("symbol").field("path")),
            ("definition_id", col("symbol").field("definition_id")),
            ("selection", selection),
            ("execution", col("retained_execution")),
        ],
    )?;
    let frame = frame.with_column("recovery_request", request)?;
    let request = variant(
        &ResearchRequest::data_type(),
        "inspect_symbol",
        &InspectRequest::fields()
            .iter()
            .map(|field| {
                (
                    field.name().as_str(),
                    col("recovery_request").field(field.name()),
                )
            })
            .collect::<Vec<_>>(),
    )?;
    let action = variant(
        &RecoveryAction::data_type(),
        "call_tool",
        &[("request", request)],
    )?;
    Ok(runtime
        .records::<Recovery>(frame.select(vec![action.alias("action")])?, 1)
        .await?
        .pop()
        .ok_or_else(|| datafusion::common::exec_datafusion_err!("inspection recovery missing"))?
        .action)
}

/// Consensus ignores absent qualifications; disagreement never selects an arbitrary row.
pub(crate) async fn ancillary(
    runtime: &QueryRuntime,
    facts: datafusion::dataframe::DataFrame,
) -> Result<AncillaryFacts> {
    let session = runtime.session();
    crate::native_catalog::work(&session, "inspection_ancillary", facts.into_view())?;
    let plan = session.sql("SELECT
        CASE WHEN count(DISTINCT cfg_hints)=1 THEN first_value(cfg_hints) FILTER (WHERE cfg_hints IS NOT NULL) ELSE NULL END AS cfg_hints,
        CASE WHEN count(DISTINCT locator)=1 THEN first_value(locator) FILTER (WHERE locator IS NOT NULL) ELSE NULL END AS locator,
        CAST(count(DISTINCT cfg_hints) AS BIGINT UNSIGNED) AS cfg_alternatives,
        CAST(count(DISTINCT locator) AS BIGINT UNSIGNED) AS locator_alternatives
        FROM inspection_ancillary").await?;
    runtime
        .records(plan, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("ancillary aggregate returned no row".into()))
}

/// Artifact consensus runs before any descriptors cross the native boundary. The exact
/// snapshot owns the input relation; this decision does not consult a mutable blob index.
pub(crate) async fn source_input(
    runtime: &QueryRuntime,
    inputs: datafusion::dataframe::DataFrame,
) -> Result<Option<enrichment_core::evidence::relational::InputArtifact>> {
    let session = runtime.session();
    crate::native_catalog::work(&session, "inspection_inputs", inputs.into_view())?;
    runtime.require_empty(
        session.sql("SELECT 'source-input-consensus' AS id FROM inspection_inputs HAVING count(DISTINCT sha256)>1").await?,
        "source_input_consensus",
        "inspection_source",
    ).await?;
    let selected = session
        .sql("SELECT * FROM inspection_inputs ORDER BY input_id LIMIT 1")
        .await?;
    let rows: Vec<enrichment_core::evidence::relational::InputArtifact> =
        runtime.records(selected, 1).await?;
    let selected = rows.into_iter().next();
    if let Some(input) = &selected {
        input.validate().map_err(DataFusionError::Execution)?;
    }
    Ok(selected)
}

enrichment_core::native_struct! { struct Presentation {
    coverage: Coverage => Rule::Text,
    ancillary: Option<AncillaryFacts> => Rule::Text,
    source_requested: bool => Rule::Text,
    source_available: bool => Rule::Text,
} }

pub async fn coverage(
    runtime: &QueryRuntime,
    coverage: Coverage,
    ancillary: Option<AncillaryFacts>,
    source_requested: bool,
    source_available: bool,
    observations: &[ExecutionObservation],
) -> Result<Coverage> {
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "inspection_presentation",
        Presentation::batch(&[Presentation {
            coverage,
            ancillary,
            source_requested,
            source_available,
        }])?,
    )?;
    crate::native_catalog::input(
        &session,
        "inspection_execution",
        ExecutionObservation::batch(observations)?,
    )?;
    let executed = session.sql("SELECT d.evidence_kind AS kind, coalesce(o.payload.semantic_query.outcome,o.payload.runtime_object.outcome) AS outcome,coalesce(o.payload.semantic_query.limitations,o.payload.runtime_object.limitations) AS limitations FROM inspection_execution o JOIN operation.declarations.execution_payloads d ON o.payload.kind=d.kind WHERE o.payload.kind IN ('semantic_query','runtime_object')").await?;
    crate::native_catalog::work(&session, "presentation_execution", executed.into_view())?;
    let additions = session.sql(r#"
        WITH missing AS (
          SELECT unnest(coverage.missing) AS kind FROM inspection_presentation
          UNION SELECT 'source_excerpts' AS kind FROM inspection_presentation WHERE source_requested AND NOT source_available
          UNION SELECT kind FROM presentation_execution WHERE outcome NOT IN ('results','empty')
        ), notes AS (
          SELECT unnest(coverage.limitations) AS note FROM inspection_presentation
          UNION SELECT unnest(limitations) AS note FROM presentation_execution
          UNION SELECT 'Qualified observations disagree on cfg hints; read the independent signature observations.' AS note FROM inspection_presentation WHERE ancillary.cfg_alternatives>1
          UNION SELECT 'Qualified observations have different source coordinates; no single source window was selected.' AS note FROM inspection_presentation WHERE ancillary.locator_alternatives>1
          UNION SELECT 'The recorded span could not be located in the extracted crate archive; the definition may live in a generated or external file.' AS note FROM inspection_presentation WHERE source_requested AND NOT source_available
          UNION SELECT 'Availability describes the documented build; whether the project has this item depends on its features and target, which are reported separately.' AS note
        ) SELECT (SELECT array_agg(kind ORDER BY kind) FROM missing) AS missing,
                 (SELECT array_agg(note ORDER BY note) FROM notes) AS limitations
    "#).await?;
    crate::native_catalog::work(&session, "presentation_additions", additions.into_view())?;
    let frame = session.sql("SELECT p.coverage,a.missing,a.limitations FROM inspection_presentation p CROSS JOIN presentation_additions a").await?;
    let columns = Coverage::fields()
        .iter()
        .map(|field| {
            let name = field.name();
            let expression = if matches!(name.as_str(), "missing" | "limitations") {
                datafusion::functions::core::expr_fn::coalesce(vec![
                    col(name),
                    enrichment_core::evidence::arrow_model::expressions::literal(
                        &Vec::<String>::new(),
                    )?,
                ])
            } else {
                col("coverage").field(name)
            };
            Ok(expression.alias(name))
        })
        .collect::<Result<Vec<_>>>()?;
    runtime
        .records(frame.select(columns)?, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("inspection coverage projection missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aspect(aspect: InspectionAspect) -> AspectSelection {
        AspectSelection {
            aspect,
            cursor: None,
            max_items: 4,
            max_characters: None,
        }
    }

    #[tokio::test]
    async fn aspect_observations_preserve_order_failures_and_scope() -> Result<()> {
        use enrichment_core::wire::{AspectState, ErrorCode};
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let selection = vec![
            aspect(InspectionAspect::Source),
            aspect(InspectionAspect::Documentation),
        ];
        let (selected, requirements) = routes(&runtime, &selection).await?;
        assert!(requirements.source && requirements.ancillary && !requirements.configuration);
        assert!(selected[0].fragment_kinds.is_empty());
        assert_eq!(
            selected[1].fragment_kinds,
            vec![
                enrichment_core::evidence::FragmentKind::DocText,
                enrichment_core::evidence::FragmentKind::ReadmeSection
            ]
        );
        let pages = vec![PageObservation {
            aspect: InspectionAspect::Documentation,
            page: Page::new(2, None, false, None),
        }];
        let failures = vec![
            Failure {
                aspect: InspectionAspect::Source,
                stage: FailureStage::Source,
                reason: "source file unavailable".into(),
                diagnostic: Diagnostic::for_error(
                    ErrorCode::ArtifactUnavailable,
                    "read retained evidence".into(),
                ),
            },
            Failure {
                aspect: InspectionAspect::Source,
                stage: FailureStage::Ancillary,
                reason: "coordinates disagree".into(),
                diagnostic: Diagnostic::for_error(
                    ErrorCode::QueryFailed,
                    "inspect qualifications".into(),
                ),
            },
        ];
        let result = outcomes(&runtime, &selection, &pages, &failures).await?;
        assert_eq!(
            result.iter().map(|row| row.aspect).collect::<Vec<_>>(),
            vec![InspectionAspect::Source, InspectionAspect::Documentation]
        );
        assert_eq!(result[0].state, AspectState::Failed);
        assert_eq!(
            result[0].reason.as_deref(),
            Some("coordinates disagree; source file unavailable")
        );
        assert_eq!(result[0].diagnostic, Some(failures[1].diagnostic.clone()));
        assert_eq!(result[1].page, Some(pages[0].page.clone()));
        assert_eq!(result[1].state, AspectState::Available);
        let mut reversed = failures.clone();
        reversed.reverse();
        assert_eq!(
            outcomes(&runtime, &selection, &pages, &reversed).await?,
            result
        );
        assert!(
            outcomes(&runtime, &selection, &[], &failures)
                .await
                .is_err()
        );
        assert!(
            outcomes(
                &runtime,
                &selection,
                &[pages[0].clone(), pages[0].clone()],
                &failures
            )
            .await
            .is_err()
        );
        assert!(
            outcomes(&runtime, &selection[..1], &pages, &failures)
                .await
                .is_err()
        );
        let mut wrong = failures[0].clone();
        wrong.stage = FailureStage::Page;
        assert!(
            outcomes(&runtime, &selection, &pages, &[wrong])
                .await
                .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn full_text_recovery_is_pinned_retained_and_rebinds_its_cursor() -> Result<()> {
        use enrichment_core::{
            evidence::{SymbolKind, model::SymbolHeader},
            identity::SnapshotId,
            operation::selections::{InspectionSelection, SelectionWitness},
            request::{InspectRequest, InspectionIntent, InspectionOptions, ResearchRequest},
            search::row_page::RowCursor,
            wire::{RecoveryAction, ResearchSelection},
        };
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let snapshot: SnapshotId = format!("snap_{}", "2".repeat(64)).try_into().unwrap();
        let symbol = SymbolHeader {
            symbol_id: format!("symbol_{}", "3".repeat(64)),
            definition_id: format!("def_{}", "4".repeat(64)),
            path: "example::Type".into(),
            name: "Type".into(),
            kind: SymbolKind::Struct,
            parent_path: Some("example".into()),
            is_reexport: false,
            definition_path: "example::Type".into(),
            defined_in_package: "example".into(),
            qualifier: None,
        };
        let request = InspectRequest {
            context_id: format!("ctx_{}", "1".repeat(64)).try_into().unwrap(),
            snapshot_id: None,
            symbol_path: "Type".into(),
            definition_id: None,
            selection: ResearchSelection::Default,
            max_bytes: Some(8192),
            execution: None,
        };
        let mut selected = aspect(InspectionAspect::Documentation);
        selected.max_characters = Some(7);
        selected.cursor = Some("old-preview-cursor".into());
        let witness = SelectionWitness {
            runtime: "runtime-witness".into(),
            policy: enrichment_core::identity::OperationPolicyId::from_record(
                &enrichment_core::config::Config::default(),
            ),
        };
        for execution in [
            None,
            Some(InspectionOptions {
                intent: InspectionIntent::Rerun,
                ..Default::default()
            }),
        ] {
            for after in [None, Some("fragment_λ🦀".into())] {
                let request = InspectRequest {
                    execution: execution.clone(),
                    ..request.clone()
                };
                let result = text_recovery(
                    &runtime,
                    TextRecovery {
                        request: request.clone(),
                        symbol: symbol.clone(),
                        snapshot: snapshot.clone(),
                        selection: selected.clone(),
                        witness: witness.clone(),
                        max_bytes: 4096,
                        after: after.clone(),
                    },
                )
                .await?;
                let RecoveryAction::CallTool { request: recovered } = result else {
                    panic!("typed tool recovery")
                };
                let ResearchRequest::Inspect(recovered) = *recovered else {
                    panic!("inspection recovery")
                };
                assert_eq!(recovered.context_id, request.context_id);
                assert_eq!(recovered.snapshot_id, Some(snapshot.clone()));
                assert_eq!(recovered.symbol_path, symbol.path);
                assert_eq!(recovered.definition_id, Some(symbol.definition_id.clone()));
                assert_eq!(recovered.max_bytes, request.max_bytes);
                let mut expected_execution = execution.clone();
                if let Some(execution) = &mut expected_execution {
                    execution.intent = InspectionIntent::Retained;
                }
                assert_eq!(recovered.execution, expected_execution);
                let ResearchSelection::Explicit { aspects } = recovered.selection else {
                    panic!("one explicit recovered aspect")
                };
                assert_eq!(aspects.len(), 1);
                assert_eq!(aspects[0].max_characters, None);
                assert_eq!(aspects[0].max_items, selected.max_items);
                let digest = InspectionSelection {
                    witness: witness.clone(),
                    symbol_id: symbol.symbol_id.clone(),
                    aspect: selected.aspect,
                    max_items: selected.max_items,
                    max_characters: None,
                    max_bytes: 4096,
                    execution: expected_execution,
                }
                .identity();
                assert_eq!(aspects[0].cursor.is_some(), after.is_some());
                if let Some(after) = after {
                    assert_eq!(
                        RowCursor::decode(
                            aspects[0].cursor.as_deref().unwrap(),
                            &snapshot,
                            &digest
                        )
                        .unwrap()
                        .after,
                        after
                    );
                }
            }
        }
        assert!(RowCursor::encode(&snapshot, "selection", "\\\"".repeat(32_768)).is_err());
        let wrong = TextRecovery {
            request,
            symbol,
            snapshot,
            selection: aspect(InspectionAspect::Source),
            witness,
            max_bytes: 4096,
            after: None,
        };
        assert!(text_recovery(&runtime, wrong).await.is_err());
        runtime.close_diagnostics().await?;
        Ok(())
    }

    enrichment_core::native_struct! { struct QualifiedAncillary {
        cfg_hints: Option<Vec<String>> => Rule::Sequence,
        locator: Option<enrichment_core::evidence::relational::Locator> => Rule::Text,
    } }

    #[tokio::test]
    async fn plan19_ancillary_consensus_ignores_nulls_but_preserves_disagreement() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let known = QualifiedAncillary {
            cfg_hints: Some(vec!["unix".into()]),
            locator: Some(
                enrichment_core::evidence::relational::Locator::RustdocItem {
                    item: 1,
                    reported_file: Some("src/lib.rs".into()),
                    reported_line: Some(5),
                },
            ),
        };
        let absent = QualifiedAncillary {
            cfg_hints: None,
            locator: None,
        };
        for rows in [
            vec![],
            vec![absent.clone()],
            vec![absent.clone(), known.clone()],
            vec![known.clone(), absent.clone(), known.clone()],
        ] {
            let frame = crate::native_catalog::batch(
                &runtime.session(),
                "research_inspection",
                QualifiedAncillary::batch(&rows)?,
            )?;
            let result = ancillary(&runtime, frame).await?;
            let count = u64::from(rows.iter().any(|row| row.cfg_hints.is_some()));
            assert_eq!(result.cfg_alternatives, count);
            assert_eq!(result.locator_alternatives, count);
            if count == 1 {
                assert_eq!(result.cfg_hints, known.cfg_hints);
                assert_eq!(result.locator, known.locator);
            } else {
                assert!(result.cfg_hints.is_none());
                assert!(result.locator.is_none());
            }
        }
        let mut different = known.clone();
        different.cfg_hints = Some(vec![]);
        different.locator = Some(enrichment_core::evidence::relational::Locator::Artifact);
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "research_inspection",
            QualifiedAncillary::batch(&[absent, known, different])?,
        )?;
        let result = ancillary(&runtime, frame).await?;
        assert_eq!(
            (result.cfg_alternatives, result.locator_alternatives),
            (2, 2)
        );
        assert!(result.cfg_hints.is_none());
        assert!(result.locator.is_none());
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_source_consensus_is_exact_bounded_and_deterministic() -> Result<()> {
        use enrichment_core::evidence::{Artifact, ArtifactKind, relational::InputArtifact};
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let input = |binding: &str, byte: &str| {
            let digest = byte.repeat(64);
            InputArtifact::new(
                binding.into(),
                "crate-source".into(),
                &Artifact {
                    artifact_id: format!("art_{digest}"),
                    sha256: digest,
                    media_type: "application/gzip".into(),
                    size_bytes: 3,
                    kind: ArtifactKind::CrateTarball,
                    source_uri: "https://example.invalid/source.crate".into(),
                    final_url: None,
                    retrieved_at: enrichment_core::native_time::AcquisitionTime::from_micros(1)
                        .unwrap(),
                    etag: None,
                    last_modified: None,
                    compression: None,
                },
            )
            .unwrap()
        };
        let a = input("binding-a", "a");
        let b = input("binding-b", "a");
        let conflict = input("binding-c", "b");
        for rows in [
            vec![],
            vec![a.clone()],
            vec![a.clone(), b.clone()],
            vec![b.clone(), a.clone()],
        ] {
            let expected = rows.iter().min_by_key(|row| &row.input_id).cloned();
            let frame = crate::native_catalog::batch(
                &runtime.session(),
                "research_inspection",
                InputArtifact::batch(&rows)?,
            )?;
            assert_eq!(source_input(&runtime, frame).await?, expected);
        }
        let frame = crate::native_catalog::batch(
            &runtime.session(),
            "research_inspection",
            InputArtifact::batch(&[a, b, conflict])?,
        )?;
        assert!(source_input(&runtime, frame).await.is_err());
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_inspection_coverage_combines_source_and_qualified_limits() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(directory.path(), Default::default())?;
        let mut input = Coverage::unassessed("symbol scope");
        input.missing =
            std::collections::BTreeSet::from(["public_api".into(), "source_excerpts".into()]);
        input.limitations = vec![
            "retained qualification".into(),
            "retained qualification".into(),
        ];
        let result = coverage(
            &runtime,
            input.clone(),
            Some(AncillaryFacts {
                cfg_hints: None,
                locator: None,
                cfg_alternatives: 2,
                locator_alternatives: 2,
            }),
            true,
            false,
            &[],
        )
        .await?;
        assert_eq!(result.scope, input.scope);
        assert_eq!(
            result.missing,
            std::collections::BTreeSet::from(["public_api".into(), "source_excerpts".into()])
        );
        assert_eq!(result.limitations.len(), 5);
        let result = coverage(
            &runtime,
            Coverage::unassessed("empty"),
            None,
            false,
            false,
            &[],
        )
        .await?;
        assert!(result.missing.is_empty());
        assert_eq!(result.limitations.len(), 1);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
