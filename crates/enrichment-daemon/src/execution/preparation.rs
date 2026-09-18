//! Physical preparation callbacks retain input, storage and cleanup owners until completion.
use super::{Runner, budget::Preparation, capsule::PreparationError};
use enrichment_store::{owned_bytes::OwnedBytes, preparation_plan};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub async fn read(
    runner: &Runner,
    path: PathBuf,
    limit: u64,
) -> Result<OwnedBytes, PreparationError> {
    let owner = runner
        .input_reader()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    let pool = runner
        .ownership
        .runtime()
        .session()
        .runtime_env()
        .memory_pool
        .clone();
    runner
        .ownership
        .runtime()
        .blocking(move || {
            let _owner = owner;
            OwnedBytes::read_file(&path, limit, &pool, "producer-preparation-input")
        })
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))?
        .map_err(|error| PreparationError::Environment(error.to_string()))
}

pub struct CargoSource {
    pub root: String,
    pub identity: preparation_plan::CargoIdentity,
}

pub async fn cargo_source(
    runner: &Runner,
    storage: Arc<Preparation>,
    root: &Path,
    archive: OwnedBytes,
    package: &str,
    version: Option<&str>,
) -> Result<CargoSource, PreparationError> {
    let owner = runner
        .input_reader()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    let destination = root.join("source");
    let held = storage.clone();
    let (top, files) = runner
        .ownership
        .runtime()
        .blocking(move || -> Result<_, String> {
            let _owner = owner;
            let extracted = enrichment_core::archive::extract_tar_gz(
                archive.as_ref(),
                &destination,
                &held.archive_policy().map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let top = extracted.top_level.ok_or("crate archive lacks one root")?;
            let files = enrichment_core::producer::python::archive::files(&destination.join(&top))?;
            Ok((top, files))
        })
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))??;
    let source = root.join("source").join(&top);
    let manifest = read(
        runner,
        source.join("Cargo.toml"),
        enrichment_core::native_cargo::MAX_BYTES as u64,
    )
    .await?;
    let identity = preparation_plan::cargo_identity(
        runner.ownership.runtime(),
        std::str::from_utf8(&manifest).map_err(|error| error.to_string())?,
        package,
        version,
    )
    .await
    .map_err(|error| error.to_string())?;
    let removals = preparation_plan::cargo_removals(runner.ownership.runtime(), files)
        .await
        .map_err(|error| error.to_string())?;
    let owner = runner
        .input_reader()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    runner
        .ownership
        .runtime()
        .blocking(move || -> std::io::Result<()> {
            let _owner = owner;
            let _storage = storage;
            for removal in removals {
                std::fs::remove_file(source.join(removal.path))?;
            }
            Ok(())
        })
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))?
        .map_err(|error| error.to_string())?;
    Ok(CargoSource {
        root: top,
        identity,
    })
}

/// Extract before native metadata selection; capture is bounded and has no selection policy.
pub async fn wheel_distribution<B: AsRef<[u8]> + Send + Sync + 'static>(
    runner: &Runner,
    storage: Arc<Preparation>,
    destination: PathBuf,
    archive: Arc<B>,
    filename: &str,
    sha256: &str,
) -> Result<enrichment_store::python_distribution::Captured, PreparationError> {
    let owner = runner
        .input_reader()
        .map_err(|error| PreparationError::Policy(error.to_string()))?;
    let storage_owner = storage.clone();
    let captured = destination.clone();
    runner
        .ownership
        .runtime()
        .blocking(move || -> Result<_, String> {
            let _owner = owner;
            enrichment_core::producer::python::archive::extract_zip(
                archive.as_ref().as_ref(),
                &captured,
                &storage
                    .archive_policy()
                    .map_err(|error| error.to_string())?,
            )?;
            Ok(())
        })
        .await
        .map_err(|error| PreparationError::Policy(error.to_string()))??;
    let owner = Arc::new((
        runner
            .input_reader()
            .map_err(|error| PreparationError::Policy(error.to_string()))?,
        storage_owner,
    ));
    enrichment_store::python_distribution::capture(
        runner.ownership.runtime(),
        destination,
        owner,
        filename,
        sha256,
        "",
    )
    .await
    .map_err(|error| PreparationError::Environment(error.to_string()))
}
