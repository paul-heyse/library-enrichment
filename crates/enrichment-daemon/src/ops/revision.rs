//! Public immutable GitHub revisions, acquired and published entirely under service state.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use enrichment_core::{
    canonical,
    evidence::{Artifact, ArtifactKind, EvidenceKind, Gap, GapReason},
    identity::{Context, Ecosystem, Release, ReleaseKey},
    policy::ArchivePolicy,
    producer::{RunOutcome, docsrs, python, revision::Revision, source},
    request::{FreshnessMode, ResolveRequest},
    wire::{Coverage, Envelope, ErrorCode, Freshness, SourceVersionMatch, data::ResolveData},
};
use serde_json::json;
use url::Url;

use super::{
    common,
    resolve::{Acquisition, environment_for},
};
use crate::{
    envelope::{self, Research},
    service::Service,
};

fn failure(message: impl Into<String>) -> Envelope {
    envelope::error(
        ErrorCode::ExtractionFailed,
        message,
        "Use a public immutable GitHub commit and the exact package root; inspect the acquisition limitation before retrying.",
        false,
    )
}
fn gap(kind: EvidenceKind, detail: impl Into<String>) -> Gap {
    Gap {
        kind,
        reason: GapReason::ExtractionFailed,
        detail: detail.into(),
        planned_fallback: None,
    }
}

pub(super) async fn resolve(
    service: &Service,
    request: ResolveRequest,
    work: &super::resolve_job::Work,
) -> Envelope {
    let identity = match Revision::from_request(&request) {
        Ok(v) => v,
        Err(e) => return failure(e),
    };
    let staging = Scratch(
        service
            .paths
            .unpacked()
            .join(format!("revision-{}", uuid::Uuid::new_v4())),
    );
    let result = acquire(service, &request, &identity, &staging.0, work).await;
    let cleanup = tokio::task::spawn_blocking(move || {
        if staging.0.exists() {
            std::fs::remove_dir_all(&staging.0)?;
        }
        Ok::<_, std::io::Error>(())
    })
    .await;
    match (result, cleanup) {
        (Ok(answer), Ok(Ok(()))) => answer,
        (Err(e), _) => failure(e),
        (_, result) => failure(format!("Revision scratch cleanup failed: {result:?}")),
    }
}
struct Scratch(std::path::PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

async fn acquire(
    service: &Service,
    request: &ResolveRequest,
    identity: &Revision,
    staging: &Path,
    work: &super::resolve_job::Work,
) -> Result<Envelope, String> {
    let mut acq = Acquisition::new(service).for_job(work);
    let started =
        enrichment_core::native_time::ObservationTime::now().map_err(|error| error.to_string())?;
    let api = service
        .config
        .producers
        .github_api_url
        .trim_end_matches('/');
    let commit_url = Url::parse(&format!(
        "{api}/repos/{}/commits/{}",
        identity.repository_path, identity.commit
    ))
    .map_err(|e| e.to_string())?;
    let commit = service
        .fetcher
        .get_with_revalidation(
            &commit_url,
            Some("application/vnd.github+json"),
            request.freshness == FreshnessMode::Revalidate,
        )
        .await
        .map_err(|e| e.to_string())?;
    if commit.status != 200 {
        return Err(format!(
            "Commit acquisition returned HTTP {}; absence, access restrictions and rate limits are not a package identity",
            commit.status
        ));
    }
    let metadata: serde_json::Value =
        serde_json::from_slice(&commit.bytes).map_err(|e| e.to_string())?;
    if metadata["sha"].as_str() != Some(identity.commit.as_str()) {
        return Err("Resolved commit SHA does not equal the requested immutable revision".into());
    }
    let tree = metadata["commit"]["tree"]["sha"]
        .as_str()
        .ok_or("Commit response has no tree identity")?;
    if tree.len() != 40 || !tree.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid commit tree identity".into());
    }
    acq.store(
        &commit,
        ArtifactKind::RegistryVersionMetadata,
        "application/json",
        commit_url.as_str(),
    )
    .map_err(|e| e.to_string())?;
    let archive_url = Url::parse(&format!(
        "{api}/repos/{}/tarball/{}",
        identity.repository_path, identity.commit
    ))
    .map_err(|e| e.to_string())?;
    let mut archive = service
        .fetcher
        .get_with_revalidation(
            &archive_url,
            Some("application/vnd.github+json"),
            request.freshness == FreshnessMode::Revalidate,
        )
        .await
        .map_err(|e| e.to_string())?;
    if archive.status != 200 {
        return Err(format!("Revision archive returned HTTP {}", archive.status));
    }
    let stored = acq
        .store(
            &archive,
            ArtifactKind::Other,
            "application/gzip",
            archive_url.as_str(),
        )
        .map_err(|e| e.to_string())?;
    let policy = ArchivePolicy {
        max_total_bytes: service.config.network.max_decompressed_bytes,
        ..ArchivePolicy::default()
    };
    let archive_bytes = std::mem::take(&mut archive.bytes);
    let destination = staging.to_owned();
    let commit_identity = identity.commit.clone();
    let package_subdir = identity.package_subdir.clone();
    let ecosystem = request.ecosystem;
    let name = request.name.clone();
    let archive_digest = stored.sha256.clone();
    let (root, text, declared_version, wrapper, extraction_inputs) = tokio::task::spawn_blocking(move || -> Result<_, String> {
        let extracted = enrichment_core::archive::extract_revision_tar_gz(
            std::io::Cursor::new(archive_bytes.as_slice()), &destination, &policy, &commit_identity).map_err(|e| e.to_string())?;
        let wrapper = extracted.top_level.clone().ok_or("Revision archive must have one common wrapper directory")?;
        if !destination.join(&wrapper).is_dir() { return Err("Archive wrapper is not a directory".into()); }
        let root = destination.join(&wrapper).join(&package_subdir);
        let manifest_name = match ecosystem { Ecosystem::Rust => "Cargo.toml", Ecosystem::Python => "pyproject.toml" };
        let bytes = source::read_file(&root.join(manifest_name)).map_err(|_| format!("Selected package root lacks a bounded {manifest_name}; supply package_subdir explicitly"))?;
        let text = String::from_utf8(bytes).map_err(|e| e.to_string())?;
        let declared = enrichment_core::producer::revision::project_identity(&text, ecosystem, &name)?;
        let disposition = enrichment_core::producer::revision::RevisionInputs::collect(
            archive_digest, &extracted, &package_subdir, ecosystem, &text)?;
        Ok((root, text, declared, wrapper, disposition))
    }).await.map_err(|e| e.to_string())??;
    let extraction = match enrichment_store::coverage::assess_revision_inputs(
        &service.repository.runtime,
        extraction_inputs,
    )
    .await
    {
        Ok(extraction) => extraction,
        Err(error) => return Ok(common::query_error(&error.into())),
    };
    let acquisition_id = uuid::Uuid::new_v4().to_string();
    let receipt = json!({"acquisition_id":acquisition_id,"repository":identity.repository,"package_subdir":identity.package_subdir,"source_revision":identity.commit,"tree":tree,"declared_project_version":declared_version,"archive_sha256":stored.sha256,"extraction":extraction,"archive_request":archive_url.as_str(),"archive_http":archive,"commit_request":commit_url.as_str(),"commit_http":commit});
    let receipt_bytes = canonical::to_canonical_string(&receipt).into_bytes();
    let retrieved_at =
        enrichment_core::native_time::AcquisitionTime::now().map_err(|error| error.to_string())?;
    let receipt_artifact = service
        .blobs
        .put(&receipt_bytes, |_| {
            Artifact::describe(
                &receipt_bytes,
                ArtifactKind::Other,
                "application/json",
                &format!(
                    "{}@{}#{}",
                    identity.repository, identity.commit, identity.package_subdir
                ),
                retrieved_at,
            )
        })
        .map_err(|e| e.to_string())?
        // This call's record: the receipt URI is `repository@commit#subdir`, so two commits with
        // an identical receipt would otherwise cite the earlier commit.
        .acquired;
    acq.receipt_ids.insert(receipt_artifact.artifact_id.clone());
    acq.remember_artifact(receipt_artifact.clone())
        .map_err(|e| e.to_string())?;
    let mut release = Release::new(ReleaseKey {
        ecosystem: request.ecosystem,
        registry: identity.registry(),
        package: request.name.clone(),
        version: identity.commit.clone(),
        artifact_digest: Some(stored.sha256.clone()),
    });
    release.links.repository = Some(identity.repository.clone());
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    if extraction.source_closure == enrichment_core::producer::revision::SourceClosure::Incomplete {
        acq.gaps.push(gap(if request.ecosystem == Ecosystem::Rust { EvidenceKind::CrateSource }
            else { EvidenceKind::DistributionSource }, format!(
            "Revision source closure is incomplete; missing declared inputs: {:?}; omitted potentially required inputs: {:?}. See extraction receipt {}.",
            extraction.missing_inputs, extraction.affected_omissions, receipt_artifact.artifact_id)));
    }
    acq.run(
        "github-revision",
        "2",
        acq.semantic_inputs(),
        started,
        RunOutcome::Succeeded,
        Vec::new(),
    )?;
    acq.runs
        .last_mut()
        .ok_or("revision acquisition attempt missing")?
        .log = Some(receipt_artifact.artifact_id);
    if request.ecosystem == Ecosystem::Python {
        let file = python::DistributionFile {
            filename: format!("{}-{}.tar.gz", request.name, identity.commit),
            packagetype: "sdist".into(),
            url: archive_url.into(),
            digests: BTreeMap::from([("sha256".into(), stored.sha256.clone())]),
            requires_python: None,
            yanked: false,
        };
        super::python::produce(
            service,
            request,
            &mut acq,
            &mut release,
            &context,
            &environment,
            super::python::Production {
                input_root: &root,
                source_root: &format!("{wrapper}/{}", identity.package_subdir),
                file: &file,
                archive: &stored,
                started,
                revision_tree: true,
            },
        )
        .await
    } else {
        publish_rust(
            service,
            request,
            &mut acq,
            &mut release,
            &context,
            &environment,
            &root,
            &text,
        )
        .await
    }
}

struct RustSources {
    documents: super::source_documents::SourceDocuments,
    inputs: BTreeMap<String, String>,
}
async fn rust_sources(
    acq: &mut Acquisition<'_>,
    root: &Path,
    revision: &Revision,
    manifest: &str,
) -> Result<RustSources, String> {
    let facts = docsrs::manifest_facts(manifest).map_err(|e| e.to_string())?;
    let mut files = BTreeMap::new();
    let mut inputs = BTreeMap::new();
    let source_directory = root.to_owned();
    let (source_files, document_files) = tokio::task::spawn_blocking(move || {
        Ok::<_, String>((
            python::archive::files(&source_directory)?,
            source::text_files(&source_directory).map_err(|e| e.to_string())?,
        ))
    })
    .await
    .map_err(|e| e.to_string())??;
    for path in source_files {
        let Some(artifact) = acq
            .store_source_file(
                root.join(&path),
                ArtifactKind::SourceFile,
                "text/plain",
                revision.source_uri(&path)?,
            )
            .await
            .map_err(|e| e.to_string())?
        else {
            continue;
        };
        inputs.insert(format!("revision-source:{path}"), artifact.sha256.clone());
        files.insert(path, artifact);
    }
    let manifest = files
        .get("Cargo.toml")
        .ok_or("Source manifest artifact missing")?
        .clone();
    let mut documents = super::source_documents::SourceDocuments::new(acq.service.blobs.clone());
    documents.features(facts, manifest)?;
    for (file, kind) in document_files {
        let artifact = files
            .get(&file)
            .ok_or("source document artifact missing")?
            .clone();
        documents.file(file, kind, artifact, true)?;
    }
    Ok(RustSources { documents, inputs })
}

async fn publish_rust(
    service: &Service,
    request: &ResolveRequest,
    acq: &mut Acquisition<'_>,
    release: &mut Release,
    context: &Context,
    environment: &enrichment_core::identity::Environment,
    root: &Path,
    text: &str,
) -> Result<Envelope, String> {
    release.lib_name = Some(request.name.replace('-', "_"));
    release.root_module = release.lib_name.clone();
    let RustSources {
        documents,
        mut inputs,
    } = rust_sources(acq, root, &Revision::from_request(request)?, text).await?;
    inputs.extend(acq.semantic_inputs());
    acq.indexed.extend([
        EvidenceKind::RegistryMetadata,
        EvidenceKind::CrateSource,
        EvidenceKind::DocumentationBuildConfig,
    ]);
    for kind in documents.kinds() {
        acq.indexed.insert(match kind {
            enrichment_core::evidence::FragmentKind::ChangelogSection => EvidenceKind::ReleaseNotes,
            enrichment_core::evidence::FragmentKind::Example => EvidenceKind::Examples,
            _ => EvidenceKind::Documentation,
        });
    }
    acq.run("revision-source","5",inputs,enrichment_core::native_time::ObservationTime::now().map_err(|error| error.to_string())?,RunOutcome::Partial,vec![gap(EvidenceKind::PublicApi,"Rust revision API requires an explicitly enabled isolated build; no released rustdoc JSON is substituted"),gap(EvidenceKind::CrateSource,"Archive contents may exclude generated files, export-ignored files, LFS objects and submodules; manifest version does not establish a published release association")])?;
    let producers = BTreeMap::from([
        ("github-revision".into(), "2".into()),
        ("revision-source".into(), "5".into()),
    ]);
    let data = ResolveData {
        release: release.clone(),
        environment: environment.clone(),
        context: context.clone(),
        upstream: None,
        observed_configuration: None,
        hosted_rustdoc_json: None,
        python: None,
        snapshot: None,
        artifacts: acq.artifacts.clone(),
        producer_runs: acq.runs.clone(),
        gaps: acq.gaps.clone(),
        answered_from_cache: false,
    };
    let coverage = Coverage {
        details: None,
        assessments: Vec::new(),
        scope: format!(
            "Immutable repository source {} {}",
            request.name, release.key.version
        ),
        indexed: acq.indexed.iter().map(|k| k.as_str().into()).collect(),
        missing: BTreeSet::from([EvidenceKind::PublicApi.as_str().into()]),
        limitations: acq.gaps.iter().map(|g| g.detail.clone()).collect(),
    };
    let freshness = Freshness {
        registry_checked_at: Some(
            enrichment_core::native_time::AcquisitionTime::now()
                .map_err(|error| error.to_string())?,
        ),
        source_version_match: SourceVersionMatch::Exact,
        latest_verified: false,
    };
    let summary = format!(
        "{} at {}: source and declarations; compiled API unobserved",
        request.name, release.key.version
    );
    let artifacts = acq
        .artifacts
        .iter()
        .take(10)
        .filter_map(|a| common::handle_for(a, "Revision source evidence".into()))
        .collect::<Vec<_>>();
    acq.delivery_template = Some(
        Research {
            summary,
            data: common::payload(&data),
            coverage,
            freshness,
            context_id: Some(context.context_id.to_string()),
            snapshot_id: None,
            evidence: Vec::new(),
            artifacts,
        }
        .partial(),
    );
    let manifest = super::publication::publish(
        service,
        acq,
        enrichment_core::evidence::snapshot::SnapshotMetadata {
            context: context.clone(),
            release: release.clone(),
            environment: environment.clone(),
            symbol_package: request.name.replace('-', "_"),
            crate_name: request.name.replace('-', "_"),
            crate_version: None,
            normalizer_version: "revision-source-5".into(),
            observed_configuration: None,
            producer_items: 0,
        },
        (documents, None),
        producers,
        acq.indexed.iter().copied().collect(),
        acq.gaps
            .iter()
            .map(|g| g.kind)
            .filter(|kind| !acq.indexed.contains(kind))
            .collect(),
        None,
    )
    .await?;
    acq.work
        .and_then(|work| work.committed.get())
        .filter(|(_, snapshot, _)| snapshot == manifest.snapshot_id.as_str())
        .map(|(_, _, result)| result.clone())
        .ok_or_else(|| "revision resolution lacks prepared committed delivery".into())
}
