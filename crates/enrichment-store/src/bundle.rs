//! Complete, independently admitted snapshot bundles with atomic destination publication.
use crate::{
    BlobStore, SnapshotReader, StatePaths,
    admission::AdmissionLimits,
    projection,
    repository::EvidenceRepository,
    runtime::{QueryLimits, QueryRuntime},
};
use datafusion::prelude::col;
use enrichment_core::{
    canonical,
    identity::{ContextId, SnapshotId},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, Read},
    path::{Component, Path, PathBuf},
};

pub const MANIFEST: &str = "MANIFEST.sha256";
pub const BUNDLE: &str = "bundle.json";
const FILE_LIMIT: u64 = 256 * 1024 * 1024;
const TOTAL_LIMIT: u64 = 4 * 1024 * 1024 * 1024;
const FILE_COUNT: usize = 16384;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exported {
    pub root: PathBuf,
    pub context_id: String,
    pub snapshot_id: Option<String>,
    pub artifacts: usize,
    pub files: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Description {
    bundle_version: String,
    context_id: ContextId,
    snapshot_id: SnapshotId,
    source_catalog_generation: u64,
    exported_at: String,
}

fn error(error: impl std::fmt::Display) -> io::Error {
    io::Error::other(error.to_string())
}

fn read_small(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|e| error(format!("{}: {e}", path.display())))?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(error("bundle descriptor exceeds byte budget"));
    }
    Ok(bytes)
}

/// Export one selected snapshot and its complete catalog/provenance/blob closure.
/// # Errors
/// A missing blob, failed query, corrupt input or exhausted budget leaves no published bundle.
pub async fn export(paths: &StatePaths, context_id: &str, out: &Path) -> io::Result<Exported> {
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
    let staging = tempfile::Builder::new()
        .prefix(".evidence-bundle-")
        .tempdir_in(parent)?;
    let root = staging.path().join("bundle");
    fs::create_dir(&root)?;
    let scratch = tempfile::tempdir()?;
    let runtime =
        QueryRuntime::new(&scratch.path().join("spill"), QueryLimits::default()).map_err(error)?;
    let repository =
        EvidenceRepository::read_only(paths.clone(), runtime.clone(), AdmissionLimits::default())
            .map_err(error)?;
    let catalog = repository.catalog.pin().await.map_err(error)?;
    let snapshot = catalog
        .current(&runtime, &id)
        .await
        .map_err(error)?
        .ok_or_else(|| error("context has no selected snapshot"))?;
    let reader = SnapshotReader::open(&repository, catalog.clone(), &snapshot)
        .await
        .map_err(error)?;
    if reader.manifest().context_id != id {
        return Err(error("snapshot does not belong to this context"));
    }
    let manifest = reader.manifest();
    let target = root.join("data/snapshots").join(snapshot.as_str());
    fs::create_dir_all(&target)?;
    crate::leases::initialize(&root.join("data"))?;
    let entry = catalog
        .snapshot(&runtime, &snapshot)
        .await
        .map_err(error)?
        .ok_or_else(|| error("catalog membership disappeared"))?;
    copy_exact(
        &reader.dir().join("manifest.json"),
        &target.join("manifest.json"),
        &entry.manifest_digest,
        entry.manifest_bytes,
    )?;
    for table in &manifest.tables {
        copy_exact(
            &reader.dir().join(&table.file),
            &target.join(&table.file),
            &table.sha256,
            table.bytes,
        )?;
    }
    let blobs = BlobStore::read_only(&paths.data_root)?;
    fs::create_dir_all(root.join("data/blobs/sha256"))?;
    let inputs = reader
        .session()
        .table("input_artifacts")
        .await
        .map_err(error)?
        .select(vec![col("sha256"), col("size_bytes")])
        .map_err(error)?
        .distinct()
        .map_err(error)?;
    let mut artifact_count = 0usize;
    let mut artifact_bytes = 0u64;
    runtime
        .visit(inputs, 1_000_000, |batch| {
            let digests = projection::TextColumn::new(
                batch
                    .column_by_name("sha256")
                    .ok_or_else(|| {
                        datafusion::error::DataFusionError::Execution("missing blob digest".into())
                    })?
                    .as_ref(),
            )?;
            let sizes = batch
                .column_by_name("size_bytes")
                .and_then(|a| a.as_any().downcast_ref::<arrow::array::UInt64Array>())
                .ok_or_else(|| {
                    datafusion::error::DataFusionError::Execution("invalid blob sizes".into())
                })?;
            for row in 0..batch.num_rows() {
                let digest = digests.get(row).ok_or_else(|| {
                    datafusion::error::DataFusionError::Execution("null blob digest".into())
                })?;
                artifact_bytes = artifact_bytes
                    .checked_add(sizes.value(row))
                    .filter(|n| *n <= 512 * 1024 * 1024)
                    .ok_or_else(|| {
                        datafusion::error::DataFusionError::ResourcesExhausted(
                            "bundle blob closure exceeds 512 MiB".into(),
                        )
                    })?;
                let target = root
                    .join("data/blobs/sha256")
                    .join(&digest[..2])
                    .join(digest);
                fs::create_dir_all(
                    target
                        .parent()
                        .ok_or_else(|| error("blob destination has no parent"))?,
                )?;
                copy_exact(&blobs.path_for(digest), &target, digest, sizes.value(row))?;
                artifact_count += 1;
            }
            Ok(())
        })
        .await
        .map_err(error)?;
    // Attempt logs are operational outputs, outside semantic snapshot identity, but are
    // mandatory offline provenance with exact acquisition descriptors and retained bytes.
    for log in reader
        .attempt_logs()
        .await
        .map_err(error)?
        .into_iter()
        .chain(reader.job_deliveries().await.map_err(error)?)
    {
        let target = root
            .join("data/blobs/sha256")
            .join(&log.sha256[..2])
            .join(&log.sha256);
        if !target.try_exists()? {
            artifact_bytes = artifact_bytes
                .checked_add(log.size_bytes)
                .filter(|n| *n <= 512 * 1024 * 1024)
                .ok_or_else(|| error("bundle blob closure exceeds 512 MiB"))?;
            fs::create_dir_all(
                target
                    .parent()
                    .ok_or_else(|| error("log destination has no parent"))?,
            )?;
            copy_exact(
                &blobs.path_for(&log.sha256),
                &target,
                &log.sha256,
                log.size_bytes,
            )?;
            artifact_count += 1;
        }
    }
    catalog
        .export_snapshot(&runtime, &snapshot, &root.join("data"))
        .await
        .map_err(error)?;
    let description = Description {
        bundle_version: "typed-evidence-bundle/3".into(),
        context_id: id.clone(),
        snapshot_id: snapshot.clone(),
        source_catalog_generation: catalog.generation(),
        exported_at: enrichment_core::clock::now_rfc3339(),
    };
    fs::write(root.join(BUNDLE), serde_json::to_vec_pretty(&description)?)?;
    let files = inventory(&root)?;
    let mut checksums = String::new();
    for path in &files {
        let (digest, _) = canonical::sha256_reader(File::open(root.join(path))?, FILE_LIMIT)?;
        checksums.push_str(&format!("{digest}  {path}\n"));
    }
    fs::write(root.join(MANIFEST), checksums)?;
    let problems = verify(&root).await?;
    if !problems.is_empty() {
        return Err(error(format!(
            "unpublished bundle failed verification: {}",
            problems.join("; ")
        )));
    }
    sync_tree(&root)?;
    fs::rename(&root, out)?;
    File::open(parent)?.sync_all()?;
    Ok(Exported {
        root: out.to_path_buf(),
        context_id: id.to_string(),
        snapshot_id: Some(snapshot.to_string()),
        artifacts: artifact_count,
        files: files.len(),
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
    let files = inventory(root)?;
    let text =
        String::from_utf8(read_small(&root.join(MANIFEST), 4 * 1024 * 1024)?).map_err(error)?;
    let mut listed = BTreeMap::new();
    for line in text.lines() {
        let (digest, path) = line
            .split_once("  ")
            .ok_or_else(|| error("malformed checksum manifest"))?;
        safe_path(path)?;
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || path == MANIFEST
            || listed.insert(path.to_owned(), digest.to_owned()).is_some()
        {
            return Err(error("invalid or duplicate checksum manifest entry"));
        }
    }
    if listed.is_empty() || listed.len() > FILE_COUNT {
        return Err(error("invalid checksum entry count"));
    }
    let mut problems = Vec::new();
    for (path, expected) in &listed {
        match File::open(root.join(path))
            .and_then(|file| canonical::sha256_reader(file, FILE_LIMIT))
        {
            Ok((actual, _)) if &actual == expected => {}
            Ok(_) => problems.push(format!("{path}: content does not match recorded digest")),
            Err(e) => problems.push(format!("{path}: {e}")),
        }
    }
    for path in files {
        if path != MANIFEST && !listed.contains_key(&path) {
            problems.push(format!("{path}: present but not listed"));
        }
    }
    if !problems.is_empty() {
        return Ok(problems);
    }
    let description: Description =
        serde_json::from_slice(&read_small(&root.join(BUNDLE), 4 * 1024 * 1024)?)?;
    if description.bundle_version != "typed-evidence-bundle/3" {
        return Err(error("unsupported bundle contract"));
    }
    let scratch = tempfile::tempdir()?;
    let runtime =
        QueryRuntime::new(&scratch.path().join("spill"), QueryLimits::default()).map_err(error)?;
    let repository = EvidenceRepository::read_only(
        StatePaths {
            data_root: root.join("data"),
            cache_root: scratch.path().to_path_buf(),
        },
        runtime.clone(),
        AdmissionLimits::default(),
    )
    .map_err(error)?;
    let catalog = repository.catalog.pin().await.map_err(error)?;
    if catalog.generation() != description.source_catalog_generation
        || catalog
            .current(&runtime, &description.context_id)
            .await
            .map_err(error)?
            .as_ref()
            != Some(&description.snapshot_id)
    {
        return Err(error("bundle identity and catalog selection disagree"));
    }
    let opened = repository
        .open_snapshot(catalog, &description.snapshot_id)
        .await
        .map_err(error)?;
    if opened.manifest.context_id != description.context_id {
        return Err(error("bundle context and snapshot disagree"));
    }
    Ok(problems)
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
    ) -> io::Result<()> {
        if depth > 16 {
            return Err(error("bundle directory depth exceeds limit"));
        }
        for entry in fs::read_dir(at)? {
            let entry = entry?;
            let metadata = fs::symlink_metadata(entry.path())?;
            if metadata.is_dir() {
                visit(root, &entry.path(), depth + 1, files, bytes)?;
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
    visit(root, root, 0, &mut files, &mut 0)?;
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
