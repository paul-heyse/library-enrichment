//! `library.resolve`: establish exact identity and environment before research (blueprint §7,
//! `resolve_library`), running the static Rust plan (§8.1) end to end.
//!
//! ```text
//! registry index  -> select the exact version, note the newest release separately (R01)
//! version record  -> links, licence, edition
//! crate tarball   -> verified against the index checksum, extracted under policy, manifest read
//! docs.rs JSON    -> available | missing | unsupported, each a distinct fact (R03, R04)
//! normalize       -> symbols, relationships, fragments; then an immutable snapshot (§8.2)
//! ```
//!
//! Every fetched byte becomes a content-addressed artifact; the release, environment and
//! context are recorded in the catalog; and the whole resolution is recorded so that
//! `cache_ok` and `offline` can replay it without a socket. A replay says so.

use std::collections::{BTreeMap, BTreeSet};

use enrichment_core::clock;
use enrichment_core::config::Config;
use enrichment_core::evidence::{
    Artifact, ArtifactKind, EvidenceKind, FragmentKind, Gap, GapReason, ObservedConfiguration,
    PlannedFallback,
};
use enrichment_core::identity::{
    Context, Ecosystem, Environment, Release, ReleaseKey, ReleaseLinks,
};
use enrichment_core::policy::ExecutionProfile;
use enrichment_core::producer::docsrs::{self, DocsRsMetadata, HostedJson, ManifestFacts};
use enrichment_core::producer::normalize;
use enrichment_core::producer::{ProducerRun, RunOutcome, cratesio, rustdoc, source};
use enrichment_core::registry::{self, IndexEntry, SelectionError, UpstreamCheck};
use enrichment_core::request::{FreshnessMode, ResolveRequest};
use enrichment_core::wire::data::{
    HostedJsonReport, HostedJsonState, ResolveData, SnapshotSummary,
};
use enrichment_core::wire::{
    ArtifactHandle, Coverage, Envelope, ErrorCode, Evidence, EvidenceClass, Freshness,
    SourceVersionMatch,
};
use serde::Deserialize;

use crate::envelope::{self, Research};
use crate::execution::Runner;
use crate::fetch::{FetchError, Fetched};
use crate::service::Service;

/// Resolve a request to a context, fetching what freshness allows.
pub async fn resolve(service: &Service, mut request: ResolveRequest) -> Envelope {
    if let Err(message) = request.validate() {
        return envelope::error(
            ErrorCode::VersionNotFound,
            message,
            "Pass an exact package version or an explicit upstream/revision request.",
            false,
        );
    }
    if request.ecosystem == Ecosystem::Python {
        request.name = enrichment_core::producer::python::normalize_name(&request.name);
    }
    let retained_version =
        if request.effective_mode() == enrichment_core::identity::ResearchMode::Revision {
            request.revision.as_deref()
        } else {
            request.version.as_deref()
        };
    if request.freshness != FreshnessMode::Revalidate
        && let Some(version) = retained_version
        && let Some(replay) = replay_recorded(service, &request, version).await
    {
        return replay;
    }
    if request.freshness == FreshnessMode::Offline {
        return envelope::error(
            ErrorCode::ArtifactUnavailable,
            "No retained exact resolution matches this request; offline forbids acquisition.",
            "Resolve this exact release with freshness=cache_ok once, then retain its context and snapshot.",
            false,
        );
    }
    if !ExecutionProfile::Static.is_enabled(&service.config) {
        return envelope::error(
            ErrorCode::PolicyDenied,
            "Static acquisition is disabled",
            "Enable the static profile in service configuration.",
            false,
        );
    }
    super::resolve_job::submit(service, request).await
}

/// What makes two acquisitions the same work (§8.2).
///
/// The ecosystem and release being acquired, the environment the resulting context binds, the
/// freshness policy in force, and whether a local build was asked for. A caller differing in any
/// of those is not asking for this run's output, and must not be handed it.
pub(super) fn acquisition_key(service: &Service, request: &ResolveRequest) -> String {
    enrichment_core::canonical::digest_hex(&serde_json::json!({
        "producer": ["resolve-job/1", cratesio::VERSION, rustdoc::NORMALIZER_VERSION, enrichment_core::producer::python::VERSION],
        "registry": [&service.config.producers.rust.crates_io_index_url, &service.config.producers.rust.crates_io_api_url, &service.config.producers.rust.docs_rs_url, &service.config.producers.python.pypi_url, &service.config.producers.python.simple_url, &service.config.producers.github_api_url],
        "request": request,
        "execution": format!("{:?}", service.config.execution),
        "ecosystem": request.ecosystem,
        "name": request.name,
        "version": request.version,
        "mode": request.effective_mode(),
        "environment": environment_for(request).environment_id,
        "freshness": request.freshness,
        "allow_local_build": request.allow_local_build,
        "profiles": service.config.policy.enabled_profiles,
    }))
}

/// The sentence a caller reads when it attached to someone else's run.
pub(super) const SHARED_RUN_LIMITATION: &str = concat!(
    "This answer came from an acquisition already in flight for the same release and ",
    "environment; one producer run served both callers (§8.2)."
);

pub(super) async fn replay_recorded(
    service: &Service,
    request: &ResolveRequest,
    version: &str,
) -> Option<Envelope> {
    match replay_checked(service, request, version).await {
        Ok(replay) => replay,
        Err(error) => Some(super::common::query_error(&error)),
    }
}

async fn replay_checked(
    service: &Service,
    request: &ResolveRequest,
    version: &str,
) -> Result<Option<Envelope>, enrichment_store::QueryError> {
    let registry = if request.effective_mode() == enrichment_core::identity::ResearchMode::Revision
    {
        enrichment_core::producer::revision::Revision::from_request(request)
            .map_err(std::io::Error::other)?
            .registry()
    } else if request.ecosystem == Ecosystem::Python {
        service.config.producers.python.pypi_url.clone()
    } else {
        registry::CRATES_IO.into()
    };
    let catalog = service.repository.catalog.pin().await?;
    let runtime = &service.repository.runtime;
    let Some(release) = catalog
        .find_release(
            runtime,
            request.ecosystem,
            Some(&registry),
            &request.name,
            version,
        )
        .await?
    else {
        return Ok(None);
    };
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    let Some(snapshot_id) = catalog.current(runtime, &context.context_id).await? else {
        return Ok(None);
    };
    let reader =
        enrichment_store::SnapshotReader::open(&service.repository, catalog, &snapshot_id).await?;
    let manifest = reader.manifest();
    let producer_runs = reader.producer_runs().await?;
    if request.allow_local_build
        && manifest.missing.contains(&EvidenceKind::PublicApi)
        && !producer_runs
            .iter()
            .any(|r| r.producer == rustdoc::LOCAL_PRODUCER)
    {
        return Ok(None);
    }
    render_retained(&reader, &release, &environment, &context)
        .await
        .map(Some)
}

pub(super) async fn render_retained(
    reader: &enrichment_store::SnapshotReader,
    release: &Release,
    environment: &Environment,
    context: &Context,
) -> Result<Envelope, enrichment_store::QueryError> {
    use enrichment_core::evidence::metadata::ReleaseDetails;
    let manifest = reader.manifest();
    let snapshot_id = &manifest.snapshot_id;
    let producer_runs = reader.producer_runs().await?;
    let mut observed_configuration = None;
    let mut python = None;
    for metadata in reader.release_metadata().await? {
        match metadata.details {
            ReleaseDetails::RustDocs(value) => {
                if observed_configuration
                    .as_ref()
                    .is_some_and(|prior| prior != &value)
                {
                    return Err(std::io::Error::other(
                        "retained Rust metadata has conflicting qualified alternatives",
                    )
                    .into());
                }
                observed_configuration = Some(value);
            }
            ReleaseDetails::PythonDistribution(value) => {
                if python.as_ref().is_some_and(|prior| prior != &value) {
                    return Err(std::io::Error::other(
                        "retained Python metadata has conflicting qualified alternatives",
                    )
                    .into());
                }
                python = Some(value);
            }
        }
    }
    let coverage_rows = reader.coverage().await?;
    let mut gaps = Vec::new();
    for coverage in coverage_rows {
        for gap in coverage.gaps {
            if !gaps.contains(&gap) {
                gaps.push(gap);
            }
        }
    }
    let artifacts = reader.artifacts().await?;
    let hosted = artifacts.iter().find(|a| {
        a.kind == ArtifactKind::RustdocJson
            && (a.source_uri.starts_with("https://") || a.source_uri.starts_with("http://"))
    });
    let hosted_rustdoc_json = hosted.map(|a| HostedJsonReport {
        state: HostedJsonState::Available,
        format_version: manifest
            .observed_configuration
            .as_ref()
            .map(|o| o.format_version),
        supported_formats: rustdoc::SUPPORTED_FORMAT_VERSIONS.to_vec(),
        target: manifest
            .observed_configuration
            .as_ref()
            .map_or_else(String::new, |o| o.target.clone()),
        url: a.source_uri.clone(),
        declared_crate_version: manifest.crate_version.clone(),
    });
    let summary = SnapshotSummary {
        snapshot_id: snapshot_id.to_string(),
        normalizer_version: manifest.normalizer_version.clone(),
        counts: manifest.counts.clone(),
        published_at: manifest.published_at.clone(),
    };
    let data = ResolveData {
        release: release.clone(),
        environment: environment.clone(),
        context: context.clone(),
        upstream: None,
        observed_configuration,
        hosted_rustdoc_json,
        python,
        snapshot: Some(summary),
        artifacts: artifacts.clone(),
        gaps,
        producer_runs,
        answered_from_cache: true,
    };
    let coverage = Coverage {
        scope: format!("retained evidence for {} {} and its exact environment", release.key.package, release.key.version),
        indexed: manifest.indexed.iter().map(|k| k.as_str().into()).collect(), missing: manifest.missing.iter().map(|k| k.as_str().into()).collect(),
        limitations: vec!["Exact validated evidence is retained without age-based expiry. This call did not consult the mutable registry; freshness=revalidate checks it.".into()],
    };
    let result = Research {
        summary: format!(
            "{} {}: retained evidence from {}",
            release.key.package, release.key.version, snapshot_id
        ),
        data: to_object(&data),
        coverage,
        freshness: Freshness {
            registry_checked_at: None,
            source_version_match: if manifest.crate_version.as_deref()
                == Some(release.key.version.as_str())
                || context.mode == enrichment_core::identity::ResearchMode::Revision
            {
                SourceVersionMatch::Exact
            } else {
                SourceVersionMatch::Unknown
            },
            latest_verified: false,
        },
        context_id: Some(context.context_id.to_string()),
        snapshot_id: Some(snapshot_id.to_string()),
        evidence: Vec::new(),
        artifacts: artifacts
            .iter()
            .filter_map(|a| super::common::handle_for(a, "Retained acquisition evidence".into()))
            .collect(),
    };
    Ok(if data.gaps.is_empty() {
        result.ok()
    } else {
        result.partial()
    })
}

/// A fresh mutable version-pointer lookup can reuse the exact evidence it selected.
/// Selection checks the artifact identity; a reused version label alone is insufficient.
pub(super) async fn replay_selected(
    service: &Service,
    request: &ResolveRequest,
    selected: &Release,
    upstream: Option<&UpstreamCheck>,
    acquisition: &mut Acquisition<'_>,
) -> Option<Envelope> {
    if request.version.is_some() || request.freshness == FreshnessMode::Revalidate {
        return None;
    }
    let catalog = match service.repository.catalog.pin().await {
        Ok(value) => value,
        Err(e) => return Some(super::common::store_error(&e)),
    };
    let recorded = match catalog
        .find_release(
            &service.repository.runtime,
            selected.key.ecosystem,
            Some(&selected.key.registry),
            &selected.key.package,
            &selected.key.version,
        )
        .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return None,
        Err(e) => return Some(super::common::store_error(&e)),
    };
    if recorded.release_id != selected.release_id {
        return None;
    }
    let mut exact = request.clone();
    exact.mode = Some(request.effective_mode());
    exact.name = selected.key.package.clone();
    exact.version = Some(selected.key.version.clone());
    let mut replay = replay_recorded(service, &exact, &selected.key.version).await?;
    replay.freshness.registry_checked_at = Some(clock::now_rfc3339());
    replay.freshness.latest_verified = upstream.is_some();
    if let Some(upstream) = upstream {
        replay
            .data
            .insert("upstream".into(), serde_json::json!(upstream));
    }
    replay.coverage.limitations.pop();
    replay.coverage.limitations.push("The mutable registry selection was revalidated; unchanged exact artifact and environment reuse retained evidence without running extraction again.".into());
    replay.evidence.extend(acquisition.evidence.clone());
    for artifact in &acquisition.artifacts {
        if let Some(handle) =
            super::common::handle_for(artifact, "Current registry selection".into())
        {
            replay.artifacts.push(handle);
        }
    }
    // The mutable lookup is real work, even when extraction is unnecessary. Commit its
    // exact selected result and actual registry attempt through the same durable publication.
    if acquisition.work.is_some() {
        let selected_context = replay.context_id.as_deref()?;
        let opened = match super::common::open_context(
            service,
            selected_context,
            replay.snapshot_id.as_deref(),
        )
        .await
        {
            Ok(opened) => opened,
            Err(e) => return Some(*e),
        };
        let parent = opened.reader.manifest();
        let metadata = enrichment_core::evidence::snapshot::SnapshotMetadata {
            context: opened.context.clone(),
            release: opened.release.clone(),
            environment: opened.environment.clone(),
            symbol_package: parent.symbol_package.clone(),
            crate_name: parent.crate_name.clone(),
            crate_version: parent.crate_version.clone(),
            normalizer_version: parent.normalizer_version.clone(),
            observed_configuration: parent.observed_configuration.clone(),
            producer_items: parent.producer_items,
        };
        acquisition.run(
            "registry-selection",
            "1",
            acquisition.semantic_inputs(),
            clock::now_rfc3339(),
            RunOutcome::Succeeded,
            Vec::new(),
        );
        acquisition.delivery_template = Some(replay.clone());
        let manifest = match super::publication::publish(
            service,
            acquisition,
            metadata,
            enrichment_core::evidence::ingest::ProducerBatch::default(),
            [("registry-selection".into(), "1".into())].into(),
            vec![EvidenceKind::RegistryMetadata],
            Vec::new(),
            None,
        )
        .await
        {
            Ok(manifest) => manifest,
            Err(e) => return Some(super::common::store_error(&std::io::Error::other(e))),
        };
        let work = acquisition.work?;
        return Some(match work.committed.get() {
            Some((_, snapshot, result)) if snapshot == manifest.snapshot_id.as_str() => {
                result.clone()
            }
            _ => super::common::store_error(
                &"selected resolution has no prepared committed delivery",
            ),
        });
    }
    Some(replay)
}

pub(super) fn environment_for(request: &ResolveRequest) -> Environment {
    if request.ecosystem == Ecosystem::Python {
        return Environment::python(
            request.python_version.clone(),
            request.target.clone(),
            request.extras.clone(),
        );
    }

    if request.declares_environment() {
        Environment::declared(
            request.target.clone(),
            request.features.clone(),
            request.default_features,
        )
    } else {
        Environment::unspecified()
    }
}

/// One acquisition, accumulating what the envelope needs.
pub(super) struct Acquisition<'a> {
    pub(super) service: &'a Service,
    pub(super) work: Option<&'a super::resolve_job::Work>,
    pub(super) config: &'a Config,
    pub(super) artifacts: Vec<Artifact>,
    artifact_bytes: usize,
    pub(super) evidence: Vec<Evidence>,
    pub(super) gaps: Vec<Gap>,
    pub(super) runs: Vec<ProducerRun>,
    pub(super) indexed: BTreeSet<EvidenceKind>,
    pub(super) receipt_ids: BTreeSet<String>,
    pub(super) delivery_template: Option<Envelope>,
}

impl<'a> Acquisition<'a> {
    pub(super) fn new(service: &'a Service) -> Self {
        Self {
            service,
            work: None,
            config: &service.config,
            artifacts: Vec::new(),
            artifact_bytes: 0,
            evidence: Vec::new(),
            gaps: Vec::new(),
            runs: Vec::new(),
            indexed: BTreeSet::new(),
            receipt_ids: BTreeSet::new(),
            delivery_template: None,
        }
    }

    pub(super) fn for_job(mut self, work: &'a super::resolve_job::Work) -> Self {
        self.work = Some(work);
        self
    }
    fn check_cancelled(&self) -> std::io::Result<()> {
        if let Some(work) = self.work {
            work.check()?;
        }
        Ok(())
    }

    pub(super) fn remember_artifact(&mut self, artifact: Artifact) -> std::io::Result<Artifact> {
        let bytes = enrichment_core::canonical::serialized_size(&artifact, 1024 * 1024)?;
        self.artifact_bytes = self
            .artifact_bytes
            .checked_add(bytes)
            .filter(|n| *n <= 16 * 1024 * 1024)
            .ok_or_else(|| std::io::Error::other("acquisition descriptors exceed 16 MiB"))?;
        if self.artifacts.len() >= 8192 {
            return Err(std::io::Error::other("acquisition descriptors exceed 8192"));
        }
        self.artifacts.push(artifact.clone());
        Ok(artifact)
    }

    /// Source file reads, hashing and durable artifact writes run on the blocking pool.
    pub(super) async fn store_source_file(
        &mut self,
        path: std::path::PathBuf,
        kind: ArtifactKind,
        media: &'static str,
        source: String,
    ) -> std::io::Result<Option<Artifact>> {
        self.check_cancelled()?;
        let blobs = self.service.blobs.clone();
        let artifact = tokio::task::spawn_blocking(move || {
            let bytes = enrichment_core::producer::source::read_file(&path)?;
            if std::str::from_utf8(&bytes).is_err() {
                return Ok::<_, std::io::Error>(None);
            }
            let stored = blobs.put(&bytes, |_| {
                Artifact::describe(&bytes, kind, media, &source, &clock::now_rfc3339())
            })?;
            Ok(Some(stored.acquired))
        })
        .await
        .map_err(std::io::Error::other)??;
        self.check_cancelled()?;
        artifact
            .map(|artifact| self.remember_artifact(artifact))
            .transpose()
    }

    /// Operational acquisition receipts are retained, but are not semantic snapshot inputs.
    pub(super) fn semantic_inputs(&self) -> BTreeMap<String, String> {
        self.artifacts
            .iter()
            .filter(|artifact| !self.receipt_ids.contains(&artifact.artifact_id))
            .map(|artifact| (artifact.artifact_id.clone(), artifact.sha256.clone()))
            .collect()
    }

    pub(super) fn store(
        &mut self,
        fetched: &Fetched,
        kind: ArtifactKind,
        media_type: &str,
        source: &str,
    ) -> std::io::Result<Artifact> {
        self.check_cancelled()?;
        let stored = self.service.blobs.put(&fetched.bytes, |_| {
            let mut artifact = Artifact::describe(
                &fetched.bytes,
                kind,
                media_type,
                source,
                &fetched.retrieved_at,
            );
            artifact.final_url = (fetched.final_url != source).then(|| fetched.final_url.clone());
            artifact.etag = fetched.etag.clone();
            artifact.last_modified = fetched.last_modified.clone();
            artifact
        })?;
        // `acquired`, not `artifact`: identical bytes share one blob, and the stored record keeps
        // the FIRST retrieval's locator. A README unchanged across a patch release hashes the
        // same, so reporting the stored record would cite the older release as the source of
        // evidence gathered from the newer one. Identity is the digest; the locator is this call's.
        self.remember_artifact(stored.acquired)
    }

    pub(super) fn store_bytes(
        &mut self,
        bytes: &[u8],
        kind: ArtifactKind,
        media_type: &str,
        source: &str,
        compression: Option<&str>,
    ) -> std::io::Result<Artifact> {
        self.check_cancelled()?;
        let now = clock::now_rfc3339();
        let stored = self.service.blobs.put(bytes, |_| {
            let mut artifact = Artifact::describe(bytes, kind, media_type, source, &now);
            artifact.compression = compression.map(str::to_owned);
            artifact
        })?;
        self.remember_artifact(stored.acquired)
    }

    pub(super) fn run(
        &mut self,
        producer: &str,
        version: &str,
        inputs: BTreeMap<String, String>,
        started_at: String,
        outcome: RunOutcome,
        gaps: Vec<Gap>,
    ) {
        self.runs.push(ProducerRun {
            attempt_id: uuid::Uuid::new_v4().to_string(),
            producer: producer.to_owned(),
            producer_version: version.to_owned(),
            config_digest: enrichment_core::producer::spec::config_digest(&serde_json::json!({
                "github": self.config.producers.github_api_url,
                "index": self.config.producers.rust.crates_io_index_url,
                "api": self.config.producers.rust.crates_io_api_url,
                "docs": self.config.producers.rust.docs_rs_url,
                "python": if producer == "python-static" {Some(serde_json::json!({
                    "pypi":self.config.producers.python.pypi_url,
                    "simple":self.config.producers.python.simple_url,
                    "worker":self.config.producers.python.worker_python,
                    "deadline":self.config.producers.python.worker_timeout_seconds,
                }))}else{None},
            })),
            inputs,
            profile: ExecutionProfile::Static,
            started_at,
            finished_at: clock::now_rfc3339(),
            outcome,
            gaps: gaps.clone(),
            log: None,
        });
        self.gaps.extend(gaps);
    }

    fn evidence(
        &mut self,
        subject: &str,
        artifact: &Artifact,
        locator: serde_json::Value,
        producer: &str,
        producer_version: &str,
        excerpt: &str,
    ) {
        let excerpt = truncate(excerpt, self.config.limits.excerpt_characters);
        let evidence_id = format!(
            "ev_{}",
            &enrichment_core::canonical::digest_hex(&serde_json::json!({
                "artifact": artifact.artifact_id, "subject": subject, "locator": locator
            }))[..16]
        );
        self.evidence.push(Evidence {
            evidence_id,
            evidence_class: EvidenceClass::Declared,
            subject: subject.to_owned(),
            artifact_id: artifact.artifact_id.clone(),
            source_uri: artifact.source_uri.clone(),
            locator: locator
                .as_object()
                .expect("acquisition locator is an object")
                .clone(),
            source_version_match: SourceVersionMatch::Exact,
            producer: producer.to_owned(),
            producer_version: producer_version.to_owned(),
            excerpt,
        });
    }

    /// The producer that would supply the missing API, and what it would take to run it.
    ///
    /// Named for the producer that actually runs (`locally_built_rustdoc`), so the name a gap
    /// advertises is the name that appears in the resulting snapshot's provenance. A caller
    /// following this advice should be able to recognise what it got.
    fn fallback_for_json(&self) -> PlannedFallback {
        let enabled = ExecutionProfile::Build.is_enabled(self.config);
        PlannedFallback {
            producer: rustdoc::LOCAL_PRODUCER.to_owned(),
            profile: ExecutionProfile::Build.to_string(),
            enabled,
            next_action: if enabled {
                "Resolve again with allow_local_build to compile this crate's documentation on \
                 the dated nightly inside a capsule. It needs admitted producer images: run \
                 `just execution-images --apply` then `just execution-qualify --apply`."
                    .to_owned()
            } else {
                "Enable the `build` profile in [policy].enabled_profiles once a sandbox is \
                 configured, then resolve again with allow_local_build; until then only registry \
                 and source evidence is available."
                    .to_owned()
            },
        }
    }
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max_chars.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn to_object(data: &ResolveData) -> enrichment_core::wire::JsonObject {
    super::common::to_object(data)
}

fn fetch_error(err: &FetchError, what: &str) -> Envelope {
    envelope::error(
        err.code(),
        format!("{what}: {err}"),
        err.next_action(),
        err.retryable(),
    )
}

/// What the tarball step established, when it succeeded.
struct Extracted {
    facts: ManifestFacts,
    crate_root: super::source_tree::SourceTree,
    tarball: Artifact,
    manifest_artifact: Option<Artifact>,
}

/// Run the §4.4 local rustdoc build, or explain why it did not run.
///
/// `Ok(None)` means the caller asked but a prerequisite was missing and the gap already says so.
/// The three prerequisites are separate facts and each gets its own message: the `build` profile
/// must be enabled, the Rust image must be qualified by an actual containment run, and the crate
/// tarball must have been acquired.
async fn local_rustdoc(
    service: &Service,
    request: &ResolveRequest,
    extracted: Option<&Extracted>,
    acq: &mut Acquisition<'_>,
) -> Result<Option<crate::execution::rustdoc::LocalBuild>, Gap> {
    let gap = |reason: GapReason, detail: String| Gap {
        kind: EvidenceKind::HostedRustdocJson,
        reason,
        detail,
        planned_fallback: Some(PlannedFallback {
            producer: rustdoc::LOCAL_PRODUCER.to_owned(),
            profile: ExecutionProfile::Build.to_string(),
            enabled: ExecutionProfile::Build.is_enabled(&service.config),
            next_action: "Enable the `build` profile, run `just execution-images --apply` and \
                          `just execution-qualify --apply`, then resolve again with \
                          allow_local_build."
                .to_owned(),
        }),
    };
    if !ExecutionProfile::Build.is_enabled(&service.config) {
        return Err(gap(
            GapReason::PolicyDenied,
            "a local rustdoc build was requested, but the `build` profile is not enabled in this \
             service's configuration. A caller selects from enabled profiles and never grants \
             itself one"
                .to_owned(),
        ));
    }
    let qualification = crate::execution::admission::qualification(
        &service.config.execution,
        &service.paths.cache_root,
    );
    if !qualification.is_qualified() {
        return Err(gap(
            GapReason::PolicyDenied,
            format!(
                "a local rustdoc build was requested, but execution is not qualified: {}",
                qualification.detail()
            ),
        ));
    }
    let Some(image) = service
        .config
        .execution
        .rust_image
        .as_deref()
        .filter(|id| Runner::valid_image(id))
    else {
        return Err(gap(
            GapReason::PolicyDenied,
            "a local rustdoc build was requested, but no admitted Rust producer image is \
             configured"
                .to_owned(),
        ));
    };
    let Some(extracted) = extracted else {
        return Err(gap(
            GapReason::UpstreamUnavailable,
            "a local rustdoc build was requested, but the crate archive was not acquired, so \
             there is nothing to compile"
                .to_owned(),
        ));
    };
    let tarball = service
        .blobs
        .read(&extracted.tarball.sha256)
        .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?;

    if let crate::execution::cleanup::Admission::Quarantined { detail, .. } =
        service.execution.admission()
    {
        return Err(gap(GapReason::PolicyDenied, detail));
    }
    let work = acq.work.ok_or_else(|| {
        gap(
            GapReason::NotAttempted,
            "local build requires its durable acquisition job".into(),
        )
    })?;
    work.check()
        .map_err(|e| gap(GapReason::NotAttempted, e.to_string()))?;
    let lease = std::sync::Arc::clone(&work.lease);
    let runner = Runner::new(
        &service.config.execution,
        &service.paths.cache_root,
        std::sync::Arc::clone(&service.execution),
    )
    .map_err(|e| gap(GapReason::PolicyDenied, e.to_string()))?
    .using_lease(lease);
    let result = crate::execution::rustdoc::build(
        &runner,
        image,
        &service.paths.cache_root.join("capsules"),
        service.config.execution.capsule_budget_mib,
        &tarball,
        &extracted.facts,
        &environment_for(request),
        std::sync::Arc::clone(&work.cancel),
    )
    .await;
    service
        .execution
        .wait_for_cleanup(&work.lease)
        .await
        .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?;
    match result {
        Ok(built) => {
            acq.indexed.insert(EvidenceKind::PublicApi);
            Ok(Some(built))
        }
        Err(crate::execution::capsule::PreparationError::Policy(detail)) => {
            Err(gap(GapReason::PolicyDenied, detail))
        }
        Err(crate::execution::capsule::PreparationError::Cancelled) => Err(gap(
            GapReason::NotAttempted,
            "the local rustdoc build was cancelled".to_owned(),
        )),
        Err(crate::execution::capsule::PreparationError::Environment(detail)) => {
            Err(gap(GapReason::ExtractionFailed, detail))
        }
        Err(crate::execution::capsule::PreparationError::Process(observation, stage)) => Err(gap(
            GapReason::ExtractionFailed,
            format!(
                "{stage} did not succeed (end={:?}, exit={:?}): {}",
                observation.end,
                observation.exit_code,
                observation.stderr.chars().take(600).collect::<String>()
            ),
        )),
    }
}

pub(super) async fn acquire(
    service: &Service,
    request: &ResolveRequest,
    work: &super::resolve_job::Work,
) -> Envelope {
    let mut acq = Acquisition::new(service).for_job(work);
    let rust = &service.config.producers.rust;

    // 1. Registry index: the spellings crates.io treats as one namespace, in order.
    let started = clock::now_rfc3339();
    let mut index: Option<(String, Fetched, Vec<IndexEntry>)> = None;
    let variants = registry::name_variants(&request.name);
    for candidate in &variants {
        let url = match cratesio::index_url(&rust.crates_io_index_url, candidate) {
            Ok(url) => url,
            Err(err) => {
                return envelope::error(
                    ErrorCode::UpstreamUnavailable,
                    err.to_string(),
                    "Fix [producers.rust].crates_io_index_url in the service configuration.",
                    false,
                );
            }
        };
        let fetched = match service
            .fetcher
            .get_with_revalidation(
                &url,
                Some("text/plain"),
                request.freshness == FreshnessMode::Revalidate || request.version.is_none(),
            )
            .await
        {
            Ok(fetched) => fetched,
            Err(err) => return fetch_error(&err, "registry index"),
        };
        match fetched.status {
            200 => {
                let text = String::from_utf8_lossy(&fetched.bytes).into_owned();
                match registry::parse_index(&text) {
                    Ok(entries) => {
                        index = Some((url.to_string(), fetched, entries));
                        break;
                    }
                    Err(err) => {
                        return envelope::error(
                            ErrorCode::ExtractionFailed,
                            format!("registry index for `{candidate}` is malformed: {err}"),
                            "Retry with freshness=revalidate; if it persists the registry \
                             changed its index format.",
                            true,
                        );
                    }
                }
            }
            404 => continue,
            other => {
                return envelope::error(
                    ErrorCode::UpstreamUnavailable,
                    format!("registry index answered HTTP {other} for `{candidate}`"),
                    "Retry later.",
                    true,
                );
            }
        }
    }
    let Some((index_url, index_fetched, entries)) = index else {
        return envelope::error(
            ErrorCode::VersionNotFound,
            format!(
                "`{}` is not published on crates.io (checked {})",
                request.name,
                variants.join(" and ")
            ),
            "Check the crate name; crates.io treats `-` and `_` as the same character.",
            false,
        );
    };
    let registry_checked_at = index_fetched.retrieved_at.clone();

    // 2. Select the exact version -- never an upgrade -- and note the newest separately.
    let selected = match registry::select_version(
        &entries,
        request.version.as_deref(),
        request.allow_prerelease,
        request.allow_yanked,
    ) {
        Ok(entry) => entry.clone(),
        Err(err) => return selection_error(err),
    };
    let upstream = registry::upstream_check(&entries, &selected.vers);
    let index_artifact = match acq.store(
        &index_fetched,
        ArtifactKind::RegistryIndexEntry,
        "text/plain",
        &index_url,
    ) {
        Ok(a) => a,
        Err(err) => return store_error(&err),
    };
    let line_no = entries
        .iter()
        .position(|e| e.vers == selected.vers)
        .map_or(0, |i| i + 1);
    let excerpt = serde_json::to_string(&selected).expect("registry selection serializes");
    acq.evidence(
        &format!("{}@{}", selected.name, selected.vers),
        &index_artifact,
        serde_json::json!({ "kind": "index_line", "line": line_no }),
        cratesio::PRODUCER,
        cratesio::VERSION,
        &excerpt,
    );
    acq.indexed.insert(EvidenceKind::RegistryMetadata);

    let mut release = Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: registry::CRATES_IO.to_owned(),
        package: selected.name.clone(),
        version: selected.vers.clone(),
        artifact_digest: Some(selected.cksum.clone()),
    });
    release.rust_version = selected.rust_version.clone();
    release.published_at = selected.pubtime.clone();
    release.yanked = selected.yanked;
    if let Some(replay) =
        replay_selected(service, request, &release, Some(&upstream), &mut acq).await
    {
        return replay;
    }

    // 3. The API version record: links and licence the index does not carry. Non-fatal.
    if let Ok(url) = cratesio::version_url(&rust.crates_io_api_url, &selected.name, &selected.vers)
        && let Ok(fetched) = service
            .fetcher
            .get_with_revalidation(
                &url,
                Some("application/json"),
                request.freshness == FreshnessMode::Revalidate,
            )
            .await
        && fetched.status == 200
        && let Ok(doc) = registry::parse_version_document(&String::from_utf8_lossy(&fetched.bytes))
    {
        release.links = ReleaseLinks {
            repository: doc.version.repository.clone(),
            documentation: doc.version.documentation.clone(),
            homepage: doc.version.homepage.clone(),
        };
        release.license = doc.version.license.clone();
        if release.rust_version.is_none() {
            release.rust_version = doc.version.rust_version.clone();
        }
        if release.published_at.is_none() {
            release.published_at = doc.version.created_at.clone();
        }
        let _ = acq.store(
            &fetched,
            ArtifactKind::RegistryVersionMetadata,
            "application/json",
            url.as_str(),
        );
    }
    let mut registry_inputs = BTreeMap::new();
    registry_inputs.insert("index".to_owned(), index_artifact.sha256.clone());
    acq.run(
        cratesio::PRODUCER,
        cratesio::VERSION,
        registry_inputs,
        started,
        RunOutcome::Succeeded,
        Vec::new(),
    );

    // 4. The crate tarball, verified against the index checksum and extracted under policy.
    let started = clock::now_rfc3339();
    let mut extracted: Option<Extracted> = None;
    let mut tarball_gaps = Vec::new();
    let mut tarball_inputs = BTreeMap::new();
    match cratesio::download_url(&rust.crates_io_api_url, &selected.name, &selected.vers) {
        Ok(url) => match service
            .fetcher
            .get_with_revalidation(&url, None, request.freshness == FreshnessMode::Revalidate)
            .await
        {
            Ok(fetched) if fetched.status == 200 => {
                let digest = enrichment_core::canonical::sha256_hex(&fetched.bytes);
                if digest != selected.cksum {
                    tarball_gaps.push(Gap {
                        kind: EvidenceKind::CrateSource,
                        reason: GapReason::ExtractionFailed,
                        detail: format!(
                            "downloaded tarball digest {digest} does not match the registry \
                             checksum {}",
                            selected.cksum
                        ),
                        planned_fallback: None,
                    });
                } else {
                    match acq.store(
                        &fetched,
                        ArtifactKind::CrateTarball,
                        "application/gzip",
                        url.as_str(),
                    ) {
                        Ok(tarball) => {
                            tarball_inputs.insert("tarball".to_owned(), tarball.sha256.clone());
                            match extract_and_read_manifest(service, &tarball, fetched.bytes).await
                            {
                                Ok((facts, manifest_text, crate_root)) => {
                                    let manifest_artifact = acq
                                        .store_bytes(
                                            manifest_text.as_bytes(),
                                            ArtifactKind::CargoManifest,
                                            "text/x-toml",
                                            &format!("{url}#Cargo.toml"),
                                            None,
                                        )
                                        .ok();
                                    if let Some(manifest_artifact) = &manifest_artifact {
                                        let rendered = serde_json::to_string(&facts.docs_rs)
                                            .expect("documentation configuration serializes");
                                        acq.evidence(
                                            "documentation_build_config",
                                            manifest_artifact,
                                            serde_json::json!({
                                                "kind": "toml_table", "path": "Cargo.toml",
                                                "table": "package.metadata.docs.rs"
                                            }),
                                            cratesio::TARBALL_PRODUCER,
                                            cratesio::VERSION,
                                            &rendered,
                                        );
                                    }
                                    release.lib_name = facts.lib_name.clone();
                                    release.root_module = facts.lib_name.clone();
                                    acq.indexed.insert(EvidenceKind::CrateSource);
                                    acq.indexed.insert(EvidenceKind::DocumentationBuildConfig);
                                    extracted = Some(Extracted {
                                        facts,
                                        crate_root,
                                        tarball,
                                        manifest_artifact,
                                    });
                                }
                                Err(detail) => tarball_gaps.push(Gap {
                                    kind: EvidenceKind::CrateSource,
                                    reason: GapReason::ExtractionFailed,
                                    detail,
                                    planned_fallback: None,
                                }),
                            }
                        }
                        Err(err) => return store_error(&err),
                    }
                }
            }
            Ok(fetched) => tarball_gaps.push(Gap {
                kind: EvidenceKind::CrateSource,
                reason: GapReason::UpstreamUnavailable,
                detail: format!("tarball download answered HTTP {}", fetched.status),
                planned_fallback: None,
            }),
            Err(err) => tarball_gaps.push(Gap {
                kind: EvidenceKind::CrateSource,
                reason: gap_reason_for(&err),
                detail: err.to_string(),
                planned_fallback: None,
            }),
        },
        Err(err) => tarball_gaps.push(Gap {
            kind: EvidenceKind::CrateSource,
            reason: GapReason::UpstreamUnavailable,
            detail: err.to_string(),
            planned_fallback: None,
        }),
    }
    if !tarball_gaps.is_empty() {
        tarball_gaps.push(Gap {
            kind: EvidenceKind::DocumentationBuildConfig,
            reason: GapReason::NotAttempted,
            detail: "the crate manifest was not read because the tarball was not extracted"
                .to_owned(),
            planned_fallback: None,
        });
    }
    let tarball_outcome = if tarball_gaps.is_empty() {
        RunOutcome::Succeeded
    } else {
        RunOutcome::Failed
    };
    acq.run(
        cratesio::TARBALL_PRODUCER,
        cratesio::VERSION,
        tarball_inputs,
        started,
        tarball_outcome,
        tarball_gaps,
    );

    // 5. Hosted rustdoc JSON, for the default docs.rs target of this release.
    let started = clock::now_rfc3339();
    let observed_target = extracted
        .as_ref()
        .map(|e| e.facts.docs_rs.default_target.clone())
        .unwrap_or_else(|| docsrs::DEFAULT_TARGET.to_owned());
    let mut json_inputs = BTreeMap::new();
    let mut json_gaps = Vec::new();
    let mut declared_crate_version = None;
    let json_url = docsrs::json_url(&rust.docs_rs_url, &selected.name, &selected.vers, None)
        .map(|u| u.to_string())
        .unwrap_or_default();
    let hosted_state = match docsrs::json_url(
        &rust.docs_rs_url,
        &selected.name,
        &selected.vers,
        None,
    ) {
        Err(err) => {
            json_gaps.push(Gap {
                kind: EvidenceKind::HostedRustdocJson,
                reason: GapReason::UpstreamUnavailable,
                detail: err.to_string(),
                planned_fallback: Some(acq.fallback_for_json()),
            });
            (HostedJsonState::NotAttempted, None)
        }
        Ok(url) => match service
            .fetcher
            .get_with_revalidation(&url, None, request.freshness == FreshnessMode::Revalidate)
            .await
        {
            Err(err) => {
                json_gaps.push(Gap {
                    kind: EvidenceKind::HostedRustdocJson,
                    reason: gap_reason_for(&err),
                    detail: err.to_string(),
                    planned_fallback: Some(acq.fallback_for_json()),
                });
                (HostedJsonState::NotAttempted, None)
            }
            Ok(fetched) => {
                let classified = docsrs::classify_response(
                    fetched.status,
                    fetched.content_type.as_deref(),
                    &fetched.bytes,
                    service.config.network.max_decompressed_bytes,
                );
                match classified {
                    Ok(HostedJson::Available {
                        payload,
                        format_version,
                    }) => {
                        declared_crate_version = peek_crate_version(&payload);
                        let compression = fetched
                            .content_type
                            .as_deref()
                            .filter(|c| c.contains("zstd") || c.contains("gzip"))
                            .map(|c| if c.contains("zstd") { "zstd" } else { "gzip" });
                        match acq.store_bytes(
                            payload.as_bytes(),
                            ArtifactKind::RustdocJson,
                            "application/json",
                            url.as_str(),
                            compression,
                        ) {
                            Ok(artifact) => {
                                if declared_crate_version
                                    .as_ref()
                                    .is_some_and(|v| v != &selected.vers)
                                {
                                    json_gaps.push(Gap {
                                            kind: EvidenceKind::PublicApi,
                                            reason: GapReason::ExtractionFailed,
                                            detail: format!("hosted JSON declares version {:?}, requested {}; raw evidence retained but not admitted as this release's API", declared_crate_version, selected.vers),
                                            planned_fallback: Some(acq.fallback_for_json()),
                                        });
                                } else {
                                    json_inputs
                                        .insert("rustdoc_json".to_owned(), artifact.sha256.clone());
                                    acq.indexed.insert(EvidenceKind::HostedRustdocJson);
                                }
                                (HostedJsonState::Available, Some(format_version))
                            }
                            Err(err) => return store_error(&err),
                        }
                    }
                    Ok(HostedJson::Unsupported {
                        format_version,
                        supported,
                    }) => {
                        json_gaps.push(Gap {
                            kind: EvidenceKind::HostedRustdocJson,
                            reason: GapReason::HostedJsonUnsupported,
                            detail: format!(
                                "docs.rs serves rustdoc JSON format {format_version}; this build \
                                 reads {supported}"
                            ),
                            planned_fallback: Some(acq.fallback_for_json()),
                        });
                        (HostedJsonState::Unsupported, Some(format_version))
                    }
                    Ok(HostedJson::Missing) => {
                        json_gaps.push(Gap {
                            kind: EvidenceKind::HostedRustdocJson,
                            reason: GapReason::HostedJsonMissing,
                            detail: format!(
                                "docs.rs has no rustdoc JSON for {} {} on {observed_target} \
                                 (HTTP 404): the release predates JSON generation, the target \
                                 was not built, or the build failed",
                                selected.name, selected.vers
                            ),
                            planned_fallback: Some(acq.fallback_for_json()),
                        });
                        (HostedJsonState::Missing, None)
                    }
                    Err(err) => {
                        json_gaps.push(Gap {
                            kind: EvidenceKind::HostedRustdocJson,
                            reason: GapReason::ExtractionFailed,
                            detail: err.to_string(),
                            planned_fallback: Some(acq.fallback_for_json()),
                        });
                        (HostedJsonState::NotAttempted, None)
                    }
                }
            }
        },
    };
    let json_outcome = if json_gaps.is_empty() {
        RunOutcome::Succeeded
    } else {
        RunOutcome::Failed
    };
    acq.run(
        docsrs::PRODUCER,
        docsrs::VERSION,
        json_inputs.clone(),
        started,
        json_outcome,
        json_gaps,
    );

    // 5b. The §4.4 fallback. Hosted JSON came first and could not be used; the caller asked for
    // a local build, and policy allows one. A dated nightly compiles the crate in a capsule.
    let mut local_build: Option<crate::execution::rustdoc::LocalBuild> = None;
    // Kept apart from `hosted_state` on purpose: that tuple is the hosted report, and a
    // document docs.rs never served has no business setting a field in it.
    let mut local_format_version: Option<u32> = None;
    let needs_requested_build = extracted
        .as_ref()
        .is_some_and(|source| hosted_requires_requested_build(request, &source.facts.docs_rs));
    if request.allow_local_build
        && (!json_inputs.contains_key("rustdoc_json") || needs_requested_build)
    {
        match local_rustdoc(service, request, extracted.as_ref(), &mut acq).await {
            Ok(Some(built)) => {
                match acq.store_bytes(
                    &built.payload,
                    ArtifactKind::RustdocJson,
                    "application/json",
                    &format!(
                        "capsule://{}/{}/rustdoc?toolchain={}",
                        release.key.package,
                        release.key.version,
                        crate::execution::rustdoc::TOOLCHAIN
                    ),
                    None,
                ) {
                    Ok(artifact) => {
                        json_inputs.insert("rustdoc_json".to_owned(), artifact.sha256.clone());
                        local_format_version = Some(built.format_version);
                        let lock = match acq.store_bytes(
                            &built.lock,
                            ArtifactKind::Other,
                            "text/toml",
                            &format!("{}#Cargo.lock", artifact.source_uri),
                            None,
                        ) {
                            Ok(value) => value,
                            Err(e) => return store_error(&e),
                        };
                        let provenance = match serde_json::to_vec(
                            &serde_json::json!({"environment":built.environment,"rustc":built.rustc_identity,"producer": "local-rustdoc/2", "format_version":built.format_version,"image_id":built.image_id,"containment_identity":built.containment_identity}),
                        ) {
                            Ok(value) => value,
                            Err(e) => return super::common::store_error(&e),
                        };
                        let configuration = match acq.store_bytes(
                            &provenance,
                            ArtifactKind::Other,
                            "application/json",
                            &format!("{}#build-configuration", artifact.source_uri),
                            None,
                        ) {
                            Ok(value) => value,
                            Err(e) => return store_error(&e),
                        };
                        let attempt_id = uuid::Uuid::new_v4().to_string();
                        let receipt_bytes = match serde_json::to_vec(&built.observations) {
                            Ok(value) => value,
                            Err(e) => return super::common::store_error(&e),
                        };
                        let receipt = match acq.store_bytes(
                            &receipt_bytes,
                            ArtifactKind::Other,
                            "application/json",
                            &format!("producer-attempt://{attempt_id}/local-rustdoc"),
                            None,
                        ) {
                            Ok(value) => value,
                            Err(e) => return store_error(&e),
                        };
                        acq.receipt_ids.insert(receipt.artifact_id.clone());
                        acq.runs.push(ProducerRun {
                            attempt_id,
                            producer: rustdoc::LOCAL_PRODUCER.into(),
                            producer_version: "local-rustdoc/2".into(),
                            config_digest: configuration.sha256.clone(),
                            inputs: BTreeMap::from([
                                ("rustdoc_json".into(), artifact.sha256.clone()),
                                ("cargo_lock".into(), lock.sha256),
                                ("configuration".into(), configuration.sha256),
                                ("crate_tarball".into(), selected.cksum.clone()),
                            ]),
                            profile: ExecutionProfile::Build,
                            started_at: built.started_at.clone(),
                            finished_at: built.finished_at.clone(),
                            outcome: RunOutcome::Succeeded,
                            gaps: Vec::new(),
                            log: Some(receipt.artifact_id),
                        });
                        local_build = Some(built);
                    }
                    Err(err) => acq.gaps.push(Gap {
                        kind: EvidenceKind::HostedRustdocJson,
                        reason: GapReason::ExtractionFailed,
                        detail: format!("the locally built JSON could not be stored: {err}"),
                        planned_fallback: None,
                    }),
                }
            }
            Ok(None) => {}
            Err(gap) => acq.gaps.push(gap),
        }
    }

    // 6. Identity, recorded before anything is published against it.
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    let source_version_match = match &declared_crate_version {
        Some(v) if v == &selected.vers => SourceVersionMatch::Exact,
        Some(_) => SourceVersionMatch::Mismatched,
        None => SourceVersionMatch::Unknown,
    };
    let observed_configuration: Option<DocsRsMetadata> =
        extracted.as_ref().map(|e| e.facts.docs_rs.clone());

    let data = ResolveData {
        release: release.clone(),
        environment: environment.clone(),
        context: context.clone(),
        upstream: Some(upstream.clone()),
        observed_configuration,
        hosted_rustdoc_json: Some(HostedJsonReport {
            state: hosted_state.0,
            format_version: hosted_state.1,
            supported_formats: rustdoc::SUPPORTED_FORMAT_VERSIONS.to_vec(),
            target: observed_target.clone(),
            url: json_url,
            declared_crate_version,
        }),
        python: None,
        snapshot: None,
        artifacts: acq.artifacts.clone(),
        gaps: acq.gaps.clone(),
        producer_runs: acq.runs.clone(),
        answered_from_cache: false,
    };

    let partial = !acq.gaps.is_empty() || hosted_state.0 != HostedJsonState::Available;
    let mut limitations = vec![
        "Hosted documentation reflects the maintainer's docs.rs build configuration \
         (observed_configuration), not the calling project's features or target."
            .to_owned(),
    ];
    if environment.resolution == enrichment_core::identity::EnvironmentResolution::Unspecified {
        limitations.push(
            "No project environment was declared; availability claims are about the documented \
             build only."
                .to_owned(),
        );
    }
    if let Some(built) = &local_build {
        // The load-bearing sentence of §4.4. A nightly build establishing the API surface says
        // nothing about whether the crate compiles on the project's stable compiler, and this
        // is the only place a caller is told so.
        limitations.push(format!(
            "This API was compiled locally by {} ({}), not downloaded from docs.rs. A successful \
             nightly build is not evidence that this crate compiles on the project's stable \
             compiler; use verify_usage for that.",
            crate::execution::rustdoc::TOOLCHAIN,
            built.rustc_identity.trim().replace('\n', "; ")
        ));
    }
    let summary = summarize(&release, &upstream, hosted_state.0, None);
    let coverage = Coverage {
        scope: format!(
            "release identity, registry metadata, crate source, hosted documentation and the \
             normalized public API of {} {}",
            release.key.package, release.key.version
        ),
        indexed: acq.indexed.iter().map(|k| k.as_str().to_owned()).collect(),
        missing: acq
            .gaps
            .iter()
            .map(|g| g.kind.as_str().to_owned())
            .collect(),
        limitations,
    };
    let freshness = Freshness {
        registry_checked_at: Some(registry_checked_at),
        source_version_match,
        latest_verified: true,
    };
    let handles: Vec<ArtifactHandle> = acq
        .artifacts
        .iter()
        .filter_map(|a| {
            envelope::artifact_uri(&format!("artifacts/{}", a.artifact_id))
                .ok()
                .map(|uri| ArtifactHandle {
                    artifact_id: a.artifact_id.clone(),
                    uri,
                    media_type: a.media_type.clone(),
                    description: format!(
                        "{:?} for {} {}",
                        a.kind, release.key.package, release.key.version
                    ),
                })
        })
        .collect();

    let research = Research {
        summary,
        data: to_object(&data),
        coverage,
        freshness,
        context_id: Some(context.context_id.to_string()),
        snapshot_id: None,
        evidence: acq.evidence.clone(),
        artifacts: handles,
    };
    acq.delivery_template = Some(if partial {
        research.partial()
    } else {
        research.ok()
    });

    // 7. Normalize the hosted JSON and publish an immutable snapshot (§8.2). Only the daemon
    // publishes, and only after every table has been re-read and counted.
    let started = clock::now_rfc3339();
    let mut snapshot_summary: Option<SnapshotSummary> = None;
    match json_inputs.get("rustdoc_json").cloned() {
        Some(json_sha) => {
            let publish = NormalizeRequest {
                json_sha: Some(&json_sha),
                context: &context,
                release: &release,
                environment: &environment,
                extracted: extracted.as_ref(),
                format_version: local_format_version.or(hosted_state.1).unwrap_or_default(),
                local_build: local_build.as_ref(),
            };
            match normalize_and_publish(service, &mut acq, &publish).await {
                Ok(summary) => {
                    snapshot_summary = Some(summary);
                }
                Err(gap) => {
                    let mut inputs = BTreeMap::new();
                    inputs.insert("rustdoc_json".to_owned(), json_sha);
                    acq.run(
                        rustdoc::PRODUCER,
                        rustdoc::NORMALIZER_VERSION,
                        inputs,
                        started,
                        RunOutcome::Failed,
                        vec![gap],
                    );
                }
            }
        }
        None => acq.gaps.push(Gap {
            kind: EvidenceKind::PublicApi,
            reason: GapReason::NotAttempted,
            detail: "no rustdoc JSON this build can read was available to normalize".to_owned(),
            planned_fallback: Some(acq.fallback_for_json()),
        }),
    }

    if snapshot_summary.is_none() {
        let source_snapshot = NormalizeRequest {
            json_sha: None,
            context: &context,
            release: &release,
            environment: &environment,
            extracted: extracted.as_ref(),
            format_version: 0,
            local_build: None,
        };
        match normalize_and_publish(service, &mut acq, &source_snapshot).await {
            Ok(_) => {}
            Err(gap) => acq.gaps.push(gap),
        }
    }

    match work.committed.get() {
        Some((_, _, result)) => result.clone(),
        None => envelope::error(
            enrichment_core::wire::ErrorCode::ExtractionFailed,
            format!("No complete snapshot was published: {:?}", acq.gaps),
            "Inspect the acquisition job and its recorded producer limitations before retrying.",
            false,
        ),
    }
}

fn gap_reason_for(err: &FetchError) -> GapReason {
    if matches!(
        err,
        FetchError::Policy(_) | FetchError::RedirectRefused { .. }
    ) {
        GapReason::PolicyDenied
    } else {
        GapReason::UpstreamUnavailable
    }
}

/// Hosted configuration is author evidence, not a proof of the requested project build.
/// When explicit requested settings cannot be established from that configuration, an
/// opted-in local build supplies the requested observation. The hosted artifact stays retained.
fn hosted_requires_requested_build(
    request: &ResolveRequest,
    docs: &docsrs::DocsRsMetadata,
) -> bool {
    request
        .target
        .as_ref()
        .is_some_and(|target| *target != docs.default_target)
        || request
            .default_features
            .is_some_and(|defaults| defaults == docs.no_default_features)
        || request.features.as_ref().is_some_and(|features| {
            docs.all_features
                || features.iter().collect::<BTreeSet<_>>()
                    != docs.features.iter().collect::<BTreeSet<_>>()
                || !docs.rustc_args.is_empty()
                || !docs.rustdoc_args.is_empty()
                || !docs.cargo_args.is_empty()
        })
}

/// What normalization needs from the acquisition so far.
struct NormalizeRequest<'a> {
    json_sha: Option<&'a str>,
    context: &'a Context,
    release: &'a Release,
    environment: &'a Environment,
    extracted: Option<&'a Extracted>,
    format_version: u32,
    /// Present when this JSON was compiled here rather than downloaded from docs.rs.
    ///
    /// The two must stay distinguishable in the published manifest: one records docs.rs' build,
    /// the other a dated nightly of ours, and "it compiled on nightly" is never "it compiles on
    /// the project's stable compiler" (§4.4).
    local_build: Option<&'a crate::execution::rustdoc::LocalBuild>,
}

/// Normalize the stored rustdoc JSON, read the crate's declared documents, and publish.
///
/// Returns the published snapshot summary, or a qualified public-API failure.
async fn normalize_and_publish(
    service: &Service,
    acq: &mut Acquisition<'_>,
    request: &NormalizeRequest<'_>,
) -> Result<SnapshotSummary, Gap> {
    let gap = |reason: GapReason, detail: String| Gap {
        kind: EvidenceKind::PublicApi,
        reason,
        detail,
        planned_fallback: None,
    };
    let normalization_started = clock::now_rfc3339();
    let normalized = if let Some(json_sha) = request.json_sha {
        let artifact = acq
            .artifacts
            .iter()
            .find(|a| a.sha256 == json_sha)
            .cloned()
            .ok_or_else(|| {
                gap(
                    GapReason::ExtractionFailed,
                    "rustdoc acquisition descriptor missing".into(),
                )
            })?;
        enrichment_store::native_rustdoc::from_artifact(
            service.blobs.clone(),
            artifact,
            service.config.limits.excerpt_characters,
        )
        .await
        .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?
    } else {
        enrichment_store::native_rustdoc::Prepared::source_only(
            request
                .extracted
                .and_then(|e| e.facts.lib_name.clone())
                .unwrap_or_else(|| request.release.key.package.replace('-', "_")),
            request
                .extracted
                .and_then(|e| e.facts.package_version.clone()),
        )
    };

    let mut documents = super::source_documents::SourceDocuments::new(service.blobs.clone());
    let mut indexed = vec![EvidenceKind::RegistryMetadata];
    if request.json_sha.is_some() {
        indexed.extend([EvidenceKind::PublicApi, EvidenceKind::Documentation]);
        if request.local_build.is_none() {
            indexed.push(EvidenceKind::HostedRustdocJson);
        }
    }
    let mut inputs = BTreeMap::new();
    if let Some(json_sha) = request.json_sha {
        inputs.insert("rustdoc_json".to_owned(), json_sha.to_owned());
    }
    for artifact in acq
        .artifacts
        .iter()
        .filter(|a| !acq.receipt_ids.contains(&a.artifact_id))
    {
        inputs.insert(artifact.artifact_id.clone(), artifact.sha256.clone());
    }
    let mut producers = BTreeMap::new();
    if request.json_sha.is_some() {
        producers.insert(
            rustdoc::PRODUCER.to_owned(),
            rustdoc::NORMALIZER_VERSION.to_owned(),
        );
        producers.insert(
            "public-api".to_owned(),
            normalize::public_api_version().to_owned(),
        );
        match request.local_build {
            Some(built) => {
                producers.insert(
                    rustdoc::LOCAL_PRODUCER.to_owned(),
                    crate::execution::rustdoc::TOOLCHAIN.to_owned(),
                );
                producers.insert("rustc".to_owned(), built.rustc_identity.trim().to_owned());
            }
            None => {
                producers.insert(docsrs::PRODUCER.to_owned(), docsrs::VERSION.to_owned());
            }
        }
    } else {
        producers.insert(
            "rust-source-snapshot".to_owned(),
            rustdoc::NORMALIZER_VERSION.to_owned(),
        );
    }

    let mut observed = ObservedConfiguration {
        features: Vec::new(),
        all_features: false,
        no_default_features: false,
        target: normalized.target.clone(),
        format_version: request.format_version,
        source: match request.local_build {
            Some(_) => format!(
                "locally built on {} inside a service capsule, then read from the rustdoc JSON \
                 header (target, format_version)",
                crate::execution::rustdoc::TOOLCHAIN
            ),
            None => "rustdoc JSON header (target, format_version)".to_owned(),
        },
    };

    if let Some(extracted) = request.extracted {
        indexed.push(EvidenceKind::CrateSource);
        indexed.push(EvidenceKind::DocumentationBuildConfig);
        inputs.insert("crate_tarball".to_owned(), extracted.tarball.sha256.clone());
        producers.insert(source::PRODUCER.to_owned(), source::VERSION.to_owned());
        if let Some(built) = request.local_build {
            observed.features = built.environment.features.clone();
            observed.all_features = false;
            observed.no_default_features = built.environment.default_features == Some(false);
        } else {
            observed.features = extracted.facts.docs_rs.features.clone();
            observed.all_features = extracted.facts.docs_rs.all_features;
            observed.no_default_features = extracted.facts.docs_rs.no_default_features;
        }
        if request.local_build.is_none() {
            observed.source =
                "Cargo.toml [package.metadata.docs.rs] plus the rustdoc JSON header".to_owned();
        }

        // Keep qualified descriptors; publication captures and visits one document at a time.
        let source_root = extracted.crate_root.to_path_buf();
        let source_files = tokio::task::spawn_blocking(move || source::text_files(&source_root))
            .await
            .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?
            .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?;
        for (file, kind) in source_files {
            let artifact_kind = match kind {
                FragmentKind::ReadmeSection => ArtifactKind::Readme,
                FragmentKind::ChangelogSection => ArtifactKind::Changelog,
                _ => ArtifactKind::SourceFile,
            };
            let media = if file.ends_with(".md") {
                "text/markdown"
            } else {
                "text/x-rust"
            };
            let source_uri = format!("{}#{file}", extracted.tarball.source_uri);
            let Some(artifact) = acq
                .store_source_file(
                    extracted.crate_root.join(&file),
                    artifact_kind,
                    media,
                    source_uri,
                )
                .await
                .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?
            else {
                continue;
            };
            documents
                .file(file, kind, artifact, true)
                .map_err(|e| gap(GapReason::ExtractionFailed, e))?;
        }
        let manifest = extracted.manifest_artifact.clone().ok_or_else(|| {
            gap(
                GapReason::ExtractionFailed,
                "source manifest descriptor missing".into(),
            )
        })?;
        documents
            .features(extracted.facts.clone(), manifest)
            .map_err(|e| gap(GapReason::ExtractionFailed, e))?;
        let kinds = documents.kinds();
        if kinds.contains(&FragmentKind::Example) {
            indexed.push(EvidenceKind::Examples);
        }
        if kinds.contains(&FragmentKind::ChangelogSection) {
            indexed.push(EvidenceKind::ReleaseNotes);
        }
    }

    let missing: Vec<EvidenceKind> = [
        EvidenceKind::PublicApi,
        EvidenceKind::HostedRustdocJson,
        EvidenceKind::CrateSource,
        EvidenceKind::DocumentationBuildConfig,
        EvidenceKind::Examples,
        EvidenceKind::ReleaseNotes,
    ]
    .into_iter()
    .filter(|k| !indexed.contains(k))
    .collect();

    acq.indexed.extend(indexed.iter().cloned());

    inputs.extend(acq.semantic_inputs());
    acq.run(
        if request.json_sha.is_some() {
            rustdoc::PRODUCER
        } else {
            "rust-source-snapshot"
        },
        rustdoc::NORMALIZER_VERSION,
        inputs,
        normalization_started,
        RunOutcome::Succeeded,
        Vec::new(),
    );
    let details = request.extracted.and_then(|e| {
        e.manifest_artifact
            .as_ref()
            .map(|artifact| super::publication::MetadataInput {
                details: enrichment_core::evidence::metadata::ReleaseDetails::RustDocs(
                    e.facts.docs_rs.clone(),
                ),
                artifact: artifact.clone(),
                locator: enrichment_core::evidence::relational::Locator::Artifact,
            })
    });
    let published = super::publication::publish(
        service,
        acq,
        enrichment_core::evidence::snapshot::SnapshotMetadata {
            context: request.context.clone(),
            release: request.release.clone(),
            environment: request.environment.clone(),
            symbol_package: normalized.crate_name.clone(),
            crate_name: normalized.crate_name.clone(),
            crate_version: normalized.crate_version.clone(),
            normalizer_version: rustdoc::NORMALIZER_VERSION.into(),
            observed_configuration: request.json_sha.map(|_| observed),
            producer_items: normalized.producer_items,
        },
        (normalized, documents),
        producers,
        indexed,
        missing,
        details,
    )
    .await
    .map_err(|e| {
        gap(
            GapReason::ExtractionFailed,
            format!("snapshot publication failed: {e}"),
        )
    })?;
    Ok(SnapshotSummary {
        snapshot_id: published.snapshot_id.to_string(),
        normalizer_version: published.normalizer_version.clone(),
        counts: published.counts,
        published_at: published.published_at,
    })
}

fn summarize(
    release: &Release,
    upstream: &UpstreamCheck,
    hosted: HostedJsonState,
    snapshot: Option<&SnapshotSummary>,
) -> String {
    let newer = match (&upstream.newest_stable, upstream.resolved_is_newest_stable) {
        (Some(newest), false) => format!("; the newest stable release is {newest}"),
        _ => String::new(),
    };
    let json = match (hosted, snapshot) {
        (HostedJsonState::Available, Some(s)) => format!(
            "snapshot {} published with {} definitions",
            s.snapshot_id, s.counts.definitions
        ),
        (HostedJsonState::Available, None) => {
            "hosted rustdoc JSON was stored but could not be normalized".to_owned()
        }
        (HostedJsonState::Missing, _) => "docs.rs has no rustdoc JSON for this release".to_owned(),
        (HostedJsonState::Unsupported, _) => {
            "docs.rs rustdoc JSON is in a format this build cannot read".to_owned()
        }
        (HostedJsonState::NotAttempted, _) => "hosted rustdoc JSON could not be checked".to_owned(),
    };
    format!(
        "Resolved {} {} on crates.io{newer}; {json}.",
        release.key.package, release.key.version
    )
}

fn selection_error(err: SelectionError) -> Envelope {
    let next = match &err {
        SelectionError::VersionNotFound { nearest, .. } if !nearest.is_empty() => {
            format!(
                "Published versions near the one requested: {}.",
                nearest.join(", ")
            )
        }
        SelectionError::Yanked { .. } => {
            "Pass allow_yanked=true to research a yanked release deliberately.".to_owned()
        }
        SelectionError::NoEligibleVersion => {
            "Pass allow_prerelease=true if a prerelease is acceptable.".to_owned()
        }
        _ => "Check the crate name and version on crates.io.".to_owned(),
    };
    envelope::error(ErrorCode::VersionNotFound, err.to_string(), next, false)
}

fn store_error(err: &std::io::Error) -> Envelope {
    envelope::error(
        ErrorCode::ArtifactUnavailable,
        format!("service state could not be written: {err}"),
        "Check that the service data directory is writable; see `library-enrichmentd status`.",
        true,
    )
}

/// Extract the tarball under the cache root and read its manifest.
///
/// Returns the manifest facts, the manifest text, and the crate root (the archive's single
/// top-level directory).
async fn extract_and_read_manifest(
    service: &Service,
    artifact: &Artifact,
    bytes: Vec<u8>,
) -> Result<(ManifestFacts, String, super::source_tree::SourceTree), String> {
    let root = service.paths.unpacked();
    let digest = artifact.sha256.clone();
    tokio::task::spawn_blocking(move || {
        let crate_root =
            super::source_tree::open(&root, &digest, &bytes[..]).map_err(|e| e.to_string())?;
        let manifest_path = crate_root.join("Cargo.toml");
        let text = String::from_utf8(
            source::read_file(&manifest_path)
                .map_err(|e| format!("cannot read {}: {e}", manifest_path.display()))?,
        )
        .map_err(|e| e.to_string())?;
        let facts =
            docsrs::manifest_facts(&text).map_err(|e| format!("Cargo.toml is not valid: {e}"))?;
        Ok((facts, text, crate_root))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[derive(Deserialize)]
struct CrateVersionPeek {
    crate_version: Option<String>,
}

fn peek_crate_version(payload: &str) -> Option<String> {
    serde_json::from_str::<CrateVersionPeek>(payload)
        .ok()
        .and_then(|p| p.crate_version)
}

#[cfg(test)]
mod provenance_tests {
    use super::*;
    fn python_digest(config: Config) -> String {
        let dir = tempfile::tempdir().expect("dir");
        let service = Service::open(
            config,
            enrichment_store::StatePaths::explicit(
                dir.path().join("cache"),
                dir.path().join("data"),
            ),
        )
        .expect("service");
        let mut acquisition = Acquisition::new(&service);
        acquisition.run(
            "python-static",
            "1",
            BTreeMap::new(),
            clock::now_rfc3339(),
            RunOutcome::Succeeded,
            Vec::new(),
        );
        acquisition.runs[0].config_digest.clone()
    }
    #[test]
    fn python_producer_identity_includes_registry_and_worker_configuration() {
        let config = Config::default();
        let baseline = python_digest(config.clone());
        let mut changed = config.clone();
        changed.producers.python.pypi_url = "https://example.com/pypi".into();
        assert_ne!(baseline, python_digest(changed));
        let mut changed = config.clone();
        changed.producers.python.worker_python = "/service/python".into();
        assert_ne!(baseline, python_digest(changed));
        let mut changed = config;
        changed.producers.python.worker_timeout_seconds += 1;
        assert_ne!(baseline, python_digest(changed));
    }
}
