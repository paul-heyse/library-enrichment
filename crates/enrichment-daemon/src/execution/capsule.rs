//! Service-owned dependency capsules, prepared from pinned acquired artifacts.
use super::Runner;
use crate::{ops::common::Opened, service::Service};
use enrichment_core::{
    canonical,
    execution::{ProcessEnd, ProcessObservation},
    identity::{Ecosystem, Environment},
    producer::python,
};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, atomic::AtomicBool},
};

#[derive(Debug)]
pub enum PreparationError {
    Cancelled,
    Environment(String),
    Process(Box<ProcessObservation>, String),
    Policy(String),
}
impl PreparationError {
    /// Classify a failure returned by [`Runner`].
    ///
    /// Every error the runner returns is an isolation-boundary failure: a target program that
    /// merely failed comes back as a `ProcessObservation`, not an `Err`. So a missing broker, a
    /// container that could not be created and a refused image are all `POLICY_DENIED` with a
    /// setup action -- never `ENVIRONMENT_UNRESOLVED`, which claims something about the studied
    /// library's environment rather than about ours.
    pub fn from_runner(error: &std::io::Error) -> Self {
        Self::Policy(format!("isolated execution is unavailable: {error}"))
    }
}
impl From<String> for PreparationError {
    fn from(value: String) -> Self {
        Self::Environment(value)
    }
}
impl From<&str> for PreparationError {
    fn from(value: &str) -> Self {
        Self::Environment(value.into())
    }
}

pub struct Capsule {
    pub root: PathBuf,
    pub environment: Environment,
    pub lock: Vec<u8>,
    pub observations: Vec<ProcessObservation>,
    /// Whether this capsule outlives the request that prepared it.
    ///
    /// A probe's capsule is scratch and goes away with the job. A warm language server's has to
    /// survive, because the session is keyed by it: removing it under a live session would
    /// leave the server answering about files that no longer exist.
    pub retain: bool,
    runner: Runner,
    storage: Option<super::budget::Preparation>,
}
impl Capsule {
    pub fn write_input(&self, relative: &str, content: impl AsRef<[u8]>) -> std::io::Result<()> {
        self.storage
            .as_ref()
            .ok_or_else(|| std::io::Error::other("retained inputs are immutable"))?
            .write(&self.root.join(relative), content)
    }
}
impl Drop for Capsule {
    fn drop(&mut self) {
        if !self.retain {
            self.runner.discard_workspace(self.root.clone());
        }
    }
}

/// What a retained capsule records about itself, so a later session can reuse it without
/// repeating the preparation that built it.
#[derive(serde::Serialize, serde::Deserialize)]
struct RetainedCapsule {
    version: u32,
    key: String,
    generation: String,
    environment: Environment,
    lock: String,
    inventory: super::inventory::Inventory,
}

/// The content identity of a capsule: same inputs, same directory, same warm session.
///
/// Deliberately not the job ID. Two questions about the same release in the same environment
/// should share one prepared capsule and one language server; keying on the request would give
/// them two of each and call it a cache.
#[must_use]
pub fn capsule_key(opened: &Opened, image: &str, containment: &str) -> String {
    canonical::digest_hex(&serde_json::json!({
        "artifact": opened.release.key.artifact_digest,
        "release": opened.release.release_id,
        "environment": opened.environment,
        "image": image,
        "context": opened.context,
        "preparation": "retained-capsule-3",
        "containment": containment,
        "profile": "build",
    }))
}

/// Prepare a capsule that outlives its request, reusing one already built for these inputs.
///
/// # Errors
///
/// Same failures as [`prepare`]. A reused capsule whose record cannot be read is rebuilt rather
/// than trusted.
pub async fn prepare_retained(
    service: &Service,
    opened: &Opened,
    runner: &Runner,
    image: &str,
    cancel: Arc<AtomicBool>,
) -> Result<Capsule, PreparationError> {
    let key = capsule_key(
        opened,
        image,
        &runner
            .containment_identity()
            .map_err(|e| PreparationError::from_runner(&e))?,
    );
    let capsules = service.paths.cache_root.join("capsules");
    fs::create_dir_all(&capsules).map_err(|e| e.to_string())?;
    runner
        .lease
        .as_ref()
        .ok_or("retained preparation requires an execution lease")?
        .hold_capsule_lock(&capsules.join(format!("{key}.lock")))
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    // One atomic host-only manifest names one complete generation. A replacement is built
    // elsewhere; neither corruption nor interruption erases the previous generation.
    let record = capsules.join(format!("{key}.json"));
    if record.is_file()
        && let Ok(bytes) = fs::read(&record)
        && let Ok(retained) = serde_json::from_slice::<RetainedCapsule>(&bytes)
        && retained.version == 3
        && retained.key == key
        && retained
            .generation
            .strip_prefix(&format!("{key}-"))
            .is_some_and(|suffix| {
                suffix.len() == 32 && suffix.bytes().all(|b| b.is_ascii_hexdigit())
            })
        && retained.environment
            == Environment::resolved(
                retained.environment.toolchain.clone().unwrap_or_default(),
                retained.environment.target.clone().unwrap_or_default(),
                retained.environment.features.clone(),
                retained.environment.default_features,
                canonical::sha256_hex(retained.lock.as_bytes()),
            )
        && canonical::sha256_hex(retained.lock.as_bytes())
            == retained.environment.lock_digest.clone().unwrap_or_default()
        && super::inventory::capture(
            &capsules.join(&retained.generation),
            service
                .config
                .execution
                .capsule_budget_mib
                .saturating_mul(1024 * 1024),
        )
        .is_ok_and(|actual| actual == retained.inventory)
    {
        return Ok(Capsule {
            root: capsules.join(&retained.generation),
            environment: retained.environment,
            lock: retained.lock.into_bytes(),
            observations: Vec::new(),
            retain: true,
            runner: runner.clone(),
            storage: None,
        });
    }
    let generation = format!("{key}-{}", uuid::Uuid::new_v4().simple());
    let mut capsule = prepare(service, opened, runner, image, &generation, cancel).await?;
    let written = serde_json::to_vec(&RetainedCapsule {
        version: 3,
        key,
        generation,
        environment: capsule.environment.clone(),
        lock: String::from_utf8(capsule.lock.clone()).map_err(|e| e.to_string())?,
        inventory: super::inventory::capture(
            &capsule.root,
            service
                .config
                .execution
                .capsule_budget_mib
                .saturating_mul(1024 * 1024),
        )
        .map_err(|e| e.to_string())?,
    })
    .map_err(|e| e.to_string())?;
    let manifest_reservation = super::budget::Reservation::acquire(
        &runner.ownership,
        written.len() as u64,
        service
            .config
            .execution
            .capsule_budget_mib
            .saturating_mul(1024 * 1024),
    )
    .await
    .map_err(|e| PreparationError::Policy(e.to_string()))?;
    // Persist the entire generation before the manifest can point at it. Once publication
    // starts, keep this generation even on a failed fsync; the pointer may already name it.
    let inventory =
        super::inventory::capture(&capsule.root, service.config.execution.scratch_bytes())
            .map_err(|e| e.to_string())?;
    for path in inventory.keys().rev() {
        fs::File::open(capsule.root.join(path))
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())?;
    }
    fs::File::open(&capsule.root)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    fs::File::open(&capsules)
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())?;
    capsule.retain = true;
    enrichment_store::atomic::write_atomic(&record, &written).map_err(|e| e.to_string())?;
    capsule.storage.take();
    manifest_reservation
        .close()
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    Ok(capsule)
}
struct Staging {
    path: Option<PathBuf>,
    runner: Runner,
}
impl Drop for Staging {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            self.runner.discard_workspace(path);
        }
    }
}

pub async fn prepare(
    service: &Service,
    opened: &Opened,
    runner: &Runner,
    image: &str,
    job: &str,
    cancel: Arc<AtomicBool>,
) -> Result<Capsule, PreparationError> {
    let capsules = service.paths.cache_root.join("capsules");
    let root = capsules.join(job);
    let storage = super::budget::Preparation::acquire(
        &runner.ownership,
        &root,
        service.config.execution.scratch_bytes(),
        service
            .config
            .execution
            .capsule_budget_mib
            .saturating_mul(1024 * 1024),
    )
    .await
    .map_err(|error| PreparationError::Policy(error.to_string()))?;
    if root.exists() {
        return Err("capsule already exists; refusing to merge mutable state".into());
    }
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let mut staging = Staging {
        path: Some(root.clone()),
        runner: runner.clone(),
    };
    let digest = opened
        .release
        .key
        .artifact_digest
        .as_deref()
        .ok_or("selected release has no artifact digest")?;
    let bytes = service.blobs.read(digest).map_err(|e| e.to_string())?;
    if canonical::sha256_hex(&bytes) != digest {
        return Err("selected artifact digest mismatch".into());
    }
    let mut observations = Vec::new();
    let versions = match opened.release.key.ecosystem {
        Ecosystem::Python => strings(&[
            "/usr/local/bin/python3",
            "-I",
            "-S",
            "-c",
            "import sys,os; assert os.getuid()==65532; assert sys.version_info[:3]==(3,14,7); print(sys.version)",
        ]),
        Ecosystem::Rust => strings(&["/usr/local/cargo/bin/rustc", "+1.98.1", "-vV"]),
    };
    let observed = runner
        .run(image, &root, &versions, cancel.clone())
        .await
        .map_err(|e| PreparationError::Policy(format!("isolation unavailable: {e}")))?;
    require_success(&observed, "producer identity qualification")?;
    if opened.release.key.ecosystem == Ecosystem::Rust
        && (!observed.stdout.contains("release: 1.98.1")
            || !observed.stdout.contains("host: x86_64-unknown-linux-gnu"))
    {
        return Err(
            "Rust producer identity differs from the admitted stable compiler/target".into(),
        );
    }
    observations.push(observed);
    if opened.release.key.ecosystem == Ecosystem::Python {
        let ty = runner
            .run(
                image,
                &root,
                &strings(&["/opt/producers/bin/ty", "--version"]),
                cancel.clone(),
            )
            .await
            .map_err(|e| PreparationError::from_runner(&e))?;
        require_success(&ty, "ty producer identity")?;
        if ty.stdout.trim() != "ty 0.0.80" {
            return Err("ty producer differs from the admitted version".into());
        }
        observations.push(ty);
    }

    let (lock, toolchain, target, defaults) = match opened.release.key.ecosystem {
        Ecosystem::Python => {
            if !python_toolchain_accepted(opened.environment.toolchain.as_deref(), image) {
                return Err(
                    "admitted producer is Python3.14.7; requested interpreter is different".into(),
                );
            }
            if !python_target_accepted(opened.environment.target.as_deref()) {
                return Err("admitted Python capsule target is linux/x86_64; requested target is not reproduced".into());
            }
            let artifact = opened
                .reader
                .source_artifact()
                .await
                .map_err(|e| e.to_string())?
                .ok_or("selected snapshot lacks its source acquisition")?;
            let url = url::Url::parse(&artifact.source_uri).map_err(|e| e.to_string())?;
            let filename = url
                .path_segments()
                .and_then(|mut s| s.next_back())
                .ok_or("distribution filename missing")?;
            if !filename.ends_with(".whl") || filename.contains(['/', '\\', ':', '%']) {
                return Err("this capsule requires an admitted wheel; source/native builds need a separately qualified build producer".into());
            }
            python::archive::extract_zip(
                &bytes,
                &root.join("metadata"),
                &storage.archive_policy().map_err(|e| e.to_string())?,
            )?;
            let mut metadata = None;
            for path in python::archive::files(&root.join("metadata"))? {
                if path.ends_with(".dist-info/METADATA") {
                    metadata = Some(
                        fs::read_to_string(root.join("metadata").join(&path))
                            .map_err(|e| e.to_string())?,
                    );
                }
            }
            let metadata = metadata.ok_or("wheel METADATA missing")?;
            // Every ty invocation in this capsule -- `ty check` and the language server alike --
            // is pointed at this file. It is written once, here, because a capsule that has it
            // for one caller and not another is two different analysis environments wearing one
            // identity.
            fs::create_dir_all(root.join("probe-config")).map_err(|e| e.to_string())?;
            storage
                .write(&root.join("probe-config/ty.toml"), "")
                .map_err(|e| e.to_string())?;
            fs::create_dir(root.join("wheelhouse")).map_err(|e| e.to_string())?;
            storage
                .write(&root.join("wheelhouse").join(filename), &bytes)
                .map_err(|e| e.to_string())?;
            let (lock, requirements) = super::python_closure::resolve(
                service,
                opened,
                &root,
                filename,
                &metadata,
                cancel.clone(),
                &storage,
            )
            .await?;
            storage
                .write(&root.join("requirements.txt"), requirements)
                .map_err(|e| e.to_string())?;
            let result = runner
                .prepare_outputs(
                    image,
                    &root,
                    &strings(&[
                        "/opt/producers/bin/uv",
                        "--no-config",
                        "--no-python-downloads",
                        "pip",
                        "install",
                        "--python=/usr/local/bin/python3",
                        "--offline",
                        "--no-index",
                        "--find-links=/capsule/wheelhouse",
                        "--only-binary=:all:",
                        "--require-hashes",
                        "--no-deps",
                        "--target=/capsule/python",
                        "--link-mode=copy",
                        "-r",
                        "/capsule/requirements.txt",
                    ]),
                    cancel,
                    [(
                        "python".into(),
                        enrichment_core::capsule_protocol::OutputKind::Directory,
                    )]
                    .into(),
                )
                .await
                .map_err(|e| PreparationError::from_runner(&e))?;
            require_success(&result, "offline dependency install")?;
            observations.push(result);
            (
                lock,
                format!("python-3.14.7;{image}"),
                "linux-x86_64".to_owned(),
                None,
            )
        }
        Ecosystem::Rust => {
            if opened
                .environment
                .target
                .as_deref()
                .is_some_and(|s| s != "x86_64-unknown-linux-gnu")
            {
                return Err(
                    "only the admitted x86_64-unknown-linux-gnu Rust target is available".into(),
                );
            }
            let extracted = enrichment_core::archive::extract_tar_gz(
                bytes.as_slice(),
                &root.join("source"),
                &storage.archive_policy().map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            let top = extracted.top_level.ok_or("crate archive lacks one root")?;
            let source = root.join("source").join(&top);
            let cargo: toml::Value = toml::from_str(
                &fs::read_to_string(source.join("Cargo.toml")).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            validate_cargo_sources(&cargo)?;
            // Discard all package-supplied Cargo configuration, not just the top-level file.
            for path in python::archive::files(&source)? {
                if path.split('/').any(|part| part == ".cargo")
                    || path.ends_with("rust-toolchain")
                    || path.ends_with("rust-toolchain.toml")
                {
                    fs::remove_file(source.join(path)).map_err(|e| e.to_string())?;
                }
            }
            fs::create_dir(root.join("src")).map_err(|e| e.to_string())?;
            let features =
                serde_json::to_string(&opened.environment.features).map_err(|e| e.to_string())?;
            let default_features = opened.environment.default_features.unwrap_or(true);
            let manifest = format!(
                "[package]\nname=\"enrichment-consumer\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[dependencies]\n{}={{path=\"source/{}\",features={},default-features={}}}\n",
                serde_json::to_string(&opened.release.key.package).map_err(|e| e.to_string())?,
                top,
                features,
                default_features
            );
            storage
                .write(&root.join("Cargo.toml"), manifest)
                .map_err(|e| e.to_string())?;
            storage
                .write(&root.join("src/main.rs"), "fn main() {}\n")
                .map_err(|e| e.to_string())?;
            // Target config cannot influence acquisition: fixed compiler/toolchain and manifest, no source Git/path overrides.
            fs::create_dir_all(root.join("cargo-home")).map_err(|e| e.to_string())?;
            let fetch = runner
                .acquire(
                    image,
                    &root,
                    &strings(&[
                        "/usr/local/cargo/bin/cargo",
                        "+1.98.1",
                        "fetch",
                        "--manifest-path=/capsule/Cargo.toml",
                        "--target=x86_64-unknown-linux-gnu",
                    ]),
                    cancel,
                    [
                        (
                            "Cargo.lock".into(),
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
            require_success(&fetch, "registry dependency acquisition")?;
            observations.push(fetch);
            let lock = fs::read(root.join("Cargo.lock")).map_err(|e| e.to_string())?;
            (
                lock,
                format!("rust-1.98.1;{image}"),
                "x86_64-unknown-linux-gnu".into(),
                Some(default_features),
            )
        }
    };
    let environment = Environment::resolved(
        toolchain,
        target,
        opened.environment.features.clone(),
        defaults,
        canonical::sha256_hex(&lock),
    );
    staging.path.take();
    Ok(Capsule {
        root,
        environment,
        lock,
        observations,
        retain: false,
        runner: runner.clone(),
        storage: Some(storage),
    })
}

pub(super) fn validate_cargo_sources(value: &toml::Value) -> Result<(), String> {
    match value {
        toml::Value::Table(table) => {
            for (key, value) in table {
                if matches!(
                    key.as_str(),
                    "dependencies" | "dev-dependencies" | "build-dependencies"
                ) && let Some(dependencies) = value.as_table()
                {
                    for dependency in dependencies.values() {
                        if dependency.as_table().is_some_and(|fields| {
                            fields.contains_key("path")
                                || fields.contains_key("git")
                                || fields.contains_key("registry")
                        }) {
                            return Err("unadmitted dependency source: path/Git/alternate registry dependencies require explicit admission".into());
                        }
                    }
                }
                if matches!(
                    key.as_str(),
                    "git" | "path" | "registry" | "patch" | "replace" | "members"
                ) && key != "path"
                {
                    return Err(format!("unadmitted Cargo source/workspace setting: {key}"));
                }
                if key == "path"
                    && value
                        .as_str()
                        .is_some_and(|s| s.starts_with('/') || s.split('/').any(|c| c == ".."))
                {
                    return Err("escaping Cargo source path".into());
                }
                validate_cargo_sources(value)?;
            }
        }
        toml::Value::Array(items) => {
            for item in items {
                validate_cargo_sources(item)?;
            }
        }
        _ => {}
    }
    Ok(())
}
pub fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| (*s).to_owned()).collect()
}
pub fn require_success(result: &ProcessObservation, stage: &str) -> Result<(), PreparationError> {
    if result.end != ProcessEnd::Exited || result.exit_code != Some(0) {
        return Err(PreparationError::Process(
            Box::new(result.clone()),
            stage.into(),
        ));
    }
    // A preparation step whose container could not be removed did not succeed, whatever its
    // exit code says. `verify_usage` reports an unconfirmed probe cleanup as `partial` because
    // the probe evidence is still real; a preparation step has no evidence to preserve, so the
    // honest answer is that the stage failed.
    if !result.cleanup_confirmed {
        return Err(PreparationError::Process(
            Box::new(result.clone()),
            format!("{stage} (its container was not confirmed removed)"),
        ));
    }
    Ok(())
}

/// One declared-interpreter contract for preparing and discovering the qualified capsule.
pub(crate) fn python_toolchain_accepted(declared: Option<&str>, image: &str) -> bool {
    declared.is_none_or(|v| {
        matches!(v, "python-3.14" | "python-3.14.7") || v == format!("python-3.14.7;{image}")
    })
}
pub(crate) fn python_target_accepted(declared: Option<&str>) -> bool {
    declared.is_none_or(|v| matches!(v, "linux" | "x86_64-manylinux_2_40" | "linux-x86_64"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_cargo_dependency_sources_are_rejected_before_acquisition() {
        for source in [
            "[dependencies.local]\npath='deps/local'",
            "[target.'cfg(unix)'.dependencies.local]\npath='deps/local'",
            "[build-dependencies.local]\ngit='https://github.com/x/y'",
        ] {
            let value: toml::Value = toml::from_str(source).unwrap();
            assert!(validate_cargo_sources(&value).is_err());
        }
        assert!(
            validate_cargo_sources(
                &toml::from_str("[lib]\npath='src/lib.rs'\n[dependencies]\nserde='1'").unwrap()
            )
            .is_ok()
        );
    }
}
