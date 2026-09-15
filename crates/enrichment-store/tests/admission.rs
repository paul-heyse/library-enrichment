#[path = "support/write_parquet.rs"]
mod parquet_fixture;
use std::path::Path;
use std::sync::Arc;

use arrow::record_batch::RecordBatch;
use enrichment_core::canonical;
use enrichment_core::evidence::{
    Symbol, SymbolKind,
    path::PublicPath,
    relational::{Definition, PublicBinding},
};
use enrichment_core::identity::Ecosystem;
use enrichment_store::{
    admission::{AdmissionCache, AdmissionLimits, EvidenceFile, EvidenceScope, Relation},
    projection,
    runtime::{QueryLimits, QueryRuntime},
};

fn definitions() -> Vec<Definition> {
    vec![Definition {
        definition_id: Symbol::definition_id_for("p", "p::inner::f", SymbolKind::Function, None),
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
            definition_id: Symbol::definition_id_for(
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

fn files(root: &Path, definitions: &[Definition], symbols: &[PublicBinding]) -> Vec<EvidenceFile> {
    [
        (
            Relation::Definitions,
            projection::definitions(definitions).expect("definitions"),
        ),
        (
            Relation::Symbols,
            projection::bindings(symbols).expect("symbols"),
        ),
        (
            Relation::ApiObservations,
            projection::observations(&[]).expect("observations"),
        ),
        (
            Relation::ExecutionObservations,
            projection::execution::encode(&[]).expect("execution"),
        ),
        (
            Relation::Relationships,
            projection::relationships(&[]).expect("relationships"),
        ),
        (
            Relation::Fragments,
            projection::fragments(&[]).expect("fragments"),
        ),
        (
            Relation::ProducerRuns,
            projection::producer_runs(&[]).expect("runs"),
        ),
        (
            Relation::InputArtifacts,
            projection::input_artifacts(&[]).expect("inputs"),
        ),
        (
            Relation::Coverage,
            projection::coverage(&[]).expect("coverage"),
        ),
        (
            Relation::ReleaseMetadata,
            projection::metadata::encode(&[]).expect("metadata"),
        ),
    ]
    .into_iter()
    .map(|(relation, batch)| file(root, relation, &batch))
    .collect()
}

fn file(root: &Path, relation: Relation, batch: &RecordBatch) -> EvidenceFile {
    let path = root.join(format!("{}.parquet", relation.name()));
    parquet_fixture::write_parquet(&path, batch, &[]).expect("write file");
    let (sha256, bytes) =
        canonical::sha256_reader(std::fs::File::open(&path).expect("open"), 1_000_000)
            .expect("digest");
    EvidenceFile {
        relation,
        path,
        sha256,
        bytes,
        rows: batch.num_rows() as u64,
    }
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
async fn valid_aliases_share_a_definition_and_warm_admission_reuses_the_binding() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let cache = AdmissionCache::new(runtime.clone(), AdmissionLimits::default()).expect("cache");
    let files = files(dir.path(), &definitions(), &symbols());
    let first = cache
        .admit("manifest-a", &scope(), &files)
        .await
        .expect("admission");
    let second = cache
        .admit("manifest-a", &scope(), &files)
        .await
        .expect("warm admission");
    assert!(Arc::ptr_eq(&first, &second));
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
    let cache = AdmissionCache::new(runtime, AdmissionLimits::default()).expect("cache");
    let mut duplicate = symbols();
    duplicate.push(duplicate[0].clone());
    let input = files(dir.path(), &definitions(), &duplicate);
    let error = cache
        .admit("duplicate", &scope(), &input)
        .await
        .err()
        .expect("reject duplicate")
        .to_string();
    assert!(error.contains("duplicate relation key"), "{error}");
    let mut dangling = symbols();
    dangling[0].definition_id = "def_missing".into();
    let input = files(dir.path(), &definitions(), &dangling);
    let error = cache
        .admit("dangling", &scope(), &input)
        .await
        .err()
        .expect("reject dangling")
        .to_string();
    assert!(error.contains("symbol_definition"), "{error}");
}

#[tokio::test]
async fn missing_or_mutated_exact_files_fail_closed_on_warm_and_cold_opens() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let cache = AdmissionCache::new(runtime.clone(), AdmissionLimits::default()).expect("cache");
    let input = files(dir.path(), &definitions(), &symbols());
    let admitted = cache
        .admit("manifest", &scope(), &input)
        .await
        .expect("admission");
    let session = admitted.session(&runtime, None).expect("bound catalog");
    let missing = &input[1].path;
    std::fs::rename(missing, missing.with_extension("parquet.extra"))
        .expect("replace by misleading prefix");
    assert!(cache.admit("manifest", &scope(), &input).await.is_err());
    assert!(
        runtime
            .execute(
                session
                    .sql("SELECT * FROM snapshot.evidence.symbols")
                    .await
                    .expect("plan")
            )
            .await
            .is_err()
    );
    assert!(cache.admit("cold", &scope(), &input).await.is_err());
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
async fn physical_counts_eliminate_scans_without_losing_admission_or_retention() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = runtime(dir.path());
    let inputs = files(dir.path(), &definitions(), &symbols());
    let cache = AdmissionCache::new(runtime.clone(), AdmissionLimits::default()).unwrap();
    let admitted = cache.admit("counts", &scope(), &inputs).await.unwrap();
    enrichment_store::leases::initialize(dir.path()).unwrap();
    let lease = enrichment_store::leases::shared(dir.path()).unwrap();
    let session = admitted.session(&runtime, Some(lease.clone())).unwrap();
    let metadata = runtime
        .execute(
            session
                .sql("SELECT verified_rows FROM operation.metadata.relations WHERE catalog_name = 'snapshot' AND schema_name = 'evidence' AND table_name = 'symbols'")
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(metadata.rows, 1);
    assert_eq!(
        metadata.batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::UInt64Array>()
            .unwrap()
            .value(0),
        2
    );
    let plan = session
        .sql("SELECT count(*) FROM snapshot.evidence.symbols")
        .await
        .unwrap();
    let result = runtime.execute(plan).await.unwrap();
    let diagnostic = runtime.diagnostics().pop().unwrap();
    assert!(
        diagnostic.physical.contains("PlaceholderRowExec"),
        "{}",
        diagnostic.physical
    );
    assert!(
        !diagnostic.physical.contains("DataSourceExec"),
        "{}",
        diagnostic.physical
    );
    assert_eq!(
        result.batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .value(0),
        2
    );
    assert!(enrichment_store::leases::exclusive(dir.path()).is_err());
    let filtered = runtime
        .execute(
            session
                .sql("SELECT count(*) FROM snapshot.evidence.symbols WHERE is_reexport")
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        filtered.batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap()
            .value(0),
        1
    );
    assert!(!runtime.diagnostics().pop().unwrap().rules.is_empty());
    let retained = session
        .sql("SELECT count(*) FROM snapshot.evidence.symbols")
        .await
        .unwrap();
    std::fs::remove_file(
        &inputs
            .iter()
            .find(|file| file.relation == Relation::Symbols)
            .unwrap()
            .path,
    )
    .unwrap();
    assert!(admitted.session(&runtime, None).is_err());
    assert!(runtime.execute(retained).await.is_err());
    drop((session, result, filtered, metadata, lease));
    assert!(enrichment_store::leases::exclusive(dir.path()).is_ok());
}
