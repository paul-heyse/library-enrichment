#[path = "support/native_ingest.rs"]
pub mod native_ingest;
use native_ingest::EvidenceRows;
#[path = "support/claims.rs"]
mod claims;
use enrichment_core::{
    canonical,
    evidence::{
        Artifact, ArtifactKind,
        execution::*,
        relational::{FactSource, InputArtifact, Locator, SubjectRef},
        snapshot::SnapshotMetadata,
    },
    execution::{ProbeMode, ProcessEnd},
    identity::{Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode},
    policy::ExecutionProfile,
    producer::{ProducerRun, RunOutcome},
    wire::{EvidenceClass, SourceVersionMatch},
};
use enrichment_store::{
    BlobStore, SnapshotReader, StatePaths,
    admission::AdmissionLimits,
    dataset::WriteLimits,
    projection,
    repository::EvidenceRepository,
    runtime::{QueryLimits, QueryRuntime},
};

fn metadata() -> SnapshotMetadata {
    let release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Python,
        registry: "pypi.org".into(),
        package: "fixture".into(),
        version: "1.0".into(),
        artifact_digest: None,
    });
    let environment = Environment::resolved(
        "python-3.14.7".into(),
        "linux-x86_64".into(),
        vec![],
        None,
        canonical::sha256_hex(b"lock"),
    );
    SnapshotMetadata {
        context: Context::new(
            release.release_id.clone(),
            environment.environment_id.clone(),
            ResearchMode::Project,
        ),
        release,
        environment,
        symbol_package: "python:fixture".into(),
        crate_name: "fixture".into(),
        crate_version: Some("1.0".into()),
        normalizer_version: "execution/1".into(),
        observed_configuration: None,
        producer_items: 0,
    }
}
fn evidence(blobs: &BlobStore, metadata: &SnapshotMetadata) -> EvidenceRows {
    evidence_with_position(blobs, metadata, Utf8Position { line: 1, byte: 3 })
}
fn evidence_with_position(
    blobs: &BlobStore,
    metadata: &SnapshotMetadata,
    position: Utf8Position,
) -> EvidenceRows {
    let put = |bytes: &[u8], uri: &str| {
        blobs
            .put(bytes, |_| {
                Artifact::describe(
                    bytes,
                    ArtifactKind::Other,
                    "application/json",
                    uri,
                    enrichment_core::native_time::AcquisitionTime::try_from(
                        "2026-09-14T00:00:00.000000Z".to_owned(),
                    )
                    .unwrap(),
                )
            })
            .unwrap()
            .acquired
    };
    let input = put("# 😀\nfixture.f()\n".as_bytes(), "consumer://fixture/1");
    let lock = put(b"lock", "consumer://fixture/lock");
    let range = Utf8Range {
        start: Utf8Position { line: 1, byte: 0 },
        end: Utf8Position { line: 1, byte: 7 },
    };
    let payloads = vec![
        ExecutionPayload::SemanticQuery(SemanticQuery {
            method: SemanticMethod::Definition,
            document_artifact_id: input.artifact_id.clone(),
            position: Some(position),
            anchor_symbol_id: None,
            server: "ty 0.0.80".into(),
            outcome: ExecutionOutcome::Results,
            hover: None,
            locations: vec![
                ExecutionTarget::Artifact {
                    artifact_id: input.artifact_id.clone(),
                    range,
                },
                ExecutionTarget::External {
                    scope: "image:fixture".into(),
                    path: "stdlib/builtins.pyi".into(),
                    limitation: "external document not retained".into(),
                },
            ],
            diagnostics: vec![],
            limitations: vec![],
        }),
        ExecutionPayload::SemanticQuery(SemanticQuery {
            method: SemanticMethod::Diagnostics,
            document_artifact_id: input.artifact_id.clone(),
            position: None,
            anchor_symbol_id: None,
            server: "ty 0.0.80".into(),
            outcome: ExecutionOutcome::Results,
            hover: None,
            locations: vec![],
            diagnostics: vec![ExecutionDiagnostic {
                range,
                severity: Some(1),
                code: Some("unresolved-reference".into()),
                source: Some("ty".into()),
                message: "fixture is not imported".into(),
            }],
            limitations: vec![],
        }),
        ExecutionPayload::RuntimeObject(RuntimeObject {
            module: "fixture".into(),
            selection: vec!["f".into()],
            outcome: ExecutionOutcome::Results,
            type_name: Some("builtins.function".into()),
            signature: Some("(x, /)".into()),
            docstring: Some(String::new()),
            attributes: vec![],
            limitations: vec![],
        }),
        ExecutionPayload::UsageProbe(UsageProbe {
            mode: ProbeMode::Runtime,
            snippet_artifact_id: input.artifact_id.clone(),
            end: ProcessEnd::Exited,
            exit_code: Some(1),
            stdout: String::new(),
            stderr: "NameError: fixture is not defined\n".into(),
        }),
    ];
    let mut evidence = EvidenceRows::default();
    for (index, payload) in payloads.into_iter().enumerate() {
        let result = put(
            &payload.canonical_bytes().unwrap(),
            "producer-result://execution/1",
        );
        let runtime = matches!(
            payload,
            ExecutionPayload::RuntimeObject(_) | ExecutionPayload::UsageProbe(_)
        );
        let receipt = put(
            format!("fixture attempt {index}").as_bytes(),
            &format!("attempt://fixture/{index}"),
        );
        let run = ProducerRun {
            attempt_id: format!("attempt-{index}"),
            producer: "execution-fixture".into(),
            producer_version: "1".into(),
            config_digest: enrichment_core::canonical::sha256_hex(b"fixture"),
            inputs: [
                ("document".into(), input.sha256.clone()),
                ("lock".into(), lock.sha256.clone()),
                ("result".into(), result.sha256.clone()),
            ]
            .into(),
            profile: if runtime {
                ExecutionProfile::Runtime
            } else {
                ExecutionProfile::Build
            },
            started_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-14T00:00:00.000000Z".to_owned(),
            )
            .unwrap(),
            finished_at: enrichment_core::native_time::ObservationTime::try_from(
                "2026-09-14T00:00:01.000000Z".to_owned(),
            )
            .unwrap(),
            outcome: RunOutcome::Succeeded,
            gaps: vec![],
            log: Some(receipt.artifact_id.clone()),
        };
        let binding = run.semantic_binding_id();
        for (role, artifact) in [("document", &input), ("lock", &lock), ("result", &result)] {
            evidence
                .input_artifacts
                .push(InputArtifact::new(binding.clone(), role.into(), artifact).unwrap());
        }
        evidence.execution_observations.push(
            ExecutionObservation::new(
                SubjectRef::Document {
                    artifact_id: input.artifact_id.clone(),
                    heading: "consumer".into(),
                },
                metadata.environment.environment_id.to_string(),
                format!("sha256:{}", "a".repeat(64)),
                "b".repeat(64),
                payload,
                FactSource {
                    producer_binding_id: binding,
                    extractor: "execution".into(),
                    extractor_version: "1".into(),
                    artifact_id: result.artifact_id.clone(),
                    source_uri: Some(result.source_uri.clone()),
                    source_version_match: SourceVersionMatch::Exact,
                    locator: Locator::Artifact,
                    evidence_class: if runtime {
                        EvidenceClass::RuntimeObserved
                    } else {
                        EvidenceClass::TypecheckerObserved
                    },
                },
            )
            .unwrap(),
        );
        evidence.attempt_artifacts.insert(
            run.attempt_id.clone(),
            vec![input.clone(), lock.clone(), result, receipt],
        );
        evidence.producer_runs.push(run);
    }
    evidence
}

#[tokio::test]
async fn typed_execution_roundtrip_reuse_native_queries_and_complete_export() {
    let dir = tempfile::tempdir().unwrap();
    let paths = StatePaths {
        data_root: dir.path().join("data"),
        cache_root: dir.path().join("cache"),
    };
    let blobs = BlobStore::open(&paths.data_root).unwrap();
    let metadata = metadata();
    let mut evidence = evidence(&blobs, &metadata);
    let batch = projection::execution::encode(&evidence.execution_observations).unwrap();
    assert_eq!(
        batch.schema(),
        projection::execution::encode(&[]).unwrap().schema()
    );
    assert_eq!(
        projection::execution::decode(&batch).unwrap(),
        evidence.execution_observations
    );
    let runtime =
        QueryRuntime::new(&paths.cache_root.join("spill"), QueryLimits::default()).unwrap();
    let repository = EvidenceRepository::new(
        paths.clone(),
        runtime,
        WriteLimits::default(),
        AdmissionLimits::default(),
    )
    .unwrap();
    use enrichment_core::evidence::{
        SymbolHeader, SymbolKind,
        path::PublicPath,
        relational::{Definition, PublicBinding},
    };
    let definition = Definition {
        definition_id: SymbolHeader::definition_id_for(
            "python:fixture",
            "fixture.f",
            SymbolKind::Function,
            None,
        ),
        definition_path: "fixture.f".into(),
        kind: SymbolKind::Function,
        defined_in_package: "python:fixture".into(),
        qualifier: None,
    };
    let path = PublicPath::parse(Ecosystem::Python, "fixture.f").unwrap();
    let symbol = PublicBinding {
        symbol_id: PublicBinding::id_for("python:fixture", &path, SymbolKind::Function, None),
        definition_id: definition.definition_id.clone(),
        path,
        name: "f".into(),
        is_reexport: false,
        qualifier: None,
    };
    native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        EvidenceRows {
            definitions: vec![definition],
            symbols: vec![symbol.clone()],
            ..Default::default()
        },
        None,
        None,
    )
    .await
    .unwrap();
    let object = evidence.execution_observations[2].clone();
    evidence.execution_observations[2] = ExecutionObservation::new(
        SubjectRef::Symbol {
            symbol_id: symbol.symbol_id.clone(),
        },
        object.environment_id,
        object.image_id,
        object.containment_identity,
        object.payload,
        object.source,
    )
    .unwrap();
    // This delta intentionally omits its referenced static symbol. Native assembly must add
    // the pinned base before full foreign-key admission, without publishing a dangling delta.
    let manifest =
        native_ingest::publish_rows(&repository, metadata.clone(), evidence.clone(), None, None)
            .await
            .unwrap();
    let reader = SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &manifest.snapshot_id,
    )
    .await
    .unwrap();
    let retained = reader.execution_observations(None, None).await.unwrap();
    assert_eq!(retained.len(), 4);
    assert_eq!(reader.runtime().execute(reader.session().sql("SELECT payload.runtime_object.signature FROM snapshot.evidence.execution_observations WHERE payload.kind = 'runtime_object'").await.unwrap()).await.unwrap().rows,1);
    use enrichment_store::query::ExecutionSelection;
    let selection = enrichment_core::request::RuntimeSelection {
        module: "fixture".into(),
        attributes: vec!["f".into()],
    };
    assert_eq!(
        reader
            .execution_selection(ExecutionSelection {
                runtime: Some(&selection),
                ..Default::default()
            })
            .await
            .unwrap()
            .len(),
        1
    );
    let absent = enrichment_core::request::RuntimeSelection {
        module: "fixture".into(),
        attributes: vec!["different".into()],
    };
    assert!(
        reader
            .execution_selection(ExecutionSelection {
                runtime: Some(&absent),
                ..Default::default()
            })
            .await
            .unwrap()
            .is_empty()
    );
    let queries = reader
        .execution_selection(ExecutionSelection {
            methods: &[SemanticMethod::Diagnostics],
            position: Some(Utf8Position { line: 0, byte: 0 }),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(
        queries.len(),
        1,
        "diagnostics are selected independently of a cursor position"
    );
    let mut reordered = evidence.clone();
    reordered.execution_observations.reverse();
    reordered.input_artifacts.reverse();
    let again = native_ingest::publish_rows(&repository, metadata.clone(), reordered, None, None)
        .await
        .unwrap();
    assert_eq!(again.snapshot_id, manifest.snapshot_id);
    let bundle = dir.path().join("bundle");
    enrichment_store::bundle::export(&paths, metadata.context.context_id.as_str(), &bundle)
        .await
        .unwrap();
    enrichment_store::bundle::verify(&bundle).await.unwrap();
    drop(reader);
    drop(repository);
    std::fs::remove_dir_all(&paths.data_root).unwrap();
    enrichment_store::bundle::verify(&bundle).await.unwrap();
}

#[test]
fn execution_identity_is_nonrecursive_and_malformed_payloads_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    let evidence = evidence(&BlobStore::open(dir.path()).unwrap(), &metadata());
    let mut observation = evidence.execution_observations[0].clone();
    let before = observation.observation_id.clone();
    observation.source.evidence_class = EvidenceClass::StaticallyExtracted;
    assert!(observation.validate().is_err());
    let mut observation = evidence.execution_observations[0].clone();
    if let ExecutionPayload::SemanticQuery(query) = &mut observation.payload {
        query.outcome = ExecutionOutcome::Empty;
    }
    assert!(observation.validate().is_err());
    assert_eq!(before, evidence.execution_observations[0].observation_id);
    assert!(
        Utf8Position { line: 0, byte: 3 }
            .validate("# 😀\n")
            .is_err()
    );
    Utf8Position { line: 0, byte: 6 }
        .validate("# 😀\n")
        .unwrap();
    Utf8Position { line: 1, byte: 0 }
        .validate("# 😀\n")
        .unwrap();
    assert!(
        Utf8Position { line: 2, byte: 0 }
            .validate("# 😀\n")
            .is_err()
    );
    assert!(
        serde_json::from_str::<enrichment_core::evidence::relational::ApiOrigin>("\"runtime\"")
            .is_err()
    );
}

#[tokio::test]
async fn catalog_job_publication_is_atomic_recoverable_and_requires_result_closure() {
    publication_case(false, false).await;
}
#[tokio::test]
async fn delivery_failure_prevents_job_catalog_commit() {
    publication_case(true, false).await;
}
#[tokio::test]
async fn exported_job_delivery_has_complete_artifact_closure() {
    publication_case(false, true).await;
}
async fn publication_case(write_failure: bool, export_delivery: bool) {
    use enrichment_core::evidence::catalog::PublishedJobKind;
    use enrichment_store::repository::JobCompletion;
    let dir = tempfile::tempdir().unwrap();
    let paths = StatePaths {
        data_root: dir.path().join("data"),
        cache_root: dir.path().join("cache"),
    };
    let blobs = BlobStore::open(&paths.data_root).unwrap();
    let metadata = metadata();
    let evidence = evidence(&blobs, &metadata);
    let runtime =
        QueryRuntime::new(&paths.cache_root.join("spill"), QueryLimits::default()).unwrap();
    let repository = EvidenceRepository::new(
        paths.clone(),
        runtime.clone(),
        WriteLimits::default(),
        AdmissionLimits::default(),
    )
    .unwrap();
    let job_id = format!("job_{}", "1".repeat(32));
    let dependency_bytes = br#"{"observed":"complete indivisible value","optional":null}"#;
    let dependency = blobs
        .put(dependency_bytes, |_| {
            Artifact::describe(
                dependency_bytes,
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
    let delivery_dependency = dependency.clone();
    let mut result: enrichment_core::wire::Envelope = serde_json::from_str(include_str!(
        "../../../tests/fixtures/wire/error.fixture.json"
    ))
    .unwrap();
    result.summary = "probe failed".into();
    let error = result.error_mut().expect("error fixture");
    error.code = enrichment_core::wire::ErrorCode::VerificationFailed;
    error.message = "probe failed".into();
    result
        .artifacts
        .push(enrichment_core::wire::ArtifactHandle {
            receipt: delivery_dependency.clone(),
            uri: format!(
                "library-evidence://artifacts/{}",
                delivery_dependency.artifact_id
            )
            .try_into()
            .unwrap(),
            description: "Complete observed value".into(),
        });
    let result = enrichment_core::operation::results::ResultRecord::from_envelope(&result).unwrap();
    let publication_fence = claims::publication_fence(
        &repository,
        &job_id,
        enrichment_store::control_jobs::Arguments::Verify {
            request: enrichment_core::execution::VerifyRequest {
                context_id: metadata.context.context_id.to_string(),
                snapshot_id: None,
                snippet: "print(1)".into(),
                mode: enrichment_core::execution::ProbeMode::Runtime,
                profile: ExecutionProfile::Runtime,
                test_intent: None,
                max_bytes: None,
            },
        },
    )
    .await;
    let completion = JobCompletion {
        publication_fence,
        job_id: job_id.clone(),
        result,
        kind: PublishedJobKind::Verify,

        attempt_id: evidence.producer_runs[3].attempt_id.clone(),
        result_artifact_ids: vec![
            evidence.execution_observations[3]
                .source
                .artifact_id
                .clone(),
        ],
    };
    if write_failure {
        let staging = blobs.root().join(".staging");
        std::fs::remove_dir(&staging).unwrap();
        std::fs::write(&staging, b"artifact write must fail").unwrap();
        assert!(
            native_ingest::publish_rows(
                &repository,
                metadata.clone(),
                evidence.clone(),
                None,
                Some(completion.clone())
            )
            .await
            .is_err()
        );
        let catalog = repository.catalog.pin().await.unwrap();
        assert!(
            catalog
                .job_publication(&runtime, &job_id)
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            catalog
                .current(&runtime, &metadata.context.context_id)
                .await
                .unwrap()
                .is_none()
        );
        std::fs::remove_file(&staging).unwrap();
        std::fs::create_dir(staging).unwrap();
    }
    let manifest = native_ingest::publish_rows(
        &repository,
        metadata.clone(),
        evidence.clone(),
        None,
        Some(completion.clone()),
    )
    .await
    .unwrap();
    let publication = repository
        .catalog
        .pin()
        .await
        .unwrap()
        .job_publication(&runtime, &job_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(publication.snapshot_id, manifest.snapshot_id);
    assert_eq!(publication.state, enrichment_core::wire::JobState::Failed);
    if export_delivery {
        let bundle = dir.path().join("bundle");
        enrichment_store::bundle::export(&paths, metadata.context.context_id.as_str(), &bundle)
            .await
            .unwrap();
        assert!(
            enrichment_store::bundle::verify(&bundle)
                .await
                .unwrap()
                .is_empty()
        );
        let delivery_file = bundle
            .join("data/blobs/sha256")
            .join(&publication.delivery.sha256[..2])
            .join(&publication.delivery.sha256);
        assert_eq!(
            std::fs::metadata(&delivery_file).unwrap().len(),
            publication.delivery.size_bytes
        );
        assert!(
            !bundle.join("data/blobs/.staging").exists(),
            "bundle verification is read-only"
        );
        let offline = BlobStore::read_only(&bundle.join("data")).unwrap();
        let native = enrichment_store::control::ControlStore::read_only(
            &bundle.join("data"),
            runtime.clone(),
        )
        .unwrap()
        .pin()
        .await
        .unwrap();
        let copied = native
            .artifact(&runtime, &dependency.artifact_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(offline.read(&copied.sha256).unwrap(), dependency_bytes);
        assert_eq!(
            native
                .result_dependencies(&runtime, &offline, &publication.delivery)
                .await
                .unwrap(),
            vec![copied.clone()]
        );
        // The result is still present; removing only its referenced value breaks the closure.
        std::fs::remove_file(offline.path_for(&copied.sha256)).unwrap();
        assert!(
            native
                .result_dependencies(&runtime, &offline, &publication.delivery)
                .await
                .is_err()
        );
        assert!(
            !enrichment_store::bundle::verify(&bundle)
                .await
                .unwrap()
                .is_empty()
        );
    }
    drop(repository);
    // No terminal journal was written. The catalog alone still names the completed result.
    let reopened = EvidenceRepository::new(
        paths,
        runtime.clone(),
        WriteLimits::default(),
        AdmissionLimits::default(),
    )
    .unwrap();
    assert_eq!(
        reopened
            .catalog
            .pin()
            .await
            .unwrap()
            .job_publication(&runtime, &job_id)
            .await
            .unwrap(),
        Some(publication)
    );
    let mut invalid = completion;
    invalid.job_id = format!("job_{}", "2".repeat(32));
    invalid.result_artifact_ids = vec![enrichment_core::evidence::artifact_id_for(&"e".repeat(64))];
    assert!(
        native_ingest::publish_rows(&reopened, metadata, evidence, None, Some(invalid.clone()))
            .await
            .is_err()
    );
    assert!(
        reopened
            .catalog
            .pin()
            .await
            .unwrap()
            .job_publication(&runtime, &invalid.job_id)
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn canonical_results_with_nonexistent_or_split_character_positions_are_not_published() {
    for position in [
        Utf8Position { line: 0, byte: 3 },
        Utf8Position { line: 30, byte: 0 },
        Utf8Position { line: 1, byte: 100 },
    ] {
        let dir = tempfile::tempdir().unwrap();
        let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
        let blobs = BlobStore::open(&paths.data_root).unwrap();
        let metadata = metadata();
        let evidence = evidence_with_position(&blobs, &metadata, position);
        let runtime =
            QueryRuntime::new(&paths.cache_root.join("spill"), QueryLimits::default()).unwrap();
        let repository = EvidenceRepository::new(
            paths,
            runtime,
            WriteLimits::default(),
            AdmissionLimits::default(),
        )
        .unwrap();
        let error =
            native_ingest::publish_rows(&repository, metadata.clone(), evidence, None, None)
                .await
                .unwrap_err();
        assert!(error.to_string().contains("position"), "{error}");
        assert!(
            repository
                .catalog
                .pin()
                .await
                .unwrap()
                .current(&repository.runtime, &metadata.context.context_id)
                .await
                .unwrap()
                .is_none()
        );
    }
}

#[tokio::test]
async fn repeated_environment_derivation_preserves_child_execution_observations() {
    let dir = tempfile::tempdir().unwrap();
    let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
    let blobs = BlobStore::open(&paths.data_root).unwrap();
    let repository = EvidenceRepository::new(
        paths.clone(),
        QueryRuntime::new(&paths.cache_root.join("spill"), QueryLimits::default()).unwrap(),
        WriteLimits::default(),
        AdmissionLimits::default(),
    )
    .unwrap();
    let parent_metadata = metadata();
    let parent_manifest = native_ingest::publish_rows(
        &repository,
        parent_metadata.clone(),
        evidence(&blobs, &parent_metadata),
        None,
        None,
    )
    .await
    .unwrap();
    let parent = repository
        .open_snapshot(
            repository.catalog.pin().await.unwrap(),
            &parent_manifest.snapshot_id,
        )
        .await
        .unwrap();
    let environment = Environment::resolved(
        "python-3.14.7".into(),
        "linux-x86_64".into(),
        vec![],
        None,
        canonical::sha256_hex(b"different exact lock"),
    );
    let context = parent_metadata
        .context
        .derived_with(environment.environment_id.clone());
    let derived = repository
        .derive_environment(&parent, context.clone(), environment.clone())
        .await
        .unwrap();
    let reader = SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &derived.snapshot_id,
    )
    .await
    .unwrap();
    assert!(
        reader
            .execution_observations(None, None)
            .await
            .unwrap()
            .is_empty()
    );
    let child_metadata = SnapshotMetadata {
        context: context.clone(),
        environment: environment.clone(),
        ..parent_metadata
    };
    let child_evidence = evidence(&blobs, &child_metadata);
    let mut expected = child_evidence.execution_observations.clone();
    expected.sort_by(|a, b| a.observation_id.cmp(&b.observation_id));
    native_ingest::publish_rows(&repository, child_metadata, child_evidence, None, None)
        .await
        .unwrap();
    let repeated = repository
        .derive_environment(&parent, context, environment)
        .await
        .unwrap();
    let reader = SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &repeated.snapshot_id,
    )
    .await
    .unwrap();
    assert_eq!(
        reader.execution_observations(None, None).await.unwrap(),
        expected
    );
}

#[tokio::test]
async fn execution_coverage_does_not_borrow_a_different_query_on_the_same_subject() {
    use enrichment_core::evidence::{
        EvidenceKind, Gap, GapReason,
        relational::{CoverageFact, CoverageOutcome},
    };
    let root = tempfile::tempdir().unwrap();
    let paths = StatePaths::explicit(root.path().join("cache"), root.path().join("data"));
    let blobs = BlobStore::open(&paths.data_root).unwrap();
    let metadata = metadata();
    let mut evidence = evidence(&blobs, &metadata);
    for (index, outcome) in [(0, CoverageOutcome::Indexed), (1, CoverageOutcome::Partial)] {
        let observation = &evidence.execution_observations[index];
        let gaps = if outcome == CoverageOutcome::Indexed {
            vec![]
        } else {
            vec![Gap {
                kind: EvidenceKind::SemanticQueries,
                reason: GapReason::ExtractionFailed,
                detail: "one selected query has incomplete external evidence".into(),
                planned_fallback: None,
            }]
        };
        evidence.coverage.push(
            CoverageFact::new(
                observation.source.producer_binding_id.clone(),
                observation.subject.clone(),
                EvidenceKind::SemanticQueries,
                outcome,
                gaps,
            )
            .unwrap(),
        );
    }
    let ids = evidence.execution_observations[..2]
        .iter()
        .map(|fact| fact.source.artifact_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        evidence.execution_observations[0].subject,
        evidence.execution_observations[1].subject
    );
    let runtime =
        QueryRuntime::new(&paths.cache_root.join("spill"), QueryLimits::default()).unwrap();
    let repository = EvidenceRepository::new(
        paths,
        runtime,
        WriteLimits::default(),
        AdmissionLimits::default(),
    )
    .unwrap();
    let manifest = native_ingest::publish_rows(&repository, metadata, evidence, None, None)
        .await
        .unwrap();
    let reader = SnapshotReader::open(
        &repository,
        repository.catalog.pin().await.unwrap(),
        &manifest.snapshot_id,
    )
    .await
    .unwrap();
    let successful = reader.assess_execution(&ids[..1]).await.unwrap();
    assert!(successful.complete());
    let limited = reader.assess_execution(&ids[1..]).await.unwrap();
    assert!(!limited.complete());
    assert_eq!(
        limited.assessments[0].state,
        enrichment_core::wire::ScopeState::Partial
    );
    assert_ne!(
        successful.assessments[0].witness_id,
        limited.assessments[0].witness_id
    );
    let both = reader.assess_execution(&ids).await.unwrap();
    assert_eq!(both.assessments.len(), 2);
    assert!(!both.complete());
    assert!(both.indexed.is_empty());
}
