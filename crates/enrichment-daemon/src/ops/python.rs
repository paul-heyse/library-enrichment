//! Exact static Python evidence over the same catalog, snapshots and retrieval tools as Rust.
use std::collections::BTreeMap;
use std::time::Duration;

use enrichment_core::canonical;
use enrichment_core::evidence::{ArtifactKind, EvidenceKind, Gap, GapReason, PlannedFallback};
use enrichment_core::identity::{Context, Release};
use enrichment_core::policy::ArchivePolicy;
use enrichment_core::producer::{
    RunOutcome,
    python::{self, DistributionFile},
};
use enrichment_core::request::{FreshnessMode, ResolveRequest};
use enrichment_core::wire::data::ResolveData;
use enrichment_core::wire::{Envelope, ErrorCode, SourceVersionMatch};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use url::Url;

use super::{
    common,
    resolve::{Acquisition, environment_for},
};
use crate::{
    envelope::{self, Research},
    service::Service,
};

fn failure(detail: impl Into<String>) -> Envelope {
    envelope::error(
        ErrorCode::ExtractionFailed,
        detail,
        "Read the producer limitation, correct the prerequisite and resolve again.",
        false,
    )
}
fn gap(kind: EvidenceKind, detail: impl Into<String>) -> Gap {
    Gap {kind,reason:GapReason::ExtractionFailed,detail:detail.into(),planned_fallback:Some(PlannedFallback{producer:"python-isolated-verification".into(),profile:enrichment_core::policy::ExecutionProfile::Runtime,enabled:false,next_action:"Static evidence is incomplete; use an explicitly enabled isolated verification environment when available.".into()})}
}

pub(super) async fn acquire(
    service: &Service,
    request: &ResolveRequest,
    work: &super::resolve_job::Work,
) -> Envelope {
    match acquire_inner(service, request, work).await {
        Ok(result) => result,
        Err(detail) => failure(detail),
    }
}

async fn acquire_inner(
    service: &Service,
    request: &ResolveRequest,
    work: &super::resolve_job::Work,
) -> Result<Envelope, String> {
    let mut acq = Acquisition::new(service).for_job(work);
    let started =
        enrichment_core::native_time::ObservationTime::now().map_err(|error| error.to_string())?;
    let registry = &service.config.producers.python.pypi_url;
    let captures = enrichment_store::registry_capture::RegistryStore::new(
        service.repository.catalog.clone(),
        service.repository.runtime.clone(),
    );
    let versions = if let Some(v) = &request.version {
        vec![v.clone()]
    } else {
        let url = Url::parse(&format!(
            "{}/{}/",
            service
                .config
                .producers
                .python
                .simple_url
                .trim_end_matches('/'),
            request.name
        ))
        .map_err(|e| e.to_string())?;
        let response = service
            .fetcher
            .get_with_revalidation(&url, Some("application/vnd.pypi.simple.v1+json"), true)
            .await
            .map_err(|e| e.to_string())?;
        if response.status != 200 {
            return Err(format!("Python index returned HTTP {}", response.status));
        }
        let artifact = acq
            .store(
                &response,
                ArtifactKind::RegistryIndexEntry,
                "application/json",
                url.as_str(),
            )
            .map_err(|e| e.to_string())?;
        let index: python::registry::Versions = serde_json::from_slice(&response.bytes)
            .map_err(|e| format!("index versions unavailable: {e}"))?;
        let versions = captures
            .python_versions(
                &artifact,
                Some("application/vnd.pypi.simple.v1+json"),
                &index.versions,
            )
            .await
            .map_err(|e| e.to_string())?;
        enrichment_store::python_registry::ordered_versions(
            &service.repository.runtime,
            versions,
            request.allow_prerelease,
            128,
        )
        .await
        .map_err(|e| e.to_string())?
    };
    let mut selected = None;
    let mut found_release = false;
    for version in versions {
        let url = Url::parse(&format!(
            "{}/{}/{}/json",
            registry.trim_end_matches('/'),
            request.name,
            version
        ))
        .map_err(|e| e.to_string())?;
        let response = service
            .fetcher
            .get_with_revalidation(
                &url,
                Some("application/json"),
                request.freshness == FreshnessMode::Revalidate || request.version.is_none(),
            )
            .await
            .map_err(|e| e.to_string())?;
        if response.status == 404 {
            continue;
        }
        if response.status != 200 {
            return Err(format!(
                "Python release metadata returned HTTP {}",
                response.status
            ));
        }
        found_release = true;
        let metadata: python::registry::ReleaseMetadata =
            serde_json::from_slice(&response.bytes).map_err(|e| e.to_string())?;
        let candidates = BTreeMap::from([(version.clone(), metadata.urls)]);
        let artifact = acq
            .store(
                &response,
                ArtifactKind::RegistryVersionMetadata,
                "application/json",
                url.as_str(),
            )
            .map_err(|e| e.to_string())?;
        let candidates = captures
            .python_files(
                &artifact,
                Some("application/json"),
                python::facts::decode(&candidates, 1024).map_err(|e| e.to_string())?,
            )
            .await
            .map_err(|e| e.to_string())?;
        if let Some(choice) = enrichment_store::python_registry::select(
            &service.repository.runtime,
            candidates,
            request,
            None,
            false,
        )
        .await
        .map_err(|e| e.to_string())?
        {
            let release = enrichment_store::python_registry::release(
                &service.repository.runtime,
                registry,
                &request.name,
                &choice,
                &metadata.info,
            )
            .await
            .map_err(|e| e.to_string())?;
            selected = Some((choice.file, release));
            break;
        }
    }
    let Some((file, mut release)) = selected else {
        return Ok(envelope::error(
            if !found_release && request.version.is_some() {
                ErrorCode::VersionNotFound
            } else {
                ErrorCode::EnvironmentUnresolved
            },
            "No eligible artifact in the bounded release selection.",
            "Check the exact version, interpreter, platform and prerelease/yanked policy.",
            false,
        ));
    };
    if let Some(replay) =
        super::resolve::replay_selected(service, request, &release, None, &mut acq).await
    {
        return Ok(replay);
    }
    let url = Url::parse(&file.url).map_err(|e| e.to_string())?;
    let fetched = service
        .fetcher
        .get_with_revalidation(&url, None, request.freshness == FreshnessMode::Revalidate)
        .await
        .map_err(|e| e.to_string())?;
    if fetched.status != 200 {
        return Err(format!("distribution returned HTTP {}", fetched.status));
    }
    let sha = file
        .digests
        .get("sha256")
        .ok_or("missing artifact sha256")?;
    if canonical::sha256_hex(&fetched.bytes) != sha.to_ascii_lowercase() {
        return Err("distribution checksum disagrees with PyPI".into());
    }
    let archive_artifact = acq
        .store(
            &fetched,
            ArtifactKind::Other,
            "application/octet-stream",
            url.as_str(),
        )
        .map_err(|e| e.to_string())?;
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    let staging = service
        .repository
        .source_directory()
        .await
        .map_err(|e| e.to_string())?;
    let tar = file.filename.ends_with(".tar.gz");
    let sdist = file.packagetype == "sdist";
    let bytes = fetched.bytes;
    let (staging, source_root) = service
        .repository
        .runtime
        .blocking(move || -> Result<_, String> {
            let mut root = if tar {
                enrichment_core::archive::extract_tar_gz(
                    bytes.as_ref(),
                    staging.path(),
                    &ArchivePolicy::default(),
                )
                .map_err(|e| e.to_string())?
                .top_level
                .unwrap_or_default()
            } else {
                python::archive::extract_zip(&bytes, staging.path(), &ArchivePolicy::default())?;
                String::new()
            };
            if root.is_empty() && sdist {
                let mut entries = std::fs::read_dir(staging.path()).map_err(|e| e.to_string())?;
                if let Some(entry) = entries.next() {
                    let entry = entry.map_err(|e| e.to_string())?;
                    if entries.next().is_none()
                        && entry.file_type().map_err(|e| e.to_string())?.is_dir()
                    {
                        root = entry
                            .file_name()
                            .into_string()
                            .map_err(|_| "non UTF8 sdist root")?;
                    }
                }
            }
            Ok((staging, root))
        })
        .await
        .map_err(|e| e.to_string())??;
    let input_root =
        super::source_tree::SourceTree::owned(staging.clone(), staging.path().join(&source_root))
            .map_err(|e| e.to_string())?;
    produce(
        service,
        request,
        &mut acq,
        &mut release,
        &context,
        &environment,
        Production {
            input_root: &input_root,
            source_root: &source_root,
            file: &file,
            archive: &archive_artifact,
            started,
            revision_tree: false,
        },
    )
    .await
}

pub(super) struct Production<'a> {
    pub(super) input_root: &'a super::source_tree::SourceTree,
    pub(super) source_root: &'a str,
    pub(super) file: &'a DistributionFile,
    pub(super) archive: &'a enrichment_core::evidence::Artifact,
    pub(super) started: enrichment_core::native_time::ObservationTime,
    pub(super) revision_tree: bool,
}

pub(super) async fn produce(
    service: &Service,
    request: &ResolveRequest,
    acq: &mut Acquisition<'_>,
    release: &mut Release,
    context: &Context,
    environment: &enrichment_core::identity::Environment,
    production: Production<'_>,
) -> Result<Envelope, String> {
    let Production {
        input_root,
        source_root,
        file,
        archive,
        started,
        revision_tree,
    } = production;
    let captured = enrichment_store::python_distribution::capture(
        &service.repository.runtime,
        input_root.to_path_buf(),
        input_root.owner(),
        &file.filename,
        &archive.sha256,
        source_root,
    )
    .await
    .map_err(|error| error.to_string())?;
    let warnings = if revision_tree {
        Vec::new()
    } else {
        enrichment_store::python_distribution::admit(
            &service.repository.runtime,
            &captured.distribution,
            captured.texts(),
            request,
            &release.key.version,
        )
        .await
        .map_err(|error| error.to_string())?
    };
    let mut distribution = captured.distribution.clone();
    if revision_tree {
        acq.gaps.push(gap(EvidenceKind::DistributionSource,"Revision source layout is inferred statically; built/generated files, export exclusions, LFS and submodule contents are unverified"));
    }
    for warning in warnings {
        acq.gaps.push(gap(EvidenceKind::RegistryMetadata, warning));
    }
    *release = enrichment_store::python_distribution::release_binding(
        &service.repository.runtime,
        release,
        &distribution.import_roots,
    )
    .await
    .map_err(|error| error.to_string())?;
    let mut inputs = BTreeMap::new();
    let mut native = None;
    let mut worker_identity = None;
    let mut producer_items = 0;
    let work = acq
        .work
        .ok_or("static extraction requires its durable acquisition job")?;
    match run_worker(
        service,
        request,
        &distribution.files,
        archive,
        work,
        input_root,
    )
    .await
    {
        Ok((facts, directory)) => {
            let source = directory.path().join("worker.arrow");
            let blobs = service.blobs.clone();
            let artifact = service
                .repository
                .runtime
                .blocking(move || {
                    let _directory = directory;
                    let retrieved_at = enrichment_core::native_time::AcquisitionTime::now()
                        .map_err(std::io::Error::other)?;
                    blobs.put_stream(
                        python::worker::MAX_BYTES,
                        |output| {
                            std::io::copy(&mut std::fs::File::open(source)?, output)?;
                            Ok(())
                        },
                        |digest, bytes| {
                            let mut artifact = enrichment_core::evidence::Artifact::describe(
                                &[],
                                ArtifactKind::Other,
                                "application/vnd.apache.arrow.stream",
                                "producer:griffe-static",
                                retrieved_at,
                            );
                            artifact.artifact_id =
                                enrichment_core::evidence::artifact_id_for(digest);
                            artifact.sha256 = digest.into();
                            artifact.size_bytes = bytes;
                            artifact
                        },
                    )
                })
                .await
                .map_err(|e| e.to_string())?
                .map_err(|e| e.to_string())?
                .acquired;
            let artifact = acq.remember_artifact(artifact).map_err(|e| e.to_string())?;
            inputs.insert("worker".into(), artifact.sha256.clone());
            distribution.worker_artifact_id = Some(artifact.artifact_id);
            for gap_row in facts.gaps().await.map_err(|e| e.to_string())? {
                acq.gaps.push(gap(
                    EvidenceKind::PublicApi,
                    format!("{}: {}", gap_row.file, gap_row.detail),
                ));
            }
            let summary = facts.summary().await.map_err(|e| e.to_string())?;
            producer_items = summary.observations;
            if summary.dynamic || !distribution.native_files.is_empty() {
                acq.gaps.push(gap(EvidenceKind::RuntimeApi,"Native implementation source and runtime signatures have not been observed; stub declarations are separate static evidence"));
            }
            for failure in facts.alias_failures().await.map_err(|e| e.to_string())? {
                acq.gaps.push(gap(
                    EvidenceKind::PublicApi,
                    format!(
                        "{} public aliases retained unresolved: {}",
                        failure.count, failure.reason
                    ),
                ));
            }
            worker_identity = Some(facts.identity().await.map_err(|e| e.to_string())?);
            native = Some(facts);
        }
        Err(e) => acq.gaps.push(gap(EvidenceKind::PublicApi, e)),
    }
    if producer_items == 0 {
        acq.gaps.push(gap(
            EvidenceKind::PublicApi,
            "No static API declarations were extracted; source or stubs are unavailable",
        ));
    }
    if request.python_version.is_none() {
        acq.gaps.push(gap(EvidenceKind::RegistryMetadata,"Interpreter unspecified: source evidence is available, interpreter compatibility is unverified"));
    }
    let mut documents = super::source_documents::SourceDocuments::new(service.blobs.clone());
    if let Some(documentation) = &release.links.documentation {
        match documentation_inventory(
            service,
            acq,
            documentation,
            &release.key.version,
            request.freshness == FreshnessMode::Revalidate,
            &mut documents,
        )
        .await
        {
            Ok(entries) => {
                distribution.inventory = entries;
            }
            Err(detail) => acq.gaps.push(gap(EvidenceKind::Inventory, detail)),
        }
    }

    let source_directory = input_root.to_owned();
    let files = service
        .repository
        .runtime
        .blocking(move || python::archive::files(&source_directory))
        .await
        .map_err(|e| e.to_string())??;
    let captures = enrichment_store::source_capture::select(
        &service.repository.runtime,
        files,
        enrichment_store::source_capture::Family::Python,
    )
    .await
    .map_err(|e| e.to_string())?;
    for capture in captures {
        let path = capture.path;
        let Some(artifact) = acq
            .store_source_file(
                input_root.clone(),
                path.clone(),
                capture.artifact_kind,
                capture.media_type,
                format!("{}#{path}", archive.source_uri),
            )
            .await
            .map_err(|e| e.to_string())?
        else {
            continue;
        };
        inputs.insert(
            capture.role.ok_or("native Python source role missing")?,
            artifact.sha256.clone(),
        );
        if let Some(kind) = capture.fragment_kind {
            documents.file(path, kind, artifact, false)?;
        }
    }
    acq.indexed.extend([
        EvidenceKind::RegistryMetadata,
        EvidenceKind::DistributionSource,
    ]);
    if producer_items > 0 {
        acq.indexed.insert(EvidenceKind::PublicApi);
    }
    if !documents.kinds().is_empty() {
        acq.indexed.insert(EvidenceKind::Documentation);
    }
    if distribution
        .files
        .iter()
        .any(|f| f.origin == python::ObservationOrigin::Stub)
    {
        acq.indexed.insert(EvidenceKind::Stubs);
    }
    inputs.extend(acq.semantic_inputs().await?);
    let outcome = if acq.gaps.is_empty() {
        RunOutcome::Succeeded
    } else {
        RunOutcome::Partial
    };
    let run_gaps = std::mem::take(&mut acq.gaps);
    acq.run(
        "python-static",
        python::VERSION,
        inputs,
        started,
        outcome,
        run_gaps,
    )
    .await?;
    for kind in documents.kinds() {
        if kind == enrichment_core::evidence::FragmentKind::ChangelogSection {
            acq.indexed.insert(EvidenceKind::ReleaseNotes);
        }
        if kind == enrichment_core::evidence::FragmentKind::Example {
            acq.indexed.insert(EvidenceKind::Examples);
        }
    }
    let mut producers = BTreeMap::from([
        ("python-static".into(), python::VERSION.into()),
        ("griffe-static".into(), python::VERSION.into()),
        ("python-distribution".into(), python::VERSION.into()),
        ("python-arrow-facts".into(), python::worker::VERSION.into()),
    ]);
    if let Some(identity) = worker_identity {
        producers.insert("griffe".into(), identity.griffe_version);
        producers.insert("python-worker".into(), identity.worker_python);
        producers.insert("pyarrow".into(), identity.encoder_version);
        producers.insert(
            "python-closure-depth".into(),
            identity.normalization_depth.to_string(),
        );
    }
    let data = ResolveData {
        release: release.clone(),
        environment: environment.clone(),
        context: context.clone(),
        upstream: None,
        observed_configuration: None,
        hosted_rustdoc_json: None,
        python: Some(distribution.clone()),
        snapshot: None,
        artifacts: acq.artifacts.clone(),
        producer_runs: acq.runs.clone(),
        gaps: acq.gaps.clone(),
        answered_from_cache: false,
    };
    let presentation = enrichment_store::research_resolution::acquisition_presentation(
        &service.repository.runtime,
        &data,
        &acq.indexed.iter().copied().collect::<Vec<_>>(),
        Some(&file.filename),
        None,
    )
    .await
    .map_err(|error| error.to_string())?;
    let freshness = enrichment_store::research_resolution::freshness(
        &service.repository.runtime,
        request,
        SourceVersionMatch::Exact,
        &acq.artifacts,
    )
    .await
    .map_err(|error| error.to_string())?;

    let result = Research {
        summary: presentation.summary,
        data: common::payload(&data),
        coverage: presentation.coverage,
        freshness,
        context_id: Some(context.context_id.clone()),
        snapshot_id: None,
        evidence: Vec::new(),
        artifacts: presentation.artifacts,
    };
    acq.delivery_template = Some(if presentation.partial {
        result.partial()
    } else {
        result.ok()
    });

    let manifest = super::publication::publish(
        service,
        acq,
        enrichment_core::evidence::snapshot::SnapshotMetadata {
            context: context.clone(),
            release: release.clone(),
            environment: environment.clone(),
            symbol_package: format!("python:{}", request.name),
            crate_name: release
                .root_module
                .clone()
                .unwrap_or_else(|| request.name.clone()),
            crate_version: (!revision_tree).then(|| release.key.version.clone()),
            normalizer_version: python::VERSION.into(),
            observed_configuration: None,
            producer_items,
        },
        (
            documents,
            native.map(super::publication::NativeFacts::Python),
        ),
        producers,
        acq.indexed.iter().copied().collect(),
        acq.gaps
            .iter()
            .map(|g| g.kind)
            .filter(|kind| !acq.indexed.contains(kind))
            .collect(),
        Some(super::publication::MetadataInput {
            details: enrichment_core::evidence::metadata::ReleaseDetails::PythonDistribution(
                distribution.clone(),
            ),
            artifact: archive.clone(),
            locator: enrichment_core::evidence::relational::Locator::Artifact,
        }),
    )
    .await?;
    acq.work
        .and_then(|work| work.committed.get())
        .filter(|(_, snapshot, _)| snapshot == &manifest.snapshot_id)
        .map(|(_, _, result)| result.clone())
        .ok_or_else(|| "Python resolution lacks prepared committed delivery".into())
}

async fn run_worker(
    service: &Service,
    request: &ResolveRequest,
    files: &[python::WorkerFile],
    archive: &enrichment_core::evidence::Artifact,
    work: &super::resolve_job::Work,
    source: &super::source_tree::SourceTree,
) -> Result<
    (
        enrichment_store::python_normalize::PythonFacts,
        std::sync::Arc<enrichment_store::PrivateDirectory>,
    ),
    String,
> {
    let parent = enrichment_store::native_effect::authorize()
        .await
        .map_err(|e| e.to_string())?;
    let directory = service
        .repository
        .worker_directory()
        .await
        .map_err(|e| e.to_string())?;
    let prepared = enrichment_store::static_worker::prepare(
        parent,
        &enrichment_store::static_worker::Source {
            root: source
                .to_str()
                .ok_or("worker source path is not UTF-8")?
                .into(),
            files: files.to_vec(),
            package: request.name.clone(),
            artifact_id: archive.artifact_id.clone(),
            artifact_sha256: archive.sha256.clone(),
            output_root: directory
                .path()
                .to_str()
                .ok_or("worker output path is not UTF-8")?
                .into(),
        },
        source.owner(),
        directory.clone(),
    )
    .await
    .map_err(|e| e.to_string())?;
    let launch = prepared.launch();
    let request = &launch.request;
    let measured = enrichment_core::json_output::measure(request).map_err(|e| e.to_string())?;
    if measured > enrichment_core::execution::static_worker::REQUEST_BYTES {
        return Err("static worker request exceeds transport bound".into());
    }
    let bytes = enrichment_core::json_output::JsonOutput::serialize(
        &service
            .repository
            .runtime
            .session()
            .runtime_env()
            .memory_pool,
        request,
    )
    .map_err(|e| e.to_string())?;
    let mut output_owner =
        enrichment_store::static_worker::Output::create(&service.repository.runtime, &prepared)
            .await
            .map_err(|e| e.to_string())?;
    let output = output_owner.file();
    let mut owned = enrichment_store::static_worker::OwnedChild::spawn(
        &service.repository.runtime,
        &prepared,
        (source.clone(), directory.clone(), work.lease.clone()),
    )
    .await
    .map_err(|e| format!("service-owned Griffe worker unavailable: {e}"))?;
    let child = owned.child();
    let mut stdin = child.stdin.take().ok_or("worker stdin unavailable")?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or("worker stdout unavailable")?
        .take(launch.output_bytes + 1);
    let mut stderr = child
        .stderr
        .take()
        .ok_or("worker stderr unavailable")?
        .take(launch.stderr_bytes + 1);
    let mut log = Vec::new();
    let run = async {
        let (_, _, _, status) = tokio::try_join!(
            async {
                stdin.write_all(bytes.as_str().as_bytes()).await?;
                drop(stdin);
                Ok::<_, std::io::Error>(())
            },
            async {
                let bytes = tokio::io::copy(&mut stdout, &mut *output).await?;
                if bytes > launch.output_bytes {
                    return Err(std::io::Error::other(
                        "worker Arrow output exceeds byte budget",
                    ));
                }
                Ok(bytes)
            },
            stderr.read_to_end(&mut log),
            child.wait()
        )?;
        Ok::<_, std::io::Error>(status)
    };
    let status = tokio::select! {
        result = tokio::time::timeout(Duration::from_secs(launch.deadline_seconds), run) => {
            match result {
                Ok(Ok(status)) => Ok(status),
                Ok(Err(e)) => Err(e.to_string()),
                Err(_) => Err("static worker deadline exceeded".into()),
            }
        }
        () = super::resolve_job::cancelled(&work.cancel) => Err("static worker cancelled".into()),
        error = prepared.revoked() => Err(format!("static worker authority revoked: {error}")),
    };
    let status = match status {
        Ok(status) => status,
        Err(detail) => {
            child
                .kill()
                .await
                .map_err(|e| format!("{detail}; worker termination failed: {e}"))?;
            child
                .wait()
                .await
                .map_err(|e| format!("{detail}; worker reap failed: {e}"))?;
            return Err(format!("{detail}; child termination confirmed"));
        }
    };
    if !status.success()
        || output.metadata().await.map_err(|e| e.to_string())?.len() > launch.output_bytes
        || log.len() as u64 > launch.stderr_bytes
    {
        return Err(format!(
            "static worker failed ({status}): {}",
            String::from_utf8_lossy(&log)
        ));
    }
    output.sync_all().await.map_err(|e| e.to_string())?;
    drop(output_owner);
    let facts = enrichment_store::python_normalize::PythonFacts::open(
        &service.repository.runtime,
        directory.clone(),
        &request.files,
    )
    .await
    .map_err(|e| e.to_string())?;
    prepared.check().await.map_err(|e| e.to_string())?;
    service
        .python_worker_qualified
        .store(true, std::sync::atomic::Ordering::Relaxed);
    Ok((facts, directory))
}

async fn documentation_inventory(
    service: &Service,
    acq: &mut Acquisition<'_>,
    documentation: &str,
    version: &str,
    revalidate: bool,
    documents: &mut super::source_documents::SourceDocuments,
) -> Result<Vec<python::inventory::Entry>, String> {
    let base = Url::parse(documentation).map_err(|e| e.to_string())?;
    let url = base.join("objects.inv").map_err(|e| e.to_string())?;
    let fetched = service
        .fetcher
        .get_with_revalidation(&url, None, revalidate)
        .await
        .map_err(|e| e.to_string())?;
    if fetched.status != 200 {
        return Err(format!(
            "documentation inventory returned HTTP {}",
            fetched.status
        ));
    }
    let artifact = acq
        .store(
            &fetched,
            ArtifactKind::Other,
            "application/octet-stream",
            url.as_str(),
        )
        .map_err(|e| e.to_string())?;
    let inventory = python::inventory::parse(&fetched.bytes, &base, 8 * 1024 * 1024, 50000)?;
    let version_match = enrichment_store::documentation_plan::version_match(
        &service.repository.runtime,
        &inventory.version,
        version,
    )
    .await
    .map_err(|e| e.to_string())?;
    let mut accepted = Vec::new();
    for entry in inventory.entries {
        let target = Url::parse(&entry.uri).map_err(|e| e.to_string())?;
        if service.fetcher.check_url(&target).await.is_err() {
            acq.gaps.push(gap(
                EvidenceKind::Inventory,
                format!("inventory target refused by URL policy: {}", entry.uri),
            ));
            continue;
        }
        accepted.push(entry);
    }
    // Follow a bounded set of navigation pages; never execute a documentation build.
    let pages = enrichment_store::documentation_plan::pages(&service.repository.runtime, &accepted)
        .await
        .map_err(|e| e.to_string())?;
    for selected in pages {
        let target = Url::parse(&selected.uri).map_err(|e| e.to_string())?;
        let page = match service
            .fetcher
            .get_with_revalidation(&target, None, revalidate)
            .await
        {
            Ok(page) if page.status == 200 => page,
            _ => continue,
        };
        let Ok(text) = std::str::from_utf8(&page.bytes) else {
            continue;
        };
        let page_artifact = acq
            .store(&page, ArtifactKind::Other, "text/html", target.as_str())
            .map_err(|e| e.to_string())?;
        let fragment = enrichment_store::documentation_plan::document(
            &service.repository.runtime,
            &selected,
            &page_artifact,
            text,
            &inventory.version,
            version_match,
        )
        .await
        .map_err(|e| e.to_string())?;
        documents.declaration(fragment)?;
    }
    documents.inventory(
        accepted.clone(),
        artifact,
        inventory.project,
        inventory.version,
        version_match,
    )?;
    if inventory.rejected > 0 {
        acq.gaps.push(gap(
            EvidenceKind::Inventory,
            format!("{} malformed inventory records omitted", inventory.rejected),
        ));
    }
    acq.indexed.insert(EvidenceKind::Inventory);
    Ok(accepted)
}
