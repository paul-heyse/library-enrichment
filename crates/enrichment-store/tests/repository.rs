use support::native_ingest::{self, EvidenceRows};
#[path = "support/claims.rs"]
mod claims;
pub mod support;

use enrichment_core::{
    evidence::{Artifact, ArtifactKind, snapshot::SnapshotMetadata},
    identity::{Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode},
};
use enrichment_store::{
    BlobStore, StatePaths,
    admission::AdmissionLimits,
    dataset::WriteLimits,
    repository::EvidenceRepository,
    runtime::{QueryLimits, QueryRuntime},
};

fn metadata() -> SnapshotMetadata {
    let release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: "crates.io".into(),
        package: "enr-fixture".into(),
        version: "0.1.0".into(),
        artifact_digest: None,
    });
    let environment = Environment::unspecified();
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        ResearchMode::Upstream,
    );
    SnapshotMetadata {
        context,
        release,
        environment,
        symbol_package: "enr_fixture".into(),
        crate_name: "enr_fixture".into(),
        crate_version: Some("0.1.0".into()),
        normalizer_version: "2".into(),
        observed_configuration: None,
        producer_items: 0,
    }
}

fn evidence(metadata: &SnapshotMetadata) -> EvidenceRows {
    support::rust_evidence_for(
        metadata.release.release_id.as_str(),
        metadata.environment.environment_id.as_str(),
    )
}

fn repository(root: &std::path::Path, store_blob: bool) -> EvidenceRepository {
    let paths = StatePaths {
        data_root: root.join("data"),
        cache_root: root.join("cache"),
    };
    if store_blob {
        let payload =
            include_str!("../../../tests/fixtures/rustdoc/enr-fixture-0.1.0-default.json");
        BlobStore::open(&paths.data_root)
            .expect("blobs")
            .put(payload.as_bytes(), |_| {
                Artifact::describe(
                    payload.as_bytes(),
                    ArtifactKind::RustdocJson,
                    "application/json",
                    "https://docs.rs/enr-fixture/0.1.0.json",
                    enrichment_core::native_time::AcquisitionTime::try_from(
                        "2026-09-14T00:00:00.000000Z".to_owned(),
                    )
                    .unwrap(),
                )
            })
            .expect("artifact");
    }
    let runtime = QueryRuntime::new(&paths.cache_root.join("spill"), QueryLimits::default())
        .expect("runtime");
    EvidenceRepository::new(
        paths,
        runtime,
        WriteLimits::default(),
        AdmissionLimits::default(),
    )
    .expect("repository")
}

#[tokio::test]
async fn comparison_publication_exports_both_inputs_and_verified_result_closure() {
    use enrichment_core::{
        evidence::catalog::ComparisonPublication,
        wire::{ArtifactHandle, Envelope, JobState},
    };
    let dir = tempfile::tempdir().unwrap();
    let repository = repository(dir.path(), true);
    let paths = StatePaths {
        data_root: dir.path().join("data"),
        cache_root: dir.path().join("cache"),
    };
    let before_metadata = metadata();
    let before = native_ingest::publish_rows(
        &repository,
        before_metadata.clone(),
        evidence(&before_metadata),
        None,
        None,
    )
    .await
    .unwrap();
    let mut after_metadata = before_metadata.clone();
    let mut release = after_metadata.release.key.clone();
    release.version = "0.2.0".into();
    after_metadata.release = Release::new(release);
    after_metadata.context = Context::new(
        after_metadata.release.release_id.clone(),
        after_metadata.environment.environment_id.clone(),
        ResearchMode::Upstream,
    );
    after_metadata.crate_version = Some("0.2.0".into());
    let after = native_ingest::publish_rows(
        &repository,
        after_metadata.clone(),
        evidence(&after_metadata),
        None,
        None,
    )
    .await
    .unwrap();
    let blobs = BlobStore::open(&paths.data_root).unwrap();
    let bytes = br#"{"complete":"indivisible comparison value"}"#;
    let dependency = blobs
        .put(bytes, |_| {
            Artifact::describe(
                bytes,
                ArtifactKind::Other,
                "application/json",
                "service:comparison-value/2",
                enrichment_core::native_time::AcquisitionTime::try_from(
                    "2026-09-15T00:00:00.000000Z".to_owned(),
                )
                .unwrap(),
            )
        })
        .unwrap()
        .acquired;
    let mut result: Envelope =
        serde_json::from_str(include_str!("../../../tests/fixtures/wire/ok.fixture.json")).unwrap();
    result.context_id = Some(after.context_id.to_string());
    result.snapshot_id = Some(after.snapshot_id.to_string());
    result.data = serde_json::from_value(serde_json::json!({
        "before": {"context_id": before.context_id, "snapshot_id": before.snapshot_id},
        "after": {"context_id": after.context_id, "snapshot_id": after.snapshot_id},
        "changes": [{"value": {"artifact_id": dependency.artifact_id}}],
    }))
    .unwrap();
    result.artifacts = vec![ArtifactHandle {
        receipt: dependency.clone(),
        uri: format!("library-evidence://artifacts/{}", dependency.artifact_id)
            .try_into()
            .unwrap(),
        description: "Complete comparison value".into(),
    }];
    let delivery =
        enrichment_store::result::store(&blobs, &result, enrichment_store::result::JOB_URI)
            .unwrap()
            .0;
    let publication = ComparisonPublication {
        job_id: format!("job_{}", "c".repeat(32)),
        request_digest: "d".repeat(64),
        before_context_id: before.context_id.clone(),
        before_snapshot_id: before.snapshot_id.clone(),
        after_context_id: after.context_id.clone(),
        after_snapshot_id: after.snapshot_id.clone(),
        state: JobState::Succeeded,
        delivery: delivery.clone(),
    };
    let fence = claims::publication_fence(
        &repository,
        &publication.job_id,
        enrichment_store::control_jobs::Arguments::Compare {
            request: enrichment_core::request::CompareRequest {
                before_snapshot_id: Some(before.snapshot_id.to_string()),
                after_snapshot_id: Some(after.snapshot_id.to_string()),
                ..Default::default()
            },
        },
    )
    .await;
    let initial = repository.catalog.pin().await.unwrap();
    let mut invalid = publication.clone();
    invalid.before_context_id = after.context_id.clone();
    assert!(
        repository
            .publish_comparison(invalid.clone(), fence.clone())
            .await
            .is_err()
    );
    assert!(
        repository
            .catalog
            .commit(enrichment_store::control::ControlBatch {
                search_projections: vec![],
                comparison: Some(invalid),
                ..Default::default()
            })
            .await
            .is_err()
    );
    assert_eq!(
        repository.catalog.pin().await.unwrap().generation(),
        initial.generation()
    );
    repository
        .publish_comparison(publication.clone(), fence.clone())
        .await
        .unwrap();
    let pin = repository.catalog.pin().await.unwrap();
    assert_eq!(
        pin.current(&repository.runtime, &after.context_id)
            .await
            .unwrap(),
        Some(after.snapshot_id.clone())
    );
    assert_eq!(
        pin.comparison_closure(&repository.runtime, &after.snapshot_id)
            .await
            .unwrap(),
        [before.snapshot_id.clone(), after.snapshot_id.clone()].into()
    );
    let exported = dir.path().join("bundle");
    enrichment_store::bundle::export(&paths, after.context_id.as_str(), &exported)
        .await
        .unwrap();
    assert!(
        enrichment_store::bundle::verify(&exported)
            .await
            .unwrap()
            .is_empty()
    );
    let runtime = repository.runtime.clone();
    let offline = BlobStore::read_only(&exported.join("data")).unwrap();
    let native =
        enrichment_store::control::ControlStore::read_only(&exported.join("data"), runtime.clone())
            .unwrap()
            .pin()
            .await
            .unwrap();
    assert!(
        native
            .snapshot(&runtime, &before.snapshot_id)
            .await
            .unwrap()
            .is_some()
    );
    assert_eq!(
        native
            .result_dependencies(&runtime, &offline, &delivery)
            .await
            .unwrap()
            .as_slice(),
        std::slice::from_ref(&dependency)
    );
    let (_, fields) = offline
        .read_result_sections(&delivery, &["data.changes"], 1024 * 1024)
        .unwrap();
    assert_eq!(
        fields["data.changes"],
        serde_json::to_value(&result.data).unwrap()["changes"]
    );
    std::fs::remove_file(offline.path_for(&dependency.sha256)).unwrap();
    assert!(
        native
            .result_dependencies(&runtime, &offline, &delivery)
            .await
            .is_err()
    );
    assert!(
        !enrichment_store::bundle::verify(&exported)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn native_navigation_and_text_projection_preserve_retained_identities() {
    use enrichment_core::evidence::{
        FragmentKind, RelationKind,
        relational::{SubjectRef, TargetRef},
    };
    use std::collections::BTreeSet;
    let dir = tempfile::tempdir().unwrap();
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let facts = evidence(&metadata);
    let manifest = native_ingest::publish_rows(&repository, metadata, facts.clone(), None, None)
        .await
        .unwrap();
    let reader = enrichment_store::SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &manifest.snapshot_id,
    )
    .await
    .unwrap();
    let owner = reader
        .symbols_at("enr_fixture::inner::Widget", None)
        .await
        .unwrap()
        .remove(0);
    assert_eq!(owner.parent_path.as_deref(), Some("enr_fixture::inner"));
    let ancillary = reader.ancillary_facts(&owner.symbol_id).await.unwrap();
    assert_eq!(ancillary.cfg_alternatives, 1);
    assert_eq!(ancillary.cfg_hints, Some(Vec::new()));
    assert_eq!(ancillary.locator_alternatives, 1);
    assert!(ancillary.locator.is_some());
    let header = serde_json::to_value(&owner).unwrap();
    assert!(
        !header
            .as_object()
            .unwrap()
            .contains_key("producer_local_id")
    );
    assert!(!header.as_object().unwrap().contains_key("signature"));
    let expected: BTreeSet<_> = facts
        .symbols
        .iter()
        .filter(|symbol| {
            facts.relationships.iter().any(|r| {
                r.relation == RelationKind::MemberOf
                    && match &r.target {
                        TargetRef::Symbol { symbol_id } => symbol_id == &owner.symbol_id,
                        TargetRef::Definition { definition_id } => {
                            definition_id == &owner.definition_id
                        }
                        _ => false,
                    }
                    && match &r.subject {
                        SubjectRef::Symbol { symbol_id } => symbol_id == &symbol.symbol_id,
                        SubjectRef::Definition { definition_id } => {
                            definition_id == &symbol.definition_id
                        }
                        _ => false,
                    }
            })
        })
        .map(|s| s.symbol_id.clone())
        .collect();
    assert!(!expected.is_empty());
    let mut members = BTreeSet::new();
    let mut after = None;
    loop {
        let page = reader
            .navigation_page(&owner.symbol_id, true, 2, after.as_deref())
            .await
            .unwrap();
        for item in page.items {
            assert!(members.insert(item.symbol_id));
        }
        if !page.has_more {
            break;
        }
        after = page.next_key;
    }
    assert_eq!(members, expected);
    let children = reader
        .navigation_page(&owner.symbol_id, false, 100, None)
        .await
        .unwrap();
    assert!(
        children
            .items
            .iter()
            .all(|child| child.path.rsplit_once("::").unwrap().0 == owner.path)
    );
    assert!(children.items.iter().any(|child| child.name == "new"));
    let full = reader
        .fragment_page(&owner.symbol_id, &[FragmentKind::DocText], 32, None, None)
        .await
        .unwrap();
    let preview = reader
        .fragment_page(
            &owner.symbol_id,
            &[FragmentKind::DocText],
            32,
            None,
            Some(4),
        )
        .await
        .unwrap();
    assert!(!full.items.is_empty());
    assert_eq!(full.items.len(), preview.items.len());
    for ((full, complete), (preview, preview_complete)) in full.items.iter().zip(&preview.items) {
        assert!(*complete);
        assert_eq!(full.fragment_id, preview.fragment_id);
        assert_eq!(preview.text, full.text.chars().take(4).collect::<String>());
        assert_eq!(*preview_complete, full.text.chars().count() <= 4);
        assert_eq!(preview.source.locator, full.source.locator);
        assert_eq!(preview.source.artifact_id, full.source.artifact_id);
    }
}

#[tokio::test]
async fn large_alternative_sets_remain_reachable_for_inspection_and_comparison() {
    use enrichment_core::{
        compare::{Scope, page::AlternativeCursor},
        evidence::relational::ApiObservation,
    };
    use enrichment_store::{SnapshotReader, comparison};
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let original = evidence(&metadata);
    let mut changed = original.clone();
    let base = original.api_observations[0].clone();
    let mut expected = std::collections::BTreeSet::new();
    expected.insert(base.payload.signature.clone());
    for n in (0..96).rev() {
        let mut payload = base.payload.clone();
        payload.signature = Some(format!("fn variant_{n}(value: i64) -> i64"));
        expected.insert(payload.signature.clone());
        let observation = ApiObservation::new(
            base.subject.clone(),
            base.origin,
            base.environment_id.clone(),
            payload,
            base.source.clone(),
        )
        .expect("alternative");
        changed.api_observations.push(observation.clone());
        if n % 10 == 0 {
            changed.api_observations.push(observation);
        }
    }
    let before = native_ingest::publish_rows(&repository, metadata.clone(), original, None, None)
        .await
        .expect("before");
    let after = native_ingest::publish_rows(
        &repository,
        metadata,
        changed,
        Some(before.snapshot_id.clone()),
        None,
    )
    .await
    .expect("after");
    let catalog = repository.catalog.pin().await.expect("catalog");
    let before = SnapshotReader::open(&repository, catalog.clone(), &before.snapshot_id)
        .await
        .expect("before reader");
    let after = SnapshotReader::open(&repository, catalog, &after.snapshot_id)
        .await
        .expect("after reader");
    let equal = comparison::page(
        &after,
        &after,
        &BlobStore::open(&dir.path().join("data")).unwrap(),
        comparison::Selection {
            scopes: &[Scope::Api],
            after_key: None,
            limit: 10,
            detail: None,
            digest: "large",
        },
    )
    .await
    .expect("large self comparison");
    assert_eq!(equal.total, 0);
    let initial = comparison::page(
        &before,
        &after,
        &BlobStore::open(&dir.path().join("data")).unwrap(),
        comparison::Selection {
            scopes: &[Scope::Api],
            after_key: None,
            limit: 10,
            detail: None,
            digest: "large",
        },
    )
    .await
    .expect("first changed keys");
    let (key, change) = initial.changes.into_iter().next().expect("changed key");
    let symbol = after
        .symbols_at(&key.subject, None)
        .await
        .expect("identity selection")
        .remove(0);
    let mut after_key = None;
    let mut observations = std::collections::BTreeMap::new();
    loop {
        let page = after
            .observation_page(&symbol.symbol_id, false, 7, after_key.as_deref())
            .await
            .expect("observation page");
        assert!(page.items.len() <= 7);
        for item in page.items {
            assert!(
                observations
                    .insert(item.observation_id, item.payload.signature)
                    .is_none(),
                "duplicate observation across pages"
            );
        }
        after_key = page.next_key;
        if !page.has_more {
            break;
        }
    }
    assert_eq!(
        observations
            .into_values()
            .collect::<std::collections::BTreeSet<_>>(),
        expected
    );
    let snapshots = format!(
        "{}:{}",
        before.manifest().snapshot_id,
        after.manifest().snapshot_id
    );
    let id = change.change_id.clone();
    let mut result = change;
    let mut values = std::collections::BTreeSet::new();
    loop {
        for alternative in result.after.as_ref().expect("after alternatives") {
            assert!(alternative.source.is_some());
            let enrichment_core::compare::AlternativeValue::Inline { value } = &alternative.value
            else {
                panic!("native inline value")
            };
            let signature = value["observation"]["signature"]
                .as_str()
                .map(str::to_owned);
            assert!(
                values.insert(signature),
                "duplicate alternative across pages"
            );
        }
        let Some(cursor) = &result.after_page.next_cursor else {
            break;
        };
        let detail = AlternativeCursor::decode(cursor, &snapshots, "large").expect("cursor");
        assert!(AlternativeCursor::decode(cursor, &snapshots, "different selection").is_err());
        result = comparison::page(
            &before,
            &after,
            &BlobStore::open(&dir.path().join("data")).unwrap(),
            comparison::Selection {
                scopes: &[Scope::Api],
                after_key: None,
                limit: 10,
                detail: Some(&detail),
                digest: "large",
            },
        )
        .await
        .expect("alternative page")
        .changes
        .remove(0)
        .1;
        assert!(
            result.before.as_ref().unwrap().is_empty(),
            "a detail page must not hydrate the unselected side again"
        );
        assert!(result.before_page.next_cursor.is_none());
        assert_eq!(
            result.before_page.count,
            enrichment_core::wire::research::MatchCount::Unknown
        );
        assert_eq!(
            result.change_id, id,
            "change identity is independent of detail page"
        );
    }
    assert_eq!(values, expected);
}

#[tokio::test]
async fn requested_coverage_distinguishes_unknown_missing_partial_and_recovered_scopes() {
    use enrichment_core::{
        evidence::{
            EvidenceKind, Gap, GapReason,
            relational::{CoverageFact, CoverageOutcome, SubjectRef},
        },
        wire::ScopeState,
    };
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let mut evidence = evidence(&metadata);
    let binding = evidence.coverage[0].producer_binding_id.clone();
    let library = SubjectRef::Library {
        release_id: metadata.release.release_id.to_string(),
    };
    let symbol = evidence.symbols[0].symbol_id.clone();
    for (kind, outcome, subject) in [
        (
            EvidenceKind::Documentation,
            CoverageOutcome::Missing,
            library.clone(),
        ),
        (
            EvidenceKind::ReleaseNotes,
            CoverageOutcome::Missing,
            library.clone(),
        ),
        (EvidenceKind::Examples, CoverageOutcome::Partial, library),
    ] {
        evidence.coverage.push(
            CoverageFact::new(
                binding.clone(),
                subject,
                kind,
                outcome,
                vec![Gap {
                    kind,
                    reason: GapReason::NotAttempted,
                    detail: "earlier attempt".into(),
                    planned_fallback: None,
                }],
            )
            .expect("gap coverage"),
        );
    }
    evidence.coverage.push(
        CoverageFact::new(
            binding,
            SubjectRef::Symbol {
                symbol_id: symbol.clone(),
            },
            EvidenceKind::RuntimeApi,
            CoverageOutcome::Indexed,
            vec![],
        )
        .expect("symbol coverage"),
    );
    let manifest = native_ingest::publish_rows(&repository, metadata, evidence, None, None)
        .await
        .expect("publish");
    let reader = enrichment_store::SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.expect("catalog"),
        &manifest.snapshot_id,
    )
    .await
    .expect("reader");
    let coverage = reader
        .assess(
            &[
                EvidenceKind::Documentation,
                EvidenceKind::ReleaseNotes,
                EvidenceKind::Examples,
                EvidenceKind::RuntimeApi,
                EvidenceKind::UsageProbes,
            ],
            None,
            "selected library scopes".into(),
        )
        .await
        .expect("assessment");
    let states: std::collections::BTreeMap<_, _> = coverage
        .assessments
        .iter()
        .map(|item| (item.kind, item.state))
        .collect();
    assert_eq!(states[&EvidenceKind::Documentation], ScopeState::Indexed);
    assert_eq!(states[&EvidenceKind::ReleaseNotes], ScopeState::Missing);
    assert_eq!(states[&EvidenceKind::Examples], ScopeState::Partial);
    assert_eq!(states[&EvidenceKind::RuntimeApi], ScopeState::Unknown);
    assert_eq!(states[&EvidenceKind::UsageProbes], ScopeState::Unknown);
    assert!(!coverage.complete());
    let symbol_scope = reader
        .assess(
            &[EvidenceKind::PublicApi, EvidenceKind::RuntimeApi],
            Some(&symbol),
            "symbol scopes".into(),
        )
        .await
        .expect("symbol assessment");
    assert!(symbol_scope.complete());
    assert!(
        symbol_scope
            .assessments
            .iter()
            .all(|row| row.witness_id.is_some())
    );
    let docs_only = reader
        .assess(
            &[EvidenceKind::Documentation],
            None,
            "documentation only".into(),
        )
        .await
        .expect("documentation assessment");
    assert!(
        docs_only.complete(),
        "unrequested gaps must not taint requested coverage"
    );
    assert!(docs_only.missing.is_empty());
}

#[tokio::test]
async fn cleanup_is_excluded_until_catalog_plan_and_exhausted_stream_are_dropped() {
    use enrichment_store::{SnapshotReader, leases};
    use futures::StreamExt;
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let manifest = native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        evidence(&metadata),
        None,
        None,
    )
    .await
    .expect("publish");
    let root = dir.path().join("data");
    drop(leases::exclusive(&root).expect("idle caches hold no lease"));
    for sql in [
        "SELECT symbol_id FROM snapshot.evidence.symbols ORDER BY symbol_id",
        "SELECT symbol_id FROM snapshot.domain.api_surface ORDER BY symbol_id",
        "SELECT context_id FROM state.records.contexts ORDER BY context_id",
    ] {
        let catalog = repository.catalog.pin().await.expect("catalog");
        assert!(leases::exclusive(&root).is_err());
        let reader = SnapshotReader::open(&repository, catalog, &manifest.snapshot_id)
            .await
            .expect("reader");
        let catalog_session = repository
            .catalog
            .pin()
            .await
            .expect("catalog view pin")
            .session(&repository.runtime)
            .await
            .expect("catalog session");
        let session = if sql.contains("FROM state.records.contexts") {
            &catalog_session
        } else {
            reader.session()
        };
        let frame = session.sql(sql).await.expect("frame");
        let task = session.task_ctx();
        drop(catalog_session);
        drop(reader);
        assert!(
            leases::exclusive(&root).is_err(),
            "logical plan keeps providers alive"
        );
        let physical = frame.create_physical_plan().await.expect("physical plan");
        drop(frame);
        assert!(
            leases::exclusive(&root).is_err(),
            "optimized native plan keeps lease"
        );
        let mut stream = physical.execute(0, task).expect("stream");
        drop(physical);
        while let Some(batch) = stream.next().await {
            batch.expect("batch");
        }
        assert!(
            leases::exclusive(&root).is_err(),
            "even an exhausted stream owns its lease"
        );
        drop(stream);
        drop(leases::exclusive(&root).expect("all active consumers dropped"));
    }
}

#[tokio::test]
async fn cold_open_rejects_missing_or_unrelated_catalog_attempts() {
    use enrichment_core::evidence::catalog::SnapshotAttempt;
    use enrichment_store::control::{ControlBatch, ControlStore, SelectionChange};
    for unrelated in [false, true] {
        let dir = tempfile::tempdir().expect("directory");
        let repository = repository(dir.path(), true);
        let metadata = metadata();
        let evidence = evidence(&metadata);
        let manifest = native_ingest::publish_rows(
            &repository,
            metadata.clone(),
            evidence.clone(),
            None,
            None,
        )
        .await
        .expect("publish");
        let pin = repository.catalog.pin().await.expect("catalog");
        let entry = pin
            .snapshot(&repository.runtime, &manifest.snapshot_id)
            .await
            .expect("lookup")
            .expect("entry");
        drop(
            repository
                .open_snapshot(pin, &manifest.snapshot_id)
                .await
                .expect("original attribution"),
        );
        // Re-encode a coherent physical catalog with the wrong semantic attempt association.
        std::fs::remove_dir_all(dir.path().join("data/catalog")).expect("replace test catalog");
        let catalog = ControlStore::open(&dir.path().join("data"), repository.runtime.clone())
            .expect("catalog");
        let mut run = evidence.producer_runs[0].clone();
        run.config_digest = "unrelated-producer-configuration".into();
        catalog
            .commit(ControlBatch {
                search_projections: vec![],
                publication_fence: None,
                publication: None,
                comparison: None,
                releases: vec![metadata.release],
                environments: vec![metadata.environment],
                contexts: vec![metadata.context.clone()],
                snapshots: vec![entry],
                attempts: if unrelated {
                    Some(
                        repository
                            .runtime
                            .session()
                            .read_batch(
                                enrichment_store::projection::catalog::attempts(&[
                                    SnapshotAttempt {
                                        snapshot_id: manifest.snapshot_id.clone(),
                                        artifacts: evidence.attempt_artifacts[&run.attempt_id]
                                            .clone(),
                                        run,
                                    },
                                ])
                                .unwrap(),
                            )
                            .unwrap(),
                    )
                } else {
                    None
                },
                selection: Some(SelectionChange {
                    context_id: metadata.context.context_id,
                    snapshot_id: manifest.snapshot_id.clone(),
                    expected_base: None,
                }),
            })
            .await
            .expect("internally coherent catalog");
        let pin = catalog.pin().await.expect("physically valid catalog");
        let error = match repository.open_snapshot(pin, &manifest.snapshot_id).await {
            Ok(_) => panic!("bad producer attribution was accepted"),
            Err(error) => error,
        };
        assert!(
            error.to_string().contains(if unrelated {
                "does not produce"
            } else {
                "no actual catalog attempt"
            }),
            "{error}"
        );
    }
}

#[tokio::test]
async fn derived_environment_retains_static_sources_with_new_consumer_identity() {
    use enrichment_store::SnapshotReader;
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let evidence = evidence(&metadata);
    let expected_sources: Vec<_> = evidence
        .api_observations
        .iter()
        .map(|o| o.source.clone())
        .collect();
    let original = native_ingest::publish_rows(&repository, metadata.clone(), evidence, None, None)
        .await
        .expect("publish");
    let catalog = repository.catalog.pin().await.expect("catalog");
    let parent = repository
        .open_snapshot(catalog, &original.snapshot_id)
        .await
        .expect("parent");
    let environment = Environment::resolved(
        "rustc-1.98.1".into(),
        "x86_64-unknown-linux-gnu".into(),
        vec![],
        Some(true),
        "lock-fixture".into(),
    );
    let context = metadata
        .context
        .derived_with(environment.environment_id.clone());
    let derived = repository
        .derive_environment(&parent, context.clone(), environment.clone())
        .await
        .expect("derive native batches");
    assert_ne!(derived.snapshot_id, original.snapshot_id);
    assert_eq!(derived.context_id, context.context_id);
    let catalog = repository.catalog.pin().await.expect("catalog");
    let reader = SnapshotReader::open(&repository, catalog, &derived.snapshot_id)
        .await
        .expect("derived reader");
    let rows = repository
        .runtime
        .execute(
            reader
                .session()
                .table("snapshot.evidence.api_observations")
                .await
                .expect("table"),
        )
        .await
        .expect("query");
    let observations = rows
        .batches
        .iter()
        .flat_map(|batch| {
            enrichment_store::projection::decode::observations(batch).expect("observations")
        })
        .collect::<Vec<_>>();
    assert_eq!(observations.len(), expected_sources.len());
    assert!(
        observations
            .iter()
            .all(|o| o.environment_id == environment.environment_id.as_str()
                && expected_sources.contains(&o.source))
    );
    let again = repository
        .derive_environment(&parent, context, environment)
        .await
        .expect("same derivation");
    assert_eq!(again.snapshot_id, derived.snapshot_id);
}

#[tokio::test]
async fn native_comparison_preserves_nested_alternatives_and_pages_complete_keys() {
    use enrichment_core::{compare::Scope, evidence::relational::ApiObservation};
    use enrichment_store::{SnapshotReader, comparison};
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let original = evidence(&metadata);
    let mut changed = original.clone();
    let selected = 0;
    let observation = &changed.api_observations[selected];
    let mut payload = observation.payload.clone();
    payload.signature = Some("fn changed(value: i64) -> i64".into());
    changed.api_observations.push(
        ApiObservation::new(
            observation.subject.clone(),
            observation.origin,
            observation.environment_id.clone(),
            payload,
            observation.source.clone(),
        )
        .expect("changed alternative"),
    );
    let before = native_ingest::publish_rows(&repository, metadata.clone(), original, None, None)
        .await
        .expect("before");
    let after = native_ingest::publish_rows(
        &repository,
        metadata,
        changed,
        Some(before.snapshot_id.clone()),
        None,
    )
    .await
    .expect("after");
    let catalog = repository.catalog.pin().await.expect("catalog");
    let before = SnapshotReader::open(&repository, catalog.clone(), &before.snapshot_id)
        .await
        .expect("read before");
    let after = SnapshotReader::open(&repository, catalog, &after.snapshot_id)
        .await
        .expect("read after");
    let scopes = [
        Scope::Api,
        Scope::Docs,
        Scope::Configuration,
        Scope::ReleaseNotes,
        Scope::Examples,
        Scope::Relationships,
    ];
    let equal = comparison::page(
        &before,
        &before,
        &BlobStore::open(&dir.path().join("data")).unwrap(),
        comparison::Selection {
            scopes: &scopes,
            after_key: None,
            limit: 10,
            detail: None,
            digest: "fixture",
        },
    )
    .await
    .expect("same snapshot");
    assert_eq!(equal.total, 0);
    assert!(equal.changes.is_empty());
    let all = comparison::page(
        &before,
        &after,
        &BlobStore::open(&dir.path().join("data")).unwrap(),
        comparison::Selection {
            scopes: &scopes,
            after_key: None,
            limit: 100,
            detail: None,
            digest: "fixture",
        },
    )
    .await
    .expect("native comparison");
    assert!(all.total > 0);
    assert!(all.changes.iter().all(|(_, c)| {
        c.scope == Scope::Api
            && c.before
                .as_ref()
                .is_some_and(|values| values.iter().any(|a| a.source.is_some()))
            && c.after
                .as_ref()
                .is_some_and(|values| values.iter().any(|a| a.source.is_some()))
    }));
    assert!(all.changes.iter().any(|(_, c)| {
        serde_json::to_string(&c.after)
            .expect("after")
            .contains("fn changed")
    }));
    let mut cursor = None;
    let mut seen = Vec::new();
    loop {
        let page = comparison::page(
            &before,
            &after,
            &BlobStore::open(&dir.path().join("data")).unwrap(),
            comparison::Selection {
                scopes: &[Scope::Api],
                after_key: cursor.as_ref(),
                limit: 1,
                detail: None,
                digest: "fixture",
            },
        )
        .await
        .expect("page");
        assert_eq!(page.total, all.total);
        cursor = page.changes.last().map(|(k, _)| k.clone());
        seen.extend(page.changes.into_iter().map(|(k, c)| (k, c.change_id)));
        if !page.has_more {
            break;
        }
    }
    assert_eq!(
        seen,
        all.changes
            .into_iter()
            .map(|(k, c)| (k, c.change_id))
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn relationship_comparison_distinguishes_same_path_qualified_endpoints() {
    use enrichment_core::{
        compare::Scope,
        evidence::{
            RelationKind, SymbolHeader,
            relational::{PublicBinding, RelationshipObservation, SubjectRef, TargetRef},
        },
    };
    use enrichment_store::{SnapshotReader, comparison};
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let mut evidence = evidence(&metadata);
    let original = evidence.symbols[0].clone();
    let mut definition = evidence
        .definitions
        .iter()
        .find(|d| d.definition_id == original.definition_id)
        .expect("definition")
        .clone();
    definition.qualifier = Some("qualified-alternative".into());
    definition.definition_id = SymbolHeader::definition_id_for(
        &definition.defined_in_package,
        &definition.definition_path,
        definition.kind,
        definition.qualifier.as_deref(),
    );
    let alternate = PublicBinding {
        symbol_id: PublicBinding::id_for(
            &metadata.symbol_package,
            &original.path,
            definition.kind,
            definition.qualifier.as_deref(),
        ),
        definition_id: definition.definition_id.clone(),
        qualifier: definition.qualifier.clone(),
        ..original.clone()
    };
    evidence.definitions.push(definition);
    evidence.symbols.push(alternate.clone());
    let source = evidence.api_observations[0].source.clone();
    let relation = |subject: &str, target: &str| {
        RelationshipObservation::new(
            SubjectRef::Symbol {
                symbol_id: subject.into(),
            },
            TargetRef::Symbol {
                symbol_id: target.into(),
            },
            RelationKind::Implements,
            None,
            source.clone(),
        )
        .expect("relationship")
    };
    evidence.relationships = vec![relation(&original.symbol_id, &original.symbol_id)];
    let baseline =
        native_ingest::publish_rows(&repository, metadata.clone(), evidence.clone(), None, None)
            .await
            .expect("baseline");
    for (subject, target, expected_changes) in [
        (&original.symbol_id, &alternate.symbol_id, 1),
        (&alternate.symbol_id, &original.symbol_id, 2),
    ] {
        let mut changed = evidence.clone();
        changed.relationships = vec![relation(subject, target)];
        let pin = repository.catalog.pin().await.expect("catalog");
        let current = pin
            .current(&repository.runtime, &metadata.context.context_id)
            .await
            .expect("current");
        drop(pin);
        let after =
            native_ingest::publish_rows(&repository, metadata.clone(), changed, current, None)
                .await
                .expect("changed endpoint");
        let catalog = repository.catalog.pin().await.expect("catalog");
        let before = SnapshotReader::open(&repository, catalog.clone(), &baseline.snapshot_id)
            .await
            .expect("before");
        let after = SnapshotReader::open(&repository, catalog, &after.snapshot_id)
            .await
            .expect("after");
        let delta = comparison::page(
            &before,
            &after,
            &BlobStore::open(&dir.path().join("data")).unwrap(),
            comparison::Selection {
                scopes: &[Scope::Relationships],
                after_key: None,
                limit: 10,
                detail: None,
                digest: "fixture",
            },
        )
        .await
        .expect("compare endpoints");
        assert_eq!(delta.total, expected_changes);
    }
}

#[tokio::test]
async fn portable_bundle_admits_without_the_original_store_and_rejects_a_missing_blob() {
    use enrichment_store::{bundle, projection::TextColumn};
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let manifest = native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        evidence(&metadata),
        None,
        None,
    )
    .await
    .expect("publication");
    let paths = StatePaths {
        data_root: dir.path().join("data"),
        cache_root: dir.path().join("cache"),
    };
    let bundle_dir = tempfile::tempdir().expect("independent bundle root");
    let destination = bundle_dir.path().join("complete");
    let exported = bundle::export(&paths, metadata.context.context_id.as_str(), &destination)
        .await
        .expect("export");
    assert!(exported.artifacts > 0);
    assert_eq!(
        exported.snapshot_id.as_deref(),
        Some(manifest.snapshot_id.as_str())
    );
    assert!(
        bundle::verify(&destination)
            .await
            .expect("verify")
            .is_empty()
    );
    let opened = repository
        .open_snapshot(
            repository.catalog.pin().await.expect("catalog"),
            &manifest.snapshot_id,
        )
        .await
        .expect("snapshot");
    let session = opened.session(&repository.runtime).expect("session");
    let output = repository
        .runtime
        .execute(
            session
                .sql("SELECT sha256 FROM snapshot.evidence.input_artifacts LIMIT 1")
                .await
                .expect("plan"),
        )
        .await
        .expect("input");
    let digest = TextColumn::new(output.batches[0].column(0).as_ref())
        .expect("digest")
        .get(0)
        .expect("value")
        .to_owned();
    std::fs::remove_file(
        BlobStore::read_only(&paths.data_root)
            .expect("blob store")
            .path_for(&digest),
    )
    .expect("remove original blob");
    assert!(
        bundle::verify(&destination)
            .await
            .expect("independent verification")
            .is_empty()
    );
    let failed = bundle_dir.path().join("must-not-publish");
    assert!(
        bundle::export(&paths, metadata.context.context_id.as_str(), &failed)
            .await
            .is_err()
    );
    assert!(!failed.exists());
    let copied = destination
        .join("data/blobs/sha256")
        .join(&digest[..2])
        .join(&digest);
    let bytes = std::fs::read(&copied).expect("copied blob");
    let mut changed = bytes;
    changed[0] ^= 1;
    std::fs::write(copied, changed).expect("same-size tampering");
    assert!(
        !bundle::verify(&destination)
            .await
            .expect("checksum failure")
            .is_empty()
    );
}

#[tokio::test]
async fn actual_rustdoc_publishes_and_reopens_only_through_catalog_membership() {
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let evidence = evidence(&metadata);
    let count = evidence.api_observations.len();
    let empty = repository.catalog.pin().await.expect("empty generation");
    let manifest = native_ingest::publish_rows(&repository, metadata, evidence, None, None)
        .await
        .expect("publication");
    assert_eq!(manifest.schema_version, "6.0");
    assert_eq!(manifest.tables.len(), 10);
    assert!(
        repository
            .open_snapshot(empty, &manifest.snapshot_id)
            .await
            .is_err()
    );
    let opened = repository
        .open_snapshot(
            repository.catalog.pin().await.expect("catalog"),
            &manifest.snapshot_id,
        )
        .await
        .expect("open");
    let session = opened.session(&repository.runtime).expect("session");
    let result = repository
        .runtime
        .execute(
            session
                .sql("SELECT observation_id FROM snapshot.evidence.api_observations")
                .await
                .expect("plan"),
        )
        .await
        .expect("execute");
    assert_eq!(result.rows, count);
}

#[tokio::test]
async fn operational_logs_survive_export_without_changing_snapshot_identity() {
    use enrichment_store::{SnapshotReader, bundle};
    let dir = tempfile::tempdir().unwrap();
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let first = native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        evidence(&metadata),
        None,
        None,
    )
    .await
    .unwrap();
    let mut again = evidence(&metadata);
    // The production semantic digest must ignore ordering and duplicate rows, including
    // duplicates separated by physical batches. This exercises the native distinct/sort.
    again.symbols.reverse();
    again.fragments.reverse();
    again.api_observations.reverse();
    again.fragments.extend(again.fragments.clone());
    let mut artifacts = again
        .attempt_artifacts
        .remove(&again.producer_runs[0].attempt_id)
        .unwrap();
    let bytes = b"{\"argv\":[\"cargo\",\"--frozen\"],\"cleanup_confirmed\":true}";
    let blobs = BlobStore::open(&dir.path().join("data")).unwrap();
    let log = blobs
        .put(bytes, |_| {
            Artifact::describe(
                bytes,
                ArtifactKind::Other,
                "application/json",
                "producer-attempt://second/log",
                enrichment_core::native_time::AcquisitionTime::try_from(
                    "2026-09-14T01:00:00.000000Z".to_owned(),
                )
                .unwrap(),
            )
        })
        .unwrap()
        .acquired;
    let run = &mut again.producer_runs[0];
    run.attempt_id = "second-with-log".into();
    run.log = Some(log.artifact_id.clone());
    artifacts.push(log.clone());
    again
        .attempt_artifacts
        .insert(run.attempt_id.clone(), artifacts);
    let second = native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        again,
        Some(first.snapshot_id.clone()),
        None,
    )
    .await
    .unwrap();
    assert_eq!(first.snapshot_id, second.snapshot_id);
    let reader = SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &second.snapshot_id,
    )
    .await
    .unwrap();
    assert_eq!(reader.attempt_logs().await.unwrap(), vec![log.clone()]);
    let destination = dir.path().join("bundle");
    let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
    bundle::export(&paths, metadata.context.context_id.as_str(), &destination)
        .await
        .unwrap();
    let copied = destination
        .join("data/blobs/sha256")
        .join(&log.sha256[..2])
        .join(&log.sha256);
    assert_eq!(std::fs::read(&copied).unwrap(), bytes);
    assert!(bundle::verify(&destination).await.unwrap().is_empty());
    std::fs::remove_file(blobs.path_for(&log.sha256)).unwrap();
    assert!(
        repository
            .open_snapshot(repository.catalog.pin().await.unwrap(), &second.snapshot_id)
            .await
            .is_err()
    );
    assert!(bundle::verify(&destination).await.unwrap().is_empty());
    std::fs::remove_file(copied).unwrap();
    let failures = bundle::verify(&destination).await.unwrap();
    assert_eq!(failures.len(), 1);
    assert!(failures[0].contains(&log.sha256));
}

#[tokio::test]
async fn reacquisition_preserves_snapshot_bytes_and_adds_attempt_attribution() {
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let first = native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        evidence(&metadata),
        None,
        None,
    )
    .await
    .expect("first");
    let path = dir
        .path()
        .join("data/snapshots")
        .join(first.snapshot_id.as_str())
        .join("manifest.json");
    let before = std::fs::read(&path).expect("manifest");
    let mut again = evidence(&metadata);
    let acquisitions = again
        .attempt_artifacts
        .remove(&again.producer_runs[0].attempt_id)
        .expect("acquisitions");
    again.producer_runs[0].attempt_id = "another-real-attempt".into();
    again
        .attempt_artifacts
        .insert(again.producer_runs[0].attempt_id.clone(), acquisitions);
    again.producer_runs[0].started_at = enrichment_core::native_time::ObservationTime::try_from(
        "2026-09-15T00:00:00.000000Z".to_owned(),
    )
    .unwrap();
    again.producer_runs[0].finished_at = enrichment_core::native_time::ObservationTime::try_from(
        "2026-09-15T00:00:01.000000Z".to_owned(),
    )
    .unwrap();
    let second = native_ingest::publish_rows(
        &repository,
        metadata,
        again,
        Some(first.snapshot_id.clone()),
        None,
    )
    .await
    .expect("second");
    assert_eq!(first.snapshot_id, second.snapshot_id);
    assert_eq!(before, std::fs::read(path).expect("same manifest"));
    let catalog = repository.catalog.pin().await.expect("catalog");
    let session = catalog.session(&repository.runtime).await.expect("session");
    assert_eq!(
        repository
            .runtime
            .execute(
                session
                    .sql("SELECT association_id FROM state.records.attempts")
                    .await
                    .expect("plan")
            )
            .await
            .expect("execute")
            .rows,
        2
    );
}

#[tokio::test]
async fn concurrent_same_context_enrichments_rebase_disjoint_observations_without_loss() {
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let mut left = evidence(&metadata);
    let mut right = left.clone();
    let expected = left.fragments.len();
    left.fragments.truncate(expected / 2);
    right.fragments.drain(..expected / 2);
    let acquisitions = right
        .attempt_artifacts
        .remove(&right.producer_runs[0].attempt_id)
        .expect("acquisitions");
    right.producer_runs[0].attempt_id = "independent-second-job".into();
    right
        .attempt_artifacts
        .insert(right.producer_runs[0].attempt_id.clone(), acquisitions);
    let (a, b) = tokio::join!(
        native_ingest::publish_rows(&repository, metadata.clone(), left, None, None),
        native_ingest::publish_rows(&repository, metadata.clone(), right, None, None)
    );
    a.expect("first job");
    b.expect("second job");
    let catalog = repository.catalog.pin().await.expect("catalog");
    let current = catalog
        .current(&repository.runtime, &metadata.context.context_id)
        .await
        .expect("current")
        .expect("snapshot");
    let opened = repository
        .open_snapshot(catalog, &current)
        .await
        .expect("open");
    assert_eq!(opened.manifest.counts.fragments as usize, expected);
    let session = opened.session(&repository.runtime).expect("session");
    assert_eq!(
        repository
            .runtime
            .execute(
                session
                    .sql("SELECT fragment_id FROM snapshot.evidence.fragments")
                    .await
                    .expect("plan")
            )
            .await
            .expect("execute")
            .rows,
        expected
    );
    assert_eq!(
        repository
            .runtime
            .execute(
                session
                    .sql("SELECT attempt_id FROM snapshot.evidence.producer_runs")
                    .await
                    .expect("plan")
            )
            .await
            .expect("execute")
            .rows,
        2
    );
}

#[tokio::test]
async fn missing_input_bytes_cannot_be_published_as_valid_evidence() {
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), false);
    let metadata = metadata();
    assert!(
        native_ingest::publish_rows(
            &repository,
            metadata.clone(),
            evidence(&metadata),
            None,
            None
        )
        .await
        .is_err()
    );
    assert_eq!(
        repository
            .catalog
            .pin()
            .await
            .expect("catalog")
            .generation(),
        0
    );
}

#[tokio::test]
async fn overview_pages_conflicting_feature_definitions_without_overwriting_sources() {
    use enrichment_core::evidence::{
        FragmentKind,
        relational::{SubjectRef, TextFragment},
    };
    let dir = tempfile::tempdir().unwrap();
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let mut evidence = evidence(&metadata);
    let source = evidence.fragments[0].source.clone();
    for value in ["a", "b"] {
        evidence.fragments.push(
            TextFragment::new(
                FragmentKind::FeatureDefinition,
                SubjectRef::Feature {
                    name: "feature".into(),
                },
                "feature".into(),
                value.into(),
                source.clone(),
            )
            .unwrap(),
        );
    }
    let manifest = native_ingest::publish_rows(&repository, metadata, evidence, None, None)
        .await
        .unwrap();
    let reader = enrichment_store::SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &manifest.snapshot_id,
    )
    .await
    .unwrap();
    reader.overview(None, 8).await.unwrap();
    let first = reader
        .discovery_page(FragmentKind::FeatureDefinition, 1, None, None)
        .await
        .unwrap();
    assert!(first.has_more);
    let second = reader
        .discovery_page(
            FragmentKind::FeatureDefinition,
            1,
            first.next_key.as_deref(),
            None,
        )
        .await
        .unwrap();
    assert!(!second.has_more);
    let rows = [first.items[0].0.clone(), second.items[0].0.clone()];
    assert_ne!(rows[0].fragment_id, rows[1].fragment_id);
    assert_eq!(
        rows.iter()
            .map(|row| row.text.as_str())
            .collect::<std::collections::BTreeSet<_>>(),
        ["a", "b"].into()
    );
    assert!(
        rows.iter()
            .all(|row| row.source.artifact_id == source.artifact_id)
    );
}
