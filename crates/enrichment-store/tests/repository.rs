mod support;

use enrichment_core::{
    evidence::{Artifact, ArtifactKind, ingest::EvidenceBatch, snapshot::SnapshotMetadata},
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

fn evidence(metadata: &SnapshotMetadata) -> EvidenceBatch {
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
                    "2026-09-14T00:00:00Z",
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
async fn cleanup_is_excluded_until_catalog_plan_and_exhausted_stream_are_dropped() {
    use enrichment_store::{SnapshotReader, leases};
    use futures::StreamExt;
    let dir = tempfile::tempdir().expect("directory");
    let repository = repository(dir.path(), true);
    let metadata = metadata();
    let manifest = repository
        .publish(metadata.clone(), evidence(&metadata), None)
        .await
        .expect("publish");
    let root = dir.path().join("data");
    drop(leases::exclusive(&root).expect("idle caches hold no lease"));
    for sql in [
        "SELECT symbol_id FROM symbols ORDER BY symbol_id",
        "SELECT symbol_id FROM api_surface ORDER BY symbol_id",
        "SELECT context_id FROM contexts ORDER BY context_id",
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
        let session = if sql.contains("FROM contexts") {
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
    use enrichment_store::catalog_generation::{CatalogDelta, RelationalCatalog, SelectionChange};
    for unrelated in [false, true] {
        let dir = tempfile::tempdir().expect("directory");
        let repository = repository(dir.path(), true);
        let metadata = metadata();
        let evidence = evidence(&metadata);
        let manifest = repository
            .publish(metadata.clone(), evidence.clone(), None)
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
        let catalog = RelationalCatalog::open(&dir.path().join("data"), repository.runtime.clone())
            .expect("catalog");
        let mut run = evidence.producer_runs[0].clone();
        run.config_digest = "unrelated-producer-configuration".into();
        catalog
            .commit(CatalogDelta {
                publication: None,
                releases: vec![metadata.release],
                environments: vec![metadata.environment],
                contexts: vec![metadata.context.clone()],
                snapshots: vec![entry],
                attempts: if unrelated {
                    vec![SnapshotAttempt {
                        snapshot_id: manifest.snapshot_id.clone(),
                        artifacts: evidence.attempt_artifacts[&run.attempt_id].clone(),
                        run,
                    }]
                } else {
                    vec![]
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
    let original = repository
        .publish(metadata.clone(), evidence, None)
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
                .table("api_observations")
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
    let before = repository
        .publish(metadata.clone(), original, None)
        .await
        .expect("before");
    let after = repository
        .publish(metadata, changed, Some(before.snapshot_id.clone()))
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
    let equal = comparison::page(&before, &before, &scopes, None, 10)
        .await
        .expect("same snapshot");
    assert_eq!(equal.total, 0);
    assert!(equal.changes.is_empty());
    let all = comparison::page(&before, &after, &scopes, None, 100)
        .await
        .expect("native comparison");
    assert!(all.total > 0);
    assert!(all.changes.iter().all(|(_, c)| c.scope == Scope::Api
        && !c.before_sources.is_empty()
        && !c.after_sources.is_empty()));
    assert!(all.changes.iter().any(|(_, c)| {
        c.after
            .as_ref()
            .expect("after")
            .to_string()
            .contains("fn changed")
    }));
    let mut cursor = None;
    let mut seen = Vec::new();
    loop {
        let page = comparison::page(&before, &after, &[Scope::Api], cursor.as_ref(), 1)
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
            RelationKind, Symbol,
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
    definition.definition_id = Symbol::definition_id_for(
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
    let baseline = repository
        .publish(metadata.clone(), evidence.clone(), None)
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
        let after = repository
            .publish(metadata.clone(), changed, current)
            .await
            .expect("changed endpoint");
        let catalog = repository.catalog.pin().await.expect("catalog");
        let before = SnapshotReader::open(&repository, catalog.clone(), &baseline.snapshot_id)
            .await
            .expect("before");
        let after = SnapshotReader::open(&repository, catalog, &after.snapshot_id)
            .await
            .expect("after");
        let delta = comparison::page(&before, &after, &[Scope::Relationships], None, 10)
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
    let manifest = repository
        .publish(metadata.clone(), evidence(&metadata), None)
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
                .sql("SELECT sha256 FROM input_artifacts LIMIT 1")
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
    let manifest = repository
        .publish(metadata, evidence, None)
        .await
        .expect("publication");
    assert_eq!(manifest.schema_version, "5.0");
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
                .sql("SELECT observation_id FROM api_observations")
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
    let first = repository
        .publish(metadata.clone(), evidence(&metadata), None)
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
                "2026-09-14T01:00:00Z",
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
    let second = repository
        .publish(metadata.clone(), again, Some(first.snapshot_id.clone()))
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
    let first = repository
        .publish(metadata.clone(), evidence(&metadata), None)
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
    again.producer_runs[0].started_at = "2026-09-15T00:00:00Z".into();
    again.producer_runs[0].finished_at = "2026-09-15T00:00:01Z".into();
    let second = repository
        .publish(metadata, again, Some(first.snapshot_id.clone()))
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
                    .sql("SELECT association_id FROM attempts")
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
        repository.publish(metadata.clone(), left, None),
        repository.publish(metadata.clone(), right, None)
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
                    .sql("SELECT fragment_id FROM fragments")
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
                    .sql("SELECT attempt_id FROM producer_runs")
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
        repository
            .publish(metadata.clone(), evidence(&metadata), None)
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
async fn overview_refuses_to_overwrite_conflicting_feature_definitions() {
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
    let manifest = repository.publish(metadata, evidence, None).await.unwrap();
    let reader = enrichment_store::SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &manifest.snapshot_id,
    )
    .await
    .unwrap();
    let error = match reader.overview(None, 8).await {
        Ok(_) => panic!("conflicting values cannot become a single feature fact"),
        Err(error) => error,
    };
    assert!(error.is_budget(), "{error}");
    assert!(
        error
            .to_string()
            .contains("conflicting qualified definitions")
    );
}
