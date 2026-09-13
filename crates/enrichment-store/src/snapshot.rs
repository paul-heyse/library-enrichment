//! Immutable snapshot publication (blueprint §8.2).
//!
//! A snapshot is assembled under `staging/`, validated -- every table re-read and its row
//! count compared with the manifest -- and then moved into `snapshots/<id>/` with one
//! `rename`. The context's `current` pointer is swapped afterwards, also with one rename.
//! A reader therefore sees a complete snapshot or none; a crash mid-publication leaves only a
//! staging directory, which is never read as evidence and is swept on the next open.
//!
//! Publishing the same snapshot identity twice is a no-op that keeps the first copy: the
//! identity is content-derived, so the bytes are the same.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use enrichment_core::evidence::{
    EvidenceFragment, Relationship, SnapshotManifest, Symbol, TableRef,
};
use enrichment_core::identity::SnapshotId;

use crate::catalog::{Catalog, write_atomic};
use crate::paths::StatePaths;
use crate::tables;

static STAGING_COUNTER: AtomicU64 = AtomicU64::new(0);

/// The tables of one snapshot, before publication.
#[derive(Debug, Clone, Default)]
pub struct SnapshotTables {
    /// Symbols.
    pub symbols: Vec<Symbol>,
    /// Relationships.
    pub relationships: Vec<Relationship>,
    /// Fragments.
    pub fragments: Vec<EvidenceFragment>,
}

/// Why publication failed.
#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    /// A table could not be encoded.
    #[error("cannot encode the {table} table: {message}")]
    Encode {
        /// Table name.
        table: &'static str,
        /// Encoder message.
        message: String,
    },
    /// A written table did not read back with the expected row count.
    #[error("the {table} table read back {found} rows; {expected} were written")]
    Verify {
        /// Table name.
        table: &'static str,
        /// Rows written.
        expected: u64,
        /// Rows read back.
        found: u64,
    },
    /// Filesystem failure.
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Where a published snapshot lives.
#[must_use]
pub fn snapshot_dir(paths: &StatePaths, id: &SnapshotId) -> PathBuf {
    paths.snapshots().join(id.as_str())
}

/// Whether a snapshot is published (its manifest is present in its final directory).
#[must_use]
pub fn is_published(paths: &StatePaths, id: &SnapshotId) -> bool {
    snapshot_dir(paths, id)
        .join(tables::MANIFEST_FILE)
        .is_file()
}

/// Read a published snapshot's manifest.
///
/// # Errors
///
/// Fails on I/O error or a malformed manifest; an unpublished snapshot is `Ok(None)`.
pub fn read_manifest(paths: &StatePaths, id: &SnapshotId) -> io::Result<Option<SnapshotManifest>> {
    let path = snapshot_dir(paths, id).join(tables::MANIFEST_FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let bytes = fs::read(path)?;
    Ok(Some(serde_json::from_slice(&bytes)?))
}

/// Stage, verify and publish a snapshot, then point the context at it.
///
/// `manifest.tables` and `manifest.counts` are filled in here from what was actually written.
///
/// # Errors
///
/// See [`PublishError`]. On any error the staging directory is removed and nothing under
/// `snapshots/` or `contexts/` has changed.
pub fn publish(
    paths: &StatePaths,
    catalog: &Catalog,
    mut manifest: SnapshotManifest,
    tables_in: &SnapshotTables,
) -> Result<SnapshotManifest, PublishError> {
    let final_dir = snapshot_dir(paths, &manifest.snapshot_id);
    if is_published(paths, &manifest.snapshot_id) {
        // Same identity, same content: keep the first publication and just repoint.
        let existing =
            read_manifest(paths, &manifest.snapshot_id)?.expect("is_published implies a manifest");
        catalog.set_current_snapshot(&manifest.context_id, &manifest.snapshot_id)?;
        return Ok(existing);
    }

    fs::create_dir_all(paths.staging())?;
    fs::create_dir_all(paths.snapshots())?;
    let n = STAGING_COUNTER.fetch_add(1, Ordering::Relaxed);
    let staging = paths.staging().join(format!(
        "{}-{}-{n}",
        manifest.snapshot_id.as_str(),
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&staging);
    fs::create_dir_all(&staging)?;

    let result = stage_tables(&staging, &mut manifest, tables_in);
    if let Err(err) = result {
        let _ = fs::remove_dir_all(&staging);
        return Err(err);
    }

    let manifest_bytes = serde_json::to_vec_pretty(&manifest).map_err(io::Error::from)?;
    write_atomic(&staging.join(tables::MANIFEST_FILE), &manifest_bytes)?;

    match fs::rename(&staging, &final_dir) {
        Ok(()) => {}
        Err(err) => {
            let _ = fs::remove_dir_all(&staging);
            if is_published(paths, &manifest.snapshot_id) {
                // Lost a race to an identical publication.
                catalog.set_current_snapshot(&manifest.context_id, &manifest.snapshot_id)?;
                return Ok(manifest);
            }
            return Err(err.into());
        }
    }
    catalog.set_current_snapshot(&manifest.context_id, &manifest.snapshot_id)?;
    Ok(manifest)
}

fn stage_tables(
    staging: &Path,
    manifest: &mut SnapshotManifest,
    tables_in: &SnapshotTables,
) -> Result<(), PublishError> {
    let metadata = [
        ("schema_version", manifest.schema_version.as_str()),
        ("normalizer_version", manifest.normalizer_version.as_str()),
        ("snapshot_id", manifest.snapshot_id.as_str()),
        ("context_id", manifest.context_id.as_str()),
    ];
    let encode = |table: &'static str| {
        move |e: arrow::error::ArrowError| PublishError::Encode {
            table,
            message: e.to_string(),
        }
    };

    let symbols = tables::symbols_to_batch(&tables_in.symbols).map_err(encode("symbols"))?;
    let relationships = tables::relationships_to_batch(&tables_in.relationships)
        .map_err(encode("relationships"))?;
    let fragments =
        tables::fragments_to_batch(&tables_in.fragments).map_err(encode("fragments"))?;

    for (table, file, batch) in [
        ("symbols", tables::SYMBOLS_FILE, &symbols),
        ("relationships", tables::RELATIONSHIPS_FILE, &relationships),
        ("fragments", tables::FRAGMENTS_FILE, &fragments),
    ] {
        let path = staging.join(file);
        let written = tables::write_parquet(&path, batch, &metadata)?;
        let found = tables::row_count(&tables::read_parquet(&path)?);
        if found != written {
            return Err(PublishError::Verify {
                table,
                expected: written,
                found,
            });
        }
        manifest.tables.insert(
            table.to_owned(),
            TableRef {
                file: file.to_owned(),
                rows: written,
            },
        );
    }
    manifest.counts.symbols = tables_in.symbols.len() as u64;
    manifest.counts.relationships = tables_in.relationships.len() as u64;
    manifest.counts.fragments = tables_in.fragments.len() as u64;
    Ok(())
}

/// Remove staging directories left by a crashed publication. Safe to call at every start:
/// nothing under `staging/` is ever read as evidence.
///
/// # Errors
///
/// Fails on I/O error.
pub fn sweep_staging(paths: &StatePaths) -> io::Result<usize> {
    let staging = paths.staging();
    if !staging.is_dir() {
        return Ok(0);
    }
    let mut removed = 0;
    for entry in fs::read_dir(&staging)? {
        let entry = entry?;
        if entry.path().is_dir() {
            fs::remove_dir_all(entry.path())?;
            removed += 1;
        }
    }
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use enrichment_core::evidence::{ObservedConfiguration, SnapshotCounts, SymbolKind};
    use enrichment_core::identity::{
        Context, Ecosystem, Environment, Release, ReleaseKey, ResearchMode, SnapshotInputs,
    };

    use super::*;

    fn manifest(context_dir_root: &Path) -> (Catalog, SnapshotManifest) {
        let catalog = Catalog::open(context_dir_root).expect("catalog");
        let release = Release::new(ReleaseKey {
            ecosystem: Ecosystem::Rust,
            registry: "crates.io".into(),
            package: "enr-fixture".into(),
            version: "0.2.0".into(),
            artifact_digest: None,
        });
        let environment = Environment::unspecified();
        let context = Context::new(
            release.release_id.clone(),
            environment.environment_id.clone(),
            ResearchMode::Project,
        );
        let inputs = SnapshotInputs {
            schema_version: "1.0".into(),
            normalizer_version: "1".into(),
            context_id: context.context_id.clone(),
            input_digests: BTreeMap::from([("rustdoc_json".to_owned(), "ab".repeat(32))]),
            producers: BTreeMap::new(),
        };
        let manifest = SnapshotManifest {
            snapshot_id: SnapshotId::derive(&inputs),
            schema_version: "1.0".into(),
            normalizer_version: "1".into(),
            context_id: context.context_id,
            release_id: release.release_id,
            environment_id: environment.environment_id,
            crate_name: "enr_fixture".into(),
            crate_version: Some("0.2.0".into()),
            inputs: inputs.input_digests,
            producers: BTreeMap::new(),
            producer_runs: Vec::new(),
            tables: BTreeMap::new(),
            counts: SnapshotCounts::default(),
            observed_configuration: ObservedConfiguration {
                features: vec![],
                all_features: true,
                no_default_features: false,
                target: "x86_64-unknown-linux-gnu".into(),
                format_version: 61,
                source: "test".into(),
            },
            indexed: vec![],
            missing: vec![],
            published_at: "2026-09-13T00:00:00Z".into(),
        };
        (catalog, manifest)
    }

    fn one_symbol() -> Symbol {
        Symbol {
            symbol_id: "sym_1".into(),
            definition_id: "def_1".into(),
            path: "enr_fixture::describe".into(),
            name: "describe".into(),
            kind: SymbolKind::Function,
            parent_path: Some("enr_fixture".into()),
            signature: None,
            doc_summary: None,
            docs: None,
            deprecated: None,
            span_file: None,
            span_line: None,
            is_reexport: false,
            definition_path: "enr_fixture::describe".into(),
            defined_in_crate: "enr_fixture".into(),
            producer_local_id: 1,
            cfg_hints: vec![],
        }
    }

    #[test]
    fn publish_is_atomic_and_repoints_the_context() {
        let dir = tempfile::tempdir().expect("dir");
        let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
        let (catalog, manifest) = manifest(&paths.data_root);
        let tables_in = SnapshotTables {
            symbols: vec![one_symbol()],
            ..SnapshotTables::default()
        };
        let published = publish(&paths, &catalog, manifest.clone(), &tables_in).expect("publish");
        assert_eq!(published.tables["symbols"].rows, 1);
        assert_eq!(published.counts.symbols, 1);
        assert!(is_published(&paths, &manifest.snapshot_id));
        assert_eq!(
            catalog
                .current_snapshot(&manifest.context_id)
                .expect("pointer"),
            Some(manifest.snapshot_id.clone())
        );
        assert!(
            fs::read_dir(paths.staging())
                .expect("staging")
                .next()
                .is_none(),
            "staging is empty after publication"
        );
        // Publishing the same identity again keeps the first copy.
        let again = publish(&paths, &catalog, manifest, &SnapshotTables::default()).expect("again");
        assert_eq!(again.tables["symbols"].rows, 1);
    }

    #[test]
    fn publish_a_half_written_staging_dir_is_swept_not_read() {
        let dir = tempfile::tempdir().expect("dir");
        let paths = StatePaths::explicit(dir.path().join("cache"), dir.path().join("data"));
        let leftover = paths.staging().join("snap_dead-1-0");
        fs::create_dir_all(&leftover).expect("mkdir");
        fs::write(leftover.join("symbols.parquet"), b"partial").expect("write");
        assert_eq!(sweep_staging(&paths).expect("sweep"), 1);
        assert!(!leftover.exists());
        assert!(
            fs::read_dir(paths.snapshots())
                .map(|mut d| d.next().is_none())
                .unwrap_or(true)
        );
    }
}
