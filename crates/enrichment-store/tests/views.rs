#[path = "support/native_tables.rs"]
mod native_tables;
pub mod support;

use datafusion::prelude::{col, lit};
use enrichment_core::{evidence::path::PublicPath, identity::Ecosystem};
use enrichment_store::{
    admission::{AdmissionLimits, EvidenceScope, NativeAdmission},
    projection::TextColumn,
    runtime::{QueryLimits, QueryRuntime},
    views,
};

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
    let mut evidence = support::rust_evidence_for(
        format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        format!("env_{}", "a".repeat(64)).try_into().unwrap(),
    );
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
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let admitted = NativeAdmission::new(runtime.clone(), AdmissionLimits::default())
        .unwrap()
        .admit_native(
            &EvidenceScope {
                ecosystem: Ecosystem::Rust,
                symbol_package: "enr_fixture".into(),
                release_id: format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
                environment_id: format!("env_{}", "a".repeat(64)).try_into().unwrap(),
            },
            native_tables::providers(&dir.path().join("tables"), &runtime, &evidence).await,
        )
        .await
        .unwrap();
    let lease = if leased {
        enrichment_store::leases::initialize(dir.path()).unwrap();
        Some(enrichment_store::leases::shared(dir.path()).unwrap())
    } else {
        None
    };
    let session = admitted.research_session(&runtime, lease).await.unwrap();
    let full = runtime
        .execute(session.table("snapshot.domain.api_surface").await.unwrap())
        .await
        .unwrap();
    let bytes = |d: &enrichment_core::telemetry::QueryDiagnostics| {
        d.metrics
            .iter()
            .filter(|m| m.name == "bytes_scanned")
            .filter_map(|m| m.value)
            .sum::<usize>()
    };
    let full_bytes = bytes(runtime.diagnostics().await.unwrap().last().unwrap());
    assert!(
        !runtime
            .diagnostics()
            .await
            .unwrap()
            .last()
            .unwrap()
            .metrics_truncated,
        "full diagnostic was truncated"
    );
    let selected = runtime
        .execute(
            session
                .table("snapshot.domain.inspection_surface")
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    let selected_bytes = bytes(runtime.diagnostics().await.unwrap().last().unwrap());
    eprintln!("leased={leased}, selected_bytes={selected_bytes}, full_bytes={full_bytes}");
    assert!(
        !runtime
            .diagnostics()
            .await
            .unwrap()
            .last()
            .unwrap()
            .metrics_truncated,
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
        assert!(batch.column_by_name("docs").is_none());
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
    let evidence = support::rust_evidence_for(
        format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        format!("env_{}", "a".repeat(64)).try_into().unwrap(),
    );
    let runtime =
        QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).expect("runtime");
    let cache = NativeAdmission::new(runtime.clone(), AdmissionLimits::default()).expect("cache");
    let scope = EvidenceScope {
        ecosystem: Ecosystem::Rust,
        symbol_package: "enr_fixture".into(),
        release_id: format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        environment_id: format!("env_{}", "a".repeat(64)).try_into().unwrap(),
    };
    let admitted = cache
        .admit_native(
            &scope,
            native_tables::providers(&dir.path().join("tables"), &runtime, &evidence).await,
        )
        .await
        .expect("admit");
    let session = admitted
        .research_session(&runtime, None)
        .await
        .expect("bound domain catalog");
    // A left surface preserves every public binding, including bindings without an
    // observation. Definition observations fan out to each public alias independently.
    let mut expected = Vec::new();
    for symbol in &evidence.symbols {
        let observations = evidence
            .api_observations
            .iter()
            .filter(|observation| match &observation.subject {
                enrichment_core::evidence::relational::SubjectRef::Symbol { symbol_id } => {
                    symbol_id == &symbol.symbol_id
                }
                enrichment_core::evidence::relational::SubjectRef::Definition { definition_id } => {
                    definition_id == &symbol.definition_id
                }
                _ => false,
            })
            .collect::<Vec<_>>();
        if observations.is_empty() {
            expected.push((symbol.symbol_id.clone(), None));
        } else {
            expected.extend(observations.into_iter().map(|observation| {
                (
                    symbol.symbol_id.clone(),
                    Some(observation.observation_id.clone()),
                )
            }));
        }
    }
    expected.sort();
    let api = runtime
        .execute(
            session
                .sql("SELECT symbol_id, observation_id FROM snapshot.domain.api_surface")
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    let mut actual = Vec::new();
    for batch in &api.batches {
        let symbols = TextColumn::new(batch.column(0).as_ref()).unwrap();
        let observations = TextColumn::new(batch.column(1).as_ref()).unwrap();
        actual.extend((0..batch.num_rows()).map(|row| {
            (
                symbols.get(row).unwrap().to_owned(),
                observations.get(row).map(str::to_owned),
            )
        }));
    }
    actual.sort();
    assert_eq!(actual, expected);
    let ancestry = session
        .table("snapshot.domain.namespace_members")
        .await
        .expect("view");
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
        .table("snapshot.domain.api_surface")
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
        expected.len()
    );
    let impossible = PublicPath::new(Ecosystem::Rust, vec!["enr_%".into()]).expect("literal");
    let absent = session
        .table("snapshot.domain.api_surface")
        .await
        .expect("view")
        .filter(views::namespace("components", &impossible))
        .expect("literal prefix");
    assert_eq!(runtime.execute(absent).await.expect("empty").rows, 0);
    let namespaces = runtime
        .execute(
            session
                .table("snapshot.domain.path_nodes")
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
    assert!(other.catalog("snapshot").is_none());

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
        let search_session = admitted
            .research_session(&runtime, None)
            .await
            .expect("bound domain catalog");
        let result = enrichment_store::search_plan::page(
            &search_session,
            &runtime,
            &enrichment_core::search::spec::SearchSpec::new("enr_fixture"),
            &enrichment_store::search_plan::SearchOptions {
                include_api: true,
                fragment_kinds: vec![],
                area: None,
                page_size: 2,
                offset: returned.len() as u64,
                after,
            },
            500,
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
        if !result.boundary.has_more {
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
    let mut domain = std::collections::BTreeMap::from([
        ("api_surface".to_owned(), api.clone().into_view()),
        ("namespace_children".to_owned(), api.into_view()),
    ]);
    let nodes = session.sql("SELECT 'python' AS ecosystem, make_array('sample') AS components, 'sample' AS path, 1 AS depth")
        .await.expect("nodes");
    domain.insert("navigation_nodes".into(), nodes.into_view());
    fixture_domain(&session, domain);
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

#[tokio::test]
async fn documentation_folds_only_the_same_definition_and_source() {
    let dir = tempfile::tempdir().unwrap();
    let evidence = support::rust_evidence_for(
        format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
        format!("env_{}", "a".repeat(64)).try_into().unwrap(),
    );
    let runtime = QueryRuntime::new(&dir.path().join("spill"), QueryLimits::default()).unwrap();
    let cache = NativeAdmission::new(runtime.clone(), AdmissionLimits::default()).unwrap();
    let admitted = cache
        .admit_native(
            &EvidenceScope {
                ecosystem: Ecosystem::Rust,
                symbol_package: "enr_fixture".into(),
                release_id: format!("rel_{}", "a".repeat(64)).try_into().unwrap(),
                environment_id: format!("env_{}", "a".repeat(64)).try_into().unwrap(),
            },
            native_tables::providers(&dir.path().join("tables"), &runtime, &evidence).await,
        )
        .await
        .unwrap();
    let session = admitted
        .research_session(&runtime, None)
        .await
        .expect("bound domain catalog");
    assert!(
        session
            .deregister_table("snapshot.domain.fragment_surface")
            .is_err()
    );
    // Equal-text aliases share one exact source. Another definition and an independent
    // producer remain independent evidence even when their wording is identical.
    let fragments = session
        .sql(
            r"
        SELECT fragment_id, 'doc_text' AS kind, label, 'widget documentation' AS text,
            definition_id, named_struct('producer', producer) AS source
        FROM (VALUES
            ('a', 'widget::A', 'd1', 'p1'), ('b', 'widget::B', 'd1', 'p1'),
            ('c', 'widget::C', 'd2', 'p1'), ('d', 'widget::D', 'd1', 'p2')
        ) AS f(fragment_id, label, definition_id, producer)
    ",
        )
        .await
        .unwrap();
    let source = session
        .catalog("snapshot")
        .unwrap()
        .schema("domain")
        .unwrap();
    let mut domain = std::collections::BTreeMap::new();
    for name in source.table_names() {
        domain.insert(name.clone(), source.table(&name).await.unwrap().unwrap());
    }
    domain.insert("fragment_surface".into(), fragments.into_view());
    let session = runtime.session();
    fixture_domain(&session, domain);
    let folded = enrichment_store::search_plan::folded(
        &session,
        &enrichment_core::search::spec::SearchSpec::new("widget"),
        &enrichment_store::search_plan::SearchOptions {
            include_api: false,
            fragment_kinds: vec![enrichment_core::evidence::FragmentKind::DocText],
            area: None,
            page_size: 10,
            offset: 0,
            after: None,
        },
    )
    .await
    .unwrap();
    let output = runtime.execute(folded).await.unwrap();
    assert_eq!(output.rows, 3);
    let aliases = runtime
        .execute(
            session
                .sql("SELECT label FROM folded_fragments ORDER BY label")
                .await
                .unwrap(),
        )
        .await
        .unwrap();
    let labels = aliases
        .batches
        .iter()
        .flat_map(|batch| {
            let values = TextColumn::new(batch.column(0).as_ref()).unwrap();
            (0..batch.num_rows())
                .map(|i| values.get(i).unwrap().to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(labels, ["widget::A", "widget::C", "widget::D"]);
}

#[tokio::test]
async fn overview_uses_one_clipped_namespace_set_across_many_index_batches() {
    let dir = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            batch_rows: 32,
            partitions: 4,
            ..Default::default()
        },
    )
    .unwrap();
    let session = runtime.session();
    let nodes = session
        .sql(
            "SELECT 'python' AS ecosystem,
        make_array('sample', 'n' || CAST(value AS VARCHAR)) AS components,
        'sample.n' || CAST(value AS VARCHAR) AS path, CAST(2 AS BIGINT) AS depth
        FROM generate_series(1, 300)",
        )
        .await
        .unwrap();
    let mut domain =
        std::collections::BTreeMap::from([("navigation_nodes".to_owned(), nodes.into_view())]);
    fixture_domain(&session, domain.clone());
    let api = session
        .sql(
            "SELECT ecosystem, components AS namespace_components,
        array_append(components, 'Thing') AS components, path || '.Thing' AS path,
        'class' AS kind, path AS definition_id, path AS symbol_id,
        false AS is_reexport, false AS is_deprecated, path AS observation_id,
        repeat('Qualified documentation. ', 60) AS doc_summary FROM snapshot.domain.navigation_nodes",
        )
        .await
        .unwrap();
    domain.insert("api_surface".into(), api.clone().into_view());
    domain.insert("namespace_children".into(), api.into_view());
    let session = runtime.session();
    fixture_domain(&session, domain);
    let page = enrichment_store::browse::overview(&session, &runtime, None, 4, 256)
        .await
        .unwrap();
    let mut expected = (1..=300)
        .map(|i| format!("sample.n{i}"))
        .collect::<Vec<_>>();
    expected.sort();
    expected.truncate(256);
    assert_eq!(
        page.namespaces
            .iter()
            .map(|namespace| namespace.path.clone())
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(page.truncated_namespaces, 44);
    assert_eq!(page.definitions_by_kind.get("class"), Some(&300));
    for namespace in page.namespaces {
        assert_eq!(namespace.counts_by_kind.get("class"), Some(&1));
        assert_eq!(namespace.children.len(), 1);
        assert_eq!(
            namespace.children[0].path,
            format!("{}.Thing", namespace.path)
        );
        assert_eq!(namespace.truncated_children, 0);
    }
    let env = session.runtime_env();
    drop(session);
    assert_eq!(
        env.disk_manager.used_disk_space(),
        0,
        "overview indexes must drop with their operation catalog"
    );
}

// These are deliberately synthetic relational inputs for fold/selection semantics. Actual
// admitted-catalog mutability, consistency and ownership are exercised independently.
fn fixture_domain(
    session: &datafusion::prelude::SessionContext,
    tables: std::collections::BTreeMap<
        String,
        std::sync::Arc<dyn datafusion::catalog::TableProvider>,
    >,
) {
    use datafusion::catalog::{
        CatalogProvider, MemoryCatalogProvider, MemorySchemaProvider, SchemaProvider,
    };
    let schema = std::sync::Arc::new(MemorySchemaProvider::new());
    for (name, table) in tables {
        schema.register_table(name, table).unwrap();
    }
    let catalog = std::sync::Arc::new(MemoryCatalogProvider::new());
    catalog.register_schema("domain", schema).unwrap();
    session.register_catalog("snapshot", catalog);
}
