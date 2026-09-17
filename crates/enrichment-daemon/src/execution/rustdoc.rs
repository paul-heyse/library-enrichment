//! The §4.4 fallback: build rustdoc JSON ourselves when hosted JSON is unusable.
//!
//! Hosted docs.rs JSON comes first, always. This runs only when it is missing or in a format
//! this build refuses, the caller asked for it, and the `build` profile is both enabled and
//! qualified. Then a dated nightly compiles the crate inside a service-owned capsule.
//!
//! Three things this deliberately does not do. It never uses a bare `+nightly`, which floats --
//! the toolchain is the dated producer identity in `config/toolchains.toml`, and the recorded
//! `rustc -vV` is checked against it before the output is trusted. It never concatenates a
//! caller-supplied command; the argv is derived from typed options here. And it never claims
//! that a successful nightly build says anything about the project's stable compiler (§4.4).

use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::AtomicBool};

use enrichment_core::{
    canonical,
    execution::ProcessObservation,
    identity::Environment,
    producer::{docsrs::ManifestFacts, rustdoc},
};

use super::{Runner, capsule::PreparationError, capsule::require_success, capsule::strings};

/// The toolchain identity the fallback runs under, from `config/toolchains.toml`.
///
/// Duplicated here as a constant rather than read at run time because it is part of the
/// compiled producer's identity: changing it is a code change with a provenance consequence,
/// not a configuration tweak. `just rustdoc-format-matrix` is what re-measures it.
pub const TOOLCHAIN: &str = "nightly-2026-09-13";
/// The rustc release the toolchain must report, or the output is not admitted.
pub const RUSTC_RELEASE: &str = "1.100.0-nightly";
/// The commit hash the toolchain must report.
pub const RUSTC_COMMIT: &str = "809936eac";
/// The only target this producer builds for.
pub const TARGET: &str = "x86_64-unknown-linux-gnu";

#[derive(Debug, serde::Serialize)]
pub struct BuildSpec {
    pub target: String,
    pub features: Vec<String>,
    pub default_features: bool,
}
impl BuildSpec {
    pub fn for_environment(environment: &Environment) -> Result<Self, String> {
        let target = environment.target.as_deref().unwrap_or(TARGET);
        if target != TARGET {
            return Err(format!(
                "requested rustdoc target {target} is not installed in the admitted producer image; supported target is {TARGET}"
            ));
        }
        Ok(Self {
            target: target.into(),
            features: environment.features.clone().unwrap_or_default(),
            default_features: environment.default_features.unwrap_or(true),
        })
    }

    fn rustdoc_args(&self, manifest: &str) -> Vec<String> {
        let mut args = strings(&[
            "/usr/local/cargo/bin/cargo",
            &format!("+{TOOLCHAIN}"),
            "rustdoc",
            "--frozen",
            "--lib",
            &format!("--manifest-path={manifest}"),
            &format!("--target={}", self.target),
        ]);
        if !self.default_features {
            args.push("--no-default-features".into());
        }
        for feature in &self.features {
            args.push(format!("--features={feature}"));
        }
        args.extend(strings(&[
            "--",
            "-Z",
            "unstable-options",
            "--output-format",
            "json",
        ]));
        args
    }
}

/// What a successful local build produced.
pub struct LocalBuild {
    /// The rustdoc JSON, verified to declare a supported `format_version`.
    pub payload: Vec<u8>,
    /// The format version the document declares.
    pub format_version: u32,
    /// The exact `rustc -vV` output of the toolchain that emitted it.
    pub rustc_identity: String,
    /// Every bounded process observation, in order, for provenance.
    pub observations: Vec<ProcessObservation>,
    pub lock: Vec<u8>,
    pub environment: Environment,
    pub started_at: enrichment_core::native_time::ObservationTime,
    pub finished_at: enrichment_core::native_time::ObservationTime,
    pub image_id: String,
    pub containment_identity: String,
}

/// A capsule directory that removes itself unless the build succeeds.
struct Scratch(PathBuf, Runner);
impl Drop for Scratch {
    fn drop(&mut self) {
        self.1.discard_workspace(self.0.clone());
    }
}

/// Build rustdoc JSON for one crate tarball inside an owned capsule.
///
/// `tarball` is the already-acquired `.crate` archive; nothing is fetched from the registry
/// here except the crate's own dependencies, and that happens in a separate acquisition step
/// before the offline build.
pub async fn build(
    runner: &Runner,
    image: &str,
    capsules: &Path,
    budget_mib: u64,
    tarball: &[u8],
    facts: &ManifestFacts,
    requested: &Environment,
    cancel: Arc<AtomicBool>,
) -> Result<LocalBuild, PreparationError> {
    let started_at = enrichment_core::native_time::ObservationTime::now()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    let containment_identity = runner
        .containment_identity()
        .map_err(|e| PreparationError::from_runner(&e))?;
    let spec = BuildSpec::for_environment(requested)?;
    // This is the largest writer of capsule storage in the service -- a whole
    // `CARGO_TARGET_DIR` plus a fetched registry closure -- so it takes the same aggregate
    // ceiling as every other capsule, checked before anything is extracted.
    std::fs::create_dir_all(capsules).map_err(|e| e.to_string())?;
    let root = capsules.join(format!("rustdoc-{}", uuid::Uuid::new_v4().simple()));
    let storage = super::budget::Preparation::acquire(
        &runner.ownership,
        &root,
        runner.limits.scratch_bytes(),
        budget_mib.saturating_mul(1024 * 1024),
    )
    .await
    .map_err(|e| PreparationError::Policy(e.to_string()))?;
    std::fs::create_dir(&root).map_err(|e| e.to_string())?;
    let scratch = Scratch(root.clone(), runner.clone());

    // The toolchain is a recorded identity, so check it before trusting anything it emits.
    let identity = runner
        .run(
            image,
            &root,
            &strings(&[
                "/usr/local/cargo/bin/rustc",
                &format!("+{TOOLCHAIN}"),
                "-vV",
            ]),
            cancel.clone(),
        )
        .await
        .map_err(|e| PreparationError::from_runner(&e))?;
    require_success(&identity, "dated nightly producer identity")?;
    if !identity.stdout.contains(RUSTC_RELEASE) || !identity.stdout.contains(RUSTC_COMMIT) {
        return Err(format!(
            "the admitted image's {TOOLCHAIN} reports an unrecorded compiler; \
             config/toolchains.toml records {RUSTC_RELEASE} ({RUSTC_COMMIT}) and the image said: {}",
            identity.stdout.trim()
        )
        .into());
    }
    let rustc_identity = identity.stdout.clone();
    let mut observations = vec![identity];

    let extracted = enrichment_core::archive::extract_tar_gz(
        tarball,
        &root.join("source"),
        &storage.archive_policy().map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let top = extracted.top_level.ok_or("crate archive lacks one root")?;
    let source = root.join("source").join(&top);

    // The same admission the consumer capsule applies: no path, Git or alternate-registry
    // dependency, and no package-supplied Cargo configuration or toolchain override.
    let manifest: toml::Value = toml::from_str(
        &std::fs::read_to_string(source.join("Cargo.toml")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    super::capsule::validate_cargo_sources(&manifest)?;
    for path in enrichment_core::producer::python::archive::files(&source)? {
        if path.split('/').any(|part| part == ".cargo")
            || path.ends_with("rust-toolchain")
            || path.ends_with("rust-toolchain.toml")
        {
            std::fs::remove_file(source.join(path)).map_err(|e| e.to_string())?;
        }
    }

    let manifest_path = format!("/capsule/source/{top}/Cargo.toml");
    std::fs::create_dir_all(root.join("cargo-home")).map_err(|e| e.to_string())?;
    // Acquisition is networked and separate from the build, which is offline. Fetching is the
    // only step allowed to reach a registry, and it resolves nothing the manifest did not name.
    let fetch = runner
        .acquire(
            image,
            &root,
            &strings(&[
                "/usr/local/cargo/bin/cargo",
                &format!("+{TOOLCHAIN}"),
                "fetch",
                &format!("--manifest-path={manifest_path}"),
                &format!("--target={}", spec.target),
            ]),
            cancel.clone(),
            [
                (
                    format!("source/{top}/Cargo.lock"),
                    enrichment_core::capsule_protocol::OutputKind::File,
                ),
                (
                    "cargo-home".into(),
                    enrichment_core::capsule_protocol::OutputKind::Directory,
                ),
            ]
            .into(),
        )
        .await
        .map_err(|e| PreparationError::from_runner(&e))?;
    require_success(&fetch, "rustdoc dependency acquisition")?;
    observations.push(fetch);

    let lock = std::fs::read(source.join("Cargo.lock")).map_err(|e| e.to_string())?;
    if lock.len() > 1_048_576 {
        return Err("rustdoc dependency lock exceeds retained record bound".into());
    }
    let environment = Environment::resolved(
        TOOLCHAIN.into(),
        spec.target.clone(),
        spec.features.clone(),
        Some(spec.default_features),
        canonical::sha256_hex(&lock),
    );
    let argv = spec.rustdoc_args(&manifest_path);
    let built = runner
        .prepare_outputs(
            image,
            &root,
            &argv,
            cancel,
            [(
                format!(
                    "target/{TARGET}/doc/{}.json",
                    facts.lib_name.clone().unwrap_or_else(|| top
                        .split('-')
                        .next()
                        .unwrap_or(&top)
                        .replace('-', "_"))
                ),
                enrichment_core::capsule_protocol::OutputKind::File,
            )]
            .into(),
        )
        .await
        .map_err(|e| PreparationError::from_runner(&e))?;
    require_success(&built, "local rustdoc build")?;
    observations.push(built);

    let lib = facts
        .lib_name
        .clone()
        .unwrap_or_else(|| top.split('-').next().unwrap_or(&top).replace('-', "_"));
    let produced = root
        .join("target")
        .join(TARGET)
        .join("doc")
        .join(format!("{lib}.json"));
    let payload = std::fs::read(&produced).map_err(|e| {
        PreparationError::Environment(format!(
            "the build succeeded but produced no {}: {e}",
            produced.display()
        ))
    })?;

    // Check the emitted format before normalization, exactly as the hosted path does. A
    // nightly that moved to an unsupported format is a refusal, not a silent misparse (R04).
    let text = String::from_utf8(payload).map_err(|e| e.to_string())?;
    let probe = rustdoc::probe_format(&text).map_err(|e| {
        PreparationError::Environment(format!("the locally built JSON is unusable: {e}"))
    })?;

    if runner
        .containment_identity()
        .map_err(|e| PreparationError::from_runner(&e))?
        != containment_identity
    {
        return Err("execution helper or containment changed during rustdoc production".into());
    }
    drop(scratch);
    Ok(LocalBuild {
        payload: text.into_bytes(),
        format_version: probe.format_version,
        rustc_identity,
        observations,
        lock,
        environment,
        started_at,
        finished_at: enrichment_core::native_time::ObservationTime::now()
            .map_err(|error| PreparationError::Policy(error.to_string()))?,
        image_id: image.into(),
        containment_identity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fallback_honors_requested_features_defaults_and_frozen_lock() {
        let requested =
            Environment::declared(Some(TARGET.into()), Some(vec!["serde".into()]), Some(false));
        let spec = BuildSpec::for_environment(&requested).unwrap();
        let args = spec.rustdoc_args("/capsule/Cargo.toml");
        assert!(args.iter().any(|a| a == "--features=serde"));
        assert!(args.iter().any(|a| a == "--no-default-features"));
        assert!(args.iter().any(|a| a == "--frozen"));
        assert!(!args.iter().any(|a| a == "--all-features"));
        let missing = Environment::declared(Some("aarch64-unknown-linux-gnu".into()), None, None);
        assert!(
            BuildSpec::for_environment(&missing)
                .unwrap_err()
                .contains("not installed")
        );
    }
}
