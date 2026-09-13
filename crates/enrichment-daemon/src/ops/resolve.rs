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
use std::path::{Path, PathBuf};

use enrichment_core::SCHEMA_VERSION;
use enrichment_core::archive::{self, ArchiveError};
use enrichment_core::clock;
use enrichment_core::config::Config;
use enrichment_core::evidence::{
    Artifact, ArtifactKind, EvidenceKind, FragmentKind, Gap, GapReason, ObservedConfiguration,
    PlannedFallback, SnapshotCounts, SnapshotManifest, artifact_id_for,
};
use enrichment_core::identity::{
    Context, Ecosystem, Environment, Release, ReleaseKey, ReleaseLinks, SnapshotId, SnapshotInputs,
};
use enrichment_core::policy::{ArchivePolicy, ExecutionProfile};
use enrichment_core::producer::docsrs::{self, DocsRsMetadata, HostedJson, ManifestFacts};
use enrichment_core::producer::normalize::{self, NormalizeInput};
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
use enrichment_store::SnapshotTables;
use serde::{Deserialize, Serialize};

use crate::envelope::{self, Research};
use crate::fetch::{FetchError, Fetched};
use crate::service::Service;

/// The catalog document name the last resolution is recorded under.
pub const RESOLUTION_DOCUMENT: &str = "resolution";

/// Everything needed to replay a resolution without the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredResolution {
    data: ResolveData,
    coverage: Coverage,
    freshness: Freshness,
    evidence: Vec<Evidence>,
    artifacts: Vec<ArtifactHandle>,
    summary: String,
    partial: bool,
}

/// Resolve a request to a context, fetching what freshness allows.
pub async fn resolve(service: &Service, request: ResolveRequest) -> Envelope {
    if let Err(message) = request.validate() {
        return envelope::error(
            ErrorCode::VersionNotFound,
            message,
            "Pass a crate name and, for project research, an exact version such as 1.0.219.",
            false,
        );
    }
    if request.ecosystem != Ecosystem::Rust {
        return envelope::error(
            ErrorCode::UnsupportedCapability,
            "Python resolution is not implemented in this build",
            "Python lands in phase 2; call service_status to see which producers are installed.",
            false,
        );
    }

    // A recorded resolution answers cache_ok within its TTL and offline always.
    if request.freshness != FreshnessMode::Revalidate
        && let Some(version) = &request.version
        && let Some(replay) = replay_recorded(service, &request, version)
    {
        return replay;
    }
    if request.freshness == FreshnessMode::Offline {
        return envelope::error(
            ErrorCode::ArtifactUnavailable,
            format!(
                "no recorded resolution for {} {} and freshness=offline forbids the network",
                request.name,
                request.version.as_deref().unwrap_or("(latest)")
            ),
            "Resolve this release once with freshness=cache_ok or revalidate, then offline \
             replays work from the recorded snapshot.",
            false,
        );
    }

    let deadline =
        std::time::Duration::from_secs(service.config.network.acquisition_timeout_seconds);
    match tokio::time::timeout(deadline, acquire(service, &request)).await {
        Ok(envelope) => envelope,
        Err(_) => envelope::error(
            ErrorCode::UpstreamUnavailable,
            format!(
                "acquisition of {} did not finish within {}s",
                request.name, service.config.network.acquisition_timeout_seconds
            ),
            "Retry; partial artifacts are reused by digest, so a second attempt is cheaper. \
             Raise [network].acquisition_timeout_seconds if the crate is genuinely large.",
            true,
        ),
    }
}

fn replay_recorded(service: &Service, request: &ResolveRequest, version: &str) -> Option<Envelope> {
    let release = service
        .catalog
        .find_release(Ecosystem::Rust, &request.name, version)
        .ok()
        .flatten()?;
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    let stored: StoredResolution = service
        .catalog
        .document(&context.context_id, RESOLUTION_DOCUMENT)
        .ok()
        .flatten()?;

    if request.freshness == FreshnessMode::CacheOk {
        let recorded_at = stored
            .freshness
            .registry_checked_at
            .as_deref()
            .and_then(clock::parse_rfc3339)
            .unwrap_or(0);
        let age = clock::now_secs().saturating_sub(recorded_at);
        if age > service.config.freshness.registry_ttl_seconds {
            return None;
        }
    }

    let mut data = stored.data;
    data.answered_from_cache = true;
    let mut coverage = stored.coverage;
    coverage.limitations.push(format!(
        "Answered from the resolution recorded at {}; the registry was not consulted for this \
         call. Pass freshness=revalidate to re-check it.",
        stored
            .freshness
            .registry_checked_at
            .as_deref()
            .unwrap_or("an unknown time")
    ));
    let freshness = Freshness {
        latest_verified: false,
        ..stored.freshness
    };
    let research = Research {
        summary: format!("{} (replayed from cache)", stored.summary),
        data: to_object(&data),
        coverage,
        freshness,
        context_id: Some(context.context_id.to_string()),
        snapshot_id: service
            .catalog
            .current_snapshot(&context.context_id)
            .ok()
            .flatten()
            .map(|s| s.to_string()),
        evidence: stored.evidence,
        artifacts: stored.artifacts,
    };
    Some(if stored.partial {
        research.partial()
    } else {
        research.ok()
    })
}

fn environment_for(request: &ResolveRequest) -> Environment {
    if request.declares_environment() {
        Environment::declared(
            request.target.clone(),
            request.features.clone().unwrap_or_default(),
            request.default_features,
        )
    } else {
        Environment::unspecified()
    }
}

/// One acquisition, accumulating what the envelope needs.
struct Acquisition<'a> {
    service: &'a Service,
    config: &'a Config,
    artifacts: Vec<Artifact>,
    evidence: Vec<Evidence>,
    gaps: Vec<Gap>,
    runs: Vec<ProducerRun>,
    indexed: BTreeSet<EvidenceKind>,
}

impl<'a> Acquisition<'a> {
    fn new(service: &'a Service) -> Self {
        Self {
            service,
            config: &service.config,
            artifacts: Vec::new(),
            evidence: Vec::new(),
            gaps: Vec::new(),
            runs: Vec::new(),
            indexed: BTreeSet::new(),
        }
    }

    fn store(
        &mut self,
        fetched: &Fetched,
        kind: ArtifactKind,
        media_type: &str,
        source: &str,
    ) -> std::io::Result<Artifact> {
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
        self.artifacts.push(stored.artifact.clone());
        Ok(stored.artifact)
    }

    fn store_bytes(
        &mut self,
        bytes: &[u8],
        kind: ArtifactKind,
        media_type: &str,
        source: &str,
        compression: Option<&str>,
    ) -> std::io::Result<Artifact> {
        let now = clock::now_rfc3339();
        let stored = self.service.blobs.put(bytes, |_| {
            let mut artifact = Artifact::describe(bytes, kind, media_type, source, &now);
            artifact.compression = compression.map(str::to_owned);
            artifact
        })?;
        self.artifacts.push(stored.artifact.clone());
        Ok(stored.artifact)
    }

    fn run(
        &mut self,
        producer: &str,
        version: &str,
        inputs: BTreeMap<String, String>,
        started_at: String,
        outcome: RunOutcome,
        gaps: Vec<Gap>,
    ) {
        self.runs.push(ProducerRun {
            producer: producer.to_owned(),
            producer_version: version.to_owned(),
            config_digest: enrichment_core::producer::spec::config_digest(&serde_json::json!({
                "index": self.config.producers.rust.crates_io_index_url,
                "api": self.config.producers.rust.crates_io_api_url,
                "docs": self.config.producers.rust.docs_rs_url,
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
            locator: locator.as_object().cloned().unwrap_or_default(),
            source_version_match: SourceVersionMatch::Exact,
            producer: producer.to_owned(),
            producer_version: producer_version.to_owned(),
            excerpt,
        });
    }

    fn fallback_for_json(&self) -> PlannedFallback {
        let enabled = ExecutionProfile::Build.is_enabled(self.config);
        PlannedFallback {
            producer: "rustdoc-json-local-build".to_owned(),
            profile: ExecutionProfile::Build.to_string(),
            enabled,
            next_action: if enabled {
                "The local rustdoc fallback (blueprint §4.4) is not implemented in this build; \
                 it lands with the build profile in phase 4."
                    .to_owned()
            } else {
                "Enable the `build` profile in [policy].enabled_profiles once a sandbox is \
                 configured; until then only registry and source evidence is available."
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
    serde_json::to_value(data)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
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
    crate_root: PathBuf,
    tarball: Artifact,
    manifest_artifact: Option<Artifact>,
}

async fn acquire(service: &Service, request: &ResolveRequest) -> Envelope {
    let mut acq = Acquisition::new(service);
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
        let fetched = match service.fetcher.get(&url, Some("text/plain")).await {
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
    let excerpt = serde_json::to_string(&selected).unwrap_or_default();
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

    // 3. The API version record: links and licence the index does not carry. Non-fatal.
    if let Ok(url) = cratesio::version_url(&rust.crates_io_api_url, &selected.name, &selected.vers)
        && let Ok(fetched) = service.fetcher.get(&url, Some("application/json")).await
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
        Ok(url) => match service.fetcher.get(&url, None).await {
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
                            match extract_and_read_manifest(service, &tarball, &fetched.bytes) {
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
                                            .unwrap_or_default();
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
    let hosted_state =
        match docsrs::json_url(&rust.docs_rs_url, &selected.name, &selected.vers, None) {
            Err(err) => {
                json_gaps.push(Gap {
                    kind: EvidenceKind::HostedRustdocJson,
                    reason: GapReason::UpstreamUnavailable,
                    detail: err.to_string(),
                    planned_fallback: Some(acq.fallback_for_json()),
                });
                (HostedJsonState::NotAttempted, None)
            }
            Ok(url) => match service.fetcher.get(&url, None).await {
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
                                    json_inputs
                                        .insert("rustdoc_json".to_owned(), artifact.sha256.clone());
                                    acq.indexed.insert(EvidenceKind::HostedRustdocJson);
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

    // 6. Identity, recorded before anything is published against it.
    let environment = environment_for(request);
    let context = Context::new(
        release.release_id.clone(),
        environment.environment_id.clone(),
        request.effective_mode(),
    );
    if let Err(err) = service.catalog.record_release(&release) {
        return store_error(&err);
    }
    if let Err(err) = service.catalog.record_context(&context, &environment) {
        return store_error(&err);
    }

    // 7. Normalize the hosted JSON and publish an immutable snapshot (§8.2). Only the daemon
    // publishes, and only after every table has been re-read and counted.
    let started = clock::now_rfc3339();
    let mut snapshot_summary: Option<SnapshotSummary> = None;
    match json_inputs.get("rustdoc_json").cloned() {
        Some(json_sha) => {
            let publish = NormalizeRequest {
                json_sha: &json_sha,
                context: &context,
                release: &release,
                environment: &environment,
                extracted: extracted.as_ref(),
                format_version: hosted_state.1.unwrap_or_default(),
            };
            match normalize_and_publish(service, &mut acq, &publish) {
                Ok((summary, inputs)) => {
                    acq.run(
                        rustdoc::PRODUCER,
                        rustdoc::NORMALIZER_VERSION,
                        inputs,
                        started,
                        RunOutcome::Succeeded,
                        Vec::new(),
                    );
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
        hosted_rustdoc_json: HostedJsonReport {
            state: hosted_state.0,
            format_version: hosted_state.1,
            supported_formats: rustdoc::SUPPORTED_FORMAT_VERSIONS.to_vec(),
            target: observed_target.clone(),
            url: json_url,
            declared_crate_version,
        },
        snapshot: snapshot_summary.clone(),
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
    let summary = summarize(
        &release,
        &upstream,
        hosted_state.0,
        snapshot_summary.as_ref(),
    );
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

    let stored = StoredResolution {
        data: data.clone(),
        coverage: coverage.clone(),
        freshness: freshness.clone(),
        evidence: acq.evidence.clone(),
        artifacts: handles.clone(),
        summary: summary.clone(),
        partial,
    };
    if let Err(err) =
        service
            .catalog
            .record_document(&context.context_id, RESOLUTION_DOCUMENT, &stored)
    {
        return store_error(&err);
    }

    let research = Research {
        summary,
        data: to_object(&data),
        coverage,
        freshness,
        context_id: Some(context.context_id.to_string()),
        snapshot_id: snapshot_summary.map(|s| s.snapshot_id),
        evidence: acq.evidence,
        artifacts: handles,
    };
    if partial {
        research.partial()
    } else {
        research.ok()
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

/// What normalization needs from the acquisition so far.
struct NormalizeRequest<'a> {
    json_sha: &'a str,
    context: &'a Context,
    release: &'a Release,
    environment: &'a Environment,
    extracted: Option<&'a Extracted>,
    format_version: u32,
}

/// Normalize the stored rustdoc JSON, read the crate's declared documents, and publish.
///
/// Returns the snapshot summary and the input digests the producer run records; on failure a
/// single `public_api` gap that names why.
fn normalize_and_publish(
    service: &Service,
    acq: &mut Acquisition<'_>,
    request: &NormalizeRequest<'_>,
) -> Result<(SnapshotSummary, BTreeMap<String, String>), Gap> {
    let gap = |reason: GapReason, detail: String| Gap {
        kind: EvidenceKind::PublicApi,
        reason,
        detail,
        planned_fallback: None,
    };
    let bytes = service.blobs.read(request.json_sha).map_err(|e| {
        gap(
            GapReason::ExtractionFailed,
            format!("cannot read the stored JSON: {e}"),
        )
    })?;
    let payload = String::from_utf8(bytes).map_err(|e| {
        gap(
            GapReason::ExtractionFailed,
            format!("stored JSON is not UTF-8: {e}"),
        )
    })?;
    let json_path = service.blobs.path_for(request.json_sha);
    let rustdoc_artifact_id = artifact_id_for(request.json_sha);
    let normalized = normalize::normalize(&NormalizeInput {
        payload: &payload,
        rustdoc_artifact_id: &rustdoc_artifact_id,
        json_path: Some(&json_path),
        summary_chars: service.config.limits.excerpt_characters,
    })
    .map_err(|e| gap(GapReason::ExtractionFailed, e.to_string()))?;

    let mut fragments = normalized.fragments;
    let mut indexed: Vec<EvidenceKind> = vec![
        EvidenceKind::RegistryMetadata,
        EvidenceKind::HostedRustdocJson,
        EvidenceKind::PublicApi,
        EvidenceKind::Documentation,
    ];
    let mut inputs = BTreeMap::new();
    inputs.insert("rustdoc_json".to_owned(), request.json_sha.to_owned());
    let mut producers = BTreeMap::new();
    producers.insert(
        rustdoc::PRODUCER.to_owned(),
        rustdoc::NORMALIZER_VERSION.to_owned(),
    );
    producers.insert(
        "public-api".to_owned(),
        normalize::public_api_version().to_owned(),
    );
    producers.insert(docsrs::PRODUCER.to_owned(), docsrs::VERSION.to_owned());

    let mut observed = ObservedConfiguration {
        features: Vec::new(),
        all_features: false,
        no_default_features: false,
        target: normalized.target.clone(),
        format_version: request.format_version,
        source: "rustdoc JSON header (target, format_version)".to_owned(),
    };

    if let Some(extracted) = request.extracted {
        indexed.push(EvidenceKind::CrateSource);
        indexed.push(EvidenceKind::DocumentationBuildConfig);
        inputs.insert("crate_tarball".to_owned(), extracted.tarball.sha256.clone());
        producers.insert(source::PRODUCER.to_owned(), source::VERSION.to_owned());
        observed.features = extracted.facts.docs_rs.features.clone();
        observed.all_features = extracted.facts.docs_rs.all_features;
        observed.no_default_features = extracted.facts.docs_rs.no_default_features;
        observed.source =
            "Cargo.toml [package.metadata.docs.rs] plus the rustdoc JSON header".to_owned();

        // The crate's own documents become text artifacts so `read_artifact` can page them.
        let mut by_file: BTreeMap<String, String> = BTreeMap::new();
        for (file, kind) in source::text_files(&extracted.crate_root) {
            let Ok(text) = std::fs::read(extracted.crate_root.join(&file)) else {
                continue;
            };
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
            if let Ok(artifact) = acq.store_bytes(&text, artifact_kind, media, &source_uri, None) {
                by_file.insert(file, artifact.artifact_id);
            }
        }
        let manifest_artifact_id = extracted
            .manifest_artifact
            .as_ref()
            .map(|a| a.artifact_id.clone())
            .unwrap_or_else(|| extracted.tarball.artifact_id.clone());
        let declared = source::source_fragments(
            &extracted.crate_root,
            &extracted.facts,
            &manifest_artifact_id,
            &extracted.tarball.artifact_id,
            &|file| by_file.get(file).cloned(),
        );
        if declared.iter().any(|f| f.kind == FragmentKind::Example) {
            indexed.push(EvidenceKind::Examples);
        }
        if declared
            .iter()
            .any(|f| f.kind == FragmentKind::ChangelogSection)
        {
            indexed.push(EvidenceKind::ReleaseNotes);
        }
        fragments.extend(declared);
    }

    let missing: Vec<EvidenceKind> = [
        EvidenceKind::CrateSource,
        EvidenceKind::DocumentationBuildConfig,
        EvidenceKind::Examples,
        EvidenceKind::ReleaseNotes,
    ]
    .into_iter()
    .filter(|k| !indexed.contains(k))
    .collect();

    acq.indexed.extend(indexed.iter().cloned());

    let snapshot_inputs = SnapshotInputs {
        schema_version: SCHEMA_VERSION.to_owned(),
        normalizer_version: rustdoc::NORMALIZER_VERSION.to_owned(),
        context_id: request.context.context_id.clone(),
        input_digests: inputs.clone(),
        producers: producers.clone(),
    };
    let snapshot_id = SnapshotId::derive(&snapshot_inputs);
    let manifest = SnapshotManifest {
        snapshot_id,
        schema_version: SCHEMA_VERSION.to_owned(),
        normalizer_version: rustdoc::NORMALIZER_VERSION.to_owned(),
        context_id: request.context.context_id.clone(),
        release_id: request.release.release_id.clone(),
        environment_id: request.environment.environment_id.clone(),
        crate_name: normalized.crate_name.clone(),
        crate_version: normalized.crate_version.clone(),
        inputs: inputs.clone(),
        producers,
        producer_runs: acq.runs.clone(),
        tables: BTreeMap::new(),
        counts: SnapshotCounts {
            symbols: normalized.stats.symbols,
            definitions: normalized.stats.definitions,
            reexports: normalized.stats.reexports,
            unresolved_reexports: normalized.stats.unresolved_reexports,
            relationships: normalized.relationships.len() as u64,
            fragments: fragments.len() as u64,
            producer_items: normalized.stats.producer_items,
        },
        observed_configuration: observed,
        indexed,
        missing,
        published_at: clock::now_rfc3339(),
    };
    let tables = SnapshotTables {
        symbols: normalized.symbols,
        relationships: normalized.relationships,
        fragments,
    };
    let published = enrichment_store::publish(&service.paths, &service.catalog, manifest, &tables)
        .map_err(|e| {
            gap(
                GapReason::ExtractionFailed,
                format!("snapshot publication failed: {e}"),
            )
        })?;
    Ok((
        SnapshotSummary {
            snapshot_id: published.snapshot_id.to_string(),
            normalizer_version: published.normalizer_version,
            counts: published.counts,
            published_at: published.published_at,
        },
        inputs,
    ))
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
fn extract_and_read_manifest(
    service: &Service,
    artifact: &Artifact,
    bytes: &[u8],
) -> Result<(ManifestFacts, String, PathBuf), String> {
    let destination = service.paths.unpacked().join(&artifact.sha256);
    let already = destination.is_dir()
        && std::fs::read_dir(&destination)
            .map(|mut d| d.next().is_some())
            .unwrap_or(false);
    if !already {
        // Extract into a sibling scratch directory and rename into place, so a failed or
        // partial extraction is never mistaken for a complete one.
        let scratch = service.paths.unpacked().join(format!(
            ".{}.partial-{}",
            artifact.sha256,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&scratch);
        match archive::extract_tar_gz(bytes, &scratch, &ArchivePolicy::default()) {
            Ok(_) => {
                let _ = std::fs::remove_dir_all(&destination);
                std::fs::rename(&scratch, &destination).map_err(|e| e.to_string())?;
            }
            Err(err) => {
                let _ = std::fs::remove_dir_all(&scratch);
                return Err(match err {
                    ArchiveError::Io(e) => format!("extraction failed: {e}"),
                    other => format!("archive refused: {other}"),
                });
            }
        }
    }
    let crate_root = single_top_level(&destination)
        .ok_or_else(|| "tarball has no single top-level directory".to_owned())?;
    let manifest_path = crate_root.join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("cannot read {}: {e}", manifest_path.display()))?;
    let facts =
        docsrs::manifest_facts(&text).map_err(|e| format!("Cargo.toml is not valid: {e}"))?;
    Ok((facts, text, crate_root))
}

fn single_top_level(destination: &Path) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(destination)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    (dirs.len() == 1).then(|| dirs.remove(0))
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
