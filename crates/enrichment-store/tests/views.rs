mod support;

use arrow::{array::UInt64Array, record_batch::RecordBatch};
use datafusion::prelude::{col, lit};
use enrichment_core::{evidence::path::PublicPath, identity::Ecosystem};
use enrichment_store::{
    admission::{AdmissionCache, AdmissionLimits, EvidenceScope},
    dataset::{self, WriteLimits},
    projection::TextColumn,
    runtime::{QueryLimits, QueryRuntime},
    views,
};

fn count(batch: &RecordBatch) -> u64 {
    batch
        .column(0)
        .as_any()
        .downcast_ref::<UInt64Array>()
        .expect("count")
        .value(0)
}

#[tokio::test]
async fn production_inspection_view_skips_large_documentation_leaves() {
    inspection_projection(false).await;
    inspection_projection(true).await;
}

async fn inspection_projection(leased: bool) {
    use arrow::{
        array::{Array, StructArray},
        datatypes::DataType,
    };
    let dir = tempfile::tempdir().unwrap();
    let mut evidence = support::rust_evidence_for("rel_fixture", "env_fixture");
    let mut seed = 17u64;
    let document: String = (0..600_000)
        .map(|_| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            char::from(b' ' + (seed % 95) as u8)
        })
        .collect();
    for observation in evidence.api_observations.iter_mut().take(4) {
        let mut payload = observation.payload.clone();
        payload.docs = Some(document.clone());
        *observation = enrichment_core::evidence::relational::ApiObservation::new(
            observation.subject.clone(),
            observation.origin,
            observation.environment_id.clone(),
            payload,
            observation.source.clone(),
        )
        .unwrap();
    }
    let files = dataset::write(
        &dir.path().join("files"),
        &evidence,
        &WriteLimits::default(),
    )
    .unwrap();
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let admitted = AdmissionCache::new(runtime.clone(), AdmissionLimits::default())
        .unwrap()
        .admit(
            "large-doc-fixture",
            &EvidenceScope {
                ecosystem: Ecosystem::Rust,
                symbol_package: "enr_fixture".into(),
                release_id: "rel_fixture".into(),
                environment_id: "env_fixture".into(),
            },
            &files,
        )
        .await
        .unwrap();
    let session = runtime.session();
    if leased {
        enrichment_store::leases::initialize(dir.path()).unwrap();
        let lease = enrichment_store::leases::shared(dir.path()).unwrap();
        admitted.register_leased(&session, lease.clone()).unwrap();
        admitted
            .register_views(&session, &runtime, lease)
            .await
            .unwrap();
    } else {
        admitted.register(&session).unwrap();
        views::register(&session).await.unwrap();
    }
    let full = runtime
        .execute(session.table("api_surface").await.unwrap())
        .await
        .unwrap();
    let bytes = |d: &enrichment_store::query_diagnostics::QueryDiagnostics| {
        d.metrics
            .iter()
            .filter(|m| m.name == "bytes_scanned")
            .filter_map(|m| m.value)
            .sum::<usize>()
    };
    let full_bytes = bytes(runtime.diagnostics().last().unwrap());
    assert!(
        !runtime.diagnostics().last().unwrap().metrics_truncated,
        "full diagnostic was truncated"
    );
    let selected = runtime
        .execute(session.table("inspection_surface").await.unwrap())
        .await
        .unwrap();
    let selected_bytes = bytes(runtime.diagnostics().last().unwrap());
    eprintln!("leased={leased}, selected_bytes={selected_bytes}, full_bytes={full_bytes}");
    assert!(
        !runtime.diagnostics().last().unwrap().metrics_truncated,
        "selected diagnostic was truncated"
    );
    assert_eq!(selected.rows, full.rows);
    assert!(full_bytes > 100_000, "full={full_bytes}");
    assert!(
        selected_bytes * 4 < full_bytes,
        "selected={selected_bytes}, full={full_bytes}"
    );
    if leased {
        assert!(enrichment_store::leases::exclusive(dir.path()).is_err());
    }
    drop(session);
    // Cached providers and diagnostic history must not retain request leases.
    if leased {
        assert!(enrichment_store::leases::exclusive(dir.path()).is_ok());
    }
    for batch in selected.batches {
        let payload = batch
            .column_by_name("payload")
            .unwrap()
            .as_any()
            .downcast_ref::<StructArray>()
            .unwrap();
        assert!(payload.column_by_name("docs").is_none());
        assert!(matches!(payload.data_type(), DataType::Struct(_)));
        assert!(payload.column_by_name("signature").is_some());
    }
}

#[tokio::test]
async fn admitted_views_preserve_observations_and_derive_ancestry_without_cross_scans() {
    let dir = tempfile::tempdir().expect("directory");
    let evidence = support::rust_evidence_for("rel_fixture", "env_fixture");
    let files = dataset::write(
        &dir.path().join("files"),
        &evidence,
        &WriteLimits::default(),
    )
    .expect("write");
    let runtime =
        QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).expect("runtime");
    let cache = AdmissionCache::new(runtime.clone(), AdmissionLimits::default()).expect("cache");
    let scope = EvidenceScope {
        ecosystem: Ecosystem::Rust,
        symbol_package: "enr_fixture".into(),
        release_id: "rel_fixture".into(),
        environment_id: "env_fixture".into(),
    };
    let admitted = cache.admit("fixture", &scope, &files).await.expect("admit");
    let session = runtime.session();
    admitted.register(&session).expect("register");
    views::register(&session).await.expect("domain views");
    let api = runtime
        .execute(
            session
                .sql("SELECT CAST(count(*) AS BIGINT UNSIGNED) FROM api_surface")
                .await
                .expect("plan"),
        )
        .await
        .expect("execute");
    assert_eq!(
        count(&api.batches[0]),
        evidence.api_observations.len() as u64
    );
    let ancestry = session.table("namespace_members").await.expect("view");
    let plan = ancestry
        .clone()
        .into_optimized_plan()
        .expect("optimized")
        .display_indent()
        .to_string();
    assert!(plan.contains("Unnest"));
    assert!(!plan.contains("CrossJoin"));
    assert!(!plan.contains("NestedLoopJoin"));
    let rows = runtime.execute(ancestry).await.expect("ancestry");
    assert_eq!(
        rows.rows,
        evidence
            .symbols
            .iter()
            .map(|s| s.path.components().len() - 1)
            .sum::<usize>()
    );
    let root = PublicPath::new(Ecosystem::Rust, vec!["enr_fixture".into()]).expect("root");
    let at_root = session
        .table("api_surface")
        .await
        .expect("view")
        .filter(views::namespace("components", &root))
        .expect("prefix");
    assert_eq!(
        runtime
            .execute(at_root)
            .await
            .expect("all descendants")
            .rows,
        evidence.api_observations.len()
    );
    let impossible = PublicPath::new(Ecosystem::Rust, vec!["enr_%".into()]).expect("literal");
    let absent = session
        .table("api_surface")
        .await
        .expect("view")
        .filter(views::namespace("components", &impossible))
        .expect("literal prefix");
    assert_eq!(runtime.execute(absent).await.expect("empty").rows, 0);
    let namespaces = runtime
        .execute(
            session
                .table("path_nodes")
                .await
                .expect("nodes")
                .filter(col("depth").eq(lit(1i64)))
                .expect("root nodes")
                .select(vec![col("path")])
                .expect("path"),
        )
        .await
        .expect("execute");
    assert_eq!(namespaces.rows, 1);
    assert_eq!(
        TextColumn::new(namespaces.batches[0].column(0).as_ref())
            .expect("text")
            .get(0),
        Some("enr_fixture")
    );
    // The catalog/view namespace is local even while sharing the same execution runtime.
    let other = runtime.session();
    assert!(!other.table_exist("api_surface").expect("isolated"));

    let overview = enrichment_store::browse::overview(&session, &runtime, Some(&root), 1, 64)
        .await
        .expect("native overview");
    assert_eq!(overview.truncated_namespaces, 0);
    let root_facet = overview
        .namespaces
        .iter()
        .find(|n| n.path == "enr_fixture")
        .expect("root facet");
    assert_eq!(root_facet.children.len(), 1);
    assert_eq!(
        root_facet.counts_by_kind.values().sum::<u64>(),
        root_facet.truncated_children + 1
    );
    assert!(root_facet.truncated_children > 0);
    assert_eq!(
        overview.definitions_by_kind.values().sum::<u64>(),
        evidence.definitions.len() as u64
    );

    let mut after = None;
    let mut returned = std::collections::BTreeSet::new();
    loop {
        let search_session = runtime.session();
        admitted.register(&search_session).expect("register");
        views::register(&search_session).await.expect("views");
        let result = enrichment_store::search_plan::page(
            &search_session,
            &runtime,
            &enrichment_core::search::spec::SearchSpec::new("enr_fixture"),
            &enrichment_store::search_plan::SearchOptions {
                include_api: true,
                fragment_kinds: vec![],
                area: None,
                page_size: 2,
                after,
            },
        )
        .await
        .expect("native paged search");
        assert_eq!(result.total, evidence.definitions.len() as u64);
        assert!(!result.rows.is_empty());
        after = result.rows.last().map(|row| row.key.clone());
        for row in result.rows {
            assert!(
                returned.insert(row.key.candidate_id),
                "no duplicate page result"
            );
        }
        if !result.has_more {
            break;
        }
    }
    assert_eq!(returned.len(), evidence.definitions.len());
}

#[tokio::test]
async fn overview_keeps_documentation_when_an_undocumented_observation_sorts_first() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime =
        QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).expect("runtime");
    let session = runtime.session();
    // A source and stub at the same public binding. The undocumented observation has the
    // earlier identity deliberately: IDs cannot decide whether an existing doc is visible.
    let api = session
        .sql(
            r"
        SELECT 'python' AS ecosystem, make_array('sample') AS namespace_components,
            make_array('sample', 'Thing') AS components, 'sample.Thing' AS path,
            'class' AS kind, 'def_thing' AS definition_id, 'sym_thing' AS symbol_id,
            false AS is_reexport, false AS is_deprecated, observation_id, doc_summary
        FROM (VALUES ('a', CAST(NULL AS VARCHAR)), ('z', 'Documented source.'))
            AS observations(observation_id, doc_summary)
    ",
        )
        .await
        .expect("observations");
    session
        .register_table("api_surface", api.clone().into_view())
        .expect("api");
    session
        .register_table("namespace_children", api.into_view())
        .expect("children");
    let nodes = session.sql("SELECT 'python' AS ecosystem, make_array('sample') AS components, 'sample' AS path, 1 AS depth")
        .await.expect("nodes");
    session
        .register_table("navigation_nodes", nodes.into_view())
        .expect("navigation");
    let page = enrichment_store::browse::overview(&session, &runtime, None, 8, 64)
        .await
        .expect("overview");
    assert_eq!(page.definitions_by_kind.get("class"), Some(&1));
    assert_eq!(page.namespaces.len(), 1);
    let children = &page.namespaces[0].children;
    assert_eq!(children.len(), 1);
    assert_eq!(
        children[0].doc_summary.as_deref(),
        Some("Documented source.")
    );
}
