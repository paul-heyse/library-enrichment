//! Pure Arrow execution only: no repository, Delta table, publication or artifact I/O.
use super::*;
use datafusion::functions::core::expr_ext::FieldAccessor;
use enrichment_core::{
    compare::{ApiComparisonObservation, ComparisonValue},
    evidence::{
        SymbolKind,
        declarations::{RustAbi, RustCallable, RustDetails, RustParameter},
        relational::{ApiOrigin, ApiPayload, FactSource},
    },
};
enrichment_core::native_struct! { struct ApiInput {
    key:String=>Rule::Text,
    label:String=>Rule::Text,
    kind:SymbolKind=>Rule::Text,
    qualifier:Option<String> =>Rule::Text,
    definition_path:String=>Rule::Text,
    defined_in_package:String=>Rule::Text,
    is_reexport:bool=>Rule::Text,
    observation_id:Option<String> =>Rule::Text,
    origin:ApiOrigin=>Rule::Text,
    payload:ApiPayload=>Rule::Text,
    source:Option<FactSource> =>Rule::Text,
} }
enrichment_core::native_struct! { struct Value {value:ComparisonValue=>Rule::Text} }
enrichment_core::native_struct! { struct Count {count:i64=>Rule::Text} }

fn api_fixture() -> ApiInput {
    let payload = ApiPayload {
        declared_kind: SymbolKind::Function,
        signature: Some("fn f(first: u32, second: u64)".into()),
        doc_summary: Some("Documentation has a separate axis".into()),
        docs: Some("Full documentation has a separate axis".into()),
        deprecated: None,
        cfg_hints: vec!["feature=\"two\"".into(), "feature=\"one\"".into()],
        python: None,
        rust: Some(RustDetails {
            stability: None,
            const_stability: None,
            body: None,
            callable: Some(RustCallable {
                parameters: vec![
                    RustParameter {
                        ordinal: 0,
                        pattern: "first".into(),
                        type_rendering: "u32".into(),
                    },
                    RustParameter {
                        ordinal: 1,
                        pattern: "second".into(),
                        type_rendering: "u64".into(),
                    },
                ],
                output: None,
                c_variadic: false,
                is_const: false,
                is_unsafe: false,
                is_async: false,
                abi: RustAbi::C { unwind: false },
                generics: "<>".into(),
            }),
        }),
    };
    ApiInput {
        key: "path_f".into(),
        label: "crate::f".into(),
        kind: SymbolKind::Function,
        qualifier: Some("trait::T".into()),
        definition_path: "crate::f".into(),
        defined_in_package: "crate".into(),
        is_reexport: false,
        observation_id: Some("observation".into()),
        origin: ApiOrigin::Rustdoc,
        payload,
        source: None,
    }
}

#[tokio::test]
async fn typed_comparison_preserves_absence_order_metadata_and_set_equality() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
    let session = runtime.session();
    let mut input = api_fixture();
    let mut changed_docs = input.clone();
    changed_docs.payload.docs = Some("Different docs".into());
    changed_docs.payload.doc_summary = None;
    let mut absent = input.clone();
    absent.observation_id = None;
    let frame = typed_values(
        crate::native_catalog::batch(
            &session,
            "typed_tests",
            ApiInput::batch(&[input.clone(), changed_docs, absent])?,
        )?,
        0,
    )?;
    crate::native_catalog::work(&session, "typed_before", frame.clone().into_view())?;
    let rows = runtime
        .records::<Value>(frame.clone().select(vec![col("value")])?, 3)
        .await?;
    let mut expected_payload = input.payload.clone();
    expected_payload.docs = None;
    expected_payload.doc_summary = None;
    assert_eq!(
        rows[0].value,
        ComparisonValue::Api {
            kind: input.kind,
            qualifier: input.qualifier.clone(),
            definition_path: input.definition_path.clone(),
            defined_in_package: input.defined_in_package.clone(),
            is_reexport: input.is_reexport,
            observation: Some(Box::new(ApiComparisonObservation {
                origin: input.origin,
                payload: expected_payload,
            })),
        }
    );
    assert_eq!(rows[0], rows[1], "documentation is compared independently");
    assert!(matches!(
        rows[2].value,
        ComparisonValue::Api {
            observation: None,
            ..
        }
    ));
    let count = runtime.records::<Count>(session.sql("SELECT count(*) AS count FROM (SELECT value FROM typed_before EXCEPT DISTINCT SELECT value FROM typed_before)").await?,1).await?;
    assert_eq!(count[0].count, 0);
    let count = runtime
        .records::<Count>(
            session
                .sql("SELECT count(*) AS count FROM (SELECT DISTINCT value FROM typed_before)")
                .await?,
            1,
        )
        .await?;
    assert_eq!(
        count[0].count, 2,
        "null observation is not a missing binding"
    );
    input
        .payload
        .rust
        .as_mut()
        .unwrap()
        .callable
        .as_mut()
        .unwrap()
        .parameters
        .swap(0, 1);
    input
        .payload
        .rust
        .as_mut()
        .unwrap()
        .callable
        .as_mut()
        .unwrap()
        .parameters[1]
        .type_rendering = "u16".into();
    let changed = typed_values(
        crate::native_catalog::batch(&session, "typed_tests", ApiInput::batch(&[input])?)?,
        0,
    )?;
    crate::native_catalog::work(&session, "typed_after", changed.into_view())?;
    let count=runtime.records::<Count>(session.sql("SELECT count(*) AS count FROM (SELECT value FROM typed_after EXCEPT DISTINCT SELECT value FROM typed_before)").await?,1).await?;
    assert_eq!(count[0].count, 1, "parameter sequence remains meaningful");
    let changed_fields = fields::select(
        &runtime,
        session.table("typed_before").await?,
        session.table("typed_after").await?,
    )
    .await?;
    assert!(
        changed_fields.iter().any(|path| path.steps.is_empty()),
        "complete alternative difference"
    );
    assert!(changed_fields.iter().any(|path| matches!(path.steps.last(),Some(compare::ComparisonPathStep::Field {name}) if name=="parameters")),"whole callable parameter order differs");
    assert!(
        changed_fields.iter().any(|path| path
            .steps
            .iter()
            .any(|step| matches!(step, compare::ComparisonPathStep::Item { ordinal: 0 }))),
        "declared parameter ordinal appears in a typed path"
    );
    // Complete child fields survive native execution and the final formatting boundary.
    let output = runtime.execute(frame.select(vec![col("value")])?).await?;
    let mut expected = rows.iter();
    for batch in output.batches {
        let field = batch.schema().field_with_name("value")?.clone();
        assert_eq!(field.data_type(), &ComparisonValue::data_type());
        for row in 0..batch.num_rows() {
            let mut encoded = Vec::new();
            enrichment_core::native_json::write_value(
                &mut encoded,
                1 << 20,
                &Arc::new(field.clone()),
                batch.column(0).as_ref(),
                row,
            )?;
            let actual: ComparisonValue = serde_json::from_slice(&encoded).unwrap();
            assert_eq!(
                actual,
                expected.next().expect("one output per comparison").value
            );
        }
    }
    assert!(expected.next().is_none());
    Ok(())
}

#[tokio::test]
async fn typed_comparison_all_axes_use_one_closed_value_contract() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
    let session = runtime.session();
    let cases = [
        (
            1,
            "fragment",
            ComparisonValue::Fragment {
                kind: enrichment_core::evidence::FragmentKind::DocText,
                text: "é\ntext".into(),
                evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
            },
        ),
        (
            2,
            "fragment",
            ComparisonValue::Fragment {
                kind: enrichment_core::evidence::FragmentKind::FeatureDefinition,
                text: "feature".into(),
                evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
            },
        ),
        (
            3,
            "rust_documentation",
            ComparisonValue::RustDocumentation {
                configuration: Default::default(),
            },
        ),
        (
            4,
            "python_header",
            ComparisonValue::PythonHeader {
                values: vec!["duplicate".into(), "duplicate".into()],
            },
        ),
        (
            5,
            "fragment",
            ComparisonValue::Fragment {
                kind: enrichment_core::evidence::FragmentKind::ChangelogSection,
                text: "release".into(),
                evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
            },
        ),
        (
            6,
            "fragment",
            ComparisonValue::Fragment {
                kind: enrichment_core::evidence::FragmentKind::Example,
                text: "example".into(),
                evidence_class: enrichment_core::wire::EvidenceClass::StaticallyExtracted,
            },
        ),
        (
            7,
            "relationship",
            ComparisonValue::Relationship {
                relation: enrichment_core::evidence::RelationKind::Reexports,
                qualifier: Some("q".into()),
                target_kind: "unresolved".into(),
                target_symbol_id: None,
                target_definition_id: None,
                target_package: None,
                target_path: Some("module::path".into()),
            },
        ),
    ];
    for (id, tag, value) in cases {
        let frame = crate::native_catalog::batch(
            &session,
            "typed_tests",
            Value::batch(&[Value {
                value: value.clone(),
            }])?,
        )?;
        let arrow::datatypes::DataType::Struct(fields) =
            enrichment_core::evidence::arrow_model::expressions::child(
                &ComparisonValue::data_type(),
                tag,
            )?
        else {
            unreachable!()
        };
        let mut fields = fields
            .iter()
            .map(|field| {
                let name = match id {
                    3 | 4 => "value",
                    _ => field.name(),
                };
                col("value").field(tag).field(field.name()).alias(name)
            })
            .collect::<Vec<_>>();
        fields.extend([
            lit("key").alias("key"),
            lit("label").alias("label"),
            enrichment_core::evidence::arrow_model::expressions::literal::<Option<FactSource>>(
                &None,
            )?
            .alias("source"),
        ]);
        let frame = typed_values(frame.select(fields)?, id)?.select(vec![col("value")])?;
        let result = runtime.records::<Value>(frame, 1).await?;
        assert_eq!(result, vec![Value { value }], "axis {id}");
    }
    Ok(())
}

#[tokio::test]
async fn comparison_consumer_reuses_complete_typed_changed_keys_for_count_and_page() -> Result<()> {
    use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
    let root = tempfile::tempdir()?;
    let runtime = crate::runtime::QueryRuntime::new(
        root.path(),
        crate::runtime::QueryLimits {
            concurrency: 1,
            ..Default::default()
        },
    )?;
    let same = api_fixture();
    let mut removed = same.clone();
    removed.key = "removed".into();
    removed.label = "crate::removed".into();
    let mut changed = same.clone();
    changed.key = "changed".into();
    changed.label = "crate::changed".into();
    let mut updated = changed.clone();
    updated
        .payload
        .rust
        .as_mut()
        .unwrap()
        .callable
        .as_mut()
        .unwrap()
        .parameters[1]
        .type_rendering = "u128".into();
    let mut documentation = same.clone();
    documentation.payload.docs = Some("Documentation alone is outside API equality".into());
    let mut added = same.clone();
    added.key = "added".into();
    added.label = "crate::added".into();
    let before = vec![same.clone(), same.clone(), removed, changed];
    let after = vec![documentation, added, updated];
    let mut catalogs = BTreeMap::new();
    for (name, values) in [("before", before), ("after", after)] {
        let input = runtime.session();
        crate::native_catalog::input(&input, "captured_api", ApiInput::batch(&values)?)?;
        let frame = input
            .table("captured_api")
            .await?
            .with_column("path_id", col("key"))?
            .with_column("path", col("label"))?;
        let tables = Tables::from([("api_surface".into(), frame.into_view())]);
        catalogs.insert(
            name.into(),
            Arc::new(BoundCatalog::default().with_schema(BindingKind::AdmittedDomain, tables))
                as Arc<dyn datafusion::catalog::CatalogProvider>,
        );
    }
    let session = runtime.bound_session(catalogs)?;
    runtime
        .operation(
            "comparison-consumer-unit".into(),
            enrichment_core::telemetry::OperationDescriptor {
                method: "unit.comparison".into(),
                request_digest: "fixture".into(),
                policy_digest: "fixture".into(),
            },
            async {
                let index = key_index(&runtime, &session, &axes(&[Scope::Api])).await?;
                let physical = index.clone().create_physical_plan().await?;
                let narrowed = runtime
                    .execute(
                        index
                            .clone()
                            .filter(col("key").eq(lit("changed")))?
                            .select_columns(&["key"])?,
                    )
                    .await?;
                assert_eq!(narrowed.rows, 1);
                assert_eq!(
                    crate::operation_index::count(&runtime, index.clone()).await?,
                    3
                );
                let page = crate::page_plan::select(
                    &runtime,
                    index.clone().sort(vec![
                        col("label").sort(true, false),
                        col("key").sort(true, false),
                    ])?,
                    crate::page_plan::Policy {
                        page_size: 2,
                        offset: 0,
                        total: Some(3),
                        detail: false,
                    },
                )
                .await?;
                assert_eq!(page.boundary.returned, 2);
                assert!(page.boundary.has_more);
                let output = runtime.execute(page.frame).await?;
                let keys = crate::projection::comparison::keys(&output.batches)?;
                assert_eq!(
                    keys.iter().map(|key| key.key.as_str()).collect::<Vec<_>>(),
                    ["added", "changed"]
                );
                assert_eq!(
                    crate::operation_index::count(&runtime, index.clone()).await?,
                    3
                );
                assert_eq!(
                    physical
                        .metrics()
                        .unwrap()
                        .sum_by_name("cache_fills")
                        .unwrap()
                        .as_usize(),
                    1
                );
                Ok::<(), DataFusionError>(())
            },
        )
        .await??;
    drop(session);
    runtime.close_diagnostics().await
}
