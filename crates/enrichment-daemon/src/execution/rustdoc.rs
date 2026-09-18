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
    execution::{ProcessObservation, producer::Invocation},
    identity::Environment,
    producer::{docsrs::ManifestFacts, rustdoc},
};

use super::{Runner, capsule::PreparationError, capsule::require_success};
use enrichment_store::owned_bytes::OwnedBytes;

/// The toolchain identity the fallback runs under, from `config/toolchains.toml`.
///
/// Duplicated here as a constant rather than read at run time because it is part of the
/// compiled producer's identity: changing it is a code change with a provenance consequence,
/// not a configuration tweak. `just rustdoc-format-matrix` is what re-measures it.
pub const TOOLCHAIN: &str = enrichment_core::execution::producer::RUSTDOC_TOOLCHAIN;
/// The only target this producer builds for.
pub const TARGET: &str = enrichment_core::execution::producer::RUST_TARGET;

/// What a successful local build produced.
pub struct LocalBuild {
    /// The rustdoc JSON, verified to declare a supported `format_version`.
    pub payload: OwnedBytes,
    /// The format version the document declares.
    pub format_version: u32,
    /// The exact `rustc -vV` output of the toolchain that emitted it.
    pub rustc_identity: String,
    /// Every bounded process observation, in order, for provenance.
    pub observations: Vec<ProcessObservation>,
    pub lock: OwnedBytes,
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
    tarball: OwnedBytes,
    facts: &ManifestFacts,
    requested: &Environment,
    cancel: Arc<AtomicBool>,
) -> Result<LocalBuild, PreparationError> {
    let started_at = enrichment_core::native_time::ObservationTime::now()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    let containment_identity = runner
        .containment_identity()
        .map_err(|e| PreparationError::from_runner(&e))?;
    let spec =
        enrichment_store::producer_plan::rustdoc_options(runner.ownership.runtime(), requested)
            .await
            .map_err(|error| PreparationError::Environment(error.to_string()))?;
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
        .run(image, &root, &Invocation::RustdocIdentity, cancel.clone())
        .await
        .map_err(|e| PreparationError::from_runner(&e))?;
    require_success(
        runner,
        &Invocation::RustdocIdentity,
        &identity,
        "dated nightly producer identity",
    )
    .await?;
    let rustc_identity = identity.stdout.clone();
    let mut observations = vec![identity];

    let storage = Arc::new(storage);
    let prepared = super::preparation::cargo_source(
        runner,
        storage.clone(),
        &root,
        tarball,
        facts
            .package_name
            .as_deref()
            .ok_or("Cargo package identity missing")?,
        facts.package_version.as_deref(),
    )
    .await?;
    let top = prepared.root;
    let lib = prepared.identity.lib_name;
    let _storage = storage;
    std::fs::create_dir_all(root.join("cargo-home")).map_err(|error| error.to_string())?;
    let source = root.join("source").join(&top);
    // Acquisition is networked and separate from the build, which is offline. Fetching is the
    // only step allowed to reach a registry, and it resolves nothing the manifest did not name.
    let fetch = runner
        .run(
            image,
            &root,
            &Invocation::RustdocFetch {
                source_root: top.clone(),
            },
            cancel.clone(),
        )
        .await
        .map_err(|e| PreparationError::from_runner(&e))?;
    require_success(
        runner,
        &Invocation::RustdocFetch {
            source_root: top.clone(),
        },
        &fetch,
        "rustdoc dependency acquisition",
    )
    .await?;
    observations.push(fetch);

    let lock = super::preparation::read(runner, source.join("Cargo.lock"), 1_048_576).await?;
    let environment = Environment::resolved(
        TOOLCHAIN.into(),
        spec.target.clone(),
        spec.features.clone(),
        Some(spec.default_features),
        canonical::sha256_hex(&lock),
    );
    let built = runner
        .run(
            image,
            &root,
            &Invocation::RustdocBuild {
                source_root: top.clone(),
                lib_name: lib.clone(),
                features: spec.features.clone(),
                default_features: spec.default_features,
            },
            cancel,
        )
        .await
        .map_err(|e| PreparationError::from_runner(&e))?;
    require_success(
        runner,
        &Invocation::RustdocBuild {
            source_root: top.clone(),
            lib_name: lib.clone(),
            features: spec.features.clone(),
            default_features: spec.default_features,
        },
        &built,
        "local rustdoc build",
    )
    .await?;
    observations.push(built);

    let produced = root
        .join("target")
        .join(TARGET)
        .join("doc")
        .join(format!("{lib}.json"));
    let payload = super::preparation::read(runner, produced, rustdoc::facts::MAX_BYTES).await?;
    // Version probing ignores the body; parsing and normalization run in the owned decoder.
    let payload = runner
        .ownership
        .runtime()
        .blocking(move || -> Result<_, String> {
            let text = std::str::from_utf8(&payload).map_err(|error| error.to_string())?;
            let probe = rustdoc::probe_format(text)
                .map_err(|error| format!("the locally built JSON is unusable: {error}"))?;
            Ok((payload, probe))
        })
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))??;
    let (payload, probe) = payload;

    if runner
        .containment_identity()
        .map_err(|e| PreparationError::from_runner(&e))?
        != containment_identity
    {
        return Err("execution helper or containment changed during rustdoc production".into());
    }
    drop(scratch);
    Ok(LocalBuild {
        payload,
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
