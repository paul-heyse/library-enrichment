//! Complete, independently admitted snapshot bundles with atomic destination publication.
use crate::{
    BlobStore, SnapshotReader, StatePaths,
    admission::AdmissionLimits,
    repository::EvidenceRepository,
    runtime::{QueryLimits, QueryRuntime},
};
use enrichment_core::native_union::NativeStruct;
use enrichment_core::{
    canonical,
    identity::{ContextId, SnapshotId},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Read},
    path::{Component, Path, PathBuf},
};

pub const MANIFEST: &str = "MANIFEST.sha256";
pub const BUNDLE: &str = "bundle.json";
use crate::bundle_plan::{FILE_COUNT, FILE_LIMIT, TEXT_LIMIT, TOTAL_LIMIT};
// Each bounded file path has at most 16 parent components. Includes incomplete writes,
// the staging owner and bundle root. Physical cleanup refuses an unbounded inventory.
pub(crate) const MAX_STAGING_ENTRIES: usize = FILE_COUNT * 17 + 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exported {
    pub root: PathBuf,
    pub context_id: String,
    pub snapshot_id: Option<String>,
    pub artifacts: usize,
    pub files: usize,
}

enrichment_core::native_struct! { pub(crate) struct Description {
    bundle_version: String => enrichment_core::native_union::Rule::Vocabulary(vec![crate::bundle_plan::VERSION.into()]),
    context_id: ContextId => enrichment_core::native_union::Rule::Text,
    snapshot_id: SnapshotId => enrichment_core::native_union::Rule::Text,
    source_control: String => enrichment_core::native_union::Rule::NonEmpty,
    control: String => enrichment_core::native_union::Rule::NonEmpty,
    exported_at: enrichment_core::native_time::ObservationTime => enrichment_core::native_union::Rule::Text
} }

fn error(error: impl std::fmt::Display) -> io::Error {
    io::Error::other(error.to_string())
}

async fn read_descriptor(
    runtime: &QueryRuntime,
    root: std::sync::Arc<crate::immutable_root::ImmutableRoot>,
    path: PathBuf,
) -> io::Result<crate::owned_bytes::OwnedBytes> {
    let pool = runtime.session().runtime_env().memory_pool.clone();
    runtime
        .blocking(move || {
            root.validate()?;
            crate::owned_bytes::OwnedBytes::read_file(&path, TEXT_LIMIT, &pool, "bundle-descriptor")
        })
        .await
        .map_err(error)?
}

fn observe(root: &Path, files: Vec<String>) -> Vec<crate::bundle_plan::Observation> {
    files
        .into_iter()
        .map(|path| {
            let result = (|| {
                let mut options = File::options();
                options.read(true);
                #[cfg(target_os = "linux")]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.custom_flags(0x20000 | 0x800);
                }
                let file = options.open(root.join(&path))?;
                let before = file.metadata()?;
                if !before.is_file() || before.len() > FILE_LIMIT {
                    return Err(io::Error::other(
                        "bundle file is not a bounded regular file",
                    ));
                }
                let (digest, bytes) = canonical::sha256_reader(&file, FILE_LIMIT)?;
                let after = file.metadata()?;
                if before.len() != bytes
                    || before.len() != after.len()
                    || before.modified()? != after.modified()?
                {
                    return Err(io::Error::other("bundle file changed during capture"));
                }
                Ok((digest, bytes))
            })();
            match result {
                Ok((digest, bytes)) => crate::bundle_plan::Observation {
                    path,
                    digest: Some(digest),
                    bytes: Some(bytes),
                    error: None,
                },
                Err(error) => crate::bundle_plan::Observation {
                    path,
                    digest: None,
                    bytes: None,
                    error: Some(error.to_string()),
                },
            }
        })
        .collect()
}

/// Export one selected snapshot and its complete catalog/provenance/blob closure.
/// # Errors
/// A missing blob, failed query, corrupt input or exhausted budget leaves no published bundle.
pub async fn export(paths: &StatePaths, context_id: &str, out: &Path) -> io::Result<Exported> {
    let scratch = std::sync::Arc::new(tempfile::tempdir()?);
    let runtime =
        QueryRuntime::new(&scratch.path().join("spill"), QueryLimits::default()).map_err(error)?;
    let (paths, context_id, out) = (paths.clone(), context_id.to_owned(), out.to_owned());
    let owned = runtime.clone();
    let task_scratch = scratch.clone();
    let result = runtime
        .spawn_root(async move {
            Box::pin(export_owned(
                &paths,
                &context_id,
                &out,
                &owned,
                task_scratch.path(),
            ))
            .await
        })
        .await
        .map_err(error)
        .and_then(std::convert::identity);
    let closed = runtime.close_diagnostics().await.map_err(error);
    finish(result, closed)
}

fn finish<T>(result: io::Result<T>, closed: io::Result<()>) -> io::Result<T> {
    match (result, closed) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(primary), Ok(())) => Err(primary),
        (Ok(_), Err(cleanup)) => Err(cleanup),
        (Err(primary), Err(cleanup)) => Err(error(format!(
            "{primary}; native shutdown also failed: {cleanup}"
        ))),
    }
}

async fn export_owned(
    paths: &StatePaths,
    context_id: &str,
    out: &Path,
    runtime: &QueryRuntime,
    scratch: &Path,
) -> io::Result<Exported> {
    let id = ContextId::try_from(context_id.to_owned()).map_err(error)?;
    match fs::symlink_metadata(out) {
        Ok(meta) if !meta.is_dir() || fs::read_dir(out)?.next().is_some() => {
            return Err(error(
                "bundle destination must be absent or an empty directory",
            ));
        }
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    let parent = out
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)?;
    let retention = crate::retention::RetentionStore::new(
        crate::control::ControlStore::open(&paths.data_root, runtime.clone())?,
        runtime.clone(),
    );
    let staging = crate::PrivateDirectory::export(&retention, runtime, parent.to_owned())
        .await
        .map_err(error)?;
    let root = staging.path().join("bundle");
    let out = staging
        .path()
        .parent()
        .ok_or_else(|| error("export parent"))?
        .join(
            out.file_name()
                .ok_or_else(|| error("export destination name"))?,
        );
    fs::create_dir(&root)?;
    crate::task_context::InputContext::owned(staging.clone())
        .scope(export_staged(
            paths, id, &out, runtime, scratch, root, staging,
        ))
        .await
}

async fn build_export(
    paths: &StatePaths,
    id: &ContextId,
    root: &Path,
    runtime: &QueryRuntime,
    staging: std::sync::Arc<crate::PrivateDirectory>,
) -> io::Result<(SnapshotId, usize, String)> {
    let repository = EvidenceRepository::new(
        paths.clone(),
        runtime.clone(),
        Default::default(),
        AdmissionLimits::default(),
    )
    .map_err(error)?;
    let catalog = repository.catalog.pin().await.map_err(error)?;
    let snapshot = catalog
        .current(runtime, id)
        .await
        .map_err(error)?
        .ok_or_else(|| error("context has no selected snapshot"))?;
    let snapshots = catalog
        .comparison_closure(runtime, &snapshot)
        .await
        .map_err(error)?;
    fs::create_dir_all(root.join("data"))?;
    crate::leases::initialize(&root.join("data"))?;
    let blobs = BlobStore::read_only(&paths.data_root)?;
    fs::create_dir_all(root.join("data/blobs/sha256"))?;
    let mut copy_inputs = Vec::new();
    let source_roots = snapshots
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let native =
        crate::delta_evidence::EvidenceTables::new(&root.join("data/delta"), runtime.clone())
            .map_err(error)?;
    let mut rebound = Vec::new();
    let mut projections = Vec::new();
    let search =
        crate::search_projection::SearchProjection::new(&root.join("data/delta"), runtime.clone())
            .map_err(error)?;
    let retention = crate::retention::RetentionStore::new(
        crate::control::ControlStore::open(&root.join("data"), runtime.clone())?,
        runtime.clone(),
    );
    let mut obligations = Vec::new();
    for input_snapshot in snapshots {
        let reader = SnapshotReader::open(&repository, catalog.clone(), &input_snapshot)
            .await
            .map_err(error)?;
        if input_snapshot == snapshot && reader.manifest().context_id != *id {
            return Err(error("snapshot does not belong to this context"));
        }
        let mut publication = reader.manifest().clone();
        let mut tables = Vec::new();
        let cohort = enrichment_core::identity::CohortId::new();
        for binding in &publication.tables {
            let relation = crate::admission::Relation::ALL
                .into_iter()
                .find(|r| r.name() == binding.relation)
                .ok_or_else(|| error("unknown exported relation"))?;
            tables.push(
                native
                    .append(
                        relation,
                        &cohort,
                        reader
                            .session()
                            .table(relation.reference())
                            .await
                            .map_err(error)?,
                        binding.rows,
                    )
                    .await
                    .map_err(error)?,
            );
        }
        publication.tables = tables;
        let opened = repository
            .open_snapshot(catalog.clone(), &input_snapshot)
            .await
            .map_err(error)?;
        let full = opened
            .binding
            .research_session(runtime, None)
            .await
            .map_err(error)?;
        let prepared = search
            .prepare(&publication, &full, None, true, &retention)
            .await
            .map_err(error)?;
        projections.push(prepared.checkpoint);
        obligations.push(prepared.obligation);
        rebound.push(enrichment_core::evidence::catalog::SnapshotEntry {
            snapshot_id: publication.snapshot_id.clone(),
            context_id: publication.context_id.clone(),
            publication,
        });
        copy_inputs.push(
            reader
                .session()
                .table("snapshot.evidence.input_artifacts")
                .await
                .map_err(error)?,
        );
        // Attempt logs are operational outputs, outside semantic snapshot identity, but are
        // mandatory offline provenance with exact acquisition descriptors and retained bytes.
        let mut operational = reader.attempt_logs().await.map_err(error)?;
        operational.extend(
            catalog
                .result_artifacts(
                    runtime,
                    &blobs,
                    &reader.job_deliveries().await.map_err(error)?,
                )
                .await
                .map_err(error)?,
        );
        copy_inputs.push(
            crate::native_catalog::batch(
                &runtime.session(),
                "bundle",
                enrichment_core::evidence::Artifact::batch(&operational).map_err(error)?,
            )
            .map_err(error)?,
        );
    }
    let copies = crate::bundle_plan::copies(runtime, copy_inputs)
        .await
        .map_err(error)?;
    let artifact_count = copies.len();
    // Enrollment precedes the physical copy and survives cancellation of this waiter.
    // Root removal can refuse a new chunk but cannot revoke already admitted readers.
    for chunk in copies.chunks(1024) {
        let guard = repository
            .retention()
            .enroll_rooted(
                format!("export/{}", snapshot),
                crate::retention::ProtectionKind::Export,
                chunk
                    .iter()
                    .map(|copy| crate::retention::Dependency::Artifact {
                        artifact_id: copy.artifact_id.clone(),
                    })
                    .collect(),
                &source_roots,
            )
            .await
            .map_err(error)?;
        let copies = chunk.to_vec();
        let source = blobs.clone();
        let destination = root.to_owned();
        let owner = staging.clone();
        runtime
            .blocking(move || {
                let _guard = guard;
                owner.export_directory(owner.path())?;
                for copy in copies {
                    let target = destination
                        .join("data/blobs/sha256")
                        .join(&copy.sha256[..2])
                        .join(&copy.sha256);
                    fs::create_dir_all(
                        target
                            .parent()
                            .ok_or_else(|| error("blob destination parent"))?,
                    )?;
                    copy_exact(
                        &source.path_for(&copy.sha256),
                        &target,
                        &copy.sha256,
                        copy.size_bytes,
                    )?;
                }
                Ok::<_, io::Error>(())
            })
            .await
            .map_err(error)??;
    }
    catalog
        .export_snapshot(
            runtime,
            &snapshot,
            &root.join("data"),
            &rebound,
            &projections,
        )
        .await
        .map_err(error)?;
    for obligation in &obligations {
        retention.settle_selected(obligation).await.map_err(error)?;
    }
    Ok((snapshot, artifact_count, catalog.identity().into()))
}

async fn export_staged(
    paths: &StatePaths,
    id: ContextId,
    out: &Path,
    runtime: &QueryRuntime,
    scratch: &Path,
    root: PathBuf,
    staging: std::sync::Arc<crate::PrivateDirectory>,
) -> io::Result<Exported> {
    let (snapshot, artifact_count, source_control) =
        build_export(paths, &id, &root, runtime, staging.clone()).await?;
    // All mutable target consumers have left scope. Join their final writes before taking
    // the control identity and immutable manifest. Read-only verification adds no writes.
    runtime.flush_releases().await.map_err(error)?;
    crate::immutable_root::ImmutableRoot::seal(&root.join("data"))?;
    let immutable = crate::immutable_root::ImmutableRoot::open(&root.join("data"))?;
    let control = crate::control::ControlStore::immutable(immutable, runtime.clone())?
        .pin()
        .await
        .map_err(error)?
        .identity()
        .to_owned();
    let description = Description {
        bundle_version: crate::bundle_plan::VERSION.into(),
        context_id: id.clone(),
        snapshot_id: snapshot.clone(),
        source_control,
        control,
        exported_at: enrichment_core::native_time::ObservationTime::now().map_err(error)?,
    };
    fs::write(root.join(BUNDLE), serde_json::to_vec_pretty(&description)?)?;
    let directory = root.clone();
    let owner = staging.clone();
    let observed = runtime
        .blocking(move || {
            owner.export_directory(owner.path())?;
            Ok::<_, io::Error>(observe(&directory, inventory(&directory)?))
        })
        .await
        .map_err(error)??;
    let checksums = crate::bundle_plan::manifest(runtime, &observed)
        .await
        .map_err(error)?;
    fs::write(root.join(MANIFEST), checksums)?;
    let problems = verify_owned(&root, runtime, scratch).await?;
    if !problems.is_empty() {
        return Err(error(format!(
            "unpublished bundle failed verification: {}",
            problems.join("; ")
        )));
    }
    let published_root = root.clone();
    let destination = out.to_owned();
    runtime
        .blocking(move || {
            // Cancellation cannot drop this staging owner while synchronization/rename runs.
            staging.export_directory(staging.path())?;
            sync_tree(&published_root)?;
            fs::rename(&published_root, &destination)?;
            File::open(
                destination
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .unwrap_or(Path::new(".")),
            )?
            .sync_all()
        })
        .await
        .map_err(error)??;
    Ok(Exported {
        root: out.to_path_buf(),
        context_id: id.to_string(),
        snapshot_id: Some(snapshot.to_string()),
        artifacts: artifact_count,
        files: observed.len(),
    })
}

fn copy_exact(source: &Path, target: &Path, digest: &str, bytes: u64) -> io::Result<()> {
    if bytes > FILE_LIMIT || !fs::symlink_metadata(source)?.is_file() {
        return Err(error("bundle input is not a bounded regular file"));
    }
    let mut destination = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(target)?;
    let copied = io::copy(&mut File::open(source)?.take(bytes + 1), &mut destination)?;
    destination.sync_all()?;
    let (actual, length) = canonical::sha256_reader(File::open(target)?, bytes)?;
    if copied != bytes || length != bytes || actual != digest {
        return Err(error(
            "bundle input changed or disagrees with its declared digest",
        ));
    }
    Ok(())
}

/// Verify physical inventory and hashes, then independently admit catalog, all typed relations,
/// semantic identities and the full artifact closure using only the bundle's contents.
/// # Errors
/// Unsafe paths, excessive bundles and malformed descriptors are explicit errors.
pub async fn verify(root: &Path) -> io::Result<Vec<String>> {
    let scratch = std::sync::Arc::new(tempfile::tempdir()?);
    let runtime =
        QueryRuntime::new(&scratch.path().join("spill"), QueryLimits::default()).map_err(error)?;
    let root = root.to_owned();
    let owned = runtime.clone();
    let task_scratch = scratch.clone();
    let result = runtime
        .spawn_root(async move { Box::pin(verify_owned(&root, &owned, task_scratch.path())).await })
        .await
        .map_err(error)
        .and_then(std::convert::identity);
    let closed = runtime.close_diagnostics().await.map_err(error);
    finish(result, closed)
}

async fn verify_owned(
    root: &Path,
    runtime: &QueryRuntime,
    scratch: &Path,
) -> io::Result<Vec<String>> {
    let immutable = crate::immutable_root::ImmutableRoot::open(&root.join("data"))?;
    let text = read_descriptor(runtime, immutable.clone(), root.join(MANIFEST)).await?;
    let expected =
        crate::bundle_plan::checksums(runtime, std::str::from_utf8(&text).map_err(error)?)
            .await
            .map_err(error)?;
    let held = immutable.clone();
    let directory = root.to_owned();
    let observed = runtime
        .blocking(move || {
            held.validate()?;
            Ok::<_, io::Error>(observe(&directory, inventory(&directory)?))
        })
        .await
        .map_err(error)??;
    let problems = crate::bundle_plan::problems(runtime, &expected, &observed)
        .await
        .map_err(error)?;
    if !problems.is_empty() {
        return Ok(problems.into_iter().map(|problem| problem.text).collect());
    }
    let description: Description = serde_json::from_slice(
        &read_descriptor(runtime, immutable.clone(), root.join(BUNDLE)).await?,
    )?;
    let repository = EvidenceRepository::immutable(
        immutable,
        StatePaths {
            data_root: root.join("data"),
            cache_root: scratch.to_path_buf(),
        },
        runtime.clone(),
        AdmissionLimits::default(),
    )
    .map_err(error)?;
    let catalog = repository.catalog.pin().await.map_err(error)?;
    crate::bundle_plan::admit_catalog(
        runtime,
        &catalog.session(runtime).await.map_err(error)?,
        &description,
        catalog.identity(),
    )
    .await
    .map_err(error)?;
    for input in catalog
        .comparison_closure(runtime, &description.snapshot_id)
        .await
        .map_err(error)?
    {
        let opened = repository
            .open_snapshot(catalog.clone(), &input)
            .await
            .map_err(error)?;
        drop(opened);
    }
    Ok(Vec::new())
}

fn safe_path(path: &str) -> io::Result<()> {
    if path.is_empty()
        || path.contains(['\\', '\n', '\r'])
        || Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(error("unsafe bundle path"));
    }
    Ok(())
}

fn inventory(root: &Path) -> io::Result<Vec<String>> {
    fn visit(
        root: &Path,
        at: &Path,
        depth: usize,
        files: &mut Vec<String>,
        bytes: &mut u64,
        path_bytes: &mut usize,
    ) -> io::Result<()> {
        if depth > 16 {
            return Err(error("bundle directory depth exceeds limit"));
        }
        for entry in fs::read_dir(at)? {
            let entry = entry?;
            let metadata = fs::symlink_metadata(entry.path())?;
            if metadata.is_dir() {
                visit(root, &entry.path(), depth + 1, files, bytes, path_bytes)?;
            } else if metadata.is_file() {
                *bytes = bytes
                    .checked_add(metadata.len())
                    .filter(|n| *n <= TOTAL_LIMIT)
                    .ok_or_else(|| error("bundle exceeds total byte limit"))?;
                if files.len() == FILE_COUNT || metadata.len() > FILE_LIMIT {
                    return Err(error("bundle exceeds file count or size limit"));
                }
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .map_err(error)?
                    .to_str()
                    .ok_or_else(|| error("non-UTF8 bundle path"))?
                    .to_owned();
                safe_path(&relative)?;
                *path_bytes = path_bytes
                    .checked_add(relative.len() + 67)
                    .filter(|bytes| *bytes <= TEXT_LIMIT as usize)
                    .ok_or_else(|| error("bundle manifest path byte bound"))?;
                files.push(relative);
            } else {
                return Err(error("bundle contains a symlink or special file"));
            }
        }
        Ok(())
    }
    if !fs::symlink_metadata(root)?.is_dir() {
        return Err(error("bundle root is not a directory"));
    }
    let mut files = Vec::new();
    visit(root, root, 0, &mut files, &mut 0, &mut 0)?;
    files.sort();
    Ok(files)
}

fn sync_tree(root: &Path) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            sync_tree(&entry.path())?;
        } else {
            File::open(entry.path())?.sync_all()?;
        }
    }
    File::open(root)?.sync_all()
}
