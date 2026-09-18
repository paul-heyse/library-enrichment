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
        release_id: format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        environment_id: format!("env_{}", "a".repeat(64)).try_into().unwrap(),
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
    assert!(
        error.contains("declared reference") && error.contains("definition_id"),
        "{error}"
    );
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
