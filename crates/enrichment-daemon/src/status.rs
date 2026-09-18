//! The `service.status` method.
//!
//! Blueprint §7 requires six facets: versions, schema compatibility, sandbox capabilities,
//! installed producers, feature support, and queue/cache health. The Phase 0 gate adds the part
//! that actually matters here -- "`service_status` truthfully reports absent components".
//!
//! So a producer that is not implemented reports `available: false` with a reason, and is never
//! omitted, defaulted to true, or described as "ready". An absent component is a fact to
//! report, not an error to raise: `.claude/rules/evidence-truthfulness.md` is explicit that
//! `ok` means successful within the declared scope, and the scope here is "what is installed".

use enrichment_core::config::Config;
use enrichment_core::wire::Envelope;
use enrichment_core::wire::status::{
    EvidenceCounters, FetchCounters, LspMetrics, SingleFlightCounts, VerificationCounters,
};

use crate::envelope;
use crate::service::Service;

// Declarations and native plans own component availability. This adapter captures facts.
pub use enrichment_core::wire::status::{
    ComponentStatus, Health, Sandbox, SchemaCompatibility, StatusData as ServiceStatus, Versions,
};

/// Report status against a specific configuration, with no store open.
#[must_use]
pub fn from_config(config: &Config) -> ServiceStatus {
    ServiceStatus {
        versions: Versions {
            daemon: env!("CARGO_PKG_VERSION").to_owned(),
            schema: enrichment_core::SCHEMA_VERSION.to_owned(),
        },
        schema_compatibility: SchemaCompatibility {
            emits: enrichment_core::SCHEMA_VERSION.to_owned(),
            accepts: vec![enrichment_core::SCHEMA_VERSION.to_owned()],
        },
        snapshot_compatibility: SchemaCompatibility {
            emits: enrichment_core::SNAPSHOT_SCHEMA_VERSION.to_owned(),
            accepts: vec![enrichment_core::SNAPSHOT_SCHEMA_VERSION.to_owned()],
        },
        sandbox: Sandbox {
            execution_routes: Vec::new(),
            enabled_profiles: config.policy.enabled_profiles.clone(),
            enabled_profiles_source: config.source.describe(),
            available_runtimes: detect_runtimes(),
            // Without a store there is no data root to read a receipt from, so the honest
            // answer here is "not qualified, and this process cannot even look".
            execution_qualified: false,
            execution_readiness: "no evidence store is open in this process, so no qualification \
                                  receipt can be read"
                .to_owned(),
            admitted_images: std::collections::BTreeMap::new(),
        },
        producers: enrichment_core::status_components::without_store(
            enrichment_core::status_components::Kind::Producer,
        ),
        features: enrichment_core::status_components::without_store(
            enrichment_core::status_components::Kind::Feature,
        ),
        health: Health {
            queued_jobs: 0,
            running_jobs: 0,
            // No store is open. Reporting `true` here would be the exact failure this method
            // exists to avoid.
            cache_ready: false,
            data_root: None,
            single_flight: SingleFlightCounts::default(),
            lsp: LspMetrics::default(),
            fetch: FetchCounters::default(),
            evidence: EvidenceCounters::default(),
            verification: VerificationCounters::default(),
            native_queries: None,
            uptime_seconds: 0,
        },
    }
}

/// Report status for a running service: configuration plus what the open store enables.
pub async fn from_service(service: &Service) -> std::io::Result<ServiceStatus> {
    let mut status = from_config(&service.config);
    let cache_ready = service.cache_ready();
    // Read fresh rather than caching at startup: an operator may qualify while the daemon runs,
    // and a cached "not qualified" would be a stale answer presented as a current one.
    let policy = crate::execution::readiness::policy(service)
        .await
        .map_err(std::io::Error::other)?;
    let qualification = policy
        .qualification()
        .await
        .map_err(std::io::Error::other)?;
    status.sandbox.execution_routes = policy.routes().await.map_err(std::io::Error::other)?;
    status.sandbox.execution_qualified = qualification.qualified;
    status.sandbox.execution_readiness = qualification.detail.clone();
    status.sandbox.admitted_images = policy
        .admitted_images()
        .await
        .map_err(std::io::Error::other)?;
    let components = enrichment_store::status_plan::components(
        &service.repository.runtime,
        enrichment_store::status_plan::Observations {
            cache_ready,
            worker_python: service.config.producers.python.worker_python.clone(),
            python_worker_qualified: service
                .python_worker_qualified
                .load(std::sync::atomic::Ordering::Relaxed),
            execution_routes: status.sandbox.execution_routes.clone(),
            execution_detail: qualification.detail,
        },
    )
    .await
    .map_err(std::io::Error::other)?;
    status.producers = components.producers;
    status.features = components.features;
    let (queued, running) = service.jobs.counts().await?;
    status.health.queued_jobs = queued as u64;
    status.health.running_jobs = running as u64;
    status.health.cache_ready = cache_ready;
    status.health.data_root = Some(service.paths.data_root.display().to_string());
    status.health.lsp = service.lsp.metrics();
    status.health.single_flight = service.single_flight.counts();
    // The §14.3 diagnostics. Every one is scoped to this process, which is why
    // `uptime_seconds` travels beside them: a count with no window is not readable.
    let counters = service.metrics.counters().await?;
    status.health.fetch = counters.fetch;
    status.health.evidence = counters.evidence;
    status.health.verification = counters.verification;
    status.health.native_queries = Some(
        service
            .repository
            .runtime
            .operational_counters()
            .await
            .map_err(std::io::Error::other)?,
    );
    status.health.uptime_seconds = service.started_instant.elapsed().as_secs();
    Ok(status)
}

/// Which isolation runtimes are on PATH.
///
/// Detection only -- availability of a runtime is not the same as an enabled profile, and this
/// never enables one. Blueprint §10: a caller selects from profiles local configuration has
/// enabled; it cannot grant itself permission.
fn detect_runtimes() -> Vec<String> {
    ["bwrap", "podman", "docker"]
        .into_iter()
        .filter(|name| which(name))
        .map(str::to_owned)
        .collect()
}

fn which(program: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| dir.join(program).is_file())
}

/// The `service.status` response as a complete wire envelope.
///
/// Built here rather than in the adapter: `coverage` is an assertion about what the service
/// looked at, and only the side that looked can make it honestly (§1.1, §7.2).
///
/// An unmatched `component` filter yields `partial` with the gap named, never an empty `ok`.
/// Those are different facts — "this build has no such component" is a much stronger claim than
/// "the filter matched nothing" — and a caller can only tell them apart if `coverage` says so.
#[must_use]
pub async fn status_envelope(service: &Service, component: Option<&str>) -> Envelope {
    let status = match from_service(service).await {
        Ok(status) => status,
        Err(error) => return crate::ops::common::operation_error(&error, "service_status"),
    };
    match enrichment_store::status_plan::select(&service.repository.runtime, status, component)
        .await
    {
        Ok(selected) => Envelope::new(
            enrichment_core::wire::EnvelopeBody {
                request_id: envelope::new_request_id(),
                summary: selected.summary,
                context_id: None,
                snapshot_id: None,
                data: selected.status.into(),
                coverage: selected.coverage,
                freshness: envelope::unverified_freshness(),
                evidence: Vec::new(),
                artifacts: Vec::new(),
                delivery: Default::default(),
            },
            selected.outcome,
        ),
        Err(error) => crate::ops::common::operation_error(&error, "service_status_selection"),
    }
}

#[cfg(test)]
mod tests {
    use enrichment_core::producer::{cratesio, rustdoc};
    use enrichment_store::StatePaths;

    use super::*;

    #[tokio::test]
    async fn native_component_selection_preserves_unknown_status() {
        let root = tempfile::tempdir().unwrap();
        let runtime =
            enrichment_store::runtime::QueryRuntime::new(root.path(), Default::default()).unwrap();
        let input = from_config(&Config::default());
        let name = input.producers[0].name.clone();
        let selected = enrichment_store::status_plan::select(&runtime, input.clone(), Some(&name))
            .await
            .unwrap();
        assert!(matches!(
            selected.outcome,
            enrichment_core::wire::Outcome::Ok { .. }
        ));
        assert!(selected.status.producers.iter().all(|row| row.name == name));
        let absent =
            enrichment_store::status_plan::select(&runtime, input.clone(), Some("not-a-component"))
                .await
                .unwrap();
        assert!(matches!(
            absent.outcome,
            enrichment_core::wire::Outcome::Partial { .. }
        ));
        assert_eq!(absent.coverage.missing.len(), 2);
        assert!(absent.status.producers.is_empty());
        assert!(absent.status.features.is_empty());
        let all = enrichment_store::status_plan::select(&runtime, input.clone(), None)
            .await
            .unwrap();
        assert_eq!(all.status, input);
    }

    fn open_service() -> (tempfile::TempDir, Service) {
        let dir = tempfile::tempdir().expect("temp dir");
        let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
        let service = Service::open(Config::default(), paths).expect("service opens");
        (dir, service)
    }

    /// Components whose implementation landed, and which must therefore never be described to a
    /// caller as waiting for a phase.
    ///
    /// The list is deliberately explicit rather than derived: a component is added here when its
    /// code ships, and forgetting to add one is the same oversight this test exists to catch --
    /// except that here the oversight is visible in a diff.
    const SHIPPED: &[&str] = &[
        "rustdoc-json",
        "crates-io-registry",
        "griffe",
        "pypi-registry",
        "rust-analyzer",
        "ty",
        "evidence-search",
        "release-comparison",
        "usage-verification",
        "jobs",
    ];

    #[tokio::test]
    async fn a_component_that_shipped_never_tells_a_caller_to_wait_for_a_phase() {
        // The failure this catches: `from_config` seeds every component with
        // "not implemented; scheduled for phase N", and a branch that only fills in the
        // *available* case leaves that default standing on an unqualified host. The caller is
        // then told to wait for a phase that already shipped, instead of being told to run
        // `just execution-qualify` -- which is the difference between a missing prerequisite and
        // a missing feature. `ty` did exactly this until 2026-09-14.
        //
        // A service with no qualified image is the interesting case, because it is the one where
        // the default survives. `open_service` has none.
        let (_dir, service) = open_service();
        let status = service.into_status().await;
        for component in status.producers.iter().chain(status.features.iter()) {
            if SHIPPED.contains(&component.name.as_str()) {
                assert!(
                    !component.detail.contains("not implemented"),
                    "{} shipped, but reports: {}",
                    component.name,
                    component.detail
                );
                assert!(
                    !component.detail.contains("scheduled for phase"),
                    "{} shipped, but points the caller at a phase: {}",
                    component.name,
                    component.detail
                );
            }
        }
    }

    #[tokio::test]
    async fn every_absent_producer_names_a_reason() {
        for status in [
            from_config(&Config::default()),
            open_service().1.into_status().await,
        ] {
            assert!(!status.producers.is_empty());
            for producer in status.producers.iter().filter(|p| !p.available) {
                assert!(
                    !producer.detail.is_empty(),
                    "{} is absent without naming why",
                    producer.name
                );
            }
        }
    }

    #[test]
    fn the_cache_is_not_claimed_ready_without_an_open_store() {
        assert!(!from_config(&Config::default()).health.cache_ready);
        assert!(
            from_config(&Config::default())
                .producers
                .iter()
                .all(|p| !p.available),
            "without a store no producer can be available"
        );
    }

    #[tokio::test]
    async fn an_open_store_makes_the_registry_producer_available_and_the_cache_ready() {
        let (_dir, service) = open_service();
        let status = from_service(&service).await.unwrap();
        assert!(status.health.cache_ready);
        let registry = status
            .producers
            .iter()
            .find(|p| p.name == "crates-io-registry")
            .expect("listed");
        assert!(registry.available);
        assert_eq!(registry.version.as_deref(), Some(cratesio::VERSION));
        // The normalizer and search follow the store too: they read and write it.
        let rustdoc = status
            .producers
            .iter()
            .find(|p| p.name == "rustdoc-json")
            .expect("listed");
        assert!(rustdoc.available);
        assert_eq!(
            rustdoc.version.as_deref(),
            Some(rustdoc::NORMALIZER_VERSION)
        );
        let search = status
            .features
            .iter()
            .find(|f| f.name == "evidence-search")
            .expect("listed");
        assert!(search.available);
        // Phase 2+ components stay absent: a store does not conjure a producer.
        assert!(
            status
                .producers
                .iter()
                .any(|p| p.name == "pypi-registry" && p.available)
        );
        for name in ["griffe", "rust-analyzer", "ty"] {
            let p = status
                .producers
                .iter()
                .find(|p| p.name == name)
                .expect(name);
            assert!(!p.available, "{name} must not borrow the store's readiness");
        }
    }

    #[test]
    fn the_emitted_schema_version_matches_the_core_constant() {
        let status = from_config(&Config::default());
        assert_eq!(
            status.schema_compatibility.emits,
            enrichment_core::SCHEMA_VERSION
        );
    }

    #[test]
    fn the_profile_list_says_where_it_came_from() {
        // Reporting a built-in default as though it were the operator's configuration would be
        // a wrong answer about policy, not a cosmetic one.
        let status = from_config(&Config::default());
        assert!(
            status.sandbox.enabled_profiles_source.contains("built-in"),
            "a caller must be able to tell a default from a configured value"
        );
    }

    #[test]
    fn the_reported_profiles_are_the_configured_ones() {
        // The regression this guards: a hardcoded list presented as policy. An operator who
        // disables every profile must be told exactly that.
        let mut config = Config::default();
        config.policy.enabled_profiles = Vec::new();
        assert!(from_config(&config).sandbox.enabled_profiles.is_empty());

        config.policy.enabled_profiles = vec!["static".to_owned(), "runtime".to_owned()];
        assert_eq!(
            from_config(&config).sandbox.enabled_profiles,
            vec!["static".to_owned(), "runtime".to_owned()]
        );
    }

    #[test]
    fn runtime_detection_does_not_enable_a_profile() {
        // podman/docker/bwrap are all present on this workstation, but detection never adds a
        // profile: the list comes from configuration alone.
        let mut config = Config::default();
        config.policy.enabled_profiles = vec!["static".to_owned()];
        let status = from_config(&config);
        assert_eq!(status.sandbox.enabled_profiles, vec!["static".to_owned()]);
        assert!(
            !status.sandbox.available_runtimes.is_empty(),
            "this workstation has bwrap, podman and docker"
        );
    }

    impl Service {
        async fn into_status(self) -> ServiceStatus {
            from_service(&self).await.unwrap()
        }
    }
}
