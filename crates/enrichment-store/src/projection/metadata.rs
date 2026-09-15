use super::{
    cells::{Row, RowSet, batch, column, invalid, list, optional, record_list, structure, text},
    decode, encode,
};
use arrow::{
    array::{ArrayRef, BooleanArray, Int64Array},
    error::ArrowError,
    record_batch::RecordBatch,
};
use enrichment_core::{
    evidence::metadata::{ReleaseDetails, ReleaseMetadata},
    producer::{
        docsrs::DocsRsMetadata,
        python::{Distribution, ObservationOrigin, WorkerFile, inventory::Entry},
    },
};
use std::{collections::BTreeMap, sync::Arc};

fn boolean(values: impl IntoIterator<Item = bool>) -> ArrayRef {
    Arc::new(BooleanArray::from_iter(values.into_iter().map(Some)))
}

/// Encode retained metadata as native structs and lists. No response JSON is persisted.
/// # Errors
/// Invalid metadata or nested Arrow shapes prevent publication.
pub fn encode(rows: &[ReleaseMetadata]) -> Result<RecordBatch, ArrowError> {
    for row in rows {
        row.validate().map_err(invalid)?;
    }
    let docs: Vec<_> = rows
        .iter()
        .map(|r| match &r.details {
            ReleaseDetails::RustDocs(v) => v.clone(),
            ReleaseDetails::PythonDistribution(_) => DocsRsMetadata::default(),
        })
        .collect();
    let python: Vec<_> = rows
        .iter()
        .map(|r| match &r.details {
            ReleaseDetails::PythonDistribution(v) => v.clone(),
            ReleaseDetails::RustDocs(_) => Distribution::default(),
        })
        .collect();
    let docs_array = structure(
        vec![
            column(
                "declared",
                boolean(docs.iter().map(|d| d.declared)),
                false,
                "declaration-present",
            ),
            column(
                "features",
                list(docs.iter().map(|d| d.features.as_slice())),
                false,
                "feature-names",
            ),
            column(
                "all_features",
                boolean(docs.iter().map(|d| d.all_features)),
                false,
                "all-features",
            ),
            column(
                "no_default_features",
                boolean(docs.iter().map(|d| d.no_default_features)),
                false,
                "no-default-features",
            ),
            column(
                "default_target",
                text(docs.iter().map(|d| d.default_target.as_str())),
                false,
                "target",
            ),
            column(
                "targets_known",
                boolean(docs.iter().map(|d| d.targets.is_some())),
                false,
                "target-selection-known",
            ),
            column(
                "targets",
                list(docs.iter().map(|d| d.targets.as_deref().unwrap_or(&[]))),
                false,
                "targets",
            ),
            column(
                "additional_targets",
                list(docs.iter().map(|d| d.additional_targets.as_slice())),
                false,
                "additional-targets",
            ),
            column(
                "rustc_args",
                list(docs.iter().map(|d| d.rustc_args.as_slice())),
                false,
                "ordered-rustc-arguments",
            ),
            column(
                "rustdoc_args",
                list(docs.iter().map(|d| d.rustdoc_args.as_slice())),
                false,
                "ordered-rustdoc-arguments",
            ),
            column(
                "cargo_args",
                list(docs.iter().map(|d| d.cargo_args.as_slice())),
                false,
                "ordered-cargo-arguments",
            ),
        ],
        Some(
            rows.iter()
                .map(|r| matches!(r.details, ReleaseDetails::RustDocs(_)))
                .collect(),
        ),
    )?;
    let files: Vec<_> = python.iter().flat_map(|p| &p.files).collect();
    let files_array = record_list(
        python.iter().map(|p| p.files.len()),
        structure(
            vec![
                column(
                    "file",
                    text(files.iter().map(|f| f.file.as_str())),
                    false,
                    "archive-member",
                ),
                column(
                    "module",
                    text(files.iter().map(|f| f.module.as_str())),
                    false,
                    "python-import-path",
                ),
                column(
                    "origin",
                    text(files.iter().map(|f| match f.origin {
                        ObservationOrigin::Source => "source",
                        ObservationOrigin::Stub => "stub",
                    })),
                    false,
                    "vocabulary:python-origin/1",
                ),
            ],
            None,
        )?,
    )?;
    let headers: Vec<_> = python.iter().flat_map(|p| &p.metadata).collect();
    let headers_array = record_list(
        python.iter().map(|p| p.metadata.len()),
        structure(
            vec![
                column(
                    "key",
                    text(headers.iter().map(|(k, _)| k.as_str())),
                    false,
                    "metadata-header",
                ),
                column(
                    "values",
                    list(headers.iter().map(|(_, v)| v.as_slice())),
                    false,
                    "ordered-header-values",
                ),
            ],
            None,
        )?,
    )?;
    let entries: Vec<_> = python.iter().flat_map(|p| &p.inventory).collect();
    let inventory_array = record_list(
        python.iter().map(|p| p.inventory.len()),
        structure(
            vec![
                column(
                    "name",
                    text(entries.iter().map(|e| e.name.as_str())),
                    false,
                    "inventory-name",
                ),
                column(
                    "role",
                    text(entries.iter().map(|e| e.role.as_str())),
                    false,
                    "inventory-domain-role",
                ),
                column(
                    "priority",
                    Arc::new(Int64Array::from_iter_values(
                        entries.iter().map(|e| i64::from(e.priority)),
                    )),
                    false,
                    "inventory-priority",
                ),
                column(
                    "uri",
                    text(entries.iter().map(|e| e.uri.as_str())),
                    false,
                    "source-uri",
                ),
                column(
                    "display",
                    text(entries.iter().map(|e| e.display.as_str())),
                    false,
                    "display",
                ),
            ],
            None,
        )?,
    )?;
    let python_array = structure(
        vec![
            column(
                "filename",
                text(python.iter().map(|p| p.filename.as_str())),
                false,
                "artifact-filename",
            ),
            column(
                "sha256",
                text(python.iter().map(|p| p.sha256.as_str())),
                false,
                "sha256",
            ),
            column(
                "name",
                optional(python.iter().map(|p| p.name.as_deref())),
                true,
                "distribution-name",
            ),
            column(
                "version",
                optional(python.iter().map(|p| p.version.as_deref())),
                true,
                "distribution-version",
            ),
            column(
                "import_roots",
                list(python.iter().map(|p| p.import_roots.as_slice())),
                false,
                "import-roots",
            ),
            column("files", files_array, false, "distribution-files"),
            column(
                "native_files",
                list(python.iter().map(|p| p.native_files.as_slice())),
                false,
                "native-file-paths",
            ),
            column(
                "typed_markers",
                list(python.iter().map(|p| p.typed_markers.as_slice())),
                false,
                "typing-markers",
            ),
            column("metadata", headers_array, false, "distribution-metadata"),
            column(
                "entry_points",
                optional(python.iter().map(|p| p.entry_points.as_deref())),
                true,
                "entry-point-declarations",
            ),
            column(
                "worker_artifact_id",
                optional(python.iter().map(|p| p.worker_artifact_id.as_deref())),
                true,
                "ref:artifact",
            ),
            column(
                "source_root",
                text(python.iter().map(|p| p.source_root.as_str())),
                false,
                "archive-source-root",
            ),
            column(
                "inventory",
                inventory_array,
                false,
                "documentation-navigation",
            ),
        ],
        Some(
            rows.iter()
                .map(|r| matches!(r.details, ReleaseDetails::PythonDistribution(_)))
                .collect(),
        ),
    )?;
    batch(
        "release_metadata",
        vec![
            column(
                "metadata_id",
                text(rows.iter().map(|r| r.metadata_id.as_str())),
                false,
                "key:release-metadata",
            ),
            column(
                "release_id",
                text(rows.iter().map(|r| r.release_id.as_str())),
                false,
                "ref:release",
            ),
            column(
                "kind",
                text(rows.iter().map(|r| match r.details {
                    ReleaseDetails::RustDocs(_) => "rust_docs",
                    ReleaseDetails::PythonDistribution(_) => "python_distribution",
                })),
                false,
                "vocabulary:release-metadata/1",
            ),
            column(
                "rust_docs",
                docs_array,
                true,
                "rust-documentation-configuration",
            ),
            column(
                "python_distribution",
                python_array,
                true,
                "python-distribution-metadata",
            ),
            column(
                "source",
                encode::source(&rows.iter().map(|r| &r.source).collect::<Vec<_>>())?,
                false,
                "qualified-source",
            ),
        ],
    )
}

fn docs(r: Row<'_>) -> Result<DocsRsMetadata, ArrowError> {
    let targets = r.list("targets")?;
    let targets_known = r.boolean("targets_known")?;
    if !targets_known && !targets.is_empty() {
        return Err(invalid("unknown targets carry a selection"));
    }
    Ok(DocsRsMetadata {
        declared: r.boolean("declared")?,
        features: r.list("features")?,
        all_features: r.boolean("all_features")?,
        no_default_features: r.boolean("no_default_features")?,
        default_target: r.text("default_target")?.into(),
        targets: targets_known.then_some(targets),
        additional_targets: r.list("additional_targets")?,
        rustc_args: r.list("rustc_args")?,
        rustdoc_args: r.list("rustdoc_args")?,
        cargo_args: r.list("cargo_args")?,
    })
}

fn distribution(r: Row<'_>) -> Result<Distribution, ArrowError> {
    let mut metadata = BTreeMap::new();
    for header in r.records("metadata")? {
        if metadata
            .insert(header.text("key")?.to_owned(), header.list("values")?)
            .is_some()
        {
            return Err(invalid("duplicate metadata header"));
        }
    }
    let files = r
        .records("files")?
        .into_iter()
        .map(|f| {
            Ok(WorkerFile {
                file: f.text("file")?.into(),
                module: f.text("module")?.into(),
                origin: match f.text("origin")? {
                    "source" => ObservationOrigin::Source,
                    "stub" => ObservationOrigin::Stub,
                    _ => return Err(invalid("unknown file origin")),
                },
            })
        })
        .collect::<Result<_, ArrowError>>()?;
    let inventory = r
        .records("inventory")?
        .into_iter()
        .map(|e| {
            Ok(Entry {
                name: e.text("name")?.into(),
                role: e.text("role")?.into(),
                priority: i32::try_from(e.signed("priority")?)
                    .map_err(|e| invalid(e.to_string()))?,
                uri: e.text("uri")?.into(),
                display: e.text("display")?.into(),
            })
        })
        .collect::<Result<_, ArrowError>>()?;
    Ok(Distribution {
        filename: r.text("filename")?.into(),
        sha256: r.text("sha256")?.into(),
        name: r.optional_text("name")?.map(str::to_owned),
        version: r.optional_text("version")?.map(str::to_owned),
        import_roots: r.list("import_roots")?,
        files,
        native_files: r.list("native_files")?,
        typed_markers: r.list("typed_markers")?,
        metadata,
        entry_points: r.owned("entry_points")?,
        worker_artifact_id: r.owned("worker_artifact_id")?,
        source_root: r.text("source_root")?.into(),
        inventory,
    })
}

/// # Errors
/// Invalid tags, duplicate headers, nulls, paths and semantic IDs fail explicitly.
pub fn decode(batch: &RecordBatch) -> Result<Vec<ReleaseMetadata>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|i| {
            let r = rows.row(i);
            let details = match r.text("kind")? {
                "rust_docs" if r.optional_struct("python_distribution")?.is_none() => {
                    ReleaseDetails::RustDocs(docs(r.structure("rust_docs")?)?)
                }
                "python_distribution" if r.optional_struct("rust_docs")?.is_none() => {
                    ReleaseDetails::PythonDistribution(distribution(
                        r.structure("python_distribution")?,
                    )?)
                }
                _ => return Err(invalid("invalid release metadata variant")),
            };
            let value = ReleaseMetadata {
                metadata_id: r.text("metadata_id")?.into(),
                release_id: r.text("release_id")?.into(),
                details,
                source: decode::source(r.structure("source")?)?,
            };
            value.validate().map_err(invalid)?;
            Ok(value)
        })
        .collect()
}
