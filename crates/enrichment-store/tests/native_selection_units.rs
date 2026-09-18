//! In-memory native policy tests: no Delta store, daemon, subprocess or MCP client.
use enrichment_core::{
    evidence::{SymbolHeader, SymbolKind},
    native_union::NativeStruct,
};
use enrichment_store::{query::inspection_bindings, runtime::QueryRuntime};

#[tokio::test]
async fn capsule_reuse_compares_complete_native_inventory() {
    use enrichment_core::{
        capsule_protocol::inventory::Entry,
        identity::{Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode},
        operation::{identities::CapsuleIdentity, ownership::RetainedCapsule},
        policy::ExecutionProfile,
    };
    let release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: "crates.io".into(),
        package: "fixture".into(),
        version: "1.0.0".into(),
        artifact_digest: None,
    });
    let environment = Environment::resolved(
        "rust-1.98.1;image".into(),
        "x86_64-unknown-linux-gnu".into(),
        vec![],
        Some(false),
        enrichment_core::canonical::sha256_hex(b"lock"),
    );
    let inputs = CapsuleIdentity {
        context: Context::new(
            release.release_id.clone(),
            environment.environment_id.clone(),
            ResearchMode::Project,
        ),
        release,
        environment: environment.clone(),
        image: "image".into(),
        containment: "containment".into(),
        profile: ExecutionProfile::Build,
    };
    let key = enrichment_core::native_key::Key::CapsuleIdentity
        .record(&inputs)
        .unwrap();
    let capsule = RetainedCapsule {
        generation: format!("{key}-{}", "a".repeat(32)),
        key,
        cache: "/service/cache".into(),
        prepared: enrichment_core::operation::ownership::PreparedCapsule {
            inputs,
            environment,
            lock: "lock".into(),
            inventory: [
                (
                    "enrichment.lock".into(),
                    Entry::File {
                        mode: 0o600,
                        bytes: 4,
                        sha256: enrichment_core::canonical::sha256_hex(b"lock"),
                    },
                ),
                (
                    "input.rs".into(),
                    Entry::File {
                        mode: 0o600,
                        bytes: 6,
                        sha256: "a".repeat(64),
                    },
                ),
            ]
            .into(),
        },
        sequence: 1,
    };
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    assert!(
        enrichment_store::physical_ownership::reusable_capsule(
            &runtime,
            &capsule,
            &capsule.prepared.inventory
        )
        .await
        .unwrap()
    );
    let mut changed = capsule.prepared.inventory.clone();
    changed.insert(
        "input.rs".into(),
        Entry::File {
            mode: 0o644,
            bytes: 6,
            sha256: "a".repeat(64),
        },
    );
    assert!(
        !enrichment_store::physical_ownership::reusable_capsule(&runtime, &capsule, &changed)
            .await
            .unwrap()
    );
    changed.clear();
    assert!(
        !enrichment_store::physical_ownership::reusable_capsule(&runtime, &capsule, &changed)
            .await
            .unwrap()
    );
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn comparison_prerequisites_and_resolution_rules_are_native() {
    use enrichment_core::{
        identity::Ecosystem,
        request::{CompareRequest, ResolveRequest},
    };
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let request = CompareRequest {
        ecosystem: Some(Ecosystem::Rust),
        name: Some("fixture".into()),
        from_version: Some("1.0.0".into()),
        to_version: Some("2.0.0".into()),
        ..Default::default()
    };
    let rows =
        enrichment_store::research_selection::comparison_prerequisites(&runtime, &request, 4096)
            .await
            .unwrap();
    assert_eq!(
        rows.iter()
            .map(|r| r.version.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["1.0.0", "2.0.0"]
    );
    for request in [
        ResolveRequest {
            name: "../bad".into(),
            ..Default::default()
        },
        ResolveRequest {
            name: "fixture".into(),
            version: Some(">=1".into()),
            ..Default::default()
        },
        ResolveRequest {
            name: "fixture".into(),
            python_version: Some("3.14".into()),
            ..Default::default()
        },
    ] {
        assert!(
            enrichment_store::request_admission::research(&runtime, &request.into(), 4096)
                .await
                .is_err()
        );
    }
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn native_defaults_preserve_explicit_empty_and_independent_limits() {
    use enrichment_core::wire::research::{AspectSelection, InspectionAspect, ResearchSelection};
    use enrichment_store::research_selection;
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let defaults = research_selection::inspection(&runtime, &ResearchSelection::Default)
        .await
        .unwrap();
    assert_eq!(
        defaults.iter().map(|s| s.aspect).collect::<Vec<_>>(),
        [
            InspectionAspect::Signature,
            InspectionAspect::Availability,
            InspectionAspect::Documentation
        ]
    );
    assert_eq!(
        (defaults[2].max_items, defaults[2].max_characters),
        (3, Some(1200))
    );
    let selected = AspectSelection {
        aspect: InspectionAspect::Examples,
        cursor: Some("cursor".into()),
        max_items: 7,
        max_characters: None,
    };
    assert_eq!(
        research_selection::inspection(
            &runtime,
            &ResearchSelection::Explicit {
                aspects: vec![selected.clone()]
            }
        )
        .await
        .unwrap(),
        [selected]
    );
    assert_eq!(
        research_selection::discovery(&runtime, &None)
            .await
            .unwrap()
            .len(),
        4
    );
    assert!(
        research_selection::discovery(&runtime, &Some(vec![]))
            .await
            .unwrap()
            .is_empty()
    );
    runtime.close_diagnostics().await.unwrap();
}

fn header(path: &str, definition: &str, reexport: bool) -> SymbolHeader {
    SymbolHeader {
        symbol_id: format!("symbol_{path}_{definition}"),
        definition_id: definition.into(),
        path: path.into(),
        name: path.rsplit("::").next().unwrap().into(),
        kind: SymbolKind::Struct,
        parent_path: None,
        is_reexport: reexport,
        definition_path: "pkg::inner::Item".into(),
        defined_in_package: "pkg".into(),
        qualifier: None,
    }
}

#[tokio::test]
async fn inspection_precedence_ambiguity_and_binding_order_are_native() {
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let session = runtime.session();
    session
        .register_batch(
            "fixture_headers",
            SymbolHeader::batch(&[
                header("pkg::Item", "def_a", true),
                header("pkg::inner::Item", "def_a", false),
                header("other::Item", "def_b", false),
                header("Unique", "def_c", false),
                header("pkg::Unique", "def_d", false),
            ])
            .unwrap(),
        )
        .unwrap();
    let headers = session
        .sql("SELECT *,string_to_array(path,'::') AS components FROM fixture_headers")
        .await
        .unwrap();
    let exact = inspection_bindings(&runtime, headers.clone(), "Unique", None)
        .await
        .unwrap();
    assert_eq!(exact.definition_count, 1);
    assert_eq!(exact.selected.unwrap().definition_id, "def_c");
    assert!(exact.candidates.is_empty());
    let ambiguous = inspection_bindings(&runtime, headers.clone(), "Item", None)
        .await
        .unwrap();
    assert_eq!(ambiguous.definition_count, 2);
    assert!(ambiguous.selected.is_none());
    assert_eq!(
        ambiguous
            .candidates
            .iter()
            .map(|candidate| candidate.path.as_str())
            .collect::<Vec<_>>(),
        ["pkg::Item", "pkg::inner::Item", "other::Item"]
    );
    let pinned = inspection_bindings(&runtime, headers.clone(), "Item", Some("def_a"))
        .await
        .unwrap();
    assert_eq!(pinned.selected.unwrap().path, "pkg::inner::Item");
    assert!(pinned.candidates.is_empty());
    let absent = inspection_bindings(&runtime, headers, "Absent", None)
        .await
        .unwrap();
    assert_eq!(absent.definition_count, 0);
    assert!(absent.selected.is_none());
    assert!(absent.candidates.is_empty());
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn native_request_admission_owns_read_selections_and_execution_scope() {
    use enrichment_core::{
        identity::ContextId,
        policy::ExecutionProfile,
        request::{InspectRequest, InspectionIntent, InspectionOptions, ResearchRequest},
        wire::{
            ResearchSelection,
            research::{AspectSelection, InspectionAspect},
        },
    };
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let mut verification = enrichment_core::execution::VerifyRequest {
        context_id: ContextId::try_from(format!("ctx_{}", "a".repeat(64))).unwrap(),
        snapshot_id: None,
        snippet: "fn main() {\n    println!(\"ok\");\n}\n".into(),
        test_intent: None,
        mode: enrichment_core::execution::ProbeMode::Compile,
        profile: ExecutionProfile::Build,
        max_bytes: None,
    };
    enrichment_store::request_admission::research(&runtime, &verification.clone().into(), 4096)
        .await
        .unwrap();
    verification.profile = ExecutionProfile::Runtime;
    assert!(
        enrichment_store::request_admission::research(&runtime, &verification.into(), 4096)
            .await
            .is_err()
    );
    let mut request = InspectRequest {
        context_id: ContextId::try_from(format!("ctx_{}", "a".repeat(64))).unwrap(),
        snapshot_id: None,
        symbol_path: "pkg::Item".into(),
        definition_id: None,
        selection: ResearchSelection::Explicit {
            aspects: vec![AspectSelection {
                aspect: InspectionAspect::Documentation,
                cursor: None,
                max_items: 2,
                max_characters: Some(256),
            }],
        },
        max_bytes: None,
        execution: None,
    };
    let admit = |request: InspectRequest| ResearchRequest::Inspect(request);
    enrichment_store::request_admission::research(&runtime, &admit(request.clone()), 4096)
        .await
        .unwrap();
    if let ResearchSelection::Explicit { aspects } = &mut request.selection {
        aspects.push(aspects[0].clone());
    }
    assert!(
        enrichment_store::request_admission::research(&runtime, &admit(request.clone()), 4096)
            .await
            .is_err()
    );
    request.selection = ResearchSelection::Default;
    request.execution = Some(InspectionOptions {
        intent: InspectionIntent::ExecuteOnMiss,
        profile: Some(ExecutionProfile::Build),
        ..Default::default()
    });
    assert!(
        enrichment_store::request_admission::research(&runtime, &admit(request.clone()), 4096)
            .await
            .is_err()
    );
    request.selection = ResearchSelection::Explicit {
        aspects: vec![AspectSelection {
            aspect: InspectionAspect::Semantics,
            cursor: None,
            max_items: 1,
            max_characters: None,
        }],
    };
    enrichment_store::request_admission::research(&runtime, &admit(request), 4096)
        .await
        .unwrap();
    runtime.close_diagnostics().await.unwrap();
}
