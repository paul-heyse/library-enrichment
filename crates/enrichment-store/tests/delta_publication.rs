use support::native_ingest::{self, EvidenceRows};
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
        metadata.release.release_id.clone(),
        metadata.environment.environment_id.clone(),
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

#[test]
fn publication_selects_native_delta_vector_and_exports_cohorts() {
    let root = std::sync::Arc::new(tempfile::tempdir().unwrap());
    let repo = repository(root.path(), true);
    let runtime = repo.runtime.clone();
    // Match the daemon's owned native entry point. Arrow construction and native
    // logical planning execute on the configured workers as well as physical I/O.
    let journey_root = root.clone();
    runtime
        .bootstrap(async move { publication_journey(journey_root, repo).await })
        .unwrap();
    let close = runtime.clone();
    runtime
        .bootstrap(async move { close.close_diagnostics().await })
        .unwrap()
        .unwrap();
}

async fn publication_journey(root: std::sync::Arc<tempfile::TempDir>, repo: EvidenceRepository) {
    let metadata = metadata();
    let publication =
        native_ingest::publish_rows(&repo, metadata.clone(), evidence(&metadata), None, None)
            .await
            .unwrap();
    assert_eq!(publication.tables.len(), 10);
    assert!(!root.path().join("data/snapshots").exists());
    let pin = repo.catalog.pin().await.unwrap();
    let opened = repo
        .open_snapshot(pin.clone(), &publication.snapshot_id)
        .await
        .unwrap();
    let session = opened.session(&repo.runtime).unwrap();
    let symbols = repo
        .runtime
        .execute(session.table("snapshot.evidence.symbols").await.unwrap())
        .await
        .unwrap();
    assert!(symbols.rows > 0);
    let selected = pin
        .snapshot(&repo.runtime, &publication.snapshot_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(selected.publication, publication);
    let mut forged = publication.clone();
    forged.tables[0].source.table.table_id = "foreign-table".into();
    let native = enrichment_store::delta_evidence::EvidenceTables::new(
        &root.path().join("data/delta"),
        repo.runtime.clone(),
    )
    .unwrap();
    let protection = enrichment_store::leases::ReadProtection::Durable(
        repo.retention()
            .enroll(
                "forged-provider-check".into(),
                enrichment_store::retention::ProtectionKind::Query,
                publication
                    .tables
                    .iter()
                    .map(|binding| enrichment_store::retention::Dependency::Table {
                        value: binding.selection(),
                    })
                    .collect(),
            )
            .await
            .unwrap(),
    );
    assert!(native.providers(&forged.tables, protection).await.is_err());
    // A private cohort and an actual native OPTIMIZE commit lie inside the next CDF window.
    // Neither changes the selected publication's semantic search contents.
    let fragments = publication
        .tables
        .iter()
        .find(|b| b.relation == "fragments")
        .unwrap();
    native
        .append(
            enrichment_store::admission::Relation::Fragments,
            &enrichment_core::identity::CohortId::new(),
            session.table("snapshot.evidence.fragments").await.unwrap(),
            fragments.rows,
        )
        .await
        .unwrap();
    let (_, metrics) = native
        .compact(
            enrichment_store::admission::Relation::Fragments,
            &repo.retention(),
        )
        .await
        .unwrap();
    assert!(metrics.num_files_removed >= 2);
    assert!(metrics.num_files_added > 0);

    let mut updated = evidence(&metadata);
    let row = updated.fragments[0].clone();
    updated.fragments[0] = enrichment_core::evidence::relational::TextFragment::new(
        row.kind,
        row.subject,
        row.display_subject,
        format!("{} Additional exact fixture evidence.", row.text),
        row.source,
    )
    .unwrap();
    let successor = native_ingest::publish_rows(
        &repo,
        metadata.clone(),
        updated,
        Some(publication.snapshot_id.clone()),
        None,
    )
    .await
    .unwrap();
    let pin = repo.catalog.pin().await.unwrap();
    assert_eq!(
        pin.previous(&repo.runtime, &successor.snapshot_id)
            .await
            .unwrap(),
        Some(publication.snapshot_id.clone())
    );
    let changes = repo
        .publication_changes(&publication, &successor)
        .await
        .unwrap();
    let fragments = changes
        .iter()
        .find(|row| row.relation == "fragments")
        .unwrap();
    assert_eq!(
        (fragments.inserted, fragments.removed, fragments.updated),
        (1, 1, 0)
    );
    assert_eq!(changes.len(), 1);
    let checkpoint = pin
        .search_projection(&repo.runtime, &successor.snapshot_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        checkpoint.mode,
        enrichment_store::search_projection::ProjectionMode::Incremental
    );
    assert_eq!(checkpoint.inputs, successor.tables);
    assert_eq!(checkpoint.outputs.len(), 2);
    let opened = repo
        .open_snapshot(pin.clone(), &successor.snapshot_id)
        .await
        .unwrap();
    let materialized = opened.research_session(&repo.runtime).await.unwrap();
    let recomputed = opened
        .binding
        .research_session(&repo.runtime, None)
        .await
        .unwrap();
    for name in ["api_surface", "fragment_surface"] {
        let reference = format!("snapshot.domain.{name}");
        let left = materialized.table(reference.clone()).await.unwrap();
        let right = recomputed.table(reference).await.unwrap();
        let difference = left
            .clone()
            .except(right.clone())
            .unwrap()
            .union(right.except(left).unwrap())
            .unwrap();
        assert_eq!(
            repo.runtime.execute(difference).await.unwrap().rows,
            0,
            "{name}"
        );
    }

    let destination = root.path().join("export");
    enrichment_store::bundle::export(
        &StatePaths {
            data_root: root.path().join("data"),
            cache_root: root.path().join("cache"),
        },
        metadata.context.context_id.to_string().as_str(),
        &destination,
    )
    .await
    .unwrap();
    assert!(!destination.join("data/snapshots").exists());

    // Retain the target snapshot with a native checkpoint, then expire a required CDF log.
    // This distinguishes a missing feed interval from an unavailable target publication.
    let delta = enrichment_store::native_delta::DeltaStore::new(
        &root.path().join("data/delta"),
        repo.runtime.clone(),
    )
    .unwrap();
    let fragment_binding = successor
        .tables
        .iter()
        .find(|b| b.relation == "fragments")
        .unwrap();
    let table = delta
        .load(
            &fragment_binding.source.table.table_uri,
            Some(fragment_binding.source.version),
        )
        .await
        .unwrap();
    deltalake::protocol::checkpoints::create_checkpoint(&table, None)
        .await
        .unwrap();
    std::fs::remove_file(
        root.path()
            .join("data/delta")
            .join(&fragment_binding.source.table.table_uri)
            .join("_delta_log")
            .join(format!("{:020}.json", fragment_binding.source.version)),
    )
    .unwrap();
    let initial = pin
        .search_projection(&repo.runtime, &publication.snapshot_id)
        .await
        .unwrap()
        .unwrap();
    let search = enrichment_store::search_projection::SearchProjection::new(
        &root.path().join("data/delta"),
        repo.runtime.clone(),
    )
    .unwrap();
    let mut prior_definition = initial.clone();
    let retention = enrichment_store::retention::RetentionStore::new(
        repo.catalog.clone(),
        repo.runtime.clone(),
    );
    prior_definition.revision = "previous-target-definition".into();
    let revision_rebuild = search
        .prepare(
            &successor,
            &recomputed,
            Some(&prior_definition),
            false,
            &retention,
        )
        .await
        .unwrap();
    assert_eq!(
        revision_rebuild.checkpoint.mode,
        enrichment_store::search_projection::ProjectionMode::RevisionRebuild
    );
    assert_eq!(
        revision_rebuild.checkpoint.revision,
        enrichment_store::search_projection::revision()
    );
    let rebuilt = search
        .prepare(&successor, &recomputed, Some(&initial), false, &retention)
        .await
        .unwrap();
    assert_eq!(
        rebuilt.checkpoint.mode,
        enrichment_store::search_projection::ProjectionMode::HistoryRebuild
    );
    // Output commits alone cannot select the candidate or advance its source offsets.
    let still_selected = repo
        .catalog
        .pin()
        .await
        .unwrap()
        .search_projection(&repo.runtime, &successor.snapshot_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(still_selected, checkpoint);
    repo.catalog
        .commit(enrichment_store::control::ControlBatch {
            search_projections: vec![rebuilt.checkpoint],
            ..Default::default()
        })
        .await
        .unwrap();
    let rebuilt_pin = repo.catalog.pin().await.unwrap();
    let selected = rebuilt_pin
        .search_projection(&repo.runtime, &successor.snapshot_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        selected.mode,
        enrichment_store::search_projection::ProjectionMode::HistoryRebuild
    );
    assert_eq!(selected.inputs, successor.tables);
    assert!(selected.sequence > checkpoint.sequence);
    let rebuilt_open = repo
        .open_snapshot(rebuilt_pin, &successor.snapshot_id)
        .await
        .unwrap();
    let rebuilt_session = rebuilt_open.research_session(&repo.runtime).await.unwrap();
    for name in ["api_surface", "fragment_surface"] {
        let reference = format!("snapshot.domain.{name}");
        let left = rebuilt_session.table(reference.clone()).await.unwrap();
        let right = recomputed.table(reference).await.unwrap();
        let difference = left
            .clone()
            .except(right.clone())
            .unwrap()
            .union(right.except(left).unwrap())
            .unwrap();
        assert_eq!(
            repo.runtime.execute(difference).await.unwrap().rows,
            0,
            "rebuilt {name}"
        );
    }
}
