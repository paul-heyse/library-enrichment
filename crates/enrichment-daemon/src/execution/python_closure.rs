//! Physical acquisition driver for the native Arrow dependency frontier.
use super::capsule::PreparationError;
use crate::{ops::common::Opened, service::Service};
use enrichment_core::{
    canonical,
    evidence::{Artifact, ArtifactKind},
    identity::Ecosystem,
    producer::python::{
        self, DistributionFile,
        requirements::{MarkerEnvironment, Requirement},
    },
    request::ResolveRequest,
};
use enrichment_store::dependency_plan::{Frontier, Package, Work};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

#[derive(Deserialize)]
struct Index {
    releases: BTreeMap<String, Vec<DistributionFile>>,
}

pub async fn resolve(
    service: &Service,
    opened: &Opened,
    root: &Path,
    filename: &str,
    distribution: &python::Distribution,
    cancel: Arc<AtomicBool>,
    storage: &Arc<super::budget::Preparation>,
    runner: &super::Runner,
) -> Result<(Vec<u8>, String), PreparationError> {
    let deadline = tokio::time::Instant::now()
        + Duration::from_secs(
            service
                .config
                .network
                .acquisition_timeout_seconds
                .clamp(1, 600),
        );
    // The capsule owner admits this Linux CPython image before invoking dependency acquisition.
    let marker_environment = MarkerEnvironment {
        python_full_version: enrichment_core::execution::producer::PYTHON_VERSION.into(),
        implementation_version: enrichment_core::execution::producer::PYTHON_VERSION.into(),
        implementation_name: "cpython".into(),
        os_name: "posix".into(),
        platform_machine: "x86_64".into(),
        platform_system: "Linux".into(),
        platform_python_implementation: "CPython".into(),
        sys_platform: "linux".into(),
    };
    let headers = &distribution.metadata;
    let name = python::normalize_name(&opened.release.key.package);
    let mut frontier = Frontier::new(
        &service.repository.runtime,
        Package {
            name,
            version: opened.release.key.version.clone(),
            filename: filename.into(),
            url: "pinned-source-artifact".into(),
            sha256: opened
                .release
                .key
                .artifact_digest
                .clone()
                .ok_or("source digest missing")?,
            requirements: headers.get("requires-dist").cloned().unwrap_or_default(),
            extras: opened
                .environment
                .features
                .iter()
                .flatten()
                .map(|v| python::normalize_name(v))
                .collect(),
            downloaded_bytes: 0,
        },
    )
    .map_err(|e| e.to_string())?;
    while let Some(work) = frontier.next().await.map_err(|e| e.to_string())? {
        check(&cancel, deadline)?;
        match work {
            Work::Expand {
                name,
                requirements,
                extras,
            } => {
                // Parse every requirement before any URI named by that metadata can be opened.
                // Marker selection and dependency identity composition remain native plans.
                let requirements = requirements
                    .iter()
                    .map(|s| Requirement::parse(s))
                    .collect::<Result<Vec<_>, _>>()?;
                let demands = enrichment_store::dependency_plan::active_demands(
                    &service.repository.runtime,
                    &requirements,
                    &marker_environment,
                    &extras,
                )
                .await
                .map_err(|e| e.to_string())?;
                frontier
                    .expanded(&name, &extras, demands)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            Work::Acquire {
                name,
                specifiers,
                extras,
            } => {
                let url = url::Url::parse(&format!(
                    "{}/{}/json",
                    service
                        .config
                        .producers
                        .python
                        .pypi_url
                        .trim_end_matches('/'),
                    name
                ))
                .map_err(|e| e.to_string())?;
                let response = fetch(service, &url, &cancel, deadline).await?;
                let index: Index =
                    serde_json::from_slice(&response.bytes).map_err(|e| e.to_string())?;
                let artifact = service
                    .blobs
                    .put(&response.bytes, |_| {
                        let mut artifact = Artifact::describe(
                            &response.bytes,
                            ArtifactKind::RegistryVersionMetadata,
                            "application/json",
                            url.as_str(),
                            response.retrieved_at,
                        );
                        artifact.final_url = (response.final_url != url.as_str())
                            .then(|| response.final_url.clone());
                        artifact.etag = response.etag.clone();
                        artifact.last_modified = response.last_modified.clone();
                        artifact
                    })
                    .map_err(|e| e.to_string())?
                    .acquired;
                let releases = enrichment_store::registry_capture::RegistryStore::new(
                    service.repository.catalog.clone(),
                    service.repository.runtime.clone(),
                )
                .python_files(
                    &artifact,
                    None,
                    python::facts::decode(&index.releases, 1024).map_err(|e| e.to_string())?,
                )
                .await
                .map_err(|e| e.to_string())?;
                let request = ResolveRequest {
                    ecosystem: Ecosystem::Python,
                    name: name.clone(),
                    python_version: Some(marker_environment.python_full_version.clone()),
                    ..ResolveRequest::default()
                };
                let choice = enrichment_store::python_registry::select(
                    &service.repository.runtime,
                    releases,
                    &request,
                    Some(&specifiers),
                    true,
                )
                .await
                .map_err(|e| e.to_string())?
                .ok_or("no eligible dependency wheel for the declared environment")?;
                let version = choice.version;
                let file = choice.file;
                let sha = file
                    .digests
                    .get("sha256")
                    .ok_or("dependency wheel hash missing")?
                    .to_ascii_lowercase();
                if file.filename.contains(['/', '\\', ':', '%']) || !file.filename.ends_with(".whl")
                {
                    return Err("unsafe dependency wheel filename".into());
                }
                let url = url::Url::parse(&file.url).map_err(|e| e.to_string())?;
                let bytes = fetch(service, &url, &cancel, deadline).await?.bytes;
                if canonical::sha256_hex(&bytes) != sha {
                    return Err("dependency wheel digest differs from registry metadata".into());
                }
                let inspect = root.join("dependency-metadata").join(&sha);
                let bytes = Arc::new(bytes);
                let metadata = super::preparation::wheel_distribution(
                    runner,
                    storage.clone(),
                    inspect.clone(),
                    bytes.clone(),
                    &file.filename,
                    &sha,
                )
                .await?;
                enrichment_store::python_distribution::admit(
                    &service.repository.runtime,
                    &metadata.distribution,
                    metadata.texts(),
                    &request,
                    &version,
                )
                .await
                .map_err(|error| error.to_string())?;
                let headers = &metadata.distribution.metadata;
                let requires = headers.get("requires-dist").cloned().unwrap_or_default();
                for text in &requires {
                    Requirement::parse(text)?;
                }
                storage
                    .write(
                        &root.join("wheelhouse").join(&file.filename),
                        bytes.as_ref(),
                    )
                    .map_err(|e| e.to_string())?;
                frontier
                    .acquired(Package {
                        name,
                        version,
                        filename: file.filename,
                        url: file.url,
                        sha256: sha,
                        requirements: requires,
                        extras,
                        downloaded_bytes: bytes.len() as u64,
                    })
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    frontier.finish().await.map_err(|e| e.to_string().into())
}

fn check(cancel: &AtomicBool, deadline: tokio::time::Instant) -> Result<(), PreparationError> {
    if cancel.load(Ordering::Acquire) {
        return Err(PreparationError::Cancelled);
    }
    if tokio::time::Instant::now() >= deadline {
        return Err("dependency preparation deadline exceeded".into());
    }
    Ok(())
}
async fn cancelled(cancel: &AtomicBool) {
    while !cancel.load(Ordering::Acquire) {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}
async fn fetch(
    service: &Service,
    url: &url::Url,
    cancel: &AtomicBool,
    deadline: tokio::time::Instant,
) -> Result<crate::fetch::Fetched, PreparationError> {
    check(cancel, deadline)?;
    let response = tokio::select! {
        _ = cancelled(cancel) => return Err(PreparationError::Cancelled),
        _ = tokio::time::sleep_until(deadline) => return Err("dependency preparation deadline exceeded".into()),
        response = service.fetcher.get(url, None) => response.map_err(|e| e.to_string())?,
    };
    if response.status != 200 {
        return Err(format!("dependency endpoint returned HTTP {}", response.status).into());
    }
    Ok(response)
}
