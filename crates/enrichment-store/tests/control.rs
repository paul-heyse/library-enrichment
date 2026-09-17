use enrichment_core::{
    evidence::catalog::SnapshotEntry,
    identity::{Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode, SnapshotId},
};
use enrichment_store::{
    control::{CommitOutcome, ControlBatch, ControlStore, SelectionChange},
    projection::catalog as projection,
    runtime::{QueryLimits, QueryRuntime},
};

fn runtime(root: &std::path::Path) -> QueryRuntime {
    QueryRuntime::new(&root.join("spill"), QueryLimits::default()).expect("runtime")
}

fn candidate(name: &str, revision: &str) -> (ControlBatch, Context, SnapshotId) {
    let release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: "crates.io".into(),
        package: name.into(),
        version: "1.0.0".into(),
        artifact_digest: None,
    });
    let environment = Environment::unspecified();
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        ResearchMode::Upstream,
    );
    let metadata = enrichment_core::evidence::snapshot::SnapshotMetadata {
        context: context.clone(),
        release: release.clone(),
        environment: environment.clone(),
        symbol_package: name.into(),
        crate_name: name.into(),
        crate_version: Some("1.0.0".into()),
        normalizer_version: "control-schema-fixture/1".into(),
        observed_configuration: None,
        producer_items: 0,
    }
    .descriptor();
    let components = [("fixture".into(), revision.into())].into_iter().collect();
    let snapshot_id =
        enrichment_core::evidence::snapshot::EvidenceManifest::derive_id(&metadata, &components)
            .unwrap();
    let publication = enrichment_core::evidence::snapshot::EvidenceManifest {
        snapshot_id: snapshot_id.clone(),
        schema_version: enrichment_core::evidence::snapshot::FORMAT.into(),
        metadata,
        components,
        tables: vec![],
        counts: Default::default(),
        indexed: vec![],
        missing: vec![],
        published_at: enrichment_core::native_time::ObservationTime::try_from(
            "2026-09-15T00:00:00.000000Z".to_owned(),
        )
        .unwrap(),
    };
    let delta = ControlBatch {
        search_projections: vec![],
        publication_fence: None,
        publication: None,
        comparison: None,
        releases: vec![release],
        environments: vec![environment],
        contexts: vec![context.clone()],
        snapshots: vec![SnapshotEntry {
            snapshot_id: snapshot_id.clone(),
            context_id: context.context_id.clone(),
            publication,
        }],
        attempts: None,
        selection: Some(SelectionChange {
            context_id: context.context_id.clone(),
            snapshot_id: snapshot_id.clone(),
            expected_base: None,
        }),
    };
    (delta, context, snapshot_id)
}

#[tokio::test]
async fn compaction_preserves_all_snapshot_membership_and_pinned_selections() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = QueryRuntime::new(
        &dir.path().join("spill"),
        QueryLimits {
            batch_rows: 1,
            native: enrichment_core::config::NativeQueryConfig {
                row_group_rows: 2,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();
    let catalog = ControlStore::open(dir.path(), runtime.clone()).expect("catalog");
    let (first, context, first_id) = candidate("library", "first");
    catalog.commit(first).await.expect("first");
    let original = catalog.pin().await.expect("original pin");
    let mut base = first_id.clone();
    let mut ids = vec![first_id.clone()];
    for revision in ["second", "third", "fourth"] {
        let (mut delta, _, id) = candidate("library", revision);
        delta.selection.as_mut().expect("selection").expected_base = Some(base);
        catalog.commit(delta).await.expect("successor");
        base = id.clone();
        ids.push(id);
    }
    let before = catalog.pin().await.expect("before compaction");
    let generation = catalog.compact().await.expect("native compaction");
    let after = catalog.pin().await.expect("after compaction");
    assert_eq!(generation, before.generation() + 1);
    assert!(after.file_count() < before.file_count());

    assert_eq!(
        after
            .current(&runtime, &context.context_id)
            .await
            .expect("current"),
        Some(base)
    );
    assert_eq!(
        original
            .current(&runtime, &context.context_id)
            .await
            .expect("pinned current"),
        Some(first_id)
    );
    for id in ids {
        assert!(
            after
                .snapshot(&runtime, &id)
                .await
                .expect("membership")
                .is_some()
        );
    }
    let cold = ControlStore::read_only(dir.path(), runtime.clone()).expect("read only");
    assert_eq!(
        cold.pin().await.expect("cold admission").identity(),
        after.identity()
    );
}

#[tokio::test]
async fn concurrent_distinct_context_commits_survive_and_old_readers_remain_pinned() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let catalog = ControlStore::open(dir.path(), runtime.clone()).expect("catalog");
    let empty = catalog.pin().await.expect("empty pin");
    let (a, ca, sa) = candidate("a", "a");
    let (b, cb, sb) = candidate("b", "b");
    let (a, b) = tokio::join!(catalog.commit(a), catalog.commit(b));
    assert!(matches!(
        a.expect("commit a"),
        CommitOutcome::Committed { .. }
    ));
    assert!(matches!(
        b.expect("commit b"),
        CommitOutcome::Committed { .. }
    ));
    let pin = catalog.pin().await.expect("pin");
    assert_eq!(pin.generation(), 2);
    assert_eq!(
        pin.current(&runtime, &ca.context_id)
            .await
            .expect("selection"),
        Some(sa)
    );
    assert_eq!(
        pin.current(&runtime, &cb.context_id)
            .await
            .expect("selection"),
        Some(sb)
    );
    assert_eq!(
        empty
            .current(&runtime, &ca.context_id)
            .await
            .expect("old selection"),
        None
    );
    let reopened = ControlStore::open(dir.path(), runtime.clone()).expect("reopen");
    let cold = reopened.pin().await.expect("cold pin");
    let session = cold.session(&runtime).await.expect("session");
    let rows = runtime.execute(session.sql("SELECT r.* FROM state.records.releases r JOIN state.records.contexts c ON r.release_id = c.release_id ORDER BY r.package").await.expect("plan")).await.expect("execute");
    let releases = rows
        .batches
        .iter()
        .flat_map(|b| projection::releases_from_batch(b).expect("decode"))
        .collect::<Vec<_>>();
    assert_eq!(
        releases
            .iter()
            .map(|r| r.key.package.as_str())
            .collect::<Vec<_>>(),
        vec!["a", "b"]
    );
}

#[tokio::test]
async fn stale_same_context_candidate_remains_unpublished() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let catalog = ControlStore::open(dir.path(), runtime.clone()).expect("catalog");
    let (first, context, selected) = candidate("a", "first");
    catalog.commit(first).await.expect("first");
    let (stale, _, stale_id) = candidate("a", "stale");
    assert!(
        matches!(catalog.commit(stale).await.expect("stale candidate"), CommitOutcome::Conflict { current: Some(id), .. } if id == selected)
    );
    let pin = catalog.pin().await.expect("pin");
    assert_eq!(
        pin.current(&runtime, &context.context_id)
            .await
            .expect("selected"),
        Some(selected.clone())
    );
    let session = pin.session(&runtime).await.expect("session");
    assert_eq!(
        runtime
            .execute(
                session
                    .sql("SELECT snapshot_id FROM state.records.snapshots")
                    .await
                    .expect("plan")
            )
            .await
            .expect("execute")
            .rows,
        1
    );
    let (mut rebased, _, _) = candidate("a", "union-of-first-and-stale");
    rebased.selection.as_mut().expect("selection").expected_base = Some(selected);
    assert!(matches!(
        catalog.commit(rebased).await.expect("fresh precondition"),
        CommitOutcome::Committed { .. }
    ));
    assert_ne!(
        catalog
            .pin()
            .await
            .expect("pin")
            .current(&runtime, &context.context_id)
            .await
            .expect("current"),
        Some(stale_id)
    );
}

#[tokio::test]
async fn invalid_closure_leaves_root_unchanged_and_unreferenced_files_are_invisible() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let catalog = ControlStore::open(dir.path(), runtime.clone()).expect("catalog");
    let (valid, context, selected) = candidate("valid", "first");
    catalog.commit(valid).await.expect("valid");
    let before = catalog.pin().await.unwrap().identity().to_owned();
    let (mut invalid, _, _) = candidate("invalid", "orphan");
    invalid.releases.clear();
    assert!(catalog.commit(invalid).await.is_err());
    assert_eq!(before, catalog.pin().await.unwrap().identity());
    let reopened = ControlStore::open(dir.path(), runtime.clone()).expect("reopen");
    let pin = reopened.pin().await.expect("pin");
    assert_eq!(
        pin.current(&runtime, &context.context_id)
            .await
            .expect("selection"),
        Some(selected)
    );
    let session = pin.session(&runtime).await.expect("session");
    assert_eq!(
        runtime
            .execute(
                session
                    .sql("SELECT release_id FROM state.records.releases")
                    .await
                    .expect("plan")
            )
            .await
            .expect("execute")
            .rows,
        1
    );
}

#[tokio::test]
async fn missing_exact_catalog_file_is_an_error_for_cold_and_warm_readers() {
    let dir = tempfile::tempdir().expect("directory");
    let runtime = runtime(dir.path());
    let catalog = ControlStore::open(dir.path(), runtime.clone()).expect("catalog");
    catalog
        .commit(candidate("a", "first").0)
        .await
        .expect("commit");
    let pin = catalog.pin().await.expect("pin");

    let native =
        enrichment_store::native_delta::DeltaStore::new(&dir.path().join("delta"), runtime.clone())
            .unwrap();
    let table = native.load("control", None).await.unwrap();
    let uri = table.get_file_uris().unwrap().next().unwrap();
    let file = std::path::PathBuf::from(uri);
    std::fs::rename(&file, file.with_extension("parquet.extra")).expect("hide exact file");
    // Native providers resolve exact paths when executed, not by eager directory enumeration.
    for snapshot in [
        pin,
        catalog.pin().await.unwrap(),
        ControlStore::open(dir.path(), runtime.clone())
            .unwrap()
            .pin()
            .await
            .unwrap(),
    ] {
        let session = snapshot.session(&runtime).await.unwrap();
        let frame = session.table("state.records.releases").await.unwrap();
        assert!(runtime.execute(frame).await.is_err());
    }
}
