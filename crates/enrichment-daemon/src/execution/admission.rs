//! Whether the configured execution images have actually been qualified.
//!
//! Configuring an image ID is a statement of intent. It says nothing about whether that image
//! exists, whether its tools are the ones we pinned, or whether this host can contain it. Only
//! an actual containment run establishes that, and `scripts/execution-qualify.py` writes a
//! receipt naming the exact image IDs it ran against.
//!
//! This module reads that receipt and answers one question truthfully: is execution ready, and
//! if not, what is the missing prerequisite? Blueprint §13's Phase 0 gate -- "`service_status`
//! truthfully reports absent components" -- is still in force, and "an image is configured" is
//! the kind of fact that reads as readiness without being it.

use std::collections::BTreeMap;
use std::path::Path;

use enrichment_core::config::Execution;
use serde::{Deserialize, Serialize};

/// The receipt `scripts/execution-qualify.py` writes after a successful containment run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub containment_identity: String,
    /// When the containment run finished, RFC 3339.
    pub qualified_at: String,
    /// The execution root the run used.
    pub execution_root: String,
    /// Ecosystem name to the exact image ID that was qualified.
    pub images: BTreeMap<String, String>,
    /// The tool identities actually observed inside each image.
    #[serde(default)]
    pub tools: BTreeMap<String, BTreeMap<String, String>>,
    pub resources: BTreeMap<String, super::resources::ResourceProbe>,
}

/// Execution readiness, and the reason when there is none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Qualification {
    /// Every configured image is covered by a receipt from an actual containment run.
    Qualified(Box<Receipt>),
    /// Not ready. `reason` names the concrete missing prerequisite.
    NotQualified {
        /// What is missing, and the command that would supply it.
        reason: String,
    },
}

impl Qualification {
    /// True only when an actual containment run covered the currently configured images.
    #[must_use]
    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::Qualified(_))
    }

    /// A single sentence a caller can act on, whichever way this went.
    #[must_use]
    pub fn detail(&self) -> String {
        match self {
            Self::Qualified(receipt) => format!(
                "qualified {} by an actual containment run against {}",
                receipt.qualified_at,
                receipt
                    .images
                    .iter()
                    .map(|(name, id)| format!("{name}={id}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::NotQualified { reason } => reason.clone(),
        }
    }

    /// The image IDs a receipt covers, empty when there is none.
    #[must_use]
    pub fn admitted_images(&self) -> BTreeMap<String, String> {
        match self {
            Self::Qualified(receipt) => receipt.images.clone(),
            Self::NotQualified { .. } => BTreeMap::new(),
        }
    }
}

/// The execution root a configuration resolves to. Mirrors `Runner::new`.
#[must_use]
pub fn execution_root(config: &Execution, cache_root: &Path) -> std::path::PathBuf {
    config
        .storage_root
        .clone()
        .unwrap_or_else(|| cache_root.join("podman"))
}

/// Where the receipt lives: beside the images it qualifies.
///
/// Not under the data root, because qualification is a property of an image in an execution
/// root, not of a catalog. Two daemons sharing an execution root share its qualification; one
/// that is pointed at a different root has not inherited anything, which is correct.
#[must_use]
pub fn receipt_path(execution_root: &Path) -> std::path::PathBuf {
    execution_root.join("admitted-images.json")
}

/// Read the receipt fresh and compare it against the configuration in force.
///
/// Deliberately not cached at startup: an operator may qualify while the daemon is running, and
/// a cached "not qualified" would then be a stale answer presented as a current one.
#[must_use]
pub fn qualification(config: &Execution, cache_root: &Path) -> Qualification {
    let mut configured = BTreeMap::new();
    if let Some(image) = config.python_image.as_deref() {
        configured.insert("python", image);
    }
    if let Some(image) = config.rust_image.as_deref() {
        configured.insert("rust", image);
    }
    if configured.is_empty() {
        return Qualification::NotQualified {
            reason:
                "no admitted producer image is configured; run `just execution-images --apply` \
                     then `just execution-qualify --apply`, and set [execution].python_image / \
                     rust_image to the image IDs it records"
                    .to_owned(),
        };
    }

    let path = receipt_path(&execution_root(config, cache_root));
    let Ok(bytes) = std::fs::read(&path) else {
        return Qualification::NotQualified {
            reason: format!(
                "images are configured but no qualification receipt exists at {}; run \
                 `just execution-qualify --apply`. A configured image is not a contained one",
                path.display()
            ),
        };
    };
    let receipt: Receipt = match serde_json::from_slice(&bytes) {
        Ok(receipt) => receipt,
        Err(err) => {
            return Qualification::NotQualified {
                reason: format!(
                    "the qualification receipt at {} could not be read ({err}); re-run \
                     `just execution-qualify --apply`",
                    path.display()
                ),
            };
        }
    };

    let root = execution_root(config, cache_root);
    if Path::new(&receipt.execution_root) != root {
        return Qualification::NotQualified {
            reason: format!(
                "the qualification receipt was written for execution root {}, but {} is in \
                 force; a receipt copied between roots qualifies nothing. Re-run \
                 `just execution-qualify --apply` against the configured root",
                receipt.execution_root,
                root.display()
            ),
        };
    }
    for (ecosystem, image) in &configured {
        match receipt.images.get(*ecosystem) {
            Some(qualified) if qualified == image => {}
            Some(qualified) => {
                return Qualification::NotQualified {
                    reason: format!(
                        "the qualification receipt covers {ecosystem} image {qualified}, but \
                         {image} is configured; re-run `just execution-qualify --apply` against \
                         the configured image"
                    ),
                };
            }
            None => {
                return Qualification::NotQualified {
                    reason: format!(
                        "the qualification receipt covers no {ecosystem} image, but {image} is \
                         configured; re-run `just execution-qualify --apply`"
                    ),
                };
            }
        }
    }
    match super::description::containment_identity(config) {
        Ok(actual) if actual == receipt.containment_identity => {}
        Ok(_) => return Qualification::NotQualified { reason: "the helper, broker, execution contract or effective limits changed; run `just execution-qualify --apply` for the current configuration".into() },
        Err(error) => return Qualification::NotQualified { reason: format!("execution identity unavailable: {error}; build the helper and requalify") },
    }
    let resources = match config.resources() {
        Ok(resources) => resources,
        Err(error) => {
            return Qualification::NotQualified {
                reason: error.to_string(),
            };
        }
    };
    for (ecosystem, image) in configured {
        let checked = receipt
            .resources
            .get(ecosystem)
            .ok_or_else(|| std::io::Error::other("no kernel resource observation"))
            .and_then(|probe| probe.validate(&resources, image));
        if let Err(error) = checked {
            return Qualification::NotQualified {
                reason: format!(
                    "{ecosystem} resource qualification is invalid: {error}; re-run `just execution-qualify --apply`"
                ),
            };
        }
    }
    Qualification::Qualified(Box::new(receipt))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Write a receipt that names the root it is being written into, as a real one does.
    fn write(dir: &Path, mut receipt: serde_json::Value) {
        let root = dir.join("podman");
        receipt["execution_root"] = serde_json::Value::String(root.display().to_string());
        receipt["containment_identity"] = serde_json::json!(
            super::super::description::containment_identity(&Execution::default())
                .expect("current helper and broker")
        );
        let requested = Execution::default().resources().unwrap();
        let mut resources = BTreeMap::new();
        for (ecosystem, image) in receipt["images"].as_object().unwrap() {
            let process = enrichment_core::execution::ProcessObservation {
                image_id: image.as_str().unwrap().into(),
                command: vec![
                    "/bin/sh".into(),
                    "-c".into(),
                    super::super::resources::PROBE.into(),
                ],
                started_at: "2026-09-14T00:00:00Z".into(),
                finished_at: "2026-09-14T00:00:01Z".into(),
                exit_code: Some(0),
                end: enrichment_core::execution::ProcessEnd::Exited,
                stdout: "200000 100000\n1073741824\n0\n128\ntmpfs 131072 4096\n".into(),
                stderr: String::new(),
                cleanup_confirmed: true,
            };
            resources.insert(
                ecosystem.clone(),
                super::super::resources::ResourceProbe::new(requested.clone(), process).unwrap(),
            );
        }
        receipt["resources"] = serde_json::to_value(resources).unwrap();
        let path = receipt_path(&root);
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("create");
        std::fs::write(path, serde_json::to_vec(&receipt).expect("json")).expect("write");
    }

    fn config(python: Option<&str>, rust: Option<&str>) -> Execution {
        Execution {
            python_image: python.map(str::to_owned),
            rust_image: rust.map(str::to_owned),
            ..Execution::default()
        }
    }

    #[test]
    fn no_configured_image_names_the_setup_command_rather_than_failing_silently() {
        let dir = tempfile::tempdir().expect("temp dir");
        let result = qualification(&config(None, None), dir.path());
        assert!(!result.is_qualified());
        assert!(
            result.detail().contains("just execution-images"),
            "{result:?}"
        );
    }

    #[test]
    fn a_configured_image_without_a_receipt_is_not_qualified() {
        // The whole point of the distinction: configuration is intent, not containment.
        let dir = tempfile::tempdir().expect("temp dir");
        let result = qualification(&config(Some("sha256:aa"), None), dir.path());
        assert!(!result.is_qualified());
        assert!(
            result.detail().contains("no qualification receipt"),
            "{result:?}"
        );
        assert!(result.admitted_images().is_empty());
    }

    #[test]
    fn a_receipt_for_a_different_image_does_not_qualify_the_configured_one() {
        // The failure this guards: qualify an image, then edit the config to point somewhere
        // else, and have the service keep reporting itself ready on the old evidence.
        let dir = tempfile::tempdir().expect("temp dir");
        write(
            dir.path(),
            serde_json::json!({
                "qualified_at": "2026-09-14T00:00:00+00:00",
                "execution_root": "",
                "images": {"python": "sha256:aa"},
            }),
        );
        let result = qualification(&config(Some("sha256:bb"), None), dir.path());
        assert!(!result.is_qualified());
        assert!(result.detail().contains("sha256:aa"), "{result:?}");
        assert!(result.detail().contains("sha256:bb"), "{result:?}");
    }

    #[test]
    fn a_matching_receipt_qualifies_and_reports_when_and_against_what() {
        let dir = tempfile::tempdir().expect("temp dir");
        write(
            dir.path(),
            serde_json::json!({
                "qualified_at": "2026-09-14T00:00:00+00:00",
                "execution_root": "",
                "images": {"python": "sha256:aa", "rust": "sha256:bb"},
                "tools": {"python": {"ty": "ty 0.0.80"}},
            }),
        );
        let result = qualification(&config(Some("sha256:aa"), Some("sha256:bb")), dir.path());
        assert!(result.is_qualified());
        assert!(result.detail().contains("2026-09-14"), "{result:?}");
        assert_eq!(result.admitted_images().len(), 2);
        let mut changed = config(Some("sha256:aa"), Some("sha256:bb"));
        changed.scratch_mib = 256;
        let invalidated = qualification(&changed, dir.path());
        assert!(!invalidated.is_qualified());
        assert!(invalidated.detail().contains("effective limits changed"));
    }

    #[test]
    fn a_receipt_from_a_different_execution_root_qualifies_nothing() {
        // A receipt is evidence about images *in a root*. Copied elsewhere -- backed up,
        // restored, shared between checkouts -- it describes a containment run that never
        // happened on the storage now in force.
        let dir = tempfile::tempdir().expect("temp dir");
        let path = receipt_path(&dir.path().join("podman"));
        std::fs::create_dir_all(path.parent().expect("a parent")).expect("create");
        std::fs::write(
            path,
            serde_json::to_vec(&serde_json::json!({
                "qualified_at": "2026-09-14T00:00:00+00:00",
                "execution_root": "/somewhere/else",
                "containment_identity": "unrelated",
                "images": {"python": "sha256:aa"},
                "resources": {},
            }))
            .expect("json"),
        )
        .expect("write");
        let result = qualification(&config(Some("sha256:aa"), None), dir.path());
        assert!(!result.is_qualified());
        assert!(result.detail().contains("/somewhere/else"), "{result:?}");
    }

    #[test]
    fn qualifying_only_one_ecosystem_does_not_qualify_the_other() {
        let dir = tempfile::tempdir().expect("temp dir");
        write(
            dir.path(),
            serde_json::json!({
                "qualified_at": "2026-09-14T00:00:00+00:00",
                "execution_root": "",
                "images": {"python": "sha256:aa"},
            }),
        );
        let result = qualification(&config(Some("sha256:aa"), Some("sha256:bb")), dir.path());
        assert!(!result.is_qualified());
        assert!(result.detail().contains("no rust image"), "{result:?}");
    }

    #[test]
    fn missing_or_changed_resource_observations_cannot_qualify() {
        let dir = tempfile::tempdir().unwrap();
        let spec = serde_json::json!({"qualified_at":"2026-09-14T00:00:00Z", "images":{"python":"sha256:aa"}});
        write(dir.path(), spec);
        let path = receipt_path(&dir.path().join("podman"));
        let original: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        for field in ["observed", "requested", "process"] {
            let mut changed = original.clone();
            match field {
                "process" => {
                    changed["resources"]["python"][field]["stdout"] =
                        serde_json::json!("max 100000\n1073741824\n0\n128\ntmpfs 131072 4096\n")
                }
                _ => {
                    changed["resources"]["python"][field]["cpu_quota_micros"] =
                        serde_json::json!(100000)
                }
            }
            std::fs::write(&path, serde_json::to_vec(&changed).unwrap()).unwrap();
            assert!(!qualification(&config(Some("sha256:aa"), None), dir.path()).is_qualified());
        }
        let mut missing = original.clone();
        missing.as_object_mut().unwrap().remove("resources");
        std::fs::write(&path, serde_json::to_vec(&missing).unwrap()).unwrap();
        assert!(!qualification(&config(Some("sha256:aa"), None), dir.path()).is_qualified());
        std::fs::write(&path, serde_json::to_vec(&original).unwrap()).unwrap();
        let mut changed = config(Some("sha256:aa"), None);
        changed.cpus = 32;
        assert!(!qualification(&changed, dir.path()).is_qualified());
    }
}
