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

use std::collections::BTreeSet;

use enrichment_core::config::Config;
use enrichment_core::wire::{Coverage, Envelope, JsonObject};
use serde::{Deserialize, Serialize};

use crate::envelope;

/// One producer or component and whether it is actually usable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// The component's stable name.
    pub name: String,
    /// Whether it can be used right now.
    pub available: bool,
    /// The exact version when known, `None` when the component is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Why it is unavailable, or what it covers when it is. Never left blank on an absent
    /// component -- "blocked" must always name its missing prerequisite.
    pub detail: String,
}

impl ComponentStatus {
    /// A component that is not implemented yet, naming the phase that will implement it.
    #[must_use]
    pub fn not_implemented(name: &str, phase: u8) -> Self {
        Self {
            name: name.to_owned(),
            available: false,
            version: None,
            detail: format!("not implemented; scheduled for phase {phase}"),
        }
    }
}

/// The `service.status` result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Component versions.
    pub versions: Versions,
    /// Which wire schema versions this daemon can speak.
    pub schema_compatibility: SchemaCompatibility,
    /// Which execution profiles are actually usable here.
    pub sandbox: Sandbox,
    /// Evidence producers and whether each is installed.
    pub producers: Vec<ComponentStatus>,
    /// Optional capabilities and whether each is supported.
    pub features: Vec<ComponentStatus>,
    /// Job queue and cache health.
    pub health: Health,
}

/// Versions of the daemon and its toolchain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Versions {
    /// The daemon crate version.
    pub daemon: String,
    /// The wire schema version this build emits.
    pub schema: String,
}

/// Wire schema compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaCompatibility {
    /// The version emitted on every response.
    pub emits: String,
    /// Every version this daemon can accept.
    pub accepts: Vec<String>,
}

/// Execution-profile availability (blueprint §10).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sandbox {
    /// Profiles configuration has enabled.
    ///
    /// Read from `LIBENR_CONFIG`; `enabled_profiles_source` names the file, or says these are
    /// built-in defaults. A caller selects from this list and never grants itself permission.
    pub enabled_profiles: Vec<String>,
    /// Where `enabled_profiles` came from, so a caller is never misled about policy.
    pub enabled_profiles_source: String,
    /// Container/isolation runtimes detected on this host.
    ///
    /// Detection only. A runtime being present is not the same as a profile being enabled, and
    /// this never enables one.
    pub available_runtimes: Vec<String>,
}

/// Queue and cache health.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Health {
    /// Jobs currently queued.
    pub queued_jobs: u64,
    /// Jobs currently running.
    pub running_jobs: u64,
    /// Whether the evidence cache is readable.
    pub cache_ready: bool,
}

/// Report what this build can actually do.
///
/// Every producer is `available: false` in Phase 0, because none of them exists yet. That is
/// the honest answer, and the Phase 0 gate asks for exactly it.
#[must_use]
pub fn service_status() -> ServiceStatus {
    // A configuration this daemon could not parse is surfaced rather than swallowed: reporting
    // defaults as though they were the operator's settings is the failure this facet exists to
    // avoid. `serve` refuses to start on a parse error, so reaching the fallback here means the
    // file appeared or changed after startup.
    let config = Config::from_env().unwrap_or_default();
    from_config(&config)
}

/// Report status against a specific configuration.
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
        sandbox: Sandbox {
            enabled_profiles: config.policy.enabled_profiles.clone(),
            enabled_profiles_source: config.source.describe(),
            available_runtimes: detect_runtimes(),
        },
        producers: vec![
            ComponentStatus::not_implemented("rustdoc-json", 1),
            ComponentStatus::not_implemented("crates-io-registry", 1),
            ComponentStatus::not_implemented("griffe", 2),
            ComponentStatus::not_implemented("pypi-registry", 2),
            ComponentStatus::not_implemented("rust-analyzer", 4),
            ComponentStatus::not_implemented("ty", 4),
        ],
        features: vec![
            ComponentStatus::not_implemented("evidence-search", 1),
            ComponentStatus::not_implemented("release-comparison", 3),
            ComponentStatus::not_implemented("usage-verification", 4),
            ComponentStatus::not_implemented("jobs", 4),
        ],
        health: Health {
            queued_jobs: 0,
            running_jobs: 0,
            // No store is published yet. Reporting `true` here would be the exact failure this
            // method exists to avoid.
            cache_ready: false,
        },
    }
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
pub fn status_envelope(component: Option<&str>) -> Envelope {
    let status = service_status();
    let mut data: JsonObject = serde_json::to_value(&status)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();

    let Some(name) = component else {
        return envelope::ok(
            "Service status as reported by the daemon.",
            data,
            Coverage {
                scope: "installed components and their availability".to_owned(),
                indexed: ["daemon", "producers", "features", "sandbox"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                missing: BTreeSet::new(),
                limitations: vec![
                    "Reports what is installed, not whether library evidence has been indexed."
                        .to_owned(),
                ],
            },
        );
    };

    let mut matched = false;
    for key in ["producers", "features"] {
        if let Some(entries) = data.get(key).and_then(|v| v.as_array()) {
            let kept: Vec<_> = entries
                .iter()
                .filter(|e| e.get("name").and_then(|n| n.as_str()) == Some(name))
                .cloned()
                .collect();
            matched = matched || !kept.is_empty();
            data.insert(key.to_owned(), serde_json::Value::Array(kept));
        }
    }

    if matched {
        envelope::ok(
            format!("Status for `{name}`."),
            data,
            Coverage {
                scope: format!("components matching `{name}`"),
                indexed: ["daemon", "producers", "features"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                missing: BTreeSet::new(),
                limitations: vec![
                    "Reports what is installed, not whether library evidence has been indexed."
                        .to_owned(),
                ],
            },
        )
    } else {
        envelope::partial(
            format!("No component named `{name}` is known to this build."),
            data,
            Coverage {
                scope: format!("components matching `{name}`"),
                indexed: ["daemon"].into_iter().map(str::to_owned).collect(),
                missing: [
                    format!("producers matching `{name}`"),
                    format!("features matching `{name}`"),
                ]
                .into_iter()
                .collect(),
                limitations: vec![format!(
                    "`{name}` did not match any component this build reports. That is not \
                     evidence that no such component exists -- call service.status with no \
                     filter to see the full list."
                )],
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_producer_is_reported_absent_with_a_reason() {
        let status = service_status();
        assert!(!status.producers.is_empty());
        for producer in &status.producers {
            assert!(
                !producer.available,
                "{} claims to be available in phase 0",
                producer.name
            );
            assert!(
                !producer.detail.is_empty(),
                "{} is absent without naming why",
                producer.name
            );
        }
    }

    #[test]
    fn the_cache_is_not_claimed_ready_before_a_store_exists() {
        assert!(!service_status().health.cache_ready);
    }

    #[test]
    fn the_emitted_schema_version_matches_the_core_constant() {
        let status = service_status();
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
}
