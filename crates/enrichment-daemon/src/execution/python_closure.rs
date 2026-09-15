//! Bounded Rust-owned registry closure. Metadata is admitted before following each dependency.
use super::capsule::PreparationError;
use crate::{ops::common::Opened, service::Service};
use enrichment_core::{
    canonical,
    identity::Ecosystem,
    producer::python::{self, DistributionFile, requirements::Requirement},
    request::ResolveRequest,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

#[derive(Debug, Clone, Serialize)]
struct Selected {
    name: String,
    version: String,
    filename: String,
    url: String,
    sha256: String,
    requirements: Vec<String>,
    extras: BTreeSet<String>,
}
#[derive(Deserialize)]
struct Index {
    releases: BTreeMap<String, Vec<DistributionFile>>,
}

pub async fn resolve(
    service: &Service,
    opened: &Opened,
    root: &Path,
    filename: &str,
    metadata: &str,
    cancel: Arc<AtomicBool>,
    storage: &super::budget::Preparation,
) -> Result<(Vec<u8>, String), PreparationError> {
    let deadline = tokio::time::Instant::now()
        + Duration::from_secs(
            service
                .config
                .network
                .acquisition_timeout_seconds
                .clamp(1, 600),
        );
    let headers = python::metadata_headers(metadata);
    let name = python::normalize_name(&opened.release.key.package);
    let mut selected = BTreeMap::from([(
        name.clone(),
        Selected {
            name: name.clone(),
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
                .map(|v| python::normalize_name(v))
                .collect(),
        },
    )]);
    let mut work = VecDeque::from([name]);
    let mut rounds = 0usize;
    let mut total_bytes = 0usize;
    while let Some(name) = work.pop_front() {
        check(&cancel, deadline)?;
        rounds += 1;
        if rounds > 1024 || selected.len() > 256 {
            return Err("dependency closure exceeds the package/iteration limit".into());
        }
        let package = selected
            .get(&name)
            .cloned()
            .ok_or("missing queued package")?;
        // Parse all metadata before opening any source named by that metadata. Even inactive
        // URL requirements are not silently admitted as future resolver instructions.
        let requirements = package
            .requirements
            .iter()
            .map(|s| Requirement::parse(s))
            .collect::<Result<Vec<_>, _>>()?;
        let extras: Vec<_> = package.extras.iter().cloned().collect();
        for requirement in requirements {
            check(&cancel, deadline)?;
            if !requirement.applies(&extras)? {
                continue;
            }
            if let Some(existing) = selected.get_mut(&requirement.name) {
                if !requirement.permits(&existing.version) {
                    return Err(format!("dependency constraints conflict with selected {} {}; no unverified environment is published", existing.name, existing.version).into());
                }
                let before = existing.extras.len();
                existing.extras.extend(requirement.extras);
                if existing.extras.len() != before {
                    work.push_back(existing.name.clone());
                }
                continue;
            }
            let url = url::Url::parse(&format!(
                "{}/{}/json",
                service
                    .config
                    .producers
                    .python
                    .pypi_url
                    .trim_end_matches('/'),
                requirement.name
            ))
            .map_err(|e| e.to_string())?;
            let index_bytes = fetch(service, &url, &cancel, deadline).await?;
            let mut index: Index =
                serde_json::from_slice(&index_bytes).map_err(|e| e.to_string())?;
            index.releases.retain(|version, files| {
                files.retain(|f| f.packagetype == "bdist_wheel");
                requirement.permits(version) && !files.is_empty()
            });
            let request = ResolveRequest {
                ecosystem: Ecosystem::Python,
                name: requirement.name.clone(),
                python_version: Some("3.14.7".into()),
                ..ResolveRequest::default()
            };
            let (version, file) = python::select(&index.releases, &request)?;
            let file = file.clone();
            let sha = file
                .digests
                .get("sha256")
                .ok_or("dependency wheel hash missing")?
                .to_ascii_lowercase();
            if file.filename.contains(['/', '\\', ':', '%']) || !file.filename.ends_with(".whl") {
                return Err("unsafe dependency wheel filename".into());
            }
            let url = url::Url::parse(&file.url).map_err(|e| e.to_string())?;
            let bytes = fetch(service, &url, &cancel, deadline).await?;
            total_bytes = total_bytes
                .checked_add(bytes.len())
                .ok_or("dependency byte count overflow")?;
            if total_bytes > 512 * 1024 * 1024 {
                return Err("dependency closure exceeds 512 MiB".into());
            }
            if canonical::sha256_hex(&bytes) != sha {
                return Err("dependency wheel digest differs from registry metadata".into());
            }
            let inspect = root.join("dependency-metadata").join(&sha);
            python::archive::extract_zip(
                &bytes,
                &inspect,
                &storage.archive_policy().map_err(|e| e.to_string())?,
            )?;
            let mut metadata = None;
            for path in python::archive::files(&inspect)? {
                if path.ends_with(".dist-info/METADATA") {
                    if metadata.is_some() {
                        return Err("dependency wheel contains multiple METADATA identities".into());
                    }
                    metadata =
                        Some(fs::read_to_string(inspect.join(path)).map_err(|e| e.to_string())?);
                }
            }
            let metadata = metadata.ok_or("dependency METADATA missing")?;
            let headers = python::metadata_headers(&metadata);
            let header = |key: &str| {
                headers
                    .get(key)
                    .and_then(|v| if v.len() == 1 { v.first() } else { None })
                    .map(String::as_str)
            };
            if header("name").map(python::normalize_name).as_deref()
                != Some(requirement.name.as_str())
                || header("version") != Some(version.as_str())
            {
                return Err(
                    "dependency wheel identity differs from selected registry identity".into(),
                );
            }
            let distribution = python::archive::inventory(&inspect, &file.filename, &sha, "")?;
            python::archive::validate_metadata(&inspect, &distribution, &request, &version)?;
            let requires = headers.get("requires-dist").cloned().unwrap_or_default();
            for text in &requires {
                Requirement::parse(text)?;
            }
            storage
                .write(&root.join("wheelhouse").join(&file.filename), &bytes)
                .map_err(|e| e.to_string())?;
            service
                .blobs
                .put(&bytes, |_| {
                    enrichment_store::blob::describe_local(
                        &bytes,
                        enrichment_core::evidence::ArtifactKind::Other,
                        "application/octet-stream",
                        &file.url,
                        &enrichment_core::clock::now_rfc3339(),
                    )
                })
                .map_err(|e| e.to_string())?;
            let name = requirement.name;
            selected.insert(
                name.clone(),
                Selected {
                    name: name.clone(),
                    version,
                    filename: file.filename,
                    url: file.url,
                    sha256: sha,
                    requirements: requires,
                    extras: requirement.extras.into_iter().collect(),
                },
            );
            work.push_back(name);
        }
    }
    let requirements = selected
        .values()
        .map(|p| format!("{}=={} --hash=sha256:{}\n", p.name, p.version, p.sha256))
        .collect();
    let lock = canonical::to_canonical_string(&serde_json::json!({"resolver":"admitted-registry-closure-1", "python":"3.14.7", "platform":"linux-x86_64", "packages":selected, "limitations":["Conservative pure-wheel selection; conflicting greedy choices are unresolved, never reported as a solved environment."]})).into_bytes();
    Ok((lock, requirements))
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
) -> Result<Vec<u8>, PreparationError> {
    check(cancel, deadline)?;
    let response = tokio::select! {
        _ = cancelled(cancel) => return Err(PreparationError::Cancelled),
        _ = tokio::time::sleep_until(deadline) => return Err("dependency preparation deadline exceeded".into()),
        response = service.fetcher.get(url, None) => response.map_err(|e| e.to_string())?,
    };
    if response.status != 200 {
        return Err(format!("dependency endpoint returned HTTP {}", response.status).into());
    }
    Ok(response.bytes)
}
