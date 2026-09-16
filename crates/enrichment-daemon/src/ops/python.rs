//! Exact static Python evidence over the same catalog, snapshots and retrieval tools as Rust.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use enrichment_core::evidence::{ArtifactKind, EvidenceKind, Gap, GapReason, PlannedFallback};
use enrichment_core::identity::{Context, Ecosystem, Release, ReleaseKey};
use enrichment_core::policy::ArchivePolicy;
use enrichment_core::producer::{
    RunOutcome,
    python::{self, DistributionFile, WorkerRequest},
};
use enrichment_core::request::{FreshnessMode, ResolveRequest};
use enrichment_core::wire::data::ResolveData;
use enrichment_core::wire::{Coverage, Envelope, ErrorCode, Freshness, SourceVersionMatch};
use enrichment_core::{canonical, clock};
use serde_json::Value;
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
    Gap {kind,reason:GapReason::ExtractionFailed,detail:detail.into(),planned_fallback:Some(PlannedFallback{producer:"python-isolated-verification".into(),profile:"runtime".into(),enabled:false,next_action:"Static evidence is incomplete; use an explicitly enabled isolated verification environment when available.".into()})}
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
    let started = clock::now_rfc3339();
    let registry = &service.config.producers.python.pypi_url;
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
        acq.store(
            &response,
            ArtifactKind::RegistryIndexEntry,
            "application/json",
            url.as_str(),
        )
        .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&response.bytes).map_err(|e| e.to_string())?;
        let versions: Vec<String> = serde_json::from_value(value["versions"].clone())
            .map_err(|e| format!("index versions unavailable: {e}"))?;
        enrichment_store::python_registry::ordered_versions(
            &service.repository.runtime,
            &versions,
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
        let metadata: Value = serde_json::from_slice(&response.bytes).map_err(|e| e.to_string())?;
        let files: Vec<DistributionFile> =
            serde_json::from_value(metadata["urls"].clone()).map_err(|e| e.to_string())?;
        let candidates = BTreeMap::from([(version.clone(), files)]);
        if let Some(choice) = enrichment_store::python_registry::select(
            &service.repository.runtime,
            &candidates,
            request,
            None,
            false,
        )
        .await
        .map_err(|e| e.to_string())?
        {
            let file = choice.file;
            acq.store(
                &response,
                ArtifactKind::RegistryVersionMetadata,
                "application/json",
                url.as_str(),
            )
            .map_err(|e| e.to_string())?;
            selected = Some((choice.version, file, metadata));
            break;
        }
    }
    let Some((version, file, metadata)) = selected else {
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
    let selected = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Python,
        registry: registry.clone(),
        package: request.name.clone(),
        version: version.clone(),
        artifact_digest: file.digests.get("sha256").cloned(),
    });
    if let Some(replay) =
        super::resolve::replay_selected(service, request, &selected, None, &mut acq).await
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
    let mut release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Python,
        registry: registry.clone(),
        package: request.name.clone(),
        version: version.clone(),
        artifact_digest: Some(sha.clone()),
    });
    release.yanked = file.yanked;
    release.license = metadata["info"]["license"].as_str().map(str::to_owned);
    release.links.documentation = metadata["info"]["project_urls"]["Documentation"]
        .as_str()
        .map(str::to_owned);
    release.links.homepage = metadata["info"]["home_page"].as_str().map(str::to_owned);
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    let unpacked = service.paths.unpacked();
    let tar = file.filename.ends_with(".tar.gz");
    let sdist = file.packagetype == "sdist";
    let bytes = fetched.bytes;
    let (staging, source_root) = tokio::task::spawn_blocking(move || -> Result<_, String> {
        std::fs::create_dir_all(&unpacked).map_err(|e| e.to_string())?;
        let staging = tempfile::Builder::new()
            .prefix(".python-source-")
            .tempdir_in(unpacked)
            .map_err(|e| e.to_string())?;
        let mut root = if tar {
            enrichment_core::archive::extract_tar_gz(
                bytes.as_slice(),
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
    let input_root = staging.path().join(&source_root);
    let result = produce(
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
    .await;
    tokio::task::spawn_blocking(move || staging.close())
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    result
}

pub(super) struct Production<'a> {
    pub(super) input_root: &'a Path,
    pub(super) source_root: &'a str,
    pub(super) file: &'a DistributionFile,
    pub(super) archive: &'a enrichment_core::evidence::Artifact,
    pub(super) started: String,
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
    let directory = input_root.to_owned();
    let filename = file.filename.clone();
    let digest = archive.sha256.clone();
    let source_root = source_root.to_owned();
    let selected_request = request.clone();
    let version = release.key.version.clone();
    let (mut distribution, warnings) = tokio::task::spawn_blocking(move || -> Result<_, String> {
        let distribution =
            python::archive::inventory(&directory, &filename, &digest, &source_root)?;
        let warnings = if revision_tree {
            Vec::new()
        } else {
            python::archive::validate_metadata(
                &directory,
                &distribution,
                &selected_request,
                &version,
            )?
        };
        Ok((distribution, warnings))
    })
    .await
    .map_err(|e| e.to_string())??;
    if revision_tree {
        acq.gaps.push(gap(EvidenceKind::DistributionSource,"Revision source layout is inferred statically; built/generated files, export exclusions, LFS and submodule contents are unverified"));
    }
    for warning in warnings {
        acq.gaps.push(gap(EvidenceKind::RegistryMetadata, warning));
    }
    release.root_module = distribution.import_roots.first().cloned();
    release.lib_name = release.root_module.clone();
    let worker_request = WorkerRequest {
        schema_version: python::worker::PROTOCOL.into(),
        root: input_root.to_string_lossy().into_owned(),
        files: distribution.files.clone(),
        max_observations: 100000,
        max_memory_bytes: 1024 * 1024 * 1024,
        max_cpu_seconds: service.config.producers.python.worker_timeout_seconds,
    };
    let mut inputs = BTreeMap::new();
    let mut native = None;
    let mut worker_identity = None;
    let mut producer_items = 0;
    let work = acq
        .work
        .ok_or("static extraction requires its durable acquisition job")?;
    match run_worker(service, &worker_request, &work.cancel).await {
        Ok((facts, directory)) => {
            let source = directory.path().join("worker.arrow");
            let blobs = service.blobs.clone();
            let artifact = tokio::task::spawn_blocking(move || {
                let _directory = directory;
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
                            &clock::now_rfc3339(),
                        );
                        artifact.artifact_id = enrichment_core::evidence::artifact_id_for(digest);
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
    let files = tokio::task::spawn_blocking(move || python::archive::files(&source_directory))
        .await
        .map_err(|e| e.to_string())??;
    for path in files {
        if !(path.ends_with(".py")
            || path.ends_with(".pyi")
            || path.ends_with(".md")
            || path.ends_with(".rst")
            || path.ends_with(".txt")
            || path.ends_with("METADATA")
            || path.ends_with("PKG-INFO")
            || path.ends_with("pyproject.toml"))
        {
            continue;
        }
        let lower = path.to_ascii_lowercase();
        let kind = if lower.contains("readme") {
            ArtifactKind::Readme
        } else if lower.contains("changelog") || lower.contains("changes") {
            ArtifactKind::Changelog
        } else {
            ArtifactKind::SourceFile
        };
        let Some(artifact) = acq
            .store_source_file(
                input_root.join(&path),
                kind,
                "text/plain",
                format!("{}#{path}", archive.source_uri),
            )
            .await
            .map_err(|e| e.to_string())?
        else {
            continue;
        };
        inputs.insert(format!("python-source:{path}"), artifact.sha256.clone());
        if matches!(kind, ArtifactKind::Readme | ArtifactKind::Changelog)
            || lower.starts_with("examples/")
        {
            let fk = if kind == ArtifactKind::Changelog {
                enrichment_core::evidence::FragmentKind::ChangelogSection
            } else if lower.starts_with("examples/") {
                enrichment_core::evidence::FragmentKind::Example
            } else {
                enrichment_core::evidence::FragmentKind::ReadmeSection
            };
            documents.file(path, fk, artifact, false)?;
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
    inputs.extend(acq.semantic_inputs());
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
    );
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
    let coverage=Coverage{details:None,assessments: Vec::new(),scope:format!("Static contents of {} {} ({})",request.name,release.key.version,file.filename),indexed:acq.indexed.iter().map(|k|k.as_str().into()).collect(),missing:acq.gaps.iter().map(|g|g.kind.as_str().into()).collect(),limitations:vec!["Static source/stub declarations are not executed or typechecker observations; dependencies and namespace contributions are not complete environments.".into()]};
    let freshness = Freshness {
        registry_checked_at: Some(clock::now_rfc3339()),
        source_version_match: SourceVersionMatch::Exact,
        latest_verified: !revision_tree && request.version.is_none(),
    };
    let partial = !acq.gaps.is_empty();
    let summary = format!(
        "{} {}: static distribution evidence",
        request.name, release.key.version
    );
    let artifacts = acq
        .artifacts
        .iter()
        .take(10)
        .filter_map(|a| common::handle_for(a, "Distribution evidence".into()))
        .collect::<Vec<_>>();
    let result = Research {
        summary,
        data: common::to_object(&data),
        coverage,
        freshness,
        context_id: Some(context.context_id.to_string()),
        snapshot_id: None,
        evidence: Vec::new(),
        artifacts,
    };
    acq.delivery_template = Some(if partial {
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
        .filter(|(_, snapshot, _)| snapshot == manifest.snapshot_id.as_str())
        .map(|(_, _, result)| result.clone())
        .ok_or_else(|| "Python resolution lacks prepared committed delivery".into())
}

async fn run_worker(
    service: &Service,
    request: &WorkerRequest,
    cancel: &std::sync::atomic::AtomicBool,
) -> Result<
    (
        enrichment_store::python_normalize::PythonFacts,
        std::sync::Arc<tempfile::TempDir>,
    ),
    String,
> {
    let cwd = service.paths.cache_root.join("workers");
    std::fs::create_dir_all(&cwd).map_err(|e| e.to_string())?;
    let directory = std::sync::Arc::new(
        tempfile::Builder::new()
            .prefix("python-arrow-")
            .tempdir_in(&cwd)
            .map_err(|e| e.to_string())?,
    );
    let mut output = tokio::fs::File::create(directory.path().join("worker.arrow"))
        .await
        .map_err(|e| e.to_string())?;
    let mut command = tokio::process::Command::new(&service.config.producers.python.worker_python);
    command
        .args(["-I", "-B", "-m", "enrichment_worker"])
        .current_dir(&cwd)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("HOME", &cwd)
        .env("TMPDIR", &cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    enrichment_store::native_effect::authorize()
        .await
        .map_err(|e| e.to_string())?;
    let mut child = command
        .spawn()
        .map_err(|e| format!("service-owned Griffe worker unavailable: {e}"))?;
    let bytes = serde_json::to_vec(request).map_err(|e| e.to_string())?;
    let mut stdin = child.stdin.take().ok_or("worker stdin unavailable")?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or("worker stdout unavailable")?
        .take(python::worker::MAX_BYTES + 1);
    let mut stderr = child
        .stderr
        .take()
        .ok_or("worker stderr unavailable")?
        .take(16385);
    let mut log = Vec::new();
    let run = async {
        let (_, _, _, status) = tokio::try_join!(
            async {
                stdin.write_all(&bytes).await?;
                drop(stdin);
                Ok::<_, std::io::Error>(())
            },
            async {
                let bytes = tokio::io::copy(&mut stdout, &mut output).await?;
                if bytes > python::worker::MAX_BYTES {
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
        result = tokio::time::timeout(Duration::from_secs(service.config.producers.python.worker_timeout_seconds), run) => {
            match result {
                Ok(Ok(status)) => Ok(status),
                Ok(Err(e)) => Err(e.to_string()),
                Err(_) => Err("static worker deadline exceeded".into()),
            }
        }
        () = super::resolve_job::cancelled(cancel) => Err("static worker cancelled".into()),
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
        || output.metadata().await.map_err(|e| e.to_string())?.len() > python::worker::MAX_BYTES
        || log.len() > 16384
    {
        return Err(format!(
            "static worker failed ({status}): {}",
            String::from_utf8_lossy(&log)
        ));
    }
    output.sync_all().await.map_err(|e| e.to_string())?;
    drop(output);
    let facts = enrichment_store::python_normalize::PythonFacts::open(
        &service.repository.runtime,
        directory.clone(),
        &request.files,
    )
    .await
    .map_err(|e| e.to_string())?;
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
    use enrichment_core::evidence::FragmentKind;
    use enrichment_core::wire::EvidenceClass;
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
    let version_match = if inventory.version == version {
        SourceVersionMatch::CompatibleClaimed
    } else {
        SourceVersionMatch::Unknown
    };
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
    let mut pages = BTreeSet::new();
    for entry in accepted.iter().take(3) {
        let mut target = Url::parse(&entry.uri).map_err(|e| e.to_string())?;
        target.set_fragment(None);
        if !pages.insert(target.to_string()) {
            continue;
        }
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
        let mut fragment = enrichment_core::evidence::document::DocumentFact::new(
            FragmentKind::DocText,
            &entry.name,
            &page_artifact.artifact_id,
            enrichment_core::evidence::relational::Locator::WebDocument {
                uri: entry.uri.clone(),
                inventory_version: inventory.version.clone(),
            },
            text.chars().take(8000).collect(),
            EvidenceClass::Declared,
            "official-document",
            "1",
        )?;
        fragment.source_version_match = Some(version_match);
        fragment.source_uri = Some(page_artifact.source_uri);
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
