//! Pure Arrow presentation/scoring fixture; no snapshot or publication I/O.
use super::*;
use enrichment_core::{
    evidence::{
        SymbolKind,
        relational::{FactSource, Locator, SubjectRef},
    },
    native_union::NativeStruct,
    wire::{EvidenceClass, SourceVersionMatch},
};
enrichment_core::native_struct! { struct Input {
    hit_order:u32=>Rule::Text,
    candidate_id:String=>Rule::NonEmpty,
    fact_id:String=>Rule::Text,
    label:String=>Rule::Text,
    path:Option<String> =>Rule::Text,
    symbol_kind:Option<SymbolKind> =>Rule::Text,
    signature:Option<String> =>Rule::Text,
    fragment_kind:Option<FragmentKind> =>Rule::Text,
    excerpt:String=>Rule::Text,
    also_at:Vec<String> =>Rule::Sequence,
    deprecated:bool=>Rule::Text,
    subject_ref:SubjectRef=>Rule::Text,
    source:FactSource=>Rule::Text,
} }
fn fixture() -> Input {
    Input {
        hit_order: 0,
        candidate_id: "definition".into(),
        fact_id: "observation".into(),
        label: "λ::Type".into(),
        path: Some("λ::Type".into()),
        symbol_kind: Some(SymbolKind::Struct),
        signature: Some("struct Type".into()),
        fragment_kind: None,
        excerpt: "λBCDEF".into(),
        also_at: vec!["other::Type".into()],
        deprecated: false,
        subject_ref: SubjectRef::Definition {
            definition_id: format!("def_{}", "3".repeat(64)),
        },
        source: FactSource {
            producer_binding_id: format!("producer_{}", "1".repeat(64)),
            extractor: "rustdoc".into(),
            extractor_version: "61".into(),
            artifact_id: format!("art_{}", "2".repeat(64)),
            source_uri: None,
            source_version_match: SourceVersionMatch::Exact,
            locator: Locator::RustdocItem {
                item: 8,
                reported_file: None,
                reported_line: None,
            },
            evidence_class: EvidenceClass::CompilerDerived,
        },
    }
}

#[tokio::test]
async fn native_search_projection_binds_citation_excerpt_and_factor_sum() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let input = fixture();
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "tests",
        Input::batch(std::slice::from_ref(&input))?,
    )?
    .with_column(
        "ranking",
        scoring::ranking(
            ScoreKind::Symbol,
            &SearchSpec::new("Type"),
            vec![
                col("path"),
                lit("Type"),
                col("signature"),
                lit("A type"),
                lit("Type documentation"),
                lit(false),
            ],
        )?,
    )?;
    let invalid = frame.clone().with_column("hit_order", lit(2u32))?;
    assert!(
        runtime
            .records::<SearchResult>(present(invalid, 3)?, 1)
            .await
            .is_err(),
        "unknown hit discriminator refuses"
    );
    let short = runtime
        .records::<SearchResult>(present(frame.clone(), 3)?, 1)
        .await?
        .remove(0);
    let full = runtime
        .records::<SearchResult>(present(frame, 20)?, 1)
        .await?
        .remove(0);
    assert_eq!(short.hit.excerpt, "λB…");
    assert_eq!(full.hit.excerpt, "λBCDEF");
    assert_eq!(short.hit.excerpt, short.citation.excerpt);
    assert_eq!(short.hit.evidence_id, full.hit.evidence_id);
    let expected = Evidence::new(
        input.fact_id,
        input.subject_ref,
        input.label,
        input.source,
        "λB…".into(),
    )?;
    assert_eq!(short.citation, expected);
    assert_eq!(short.key.score, short.hit.score);
    assert_eq!(
        short.hit.score,
        short
            .hit
            .factors
            .iter()
            .map(|factor| factor.points)
            .sum::<u32>()
    );
    assert_eq!(
        scoring::tokens(&runtime, "Type name").await?,
        vec!["type", "name"]
    );
    let too_many = (0..65)
        .map(|n| format!("term{n}"))
        .collect::<Vec<_>>()
        .join(" ");
    let error = scoring::tokens(&runtime, &too_many).await.unwrap_err();
    assert_eq!(
        crate::query::QueryError::from(error).diagnostic().cause,
        enrichment_core::wire::DiagnosticCause::InvalidInput
    );
    Ok(())
}

#[tokio::test]
async fn search_consumer_folds_aliases_counts_and_pages_from_native_materialization() -> Result<()>
{
    use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
    use enrichment_core::evidence::{arrow_model::expressions::record, relational::ApiPayload};
    use std::{collections::BTreeMap, sync::Arc};
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(
        root.path(),
        crate::runtime::QueryLimits {
            concurrency: 1,
            ..Default::default()
        },
    )?;
    let mut rows = Vec::new();
    for (ordinal, name) in [(1, "TypeA"), (2, "TypeB"), (3, "TypeC")] {
        let mut input = fixture();
        input.candidate_id = format!("symbol{ordinal}");
        input.fact_id = format!("observation{ordinal}");
        input.label = format!("λ::{name}");
        input.path = Some(input.label.clone());
        input.subject_ref = SubjectRef::Definition {
            definition_id: format!("def_{}", format!("{ordinal}").repeat(64)),
        };
        rows.push(input);
    }
    let mut alias = rows[0].clone();
    alias.label = "λ::alias::TypeA".into();
    alias.path = Some(alias.label.clone());
    alias.candidate_id = "alias".into();
    rows.push(alias);
    let source = runtime.session();
    let captured = Input::batch(&rows)?;
    // The real evidence providers expose the declared Delta nullable child layout.
    let captured = enrichment_core::evidence::arrow_model::cells::batch(
        "search_fixture",
        captured
            .schema()
            .fields()
            .iter()
            .zip(captured.columns())
            .map(|(field, array)| (field.as_ref().clone(), array.clone()))
            .collect(),
    )?;
    crate::native_catalog::input(&source, "captured_search", captured)?;
    let captured = source.table("captured_search").await?;
    let api=source.sql("SELECT candidate_id AS symbol_id,subject_ref.definition.definition_id AS definition_id,path,regexp_replace(path,'^.*::','') AS name,symbol_kind AS kind,symbol_kind AS declared_kind,signature,excerpt AS doc_summary,excerpt AS docs,fact_id AS observation_id,candidate_id='alias' AS is_reexport,deprecated AS is_deprecated FROM captured_search").await?;
    let fragments=source.sql("SELECT candidate_id AS fragment_id,fragment_kind AS kind,label,excerpt AS text,subject_ref.definition.definition_id AS definition_id,source,subject_ref FROM captured_search WHERE false").await?;
    let payload = record(
        &ApiPayload::data_type(),
        &[
            ("declared_kind", lit("struct")),
            ("signature", col("signature")),
            ("doc_summary", col("excerpt")),
            ("docs", col("excerpt")),
            (
                "cfg_hints",
                enrichment_core::evidence::arrow_model::expressions::literal(&Vec::<String>::new())?,
            ),
        ],
    )?;
    let evidence = crate::native_rows::distinct(
        captured.select(vec![
            col("fact_id").alias("observation_id"),
            payload.alias("payload"),
            col("excerpt").alias("docs"),
            col("source"),
            col("subject_ref").alias("subject"),
        ])?,
        "search-fixture-observation/1",
    )?;
    let raw_fragments = fragments
        .clone()
        .with_column("subject", col("subject_ref"))?;
    let catalog = BoundCatalog::default()
        .with_schema(
            BindingKind::AdmittedDomain,
            Tables::from([
                ("api_surface".into(), api.into_view()),
                ("fragment_surface".into(), fragments.into_view()),
            ]),
        )
        .with_schema(
            BindingKind::AdmittedEvidence,
            Tables::from([
                ("api_observations".into(), evidence.into_view()),
                ("fragments".into(), raw_fragments.into_view()),
            ]),
        );
    let catalogs = BTreeMap::from([(
        "snapshot".into(),
        Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
    )]);
    let session = runtime.bound_session(catalogs.clone())?;
    runtime
        .operation(
            "search-consumer-unit".into(),
            enrichment_core::telemetry::OperationDescriptor {
                method: "unit.search".into(),
                request_digest: "fixture".into(),
                policy_digest: "fixture".into(),
            },
            async {
                let mut options = SearchOptions {
                    include_api: true,
                    fragment_kinds: vec![],
                    area: None,
                    page_size: 2,
                    offset: 0,
                    after: None,
                };
                let spec = SearchSpec::new("Type");
                let first = page(&session, &runtime, &spec, &options, 20).await?;
                assert_eq!(first.total, 3);
                assert_eq!(first.rows.len(), 2);
                assert!(first.boundary.has_more);
                assert_eq!(first.rows[0].hit.path.as_deref(), Some("λ::TypeA"));
                assert_eq!(first.rows[0].hit.also_at, vec!["λ::alias::TypeA"]);
                options.after = Some(first.rows[1].key.clone());
                options.offset = 2;
                let next = runtime.bound_session(catalogs.clone())?;
                let last = page(&next, &runtime, &spec, &options, 20).await?;
                assert_eq!(last.total, 3);
                assert_eq!(last.rows.len(), 1);
                assert!(!last.boundary.has_more);
                assert_eq!(last.rows[0].hit.path.as_deref(), Some("λ::TypeC"));
                options.after = None;
                options.offset = 0;
                let missing = runtime.bound_session(catalogs.clone())?;
                let absent = page(
                    &missing,
                    &runtime,
                    &SearchSpec::new("unmatched"),
                    &options,
                    20,
                )
                .await?;
                assert_eq!(absent.total, 0);
                assert!(absent.rows.is_empty());
                Ok::<(), DataFusionError>(())
            },
        )
        .await??;
    drop(session);
    runtime.close_diagnostics().await
}
