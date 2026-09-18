//! Service-owned dependency capsules, prepared from pinned acquired artifacts.
use super::Runner;
use crate::{ops::common::Opened, service::Service};
use enrichment_core::{
    canonical,
    execution::{ProcessObservation, producer::Invocation},
    identity::{Ecosystem, Environment},
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
    pub prepared: enrichment_core::operation::ownership::PreparedCapsule,
    pub observations: Vec<ProcessObservation>,
    /// Whether this capsule outlives the request that prepared it.
    ///
    /// A probe's capsule is scratch and goes away with the job. A warm language server's has to
    /// survive, because the session is keyed by it: removing it under a live session would
    /// leave the server answering about files that no longer exist.
    pub retain: bool,
    runner: Runner,
    storage: Option<Arc<super::budget::Preparation>>,
}
impl Capsule {
    pub fn input_reader(&self) -> std::io::Result<super::cleanup::InputReader> {
        self.runner.input_reader()
    }
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

/// The content identity of a capsule: same inputs, same directory, same warm session.
///
/// Deliberately not the job ID. Two questions about the same release in the same environment
/// should share one prepared capsule and one language server; keying on the request would give
/// them two of each and call it a cache.
#[must_use]
pub fn capsule_key(opened: &Opened, image: &str, containment: &str) -> String {
    enrichment_core::native_key::Key::CapsuleIdentity
        .record(&capsule_inputs(opened, image, containment))
        .expect("declared native capsule identity")
}

fn capsule_inputs(
    opened: &Opened,
    image: &str,
    containment: &str,
) -> enrichment_core::operation::identities::CapsuleIdentity {
    enrichment_core::operation::identities::CapsuleIdentity {
        release: opened.release.clone(),
        environment: opened.environment.clone(),
        context: opened.context.clone(),
        image: image.into(),
        containment: containment.into(),
        profile: enrichment_core::policy::ExecutionProfile::Build,
    }
}

/// Prepare a capsule that outlives its request, reusing one already built for these inputs.
///
/// # Errors
///
/// Same failures as [`prepare`]. Native catalog errors refuse reuse; a missing or changed
/// physical inventory requires a new prepared generation.
pub async fn prepare_retained(
    service: &Service,
    opened: &Opened,
    runner: &Runner,
    image: &str,
    cancel: Arc<AtomicBool>,
) -> Result<Capsule, PreparationError> {
    let inputs = capsule_inputs(
        opened,
        image,
        &runner
            .containment_identity()
            .map_err(|e| PreparationError::from_runner(&e))?,
    );
    let key = enrichment_core::native_key::Key::CapsuleIdentity
        .record(&inputs)
        .map_err(|e| PreparationError::Policy(e.to_string()))?;
    let capsules = service.paths.cache_root.join("capsules");
    fs::create_dir_all(&capsules).map_err(|e| e.to_string())?;
    runner
        .lease
        .as_ref()
        .ok_or("retained preparation requires an execution lease")?
        .hold_capsule_lock(&capsules.join(format!("{key}.lock")))
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    // The admitted Delta record selects the exact generation. Filesystem capture is
    // an observation; native inventory equality decides whether that generation is reusable.
    if let Some(retained) = runner
        .ownership
        .retained_capsule(&key)
        .await
        .map_err(|e| PreparationError::Policy(e.to_string()))?
    {
        let root = capsules.join(&retained.generation);
        if let Ok(actual) =
            super::inventory::capture(&root, service.config.execution.scratch_bytes())
            && enrichment_store::physical_ownership::reusable_capsule(
                &service.repository.runtime,
                &retained,
                &actual,
            )
            .await
            .map_err(|e| PreparationError::Policy(e.to_string()))?
        {
            return Ok(Capsule {
                root,
                prepared: retained.prepared,
                observations: Vec::new(),
                retain: true,
                runner: runner.clone(),
                storage: None,
            });
        }
    }
    let generation = format!("{key}-{}", uuid::Uuid::new_v4().simple());
    let mut capsule = prepare(service, opened, runner, image, &generation, cancel).await?;
    // Persist bytes before the Delta transaction can select them. A lost commit
    // acknowledgement preserves this generation for native reconciliation.
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
    runner
        .ownership
        .publish_capsule(&enrichment_core::operation::ownership::RetainedCapsule {
            key,
            cache: runner.ownership.cache().into(),
            generation,
            prepared: capsule.prepared.clone(),
            sequence: 0,
        })
        .await
        .map_err(|e| PreparationError::Policy(e.to_string()))?;
    capsule.storage.take();
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
    workspace_id: &str,
    cancel: Arc<AtomicBool>,
) -> Result<Capsule, PreparationError> {
    let producer_target = enrichment_store::environment_plan::admit(
        &service.repository.runtime,
        &opened.environment,
        opened.release.key.ecosystem,
        image,
    )
    .await
    .map_err(|error| PreparationError::Environment(error.to_string()))?;
    let capsules = service.paths.cache_root.join("capsules");
    let root = capsules.join(workspace_id);
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
    let artifact = opened
        .reader
        .source_artifact()
        .await
        .map_err(|error| error.to_string())?
        .ok_or("selected snapshot lacks its source acquisition")?;
    if artifact.sha256 != digest {
        return Err("selected source acquisition differs from release".into());
    }
    let owner = runner
        .input_reader()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    let blobs = service.blobs.clone();
    let input = artifact.clone();
    let pool = service
        .repository
        .runtime
        .session()
        .runtime_env()
        .memory_pool
        .clone();
    let bytes = service
        .repository
        .runtime
        .blocking(move || {
            let _owner = owner;
            blobs.read_owned(&input, 268_435_456, &pool)
        })
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))?
        .map_err(|error| error.to_string())?;
    let storage = Arc::new(storage);
    let mut observations = Vec::new();
    let versions = match opened.release.key.ecosystem {
        Ecosystem::Python => Invocation::PythonIdentity,
        Ecosystem::Rust => Invocation::RustIdentity,
    };
    let observed = runner
        .run(image, &root, &versions, cancel.clone())
        .await
        .map_err(|e| PreparationError::Policy(format!("isolation unavailable: {e}")))?;
    require_success(
        runner,
        &versions,
        &observed,
        "producer identity qualification",
    )
    .await?;
    observations.push(observed);
    if opened.release.key.ecosystem == Ecosystem::Python {
        let ty = runner
            .run(image, &root, &Invocation::TyIdentity, cancel.clone())
            .await
            .map_err(|e| PreparationError::from_runner(&e))?;
        require_success(runner, &Invocation::TyIdentity, &ty, "ty producer identity").await?;
        observations.push(ty);
    }

    let (lock, defaults) = match opened.release.key.ecosystem {
        Ecosystem::Python => {
            let url = url::Url::parse(&artifact.source_uri).map_err(|error| error.to_string())?;
            let filename = url
                .path_segments()
                .and_then(|mut segments| segments.next_back())
                .ok_or("distribution filename missing")?;
            let bytes = Arc::new(bytes);
            let metadata = super::preparation::wheel_distribution(
                runner,
                storage.clone(),
                root.join("metadata"),
                bytes.clone(),
                filename,
                &artifact.sha256,
            )
            .await?;

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
                .write(&root.join("wheelhouse").join(filename), bytes.as_ref())
                .map_err(|e| e.to_string())?;
            let (lock, requirements) = super::python_closure::resolve(
                service,
                opened,
                &root,
                filename,
                &metadata.distribution,
                cancel.clone(),
                &storage,
                runner,
            )
            .await?;
            storage
                .write(&root.join("requirements.txt"), requirements)
                .map_err(|e| e.to_string())?;
            let result = runner
                .run(image, &root, &Invocation::PythonInstall, cancel)
                .await
                .map_err(|e| PreparationError::from_runner(&e))?;
            require_success(
                runner,
                &Invocation::PythonInstall,
                &result,
                "offline dependency install",
            )
            .await?;
            observations.push(result);
            (lock, None)
        }
        Ecosystem::Rust => {
            let top = super::preparation::cargo_source(
                runner,
                storage.clone(),
                &root,
                bytes,
                &opened.release.key.package,
                Some(&opened.release.key.version),
            )
            .await?
            .root;
            fs::create_dir(root.join("src")).map_err(|error| error.to_string())?;
            let options = enrichment_store::producer_plan::rustdoc_options(
                &service.repository.runtime,
                &opened.environment,
            )
            .await
            .map_err(|error| error.to_string())?;
            let default_features = options.default_features;
            let manifest = enrichment_store::preparation_plan::cargo_manifest(
                &service.repository.runtime,
                &opened.release.key.package,
                &top,
                &options,
            )
            .await
            .map_err(|error| error.to_string())?;
            storage
                .write(&root.join("Cargo.toml"), manifest)
                .map_err(|e| e.to_string())?;
            storage
                .write(&root.join("src/main.rs"), "fn main() {}\n")
                .map_err(|e| e.to_string())?;
            // Target config cannot influence acquisition: fixed compiler/toolchain and manifest, no source Git/path overrides.
            fs::create_dir_all(root.join("cargo-home")).map_err(|e| e.to_string())?;
            let fetch = runner
                .run(image, &root, &Invocation::RustFetch, cancel)
                .await
                .map_err(|e| PreparationError::from_runner(&e))?;
            require_success(
                runner,
                &Invocation::RustFetch,
                &fetch,
                "registry dependency acquisition",
            )
            .await?;
            observations.push(fetch);
            let lock = super::preparation::read(runner, root.join("Cargo.lock"), 1_048_576)
                .await?
                .to_vec();
            (lock, Some(default_features))
        }
    };
    let environment = Environment::resolved(
        producer_target.toolchain,
        producer_target.target,
        opened.environment.features.clone().unwrap_or_default(),
        defaults,
        canonical::sha256_hex(&lock),
    );
    storage
        .write(&root.join("enrichment.lock"), &lock)
        .map_err(|e| e.to_string())?;
    let prepared = enrichment_core::operation::ownership::PreparedCapsule {
        inputs: capsule_inputs(
            opened,
            image,
            &runner.containment_identity().map_err(|e| e.to_string())?,
        ),
        environment,
        lock: String::from_utf8(lock).map_err(|e| e.to_string())?,
        inventory: super::inventory::capture(&root, service.config.execution.scratch_bytes())
            .map_err(|e| e.to_string())?,
    };
    staging.path.take();
    Ok(Capsule {
        root,
        prepared,
        observations,
        retain: false,
        runner: runner.clone(),
        storage: Some(storage),
    })
}

pub fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| (*s).to_owned()).collect()
}
pub async fn require_success(
    runner: &Runner,
    invocation: &Invocation,
    result: &ProcessObservation,
    stage: &str,
) -> Result<(), PreparationError> {
    let selected = enrichment_store::producer_plan::preparation_result(
        runner.ownership.runtime(),
        invocation,
        result,
        stage,
    )
    .await
    .map_err(|error| PreparationError::Policy(error.to_string()))?;
    match selected.state {
        enrichment_core::execution::producer::PreparationState::Ready => Ok(()),
        enrichment_core::execution::producer::PreparationState::ProcessFailed => Err(
            PreparationError::Process(Box::new(result.clone()), selected.detail),
        ),
        enrichment_core::execution::producer::PreparationState::IdentityMismatch => {
            Err(PreparationError::Environment(selected.detail))
        }
    }
}
