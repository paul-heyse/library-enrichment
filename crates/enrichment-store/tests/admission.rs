use std::path::Path;
use std::sync::Arc;

use arrow::record_batch::RecordBatch;
use enrichment_core::evidence::{
    SymbolHeader, SymbolKind,
    path::PublicPath,
    relational::{Definition, PublicBinding},
};
use enrichment_core::identity::Ecosystem;
use enrichment_store::{
    admission::{AdmissionLimits, EvidenceScope, NativeAdmission, Relation},
    projection,
    runtime::{QueryLimits, QueryRuntime},
};

fn definitions() -> Vec<Definition> {
    vec![Definition {
        definition_id: SymbolHeader::definition_id_for(
            "p",
            "p::inner::f",
            SymbolKind::Function,
            None,
        ),
        kind: SymbolKind::Function,
        definition_path: "p::inner::f".into(),
        defined_in_package: "p".into(),
        qualifier: None,
    }]
}

fn symbols() -> Vec<PublicBinding> {
    ["p::f", "p::inner::f"]
        .into_iter()
        .enumerate()
        .map(|(i, path)| PublicBinding {
            symbol_id: PublicBinding::id_for(
                "p",
                &PublicPath::parse(Ecosystem::Rust, path).expect("path"),
                SymbolKind::Function,
                None,
            ),
            definition_id: SymbolHeader::definition_id_for(
                "p",
                "p::inner::f",
                SymbolKind::Function,
                None,
            ),
            path: PublicPath::parse(Ecosystem::Rust, path).expect("path"),
            name: "f".into(),
            is_reexport: i == 0,
            qualifier: None,
        })
        .collect()
}

fn providers(
    definitions: &[Definition],
    symbols: &[PublicBinding],
) -> std::collections::BTreeMap<Relation, Arc<dyn datafusion::catalog::TableProvider>> {
    Relation::ALL
        .into_iter()
        .map(|relation| {
            let batch = match relation {
                Relation::Definitions => projection::definitions(definitions).unwrap(),
                Relation::Symbols => projection::bindings(symbols).unwrap(),
                _ => RecordBatch::new_empty(relation.schema().unwrap()),
            };
            (
                relation,
                Arc::new(
                    datafusion::datasource::MemTable::try_new(batch.schema(), vec![vec![batch]])
                        .unwrap(),
                ) as Arc<dyn datafusion::catalog::TableProvider>,
            )
        })
        .collect()
}

fn scope() -> EvidenceScope {
    EvidenceScope {
        ecosystem: Ecosystem::Rust,
        symbol_package: "p".into(),
        release_id: "rel_fixture".into(),
        environment_id: "env_fixture".into(),
    }
}

fn runtime(root: &Path) -> QueryRuntime {
    QueryRuntime::new(&root.join("spill"), QueryLimits::default()).expect("runtime")
}

#[tokio::test]
async fn native_provider_admission_rechecks_hashes_before_publishing_constraints() {
    use datafusion::{catalog::TableProvider, datasource::MemTable};
    use std::collections::BTreeMap;
    let dir = tempfile::tempdir().unwrap();
    let cache = NativeAdmission::new(runtime(dir.path()), AdmissionLimits::default()).unwrap();
    for corrupt in [false, true] {
        let mut providers = BTreeMap::<Relation, Arc<dyn TableProvider>>::new();
        for relation in Relation::ALL {
            let mut batch = match relation {
                Relation::Definitions => projection::definitions(&definitions()).unwrap(),
                Relation::Symbols => projection::bindings(&symbols()).unwrap(),
                _ => RecordBatch::new_empty(relation.schema().unwrap()),
            };
            if corrupt && relation == Relation::Symbols {
                let mut columns = batch.columns().to_vec();
                columns[batch.schema().index_of("symbol_id").unwrap()] =
                    Arc::new(arrow::array::StringArray::from(vec![
                        "forged_a", "forged_b",
                    ]));
                batch = RecordBatch::try_new(batch.schema(), columns).unwrap();
            }
            providers.insert(
                relation,
                Arc::new(MemTable::try_new(batch.schema(), vec![vec![batch]]).unwrap()),
            );
        }
        let admitted = cache.admit_native(&scope(), providers).await;
        if corrupt {
            let error = admitted.err().expect("forged native IDs must fail");
            assert!(
                error.to_string().contains("native public binding identity"),
                "{error}"
            );
        } else {
            admitted.expect("valid native provider contracts");
        }
    }
}

#[tokio::test]
async fn valid_aliases_share_a_definition_and_admission_exposes_proven_constraints() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let cache = NativeAdmission::new(runtime.clone(), AdmissionLimits::default()).expect("cache");
    let first = cache
        .admit_native(&scope(), providers(&definitions(), &symbols()))
        .await
        .expect("native admission");
    let session = first.session(&runtime, None).expect("bound catalog");
    let result = runtime
        .execute(
            session
                .sql("SELECT symbol_id FROM snapshot.evidence.symbols WHERE name = 'f' ORDER BY symbol_id LIMIT 1")
                .await
                .expect("plan"),
        )
        .await
        .expect("execute");
    assert_eq!(result.rows, 1);
    assert!(
        session
            .table_provider("snapshot.evidence.symbols")
            .await
            .expect("table")
            .constraints()
            .is_some_and(|c| !c.is_empty())
    );
    let other = runtime.session();
    assert!(other.catalog("snapshot").is_none());
}

#[tokio::test]
async fn duplicate_keys_and_conditional_foreign_keys_are_rejected_before_constraints() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let cache = NativeAdmission::new(runtime, AdmissionLimits::default()).expect("cache");
    let mut duplicate = symbols();
    duplicate.push(duplicate[0].clone());
    let input = providers(&definitions(), &duplicate);
    let error = cache
        .admit_native(&scope(), input)
        .await
        .err()
        .expect("reject duplicate")
        .to_string();
    assert!(error.contains("duplicate relation key"), "{error}");
    let mut dangling = symbols();
    dangling[0].definition_id = "def_missing".into();
    let input = providers(&definitions(), &dangling);
    let error = cache
        .admit_native(&scope(), input)
        .await
        .err()
        .expect("reject dangling")
        .to_string();
    assert!(error.contains("symbol_definition"), "{error}");
}

#[tokio::test]
async fn native_inventory_and_row_budgets_fail_before_constraints() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = runtime(dir.path());
    let admission = NativeAdmission::new(runtime.clone(), AdmissionLimits::default()).unwrap();
    let mut incomplete = providers(&definitions(), &symbols());
    incomplete.remove(&Relation::Coverage);
    assert!(
        admission
            .admit_native(&scope(), incomplete)
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("incomplete")
    );
    let admission = NativeAdmission::new(
        runtime.clone(),
        AdmissionLimits {
            table_rows: 1,
            ..AdmissionLimits::default()
        },
    )
    .unwrap();
    assert!(
        admission
            .admit_native(&scope(), providers(&definitions(), &symbols()))
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("row budget")
    );
    let admission = NativeAdmission::new(
        runtime,
        AdmissionLimits {
            record_bytes: 20,
            ..AdmissionLimits::default()
        },
    )
    .unwrap();
    assert!(
        admission
            .admit_native(&scope(), providers(&definitions(), &symbols()))
            .await
            .err()
            .unwrap()
            .to_string()
            .contains("record byte budget")
    );
}

#[tokio::test]
async fn resource_exhaustion_is_an_error_and_never_a_successful_empty_result() {
    let dir = tempfile::tempdir().expect("directory");
    let limits = QueryLimits {
        result_rows: 1,
        ..QueryLimits::default()
    };
    let runtime = QueryRuntime::new(&dir.path().join("spill"), limits).expect("runtime");
    let session = runtime.session();
    let result = runtime
        .execute(
            session
                .sql("SELECT * FROM (VALUES (1), (2)) AS t(n)")
                .await
                .expect("plan"),
        )
        .await;
    assert!(
        result
            .expect_err("bound exceeded")
            .to_string()
            .contains("budget")
    );
    assert_eq!(
        runtime
            .execute(session.sql("SELECT 1 WHERE false").await.expect("plan"))
            .await
            .expect("valid empty")
            .rows,
        0
    );
}

#[tokio::test]
async fn native_field_contracts_reject_bad_coordinates_without_decoding_domain_rows() {
    use enrichment_core::evidence::arrow_model::{cells, checks, encode};
    use enrichment_core::{
        evidence::relational::{FactSource, Locator},
        wire::{EvidenceClass, SourceVersionMatch},
    };
    let root = tempfile::tempdir().unwrap();
    let runtime = runtime(root.path());
    let cases = vec![
        (
            "valid_lines",
            Locator::Lines {
                file: Some("pkg/β.rs".into()),
                start: 1,
                end: 2,
            },
        ),
        (
            "zero",
            Locator::Lines {
                file: None,
                start: 0,
                end: 2,
            },
        ),
        ("reverse", Locator::Bytes { start: 2, end: 1 }),
        ("empty_bytes", Locator::Bytes { start: 0, end: 0 }),
        (
            "escape",
            Locator::ArchiveMember {
                path: "pkg/../file.rs".into(),
            },
        ),
        (
            "absolute",
            Locator::ArchiveMember {
                path: "/file.rs".into(),
            },
        ),
        (
            "backslash",
            Locator::ArchiveMember {
                path: "pkg\\file.rs".into(),
            },
        ),
        (
            "control",
            Locator::RustdocItem {
                item: 1,
                reported_file: Some("pkg\nfile".into()),
                reported_line: None,
            },
        ),
        (
            "valid_reported",
            Locator::RustdocItem {
                item: 1,
                reported_file: Some("/upstream/build/file.rs".into()),
                reported_line: None,
            },
        ),
        (
            "wrong_python_origin",
            Locator::PythonDeclaration {
                file: "pkg.py".into(),
                declaration: "pkg".into(),
                line: None,
                origin: enrichment_core::evidence::relational::ApiOrigin::Rustdoc,
                overload: None,
            },
        ),
    ];
    let sources = cases
        .iter()
        .map(|(_, locator)| FactSource {
            extractor: "fixture".into(),
            extractor_version: "1".into(),
            producer_binding_id: "producer_fixture".into(),
            artifact_id: "art_fixture".into(),
            source_uri: None,
            source_version_match: SourceVersionMatch::Exact,
            evidence_class: EvidenceClass::Declared,
            locator: locator.clone(),
        })
        .collect::<Vec<_>>();
    let batch = cells::batch(
        "coordinate_probe",
        vec![
            cells::column(
                "id",
                cells::text(cases.iter().map(|(id, _)| *id)),
                false,
                "key:probe",
            ),
            cells::column(
                "source",
                encode::source(&sources.iter().collect::<Vec<_>>()).unwrap(),
                false,
                "fact-source",
            ),
        ],
    )
    .unwrap();
    let schema = batch.schema();
    let frame = runtime.session().read_batch(batch).unwrap();
    let mut observed = std::collections::BTreeSet::new();
    for (_, plan) in checks::violations(frame, &schema, "id").unwrap() {
        for batch in runtime.execute(plan).await.unwrap().batches {
            let text = cells::TextColumn::new(batch.column(0).as_ref()).unwrap();
            for row in 0..batch.num_rows() {
                observed.insert(text.required(row).unwrap().to_owned());
            }
        }
    }
    assert_eq!(
        observed,
        [
            "zero",
            "reverse",
            "escape",
            "absolute",
            "backslash",
            "control",
            "wrong_python_origin"
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
}

#[tokio::test]
async fn nullable_nested_read_layout_still_enforces_required_semantic_fields() {
    use arrow::array::{Array, StringArray, StructArray};
    use enrichment_core::evidence::arrow_model::{cells, checks};
    let root = tempfile::tempdir().unwrap();
    let runtime = runtime(root.path());
    let batch = cells::batch(
        "presence_probe",
        vec![
            cells::column("id", cells::text(["valid", "missing"]), false, "key:probe"),
            cells::column(
                "source",
                cells::structure(
                    vec![cells::column(
                        "extractor",
                        cells::text(["extractor", "extractor"]),
                        false,
                        "extractor-name",
                    )],
                    None,
                )
                .unwrap(),
                false,
                "source",
            ),
        ],
    )
    .unwrap();
    let values = batch
        .column(1)
        .as_any()
        .downcast_ref::<StructArray>()
        .unwrap();
    assert!(values.fields()[0].is_nullable());
    assert!(checks::required(&values.fields()[0]));
    let corrupted = Arc::new(
        StructArray::try_new(
            values.fields().clone(),
            vec![Arc::new(StringArray::from(vec![Some("extractor"), None]))],
            values.nulls().cloned(),
        )
        .unwrap(),
    );
    let batch =
        RecordBatch::try_new(batch.schema(), vec![batch.column(0).clone(), corrupted]).unwrap();
    let schema = batch.schema();
    let violations =
        checks::violations(runtime.session().read_batch(batch).unwrap(), &schema, "id").unwrap();
    assert_eq!(violations.len(), 1);
    let output = runtime.execute(violations[0].1.clone()).await.unwrap();
    assert_eq!(output.rows, 1);
    assert_eq!(
        cells::TextColumn::new(output.batches[0].column(0).as_ref())
            .unwrap()
            .required(0)
            .unwrap(),
        "missing"
    );
}
