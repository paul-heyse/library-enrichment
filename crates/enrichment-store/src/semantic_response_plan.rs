//! Native semantic answer policy. The transport supplies protocol bytes and physical source facts.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::col,
};
use enrichment_core::{
    evidence::execution::{
        ExecutionTarget, SemanticMethod, SemanticQuery, Utf8Position, Utf8Range,
    },
    identity::Ecosystem,
    native_lsp::{Decoded, Input, RangeInput},
    native_union::{NativeStruct, Rule},
    wire::EvidenceClass,
};

enrichment_core::native_struct! { pub struct Capture {
    method: SemanticMethod => Rule::Text,
    document_artifact_id: String => Rule::Reference(enrichment_core::native_union::Domain::Artifact),
    position: Option<Utf8Position> => Rule::Text,
    anchor_symbol_id: Option<String> => Rule::Reference(enrichment_core::native_union::Domain::Symbol),
    server: String => Rule::Text,
    response: Decoded => Rule::Text,
    locations: Vec<ExecutionTarget> => Rule::Sequence,
    capture_issue: Option<String> => Rule::Text,
    indexing_gap: Option<String> => Rule::Text,
    ecosystem: Ecosystem => Rule::Text,
    structural_scope: bool => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Product {
    query: SemanticQuery => Rule::Text,
    evidence_class: EvidenceClass => Rule::Text,
} }

async fn kernel<I: NativeStruct + enrichment_core::native_union::Cell, O: NativeStruct>(
    runtime: &QueryRuntime,
    input: &I,
    function: datafusion::logical_expr::ScalarUDF,
) -> Result<O> {
    let frame = runtime.session().read_empty()?.select(vec![
        function
            .call(vec![
                enrichment_core::evidence::arrow_model::expressions::literal(input)?,
            ])
            .alias("value"),
    ])?;
    runtime
        .records(
            frame.select(
                O::fields()
                    .iter()
                    .map(|f| col("value").field(f.name()).alias(f.name()))
                    .collect::<Vec<_>>(),
            )?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("semantic kernel result missing".into()))
}
pub async fn decode(runtime: &QueryRuntime, input: &Input) -> Result<Decoded> {
    kernel(runtime, input, enrichment_core::native_lsp::decoder()).await
}
pub async fn range(runtime: &QueryRuntime, input: &RangeInput) -> Result<Utf8Range> {
    kernel(runtime, input, enrichment_core::native_lsp::coordinates()).await
}

pub async fn lower(runtime: &QueryRuntime, capture: Capture) -> Result<Product> {
    use enrichment_core::evidence::arrow_model::expressions::record;
    use enrichment_core::native_union::Cell;
    let session = runtime.session();
    native_catalog::input(
        &session,
        "semantic_response_capture",
        Capture::batch(&[capture])?,
    )?;
    let frame=session.sql(r#"
      WITH response AS (SELECT *,
        (response.hover IS NOT NULL OR cardinality(locations)>0 OR cardinality(response.diagnostics)>0) AS has_results,
        (response.issue IS NOT NULL OR capture_issue IS NOT NULL) AS has_error,
        (ecosystem='python' AND method='implementation' AND structural_scope) AS incomplete_structural
        FROM semantic_response_capture), outcome AS (
        SELECT *, CASE WHEN response.kind='unsupported' THEN 'unsupported'
          WHEN response.kind='null' THEN 'unresolved'
          WHEN has_error AND NOT has_results THEN 'unresolved'
          WHEN has_error OR indexing_gap IS NOT NULL OR incomplete_structural THEN 'incomplete'
          WHEN has_results THEN 'results' ELSE 'empty' END AS outcome,
        array_compact([
          CASE WHEN response.kind='null' THEN 'The server returned null; target resolution or method availability was not established by that response.' END,
          CASE WHEN response.kind='unsupported' THEN concat(server,' did not advertise ',method,'; no matching version-qualified diagnostic push was available when applicable.') END,
          response.issue,capture_issue,indexing_gap,
          CASE WHEN method='references' THEN 'References cover the opened isolated consumer and the server selected workspace; external projects were not searched.' END,
          CASE WHEN ecosystem='rust' THEN 'rust-analyzer runs with build scripts and procedural macros disabled; generated declarations may be unresolved.' END,
          CASE WHEN incomplete_structural THEN 'This query does not establish exhaustive structural protocol implementors.' END
        ]) AS limitations
        FROM response)
      SELECT method,document_artifact_id,CASE WHEN method='diagnostics' THEN NULL ELSE position END AS position,anchor_symbol_id,server,outcome,
        response.hover AS hover,locations,response.diagnostics AS diagnostics,limitations,
        CASE WHEN ecosystem='rust' THEN 'compiler_derived' ELSE 'typechecker_observed' END AS evidence_class
      FROM outcome
    "#).await?;
    let query = record(
        &SemanticQuery::data_type(),
        &SemanticQuery::fields()
            .iter()
            .map(|f| (f.name().as_str(), col(f.name())))
            .collect::<Vec<_>>(),
    )?;
    runtime
        .records(
            frame.select(vec![query.alias("query"), col("evidence_class")])?,
            1,
        )
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("semantic lowering missing".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        evidence::execution::ExecutionOutcome,
        native_lsp::ProtocolRange,
        native_semantics::{PositionEncoding, ProtocolPosition, Utf16Position},
    };
    use serde_json::json;
    #[tokio::test]
    async fn plan19_native_semantic_response_preserves_format_coordinates_and_outcomes()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let document = "# 🌎 é\r\nvalue\n";
        let start = ProtocolPosition::Utf16 {
            value: Utf16Position {
                line: 0,
                code_unit: 5,
            },
        };
        let end = ProtocolPosition::Utf16 {
            value: Utf16Position {
                line: 0,
                code_unit: 6,
            },
        };
        let span = range(
            &runtime,
            &RangeInput {
                document: document.into(),
                range: ProtocolRange { start, end },
            },
        )
        .await?;
        assert_eq!(span.start, Utf8Position { line: 0, byte: 7 });
        assert_eq!(span.end, Utf8Position { line: 0, byte: 9 });
        let split = RangeInput {
            document: document.into(),
            range: ProtocolRange {
                start: ProtocolPosition::Utf16 {
                    value: Utf16Position {
                        line: 0,
                        code_unit: 3,
                    },
                },
                end: ProtocolPosition::Utf16 {
                    value: Utf16Position {
                        line: 0,
                        code_unit: 5,
                    },
                },
            },
        };
        assert!(range(&runtime, &split).await.is_err());
        async fn decoded(
            runtime: &QueryRuntime,
            method: SemanticMethod,
            answer: serde_json::Value,
        ) -> Result<Decoded> {
            decode(
                runtime,
                &Input {
                    method,
                    answer: answer.to_string(),
                    document: "🌎é\n".into(),
                    encoding: PositionEncoding::Utf16,
                },
            )
            .await
        }
        let location = json!({"uri":"file:///capsule/file.py","range":{"start":{"line":0,"character":0},"end":{"line":0,"character":2}}});
        let link =
            json!({"targetUri":"file:///capsule/file.py","targetSelectionRange":location["range"]});
        assert_eq!(
            decoded(&runtime, SemanticMethod::Definition, location.clone())
                .await?
                .locations,
            decoded(&runtime, SemanticMethod::Definition, link)
                .await?
                .locations
        );
        let malformed =
            decoded(&runtime, SemanticMethod::Definition, json!([location, {}])).await?;
        assert_eq!(malformed.locations.len(), 1);
        assert!(malformed.issue.is_some());
        let diagnostics=decoded(&runtime,SemanticMethod::Diagnostics,json!({"kind":"full","items":[{"range":{"start":{"line":0,"character":2},"end":{"line":0,"character":3}},"severity":2,"code":1,"message":"observed"}]})).await?;
        assert_eq!(diagnostics.diagnostics[0].range.start.byte, 4);
        assert_eq!(diagnostics.diagnostics[0].range.end.byte, 6);
        assert_eq!(diagnostics.diagnostics[0].code.as_deref(), Some("1"));
        for (answer, expected) in [
            (json!({"contents":"hover"}), ExecutionOutcome::Results),
            (json!(null), ExecutionOutcome::Unresolved),
            (json!({"kind":"unsupported"}), ExecutionOutcome::Unsupported),
            (json!({"contents":""}), ExecutionOutcome::Empty),
            (json!({}), ExecutionOutcome::Unresolved),
        ] {
            let response = decoded(&runtime, SemanticMethod::Hover, answer).await?;
            let capture = Capture {
                method: SemanticMethod::Hover,
                document_artifact_id: format!("art_{}", "a".repeat(64)),
                position: Some(Utf8Position { line: 0, byte: 0 }),
                anchor_symbol_id: Some(format!("sym_{}", "b".repeat(64))),
                server: "ty 0.0.80".into(),
                response,
                locations: vec![],
                capture_issue: None,
                indexing_gap: None,
                ecosystem: Ecosystem::Python,
                structural_scope: false,
            };
            let product = lower(&runtime, capture.clone()).await?;
            assert_eq!(product.query.outcome, expected);
            assert_eq!(product.evidence_class, EvidenceClass::TypecheckerObserved);
            let mut incomplete = capture;
            incomplete.indexing_gap = Some("index pending".into());
            assert_eq!(
                lower(&runtime, incomplete).await?.query.outcome,
                if matches!(
                    expected,
                    ExecutionOutcome::Results | ExecutionOutcome::Empty
                ) {
                    ExecutionOutcome::Incomplete
                } else {
                    expected
                }
            );
        }
        runtime.close_diagnostics().await
    }
}
