//! Public immutable GitHub revisions, acquired and published entirely under service state.
use std::collections::BTreeMap;

use enrichment_core::{
    evidence::{Artifact, ArtifactKind, EvidenceKind, Gap, GapReason},
    identity::{Context, Ecosystem, Release, ReleaseKey},
    policy::ArchivePolicy,
    producer::{RunOutcome, docsrs, python, revision::Revision, source},
    request::{FreshnessMode, ResolveRequest},
    wire::{Envelope, ErrorCode, SourceVersionMatch, data::ResolveData},
};
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
    let staging = match service.repository.source_directory().await {
        Ok(directory) => directory,
        Err(error) => return failure(error.to_string()),
    };
    match acquire(service, &request, &identity, staging, work).await {
        Ok(answer) => answer,
        Err(error) => failure(error),
    }
}

async fn acquire(
    service: &Service,
    request: &ResolveRequest,
    identity: &Revision,
    staging: std::sync::Arc<enrichment_store::PrivateDirectory>,
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
    let metadata: enrichment_core::producer::revision::CommitResponse =
        serde_json::from_slice(&commit.bytes).map_err(|e| e.to_string())?;
    let tree = enrichment_store::revision_capture::commit_tree(
        &service.repository.runtime,
        &identity.commit,
        metadata,
    )
    .await
    .map_err(|e| e.to_string())?;
    let commit_artifact = acq
        .store(
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
    let extraction_owner = staging.clone();
    let commit_identity = identity.commit.clone();
    let package_subdir = identity.package_subdir.clone();
    let ecosystem = request.ecosystem;
    let name = request.name.clone();
    let archive_digest = stored.sha256.clone();
    let (root, text, declared_version, wrapper, extraction_inputs) = service.repository.runtime.blocking(move || -> Result<_, String> {
        let destination = extraction_owner.path();
        let extracted = enrichment_core::archive::extract_revision_tar_gz(
            std::io::Cursor::new(archive_bytes.as_ref()), destination, &policy, &commit_identity).map_err(|e| e.to_string())?;
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
    let root = super::source_tree::SourceTree::owned(staging, root).map_err(|e| e.to_string())?;
    let receipt = match enrichment_store::revision_capture::retain(
        service.repository.catalog.clone(),
        &service.repository.runtime,
        enrichment_core::operation::sources::RevisionCapture {
            identity: identity.clone(),
            tree: tree.to_owned(),
            declared_project_version: declared_version,
            archive: stored.clone(),
            commit: commit_artifact,
            archive_response: archive.metadata(),
            commit_response: commit.metadata(),
            inputs: extraction_inputs,
            decoder: enrichment_store::revision_capture::decoder_identity(),
        },
    )
    .await
    {
        Ok(receipt) => receipt,
        Err(error) => return Ok(common::query_error(&error.into())),
    };
    // The artifact is a bounded generated projection of the retained native source and its
    // assessment. The Delta capture is the authority; the byte receipt is a portable citation.
    use enrichment_core::native_union::Cell;
    let field = std::sync::Arc::new(enrichment_core::native_union::field::<
        enrichment_core::operation::sources::RevisionReceipt,
    >("receipt", enrichment_core::native_union::Rule::Text));
    let array = enrichment_core::operation::sources::RevisionReceipt::encode(&[Some(&receipt)])
        .map_err(|e| e.to_string())?;
    let mut receipt_bytes = Vec::new();
    enrichment_core::native_json::write_value(
        &mut receipt_bytes,
        8 * 1024 * 1024,
        &field,
        array.as_ref(),
        0,
    )
    .map_err(|e| e.to_string())?;
    let extraction = &receipt.extraction;
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
    acq.receipts.push(receipt_artifact.clone());
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
        acq.semantic_inputs().await?,
        started,
        RunOutcome::Succeeded,
        Vec::new(),
    )
    .await?;
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
    root: &super::source_tree::SourceTree,
    revision: &Revision,
    manifest: &str,
) -> Result<RustSources, String> {
    let facts = docsrs::manifest_facts(manifest).map_err(|e| e.to_string())?;
    let mut files = BTreeMap::new();
    let mut inputs = BTreeMap::new();
    let source_directory = root.to_owned();
    let source_files = acq
        .service
        .repository
        .runtime
        .blocking(move || python::archive::files(&source_directory))
        .await
        .map_err(|e| e.to_string())??;
    let captures = enrichment_store::source_capture::select(
        &acq.service.repository.runtime,
        source_files,
        enrichment_store::source_capture::Family::RustRevision,
    )
    .await
    .map_err(|e| e.to_string())?;
    for capture in &captures {
        let Some(artifact) = acq
            .store_source_file(
                root.clone(),
                capture.path.clone(),
                capture.artifact_kind,
                capture.media_type.clone(),
                revision.source_uri(&capture.path)?,
            )
            .await
            .map_err(|e| e.to_string())?
        else {
            continue;
        };
        inputs.insert(
            capture
                .role
                .clone()
                .ok_or("native revision source role missing")?,
            artifact.sha256.clone(),
        );
        files.insert(capture.path.clone(), artifact);
    }
    let manifest = files
        .get("Cargo.toml")
        .ok_or("Source manifest artifact missing")?
        .clone();
    let mut documents = super::source_documents::SourceDocuments::new(acq.service.blobs.clone());
    documents.features(facts, manifest)?;
    for capture in captures {
        let Some(kind) = capture.fragment_kind else {
            continue;
        };
        let file = capture.path;
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
    root: &super::source_tree::SourceTree,
    text: &str,
) -> Result<Envelope, String> {
    release.lib_name = Some(request.name.replace('-', "_"));
    release.root_module = release.lib_name.clone();
    let RustSources {
        documents,
        mut inputs,
    } = rust_sources(acq, root, &Revision::from_request(request)?, text).await?;
    inputs.extend(acq.semantic_inputs().await?);
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
    acq.run("revision-source","5",inputs,enrichment_core::native_time::ObservationTime::now().map_err(|error| error.to_string())?,RunOutcome::Partial,vec![gap(EvidenceKind::PublicApi,"Rust revision API requires an explicitly enabled isolated build; no released rustdoc JSON is substituted"),gap(EvidenceKind::CrateSource,"Archive contents may exclude generated files, export-ignored files, LFS objects and submodules; manifest version does not establish a published release association")]).await?;
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
    let presentation = enrichment_store::research_resolution::acquisition_presentation(
        &service.repository.runtime,
        &data,
        &acq.indexed.iter().copied().collect::<Vec<_>>(),
        None,
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

    acq.delivery_template = Some(
        Research {
            summary: presentation.summary,
            data: common::payload(&data),
            coverage: presentation.coverage,
            freshness,
            context_id: Some(context.context_id.clone()),
            snapshot_id: None,
            evidence: Vec::new(),
            artifacts: presentation.artifacts,
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
        .filter(|(_, snapshot, _)| snapshot == &manifest.snapshot_id)
        .map(|(_, _, result)| result.clone())
        .ok_or_else(|| "revision resolution lacks prepared committed delivery".into())
}
