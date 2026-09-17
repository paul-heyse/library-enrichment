use enrichment_core::{
    evidence::{EvidenceKind, relational::SubjectRef},
    identity::Ecosystem,
    wire::{
        AspectOutcome, AspectState, Coverage, Page, ScopeAssessment, ScopeState,
        data::DiscoveryFacet,
        research::{AspectSelection, DiscoveryKind, InspectionAspect},
    },
};
use enrichment_store::{
    research_outcomes,
    runtime::{QueryLimits, QueryRuntime},
};

fn coverage(kinds: &[EvidenceKind]) -> Coverage {
    let mut coverage = Coverage::unassessed("unit scope");
    coverage.assessments = kinds
        .iter()
        .map(|kind| ScopeAssessment {
            snapshot_id: format!("snap_{}", "1".repeat(64)).try_into().unwrap(),
            subject: SubjectRef::Library {
                release_id: format!("rel_{}", "2".repeat(64)).try_into().unwrap(),
            },
            kind: *kind,
            state: ScopeState::Indexed,
            witness_id: Some("unit-coverage".into()),
        })
        .collect();
    coverage
}

fn outcome(aspect: InspectionAspect, state: AspectState) -> AspectOutcome {
    AspectOutcome {
        aspect,
        state,
        reason: None,
        page: Some(Page::new(0, None, false, None)),
        diagnostic: None,
    }
}

#[tokio::test]
async fn native_inspection_outcomes_keep_order_failure_and_independent_required_observations() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let selection = vec![AspectSelection {
        aspect: InspectionAspect::Source,
        cursor: None,
        max_items: 8,
        max_characters: None,
    }];
    assert_eq!(
        research_outcomes::inspection_kinds(&runtime, Ecosystem::Python, &selection)
            .await
            .unwrap(),
        [EvidenceKind::DistributionSource]
    );
    assert_eq!(
        research_outcomes::inspection_kinds(&runtime, Ecosystem::Rust, &selection)
            .await
            .unwrap(),
        [EvidenceKind::CrateSource]
    );
    let mut failed = outcome(InspectionAspect::Semantics, AspectState::Failed);
    failed.reason = Some("producer failed".into());
    let result = research_outcomes::inspection(
        &runtime,
        Ecosystem::Rust,
        vec![
            failed.clone(),
            outcome(InspectionAspect::Signature, AspectState::Available),
            outcome(InspectionAspect::Source, AspectState::Absent),
            outcome(InspectionAspect::Availability, AspectState::Absent),
            outcome(InspectionAspect::Runtime, AspectState::Absent),
        ],
        &coverage(&[
            EvidenceKind::PublicApi,
            EvidenceKind::CrateSource,
            EvidenceKind::RuntimeApi,
        ]),
        true,
        false,
    )
    .await
    .unwrap();
    assert_eq!(result.outcomes[0], failed);
    assert_eq!(result.returned, ["signature"]);
    assert_eq!(
        result.outcomes.iter().map(|o| o.state).collect::<Vec<_>>(),
        [
            AspectState::Failed,
            AspectState::Available,
            AspectState::Absent,
            AspectState::Unavailable,
            AspectState::Unavailable
        ]
    );
    assert!(result.partial);
    let result = research_outcomes::inspection(
        &runtime,
        Ecosystem::Rust,
        vec![outcome(
            InspectionAspect::Documentation,
            AspectState::Absent,
        )],
        &coverage(&[]),
        false,
        false,
    )
    .await
    .unwrap();
    assert_eq!(result.outcomes[0].state, AspectState::Unavailable);
    assert!(result.partial);
    let result = research_outcomes::inspection(
        &runtime,
        Ecosystem::Rust,
        vec![outcome(
            InspectionAspect::Documentation,
            AspectState::Absent,
        )],
        &coverage(&[EvidenceKind::Documentation]),
        false,
        false,
    )
    .await
    .unwrap();
    assert_eq!(result.outcomes[0].state, AspectState::Absent);
    assert!(!result.partial);
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn native_discovery_requires_matching_coverage_before_reporting_absence() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let facet = |kind| DiscoveryFacet {
        kind,
        state: AspectState::Absent,
        reason: None,
        diagnostic: None,
        items: vec![],
        page: None,
    };
    let result = research_outcomes::discovery(
        &runtime,
        vec![
            facet(DiscoveryKind::Examples),
            facet(DiscoveryKind::Documentation),
        ],
        &coverage(&[EvidenceKind::Documentation]),
    )
    .await
    .unwrap();
    assert_eq!(result.facets[0].kind, DiscoveryKind::Examples);
    assert_eq!(result.facets[0].state, AspectState::Unavailable);
    assert_eq!(result.facets[1].state, AspectState::Absent);
    assert!(result.partial);
    let mut mixed = coverage(&[EvidenceKind::Examples]);
    let mut unknown = mixed.assessments[0].clone();
    unknown.state = ScopeState::Unknown;
    mixed.assessments.push(unknown);
    let result =
        research_outcomes::discovery(&runtime, vec![facet(DiscoveryKind::Examples)], &mixed)
            .await
            .unwrap();
    assert_eq!(result.facets[0].state, AspectState::Unavailable);
    assert!(result.partial);
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn source_version_preserves_unknown_and_commit_qualified_revision_scope() {
    use enrichment_core::{identity::ResearchMode, wire::SourceVersionMatch};
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    for (mode, declared, requested, expected) in [
        (
            ResearchMode::Upstream,
            Some("1.2"),
            "1.2",
            SourceVersionMatch::Exact,
        ),
        (
            ResearchMode::Project,
            Some("1.1"),
            "1.2",
            SourceVersionMatch::Mismatched,
        ),
        (
            ResearchMode::Upstream,
            None,
            "1.2",
            SourceVersionMatch::Unknown,
        ),
        (
            ResearchMode::Revision,
            None,
            "commit",
            SourceVersionMatch::Exact,
        ),
        (
            ResearchMode::Revision,
            Some("1.1"),
            "commit",
            SourceVersionMatch::Exact,
        ),
    ] {
        assert_eq!(
            research_outcomes::source_version(&runtime, mode, declared, requested)
                .await
                .unwrap(),
            expected
        );
    }
    runtime.close_diagnostics().await.unwrap();
}

#[tokio::test]
async fn generated_policy_namespace_is_shared_read_only_and_keeps_execution_scopes_separate() {
    use enrichment_core::{
        evidence::{
            execution::*,
            relational::{FactSource, Locator},
        },
        identity::Environment,
        native_union::NativeStruct,
        wire::{EvidenceClass, SourceVersionMatch},
    };
    let directory = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
    let session = runtime.session();
    let schema = session
        .catalog("operation")
        .unwrap()
        .schema("declarations")
        .unwrap();
    let table = schema.table("execution_payloads").await.unwrap().unwrap();
    let second = runtime
        .session()
        .catalog("operation")
        .unwrap()
        .schema("declarations")
        .unwrap()
        .table("execution_payloads")
        .await
        .unwrap()
        .unwrap();
    assert!(std::sync::Arc::ptr_eq(&table, &second));
    assert!(schema.register_table("replacement".into(), table).is_err());
    assert!(schema.deregister_table("execution_payloads").is_err());
    let table = schema.table("execution_payloads").await.unwrap().unwrap();
    let state = session.state();
    let input = session
        .table("operation.declarations.execution_payloads")
        .await
        .unwrap()
        .create_physical_plan()
        .await
        .unwrap();
    assert!(
        table
            .insert_into(
                &state,
                input,
                datafusion::logical_expr::dml::InsertOp::Append
            )
            .await
            .is_err()
    );
    assert!(table.delete_from(&state, vec![]).await.is_err());
    assert!(table.update(&state, vec![], vec![]).await.is_err());
    assert!(table.truncate(&state).await.is_err());

    let definitions = runtime
        .records::<ExecutionDefinition>(
            session
                .table("operation.declarations.execution_payloads")
                .await
                .unwrap(),
            3,
        )
        .await
        .unwrap();
    assert_eq!(
        definitions
            .iter()
            .map(|d| d.evidence_kind)
            .collect::<Vec<_>>(),
        [
            EvidenceKind::SemanticQueries,
            EvidenceKind::RuntimeApi,
            EvidenceKind::UsageProbes
        ]
    );
    let payload = ExecutionPayload::RuntimeObject(RuntimeObject {
        module: "fixture".into(),
        selection: vec![],
        outcome: ExecutionOutcome::Empty,
        type_name: None,
        signature: None,
        docstring: None,
        attributes: vec![],
        limitations: vec![],
    });
    let observation = ExecutionObservation::new(
        SubjectRef::Symbol {
            symbol_id: format!("sym_{}", "1".repeat(64)),
        },
        Environment::unspecified().environment_id,
        format!("sha256:{}", "2".repeat(64)),
        "3".repeat(64),
        payload.clone(),
        FactSource {
            producer_binding_id: "producer".into(),
            extractor: "unit".into(),
            extractor_version: "1".into(),
            artifact_id: enrichment_core::evidence::artifact_id_for(
                &enrichment_core::canonical::sha256_hex(&payload.canonical_bytes().unwrap()),
            ),
            source_uri: None,
            source_version_match: SourceVersionMatch::Exact,
            locator: Locator::Artifact,
            evidence_class: EvidenceClass::RuntimeObserved,
        },
    )
    .unwrap();
    let kinds = [
        EvidenceKind::RuntimeApi,
        EvidenceKind::Documentation,
        EvidenceKind::RuntimeApi,
        EvidenceKind::SemanticQueries,
    ];
    assert_eq!(
        research_outcomes::non_execution_kinds(&runtime, &kinds, &[observation])
            .await
            .unwrap(),
        [EvidenceKind::Documentation, EvidenceKind::SemanticQueries]
    );
    assert_eq!(
        research_outcomes::non_execution_kinds(&runtime, &kinds[..1], &[])
            .await
            .unwrap(),
        [EvidenceKind::RuntimeApi]
    );
    // The provider carries exactly the generated contract; it does not acquire or refresh inputs.
    assert_eq!(
        schema
            .table("execution_payloads")
            .await
            .unwrap()
            .unwrap()
            .schema()
            .fields(),
        &ExecutionDefinition::fields()
    );
    runtime.close_diagnostics().await.unwrap();
}
