use super::*;
use enrichment_core::{
    evidence::{
        Artifact, ArtifactKind,
        catalog::PublishedJobKind,
        snapshot::{EvidenceManifest, SnapshotMetadata},
    },
    execution::{
        ProbeMode, ProcessAuthority, ProcessEnd, ProcessObservation, VerificationData,
        VerifyRequest,
    },
    identity::{Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode},
    native_time::{AcquisitionTime, ObservationTime},
    operation::jobs::Resolution,
    policy::ExecutionProfile,
    wire::JobState,
};
use std::collections::BTreeMap;

enrichment_core::native_struct! { struct Fixture {
    publication: JobPublication => Rule::Text,
    arguments: Arguments => Rule::Text,
    resolution: Option<Resolution> => Rule::Text,
    manifest: EvidenceManifest => Rule::Text,
    inputs: BTreeMap<String,String> => Rule::Text,
    acquisitions: Vec<Artifact> => Rule::Sequence,
} }

fn artifact(bytes: &[u8]) -> Artifact {
    let sha256 = enrichment_core::canonical::sha256_hex(bytes);
    Artifact {
        artifact_id: format!("art_{sha256}"),
        sha256,
        media_type: "application/octet-stream".into(),
        size_bytes: bytes.len() as u64,
        kind: ArtifactKind::Other,
        source_uri: "fixture:recovery".into(),
        final_url: None,
        retrieved_at: AcquisitionTime::from_micros(1).unwrap(),
        etag: None,
        last_modified: None,
        compression: None,
    }
}

fn verification() -> (Fixture, ResultRecord) {
    let release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: "crates.io".into(),
        package: "recovery".into(),
        version: "1.0.0".into(),
        artifact_digest: None,
    });
    let environment = Environment::unspecified();
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        ResearchMode::Upstream,
    );
    let metadata = SnapshotMetadata {
        context: context.clone(),
        release,
        environment: environment.clone(),
        symbol_package: "recovery".into(),
        crate_name: "recovery".into(),
        crate_version: Some("1.0.0".into()),
        normalizer_version: "native/6".into(),
        observed_configuration: None,
        producer_items: 0,
    }
    .descriptor();
    let components = [("fixture".into(), "revision".into())]
        .into_iter()
        .collect();
    let manifest = EvidenceManifest {
        snapshot_id: EvidenceManifest::derive_id(&metadata, &components).unwrap(),
        schema_version: enrichment_core::evidence::snapshot::FORMAT.into(),
        metadata,
        components,
        tables: vec![],
        counts: Default::default(),
        indexed: vec![],
        missing: vec![],
        published_at: ObservationTime::from_micros(2).unwrap(),
    };
    let snippet = artifact(b"fn main() {}");
    let lock = artifact(b"locked dependencies");
    let process = ProcessObservation {
        operation_id: format!("process_{}", "1".repeat(64)).try_into().unwrap(),
        authority: ProcessAuthority::Command {
            effect_id: format!("process_effect_{}", "1".repeat(64))
                .try_into()
                .unwrap(),
            grant_id: format!("grant_{}", "1".repeat(64)).try_into().unwrap(),
            environment_id: Some(environment.environment_id.clone()),
            snapshot_id: Some(manifest.snapshot_id.clone()),
        },
        image_id: "image".into(),
        command: vec!["cargo".into()],
        started_at: ObservationTime::from_micros(1).unwrap(),
        finished_at: ObservationTime::from_micros(2).unwrap(),
        exit_code: Some(0),
        end: ProcessEnd::Exited,
        stdout: "observed output".into(),
        stderr: String::new(),
        cleanup_confirmed: true,
    };
    let payload = enrichment_core::evidence::execution::ExecutionPayload::UsageProbe(
        enrichment_core::evidence::execution::UsageProbe {
            mode: ProbeMode::Compile,
            snippet_artifact_id: snippet.artifact_id.clone(),
            end: process.end,
            exit_code: process.exit_code,
            stdout: process.stdout.clone(),
            stderr: process.stderr.clone(),
        },
    );
    let output = artifact(&payload.canonical_bytes().unwrap());
    let request = VerifyRequest {
        context_id: context.context_id.clone(),
        snapshot_id: Some(manifest.snapshot_id.clone()),
        snippet: "fn main() {}".into(),
        mode: ProbeMode::Compile,
        profile: ExecutionProfile::Build,
        test_intent: None,
        max_bytes: None,
    };
    let publication = JobPublication {
        job_id: format!("job_{}", "1".repeat(32)).try_into().unwrap(),
        context_id: context.context_id.clone(),
        snapshot_id: manifest.snapshot_id.clone(),
        kind: PublishedJobKind::Verify,
        state: JobState::Succeeded,
        attempt_id: enrichment_core::identity::AttemptId::new(),
        result_artifact_ids: vec![output.artifact_id.clone()],
        delivery: artifact(b"delivery"),
    };
    let error: enrichment_core::wire::Envelope = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/wire/error.fixture.json"
    ))
    .unwrap();
    let mut result = ResultRecord::from_envelope(&error).unwrap();
    result.data = VerificationData {
        evidence_class: enrichment_core::wire::EvidenceClass::CompilerDerived,
        producer_runs: vec![],
        source_context_id: context.context_id.clone(),
        source_snapshot_id: manifest.snapshot_id.clone(),
        derived_context: Some(context),
        derived_snapshot_id: Some(manifest.snapshot_id.clone()),
        environment: Some(environment),
        mode: request.mode,
        profile: request.profile,
        snippet_origin: "agent".into(),
        test_intent: None,
        snippet_artifact_id: snippet.artifact_id.clone(),
        lock_artifact_id: Some(lock.artifact_id.clone()),
        result_artifact_id: output.artifact_id.clone(),
        observations: vec![process],
        limitations: vec![],
    }
    .into();
    let inputs = [
        ("snippet".into(), snippet.sha256.clone()),
        ("dependency-lock".into(), lock.sha256.clone()),
        ("result".into(), output.sha256.clone()),
    ]
    .into_iter()
    .collect();
    (
        Fixture {
            publication,
            arguments: Arguments::Verify { request },
            resolution: None,
            manifest,
            inputs,
            acquisitions: vec![snippet, lock, output],
        },
        result,
    )
}

async fn check(runtime: &QueryRuntime, fixture: Fixture, result: ResultRecord) -> Result<()> {
    let session = runtime.session();
    let mut fields = JobPublication::fields()
        .iter()
        .map(|field| col("publication").field(field.name()).alias(field.name()))
        .collect::<Vec<_>>();
    fields.extend(
        [
            "arguments",
            "resolution",
            "manifest",
            "inputs",
            "acquisitions",
        ]
        .map(col),
    );
    native_catalog::work(
        &session,
        "recovery_scope",
        crate::native_catalog::batch(&session, "tests", Fixture::batch(&[fixture])?)?
            .select(fields)?
            .into_view(),
    )?;
    native_catalog::work(
        &session,
        "recovery_result",
        crate::native_catalog::batch(&session, "tests", ResultRecord::batch(&[result])?)?
            .into_view(),
    )?;
    admit(runtime, &session).await
}

#[tokio::test]
async fn plan19_recovery_uses_native_scope_roles_and_raw_observation() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let (fixture, result) = verification();
    check(&runtime, fixture.clone(), result.clone()).await?;
    for mutation in 0..6 {
        let mut wrong = fixture.clone();
        let mut changed = result.clone();
        match mutation {
            0 => wrong.inputs.remove("dependency-lock"),
            1 => {
                wrong.publication.result_artifact_ids = vec![artifact(b"unrelated").artifact_id];
                None
            }
            2 => {
                let enrichment_core::wire::data::ToolData::VerifyUsage(data) = &mut changed.data
                else {
                    unreachable!()
                };
                data.observations[0].stdout.push_str("changed");
                None
            }
            3 => {
                let enrichment_core::wire::data::ToolData::VerifyUsage(data) = &mut changed.data
                else {
                    unreachable!()
                };
                data.observations.clear();
                None
            }
            4 => {
                let Arguments::Verify { request } = &mut wrong.arguments else {
                    unreachable!()
                };
                request.snapshot_id = None;
                None
            }
            _ => {
                wrong.publication.kind = PublishedJobKind::Inspect;
                None
            }
        };
        assert!(
            check(&runtime, wrong, changed).await.is_err(),
            "accepted mutation {mutation}"
        );
    }
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn plan19_probe_lowering_preserves_outcomes_and_physical_facts() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let (fixture, result) = verification();
    let Arguments::Verify { request } = fixture.arguments else {
        unreachable!()
    };
    let enrichment_core::wire::data::ToolData::VerifyUsage(data) = result.data else {
        unreachable!()
    };
    let mut capture = enrichment_core::execution::VerificationCapture {
        job_id: fixture.publication.job_id,
        request,
        source_release: Release::new(ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "recovery".into(),
            version: "1.0.0".into(),
            artifact_digest: None,
        }),
        source_snapshot: fixture.manifest.snapshot_id,
        environment: data.environment.unwrap(),
        containment_identity: "native-qualification".into(),
        observations: data.observations,
        input_artifacts: fixture.acquisitions,
    };
    for (end, code, cleanup, expected) in [
        (ProcessEnd::Exited, Some(0), true, JobState::Succeeded),
        (ProcessEnd::Exited, Some(0), false, JobState::Partial),
        (ProcessEnd::Exited, Some(1), true, JobState::Failed),
        (ProcessEnd::Deadline, None, true, JobState::Failed),
        (ProcessEnd::OutputLimit, None, true, JobState::Failed),
        (ProcessEnd::Cancelled, None, false, JobState::Cancelled),
    ] {
        capture.observations[0].end = end;
        capture.observations[0].exit_code = code;
        capture.observations[0].cleanup_confirmed = cleanup;
        let lowered = crate::probe_plan::lower(&runtime, &capture).await?;
        assert_eq!(lowered.state, expected);
        let recorded = capture.clone();
        let settled = crate::probe_plan::settled(&runtime, &capture).await?;
        assert_eq!(
            capture, recorded,
            "physical observations must remain unchanged"
        );
        assert_eq!(
            settled.state,
            if expected == JobState::Partial {
                JobState::Succeeded
            } else {
                expected
            }
        );
        assert_eq!(settled.payload, lowered.payload);
        assert_eq!(
            settled.run_outcome,
            if end == ProcessEnd::Exited && code == Some(0) {
                enrichment_core::producer::RunOutcome::Succeeded
            } else {
                enrichment_core::producer::RunOutcome::Failed
            }
        );
        assert!(
            settled.limitations.iter().any(|note| note
                == "All owned execution cleanup settled before this result was published; process observations retain each initial cleanup result.")
        );
        assert!(
            !settled
                .coverage
                .missing
                .contains("confirmed execution container removal")
        );
        assert_eq!(
            settled.outcome.status(),
            if settled.state == JobState::Succeeded {
                enrichment_core::wire::Status::Ok
            } else {
                enrichment_core::wire::Status::Error
            }
        );
        assert_eq!(lowered.coverage.limitations, lowered.limitations);
        assert_eq!(
            lowered
                .coverage
                .missing
                .contains("confirmed execution container removal"),
            expected == JobState::Partial
        );
        assert_eq!(
            lowered.evidence_class,
            enrichment_core::wire::EvidenceClass::CompilerDerived
        );
        let enrichment_core::evidence::execution::ExecutionPayload::UsageProbe(probe) =
            lowered.payload
        else {
            unreachable!()
        };
        assert_eq!(probe.end, end);
        assert_eq!(probe.exit_code, code);
        assert_eq!(probe.stdout, capture.observations[0].stdout);
        assert_eq!(
            probe.snippet_artifact_id,
            artifact(capture.request.snippet.as_bytes()).artifact_id
        );
    }
    capture.observations.clear();
    assert!(crate::probe_plan::lower(&runtime, &capture).await.is_err());
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn plan19_runtime_object_has_one_native_decoder_and_outcome_rule() -> Result<()> {
    use enrichment_core::evidence::execution::{ExecutionOutcome, RuntimeObject};
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let (_, result) = verification();
    let enrichment_core::wire::data::ToolData::VerifyUsage(data) = result.data else {
        unreachable!()
    };
    let expected = RuntimeObject {
        module: "typing".into(),
        selection: vec!["Any".into()],
        outcome: ExecutionOutcome::Results,
        type_name: Some("typing._AnyMeta".into()),
        signature: None,
        docstring: Some("bounded actual text".into()),
        attributes: vec!["__module__".into()],
        limitations: vec![],
    };
    let mut capture = enrichment_core::native_runtime::Capture {
        selection: enrichment_core::request::RuntimeSelection {
            module: expected.module.clone(),
            attributes: expected.selection.clone(),
        },
        subject: enrichment_core::evidence::relational::SubjectRef::Symbol {
            symbol_id: format!("symbol_{}", "1".repeat(64)),
        },
        observation: data.observations[0].clone(),
    };
    capture.observation.stdout = serde_json::to_string(&enrichment_core::native_runtime::Report {
        result: expected.clone(),
        stdout: String::new(),
        stderr: String::new(),
        output_truncated: false,
    })
    .unwrap();
    assert_eq!(
        crate::runtime_object_plan::lower(&runtime, &capture).await?,
        expected
    );
    capture.selection.attributes = vec!["Wrong".into()];
    assert!(
        crate::runtime_object_plan::lower(&runtime, &capture)
            .await
            .is_err()
    );
    capture.selection.attributes = expected.selection.clone();
    capture.observation.stdout = "{bad external JSON".into();
    assert!(
        crate::runtime_object_plan::lower(&runtime, &capture)
            .await
            .is_err()
    );
    // A killed producer's truncated stdout is not parsed as a successful API result.
    for (end, expected) in [
        (ProcessEnd::Deadline, ExecutionOutcome::Incomplete),
        (ProcessEnd::OutputLimit, ExecutionOutcome::Incomplete),
        (ProcessEnd::Cancelled, ExecutionOutcome::Cancelled),
        (ProcessEnd::Exited, ExecutionOutcome::Failed),
    ] {
        capture.observation.end = end;
        capture.observation.exit_code = None;
        let actual = crate::runtime_object_plan::lower(&runtime, &capture).await?;
        assert_eq!(actual.outcome, expected);
        assert!(actual.type_name.is_none());
        assert!(!actual.limitations.is_empty());
    }
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn plan19_recovery_compares_command_and_result_values() -> Result<()> {
    use crate::native_catalog::{BindingKind, BoundCatalog};
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let (fixture, result) = verification();
    let session = runtime.session();
    let commands = crate::native_catalog::batch(
        &session,
        "tests",
        Requested::batch(&[Requested {
            job_id: fixture.publication.job_id,
            arguments: fixture.arguments.clone(),
        }])?,
    )?;
    let state: Arc<dyn datafusion::catalog::CatalogProvider> = Arc::new(
        BoundCatalog::default().with_schema(
            BindingKind::FoldedRecords,
            [("commands".into(), commands.into_view())]
                .into_iter()
                .collect(),
        ),
    );
    let session = runtime.bound_session([("state".into(), state.clone())].into_iter().collect())?;
    command(
        &runtime,
        &session,
        &fixture.publication.job_id,
        &fixture.arguments,
    )
    .await?;
    let mut wrong = fixture.arguments.clone();
    let Arguments::Verify { request } = &mut wrong else {
        unreachable!()
    };
    request.snippet.push_str(" // different exact request");
    let changed = runtime.bound_session([("state".into(), state)].into_iter().collect())?;
    assert!(
        command(&runtime, &changed, &fixture.publication.job_id, &wrong)
            .await
            .is_err()
    );
    crate::result_plan::admit_retained(&runtime, result.clone(), result.clone()).await?;
    let mut different = result.clone();
    let enrichment_core::wire::data::ToolData::VerifyUsage(data) = &mut different.data else {
        unreachable!()
    };
    data.observations[0].authority = ProcessAuthority::Qualification {
        definition_id: format!("process_{}", "2".repeat(64)).try_into().unwrap(),
    };
    assert!(
        crate::result_plan::admit_retained(&runtime, different, result)
            .await
            .is_err()
    );
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn plan19_resolution_recovery_requires_exact_native_input_closure() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let (mut fixture, mut result) = verification();
    let release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: "crates.io".into(),
        package: "recovery".into(),
        version: "1.0.0".into(),
        artifact_digest: None,
    });
    let environment = Environment::unspecified();
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        ResearchMode::Upstream,
    );
    fixture.publication.kind = PublishedJobKind::Resolve;
    fixture.arguments = Arguments::Resolve {
        request: enrichment_core::request::ResolveRequest {
            ecosystem: Ecosystem::Rust,
            name: "recovery".into(),
            version: Some("1.0.0".into()),
            ..Default::default()
        },
    };
    fixture.resolution = Some(Resolution {
        release_id: release.release_id.clone(),
        environment_id: environment.environment_id.clone(),
        context_id: context.context_id.clone(),
        attempt_id: fixture.publication.attempt_id,
        input_artifact_ids: fixture
            .acquisitions
            .iter()
            .map(|a| a.artifact_id.clone())
            .collect(),
        result_artifact_id: fixture.publication.result_artifact_ids[0].clone(),
    });
    result.data = enrichment_core::wire::data::ResolveData {
        release,
        environment,
        context,
        upstream: None,
        observed_configuration: None,
        hosted_rustdoc_json: None,
        python: None,
        snapshot: None,
        artifacts: vec![],
        gaps: vec![],
        producer_runs: vec![],
        answered_from_cache: false,
    }
    .into();
    check(&runtime, fixture.clone(), result.clone()).await?;
    let mut wrong = fixture.clone();
    wrong.resolution.as_mut().unwrap().input_artifact_ids.pop();
    assert!(check(&runtime, wrong, result.clone()).await.is_err());
    let mut wrong = fixture;
    wrong.resolution.as_mut().unwrap().attempt_id = enrichment_core::identity::AttemptId::new();
    assert!(check(&runtime, wrong, result).await.is_err());
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn plan19_operation_inputs_refuse_shadowing_and_replacement() -> Result<()> {
    use crate::native_catalog::{BindingKind, BoundCatalog};
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let (fixture, _) = verification();
    let value = Requested::batch(&[Requested {
        job_id: fixture.publication.job_id,
        arguments: fixture.arguments,
    }])?;
    let base = runtime.session();
    let state: Arc<dyn datafusion::catalog::CatalogProvider> = Arc::new(
        BoundCatalog::default().with_schema(
            BindingKind::FoldedRecords,
            [(
                "protected".into(),
                crate::native_catalog::batch(&base, "tests", value.clone())?.into_view(),
            )]
            .into_iter()
            .collect(),
        ),
    );
    let session = runtime.bound_session([("state".into(), state)].into_iter().collect())?;
    assert!(native_catalog::input(&session, "protected", value.clone()).is_err());
    assert!(native_catalog::input(&session, "state.records.injected", value.clone()).is_err());
    native_catalog::input(&session, "finite_input", value.clone())?;
    assert!(native_catalog::input(&session, "finite_input", value).is_err());
    assert_eq!(
        runtime
            .execute(session.sql("SELECT job_id FROM finite_input").await?)
            .await?
            .rows,
        1
    );
    runtime.close_diagnostics().await?;
    Ok(())
}
