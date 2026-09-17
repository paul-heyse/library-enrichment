// Catalog domains remain typed Arrow fields, including optional environment knowledge.

use super::cells::{RowSet, batch, column, invalid, list, optional, text};
use arrow::{
    array::{BooleanArray, UInt64Array},
    error::ArrowError,
    record_batch::RecordBatch,
};
use enrichment_core::native_union::NativeStruct;
use enrichment_core::{
    evidence::Artifact,
    evidence::catalog::{
        ComparisonPublication, JobPublication, SnapshotAttempt, SnapshotSelection,
    },
    identity::{
        Context, Ecosystem, Environment, EnvironmentResolution, Release, ReleaseKey, ReleaseLinks,
        ResearchMode,
    },
};
use std::sync::Arc;

macro_rules! publication_codec {
    ($kind:ty, $encode:ident, $decode:ident) => {
        pub fn $encode(rows: &[$kind]) -> Result<RecordBatch, ArrowError> {
            for row in rows {
                row.validate().map_err(invalid)?;
            }
            <$kind>::batch(rows)
        }
        pub fn $decode(batch: &RecordBatch) -> Result<Vec<$kind>, ArrowError> {
            let rows = RowSet::batch(batch)?;
            (0..batch.num_rows())
                .map(|i| {
                    let row = <$kind as NativeStruct>::decode(rows.row(i))?;
                    row.validate().map_err(invalid)?;
                    Ok(row)
                })
                .collect()
        }
    };
}
publication_codec!(
    JobPublication,
    job_publications,
    job_publications_from_batch
);
publication_codec!(
    ComparisonPublication,
    comparison_publications,
    comparison_publications_from_batch
);

/// Decode only native-selected acquisition descriptors after filtering and deduplication.
pub(crate) fn selected_artifacts(batch: &RecordBatch) -> Result<Vec<Artifact>, ArrowError> {
    let rows = RowSet::batch(batch)?;
    (0..batch.num_rows())
        .map(|index| super::acquisitions::decode_one(rows.row(index).structure("artifact")?))
        .collect()
}

macro_rules! required {
    ($rows:expr, $name:literal, $role:literal, $value:expr) => {
        column($name, text($rows.iter().map($value)), false, $role)
    };
}
macro_rules! nullable {
    ($rows:expr, $name:literal, $role:literal, $value:expr) => {
        column($name, optional($rows.iter().map($value)), true, $role)
    };
}

/// # Errors
/// Inconsistent identities or Arrow column shapes are rejected.
pub fn releases(rows: &[Release]) -> Result<RecordBatch, ArrowError> {
    for r in rows {
        if r.release_id != r.key.id() {
            return Err(invalid("release identity disagrees with key"));
        }
    }
    batch(
        "catalog_releases",
        vec![
            required!(rows, "release_id", "key:release", |r| r.release_id.as_str()),
            required!(
                rows,
                "ecosystem",
                "vocabulary:ecosystem/1",
                |r| match r.key.ecosystem {
                    Ecosystem::Rust => "rust",
                    Ecosystem::Python => "python",
                }
            ),
            required!(rows, "registry", "registry", |r| r.key.registry.as_str()),
            required!(rows, "package", "package-name", |r| r.key.package.as_str()),
            required!(rows, "version", "exact-version", |r| r.key.version.as_str()),
            nullable!(rows, "artifact_digest", "sha256", |r| r
                .key
                .artifact_digest
                .as_deref()),
            nullable!(rows, "lib_name", "library-target", |r| r
                .lib_name
                .as_deref()),
            nullable!(rows, "root_module", "root-module", |r| r
                .root_module
                .as_deref()),
            nullable!(rows, "repository", "source-uri", |r| r
                .links
                .repository
                .as_deref()),
            nullable!(rows, "documentation", "source-uri", |r| r
                .links
                .documentation
                .as_deref()),
            nullable!(rows, "homepage", "source-uri", |r| r
                .links
                .homepage
                .as_deref()),
            nullable!(rows, "license", "spdx-expression", |r| r.license.as_deref()),
            nullable!(rows, "rust_version", "rust-version", |r| r
                .rust_version
                .as_deref()),
            nullable!(rows, "published_at", "observed-timestamp", |r| r
                .published_at
                .as_deref()),
            column(
                "yanked",
                Arc::new(BooleanArray::from_iter(rows.iter().map(|r| Some(r.yanked)))),
                false,
                "registry-yanked",
            ),
        ],
    )
}

/// # Errors
/// Required/null/domain/identity violations are rejected.
pub fn releases_from_batch(input: &RecordBatch) -> Result<Vec<Release>, ArrowError> {
    let columns = RowSet::batch(input)?;
    (0..input.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = Release {
                release_id: r
                    .text("release_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                key: ReleaseKey {
                    ecosystem: match r.text("ecosystem")? {
                        "rust" => Ecosystem::Rust,
                        "python" => Ecosystem::Python,
                        _ => return Err(invalid("unknown ecosystem")),
                    },
                    registry: r.text("registry")?.into(),
                    package: r.text("package")?.into(),
                    version: r.text("version")?.into(),
                    artifact_digest: r.owned("artifact_digest")?,
                },
                lib_name: r.owned("lib_name")?,
                root_module: r.owned("root_module")?,
                links: ReleaseLinks {
                    repository: r.owned("repository")?,
                    documentation: r.owned("documentation")?,
                    homepage: r.owned("homepage")?,
                },
                license: r.owned("license")?,
                rust_version: r.owned("rust_version")?,
                published_at: r.owned("published_at")?,
                yanked: r.boolean("yanked")?,
            };
            if value.release_id != value.key.id() {
                return Err(invalid("release identity disagrees with key"));
            }
            Ok(value)
        })
        .collect()
}

/// # Errors
/// Environment fields must agree with the content identity.
pub fn environments(rows: &[Environment]) -> Result<RecordBatch, ArrowError> {
    if rows.iter().any(|r| !r.has_valid_identity()) {
        return Err(invalid("invalid environment identity"));
    }
    batch(
        "catalog_environments",
        vec![
            required!(rows, "environment_id", "key:environment", |r| r
                .environment_id
                .as_str()),
            required!(
                rows,
                "resolution",
                "vocabulary:environment-resolution/1",
                |r| match r.resolution {
                    EnvironmentResolution::Unspecified => "unspecified",
                    EnvironmentResolution::Declared => "declared",
                    EnvironmentResolution::Resolved => "resolved",
                    EnvironmentResolution::Verified => "verified",
                }
            ),
            nullable!(rows, "toolchain", "toolchain-identity", |r| r
                .toolchain
                .as_deref()),
            nullable!(rows, "target", "target-platform", |r| r.target.as_deref()),
            column(
                "features",
                list(rows.iter().map(|r| r.features.as_slice())),
                false,
                "set:features",
            ),
            column(
                "features_known",
                Arc::new(BooleanArray::from_iter(
                    rows.iter().map(|r| Some(r.features_known)),
                )),
                false,
                "feature-knowledge",
            ),
            column(
                "default_features",
                Arc::new(BooleanArray::from_iter(
                    rows.iter().map(|r| r.default_features),
                )),
                true,
                "default-feature-knowledge",
            ),
            nullable!(rows, "lock_digest", "sha256", |r| r.lock_digest.as_deref()),
        ],
    )
}

/// # Errors
/// Unknown enum spellings, nulls and content identity mismatches are errors.
pub fn environments_from_batch(input: &RecordBatch) -> Result<Vec<Environment>, ArrowError> {
    let columns = RowSet::batch(input)?;
    (0..input.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = Environment {
                environment_id: r
                    .text("environment_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                resolution: match r.text("resolution")? {
                    "unspecified" => EnvironmentResolution::Unspecified,
                    "declared" => EnvironmentResolution::Declared,
                    "resolved" => EnvironmentResolution::Resolved,
                    "verified" => EnvironmentResolution::Verified,
                    _ => return Err(invalid("unknown environment resolution")),
                },
                toolchain: r.owned("toolchain")?,
                target: r.owned("target")?,
                features: r.list("features")?,
                features_known: r.boolean("features_known")?,
                default_features: r.optional_bool("default_features")?,
                lock_digest: r.owned("lock_digest")?,
            };
            if !value.has_valid_identity() {
                return Err(invalid("invalid environment identity"));
            }
            Ok(value)
        })
        .collect()
}

fn validate_context(value: &Context) -> Result<(), ArrowError> {
    if value.context_id
        != Context::new(
            value.release_id.clone(),
            value.environment_id.clone(),
            value.mode,
        )
        .context_id
    {
        return Err(invalid("context identity disagrees with binding"));
    }
    if value.parent_context_id.as_ref() == Some(&value.context_id) {
        return Err(invalid("self-parent context"));
    }
    Ok(())
}

/// # Errors
/// Context identity and nullable parent domains must be valid.
pub fn contexts(rows: &[Context]) -> Result<RecordBatch, ArrowError> {
    for r in rows {
        validate_context(r)?;
    }
    batch(
        "catalog_contexts",
        vec![
            required!(rows, "context_id", "key:context", |r| r.context_id.as_str()),
            required!(rows, "release_id", "ref:release", |r| r.release_id.as_str()),
            required!(rows, "environment_id", "ref:environment", |r| r
                .environment_id
                .as_str()),
            required!(
                rows,
                "mode",
                "vocabulary:research-mode/1",
                |r| match r.mode {
                    ResearchMode::Project => "project",
                    ResearchMode::Upstream => "upstream",
                    ResearchMode::Compare => "compare",
                    ResearchMode::Revision => "revision",
                }
            ),
            nullable!(rows, "parent_context_id", "ref:context", |r| r
                .parent_context_id
                .as_ref()
                .map(|id| id.as_str())),
        ],
    )
}

/// # Errors
/// Invalid context identity/reference formats or mode spellings are errors.
pub fn contexts_from_batch(input: &RecordBatch) -> Result<Vec<Context>, ArrowError> {
    let columns = RowSet::batch(input)?;
    (0..input.num_rows())
        .map(|i| {
            let r = columns.row(i);
            let value = Context {
                context_id: r
                    .text("context_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                release_id: r
                    .text("release_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                environment_id: r
                    .text("environment_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                mode: match r.text("mode")? {
                    "project" => ResearchMode::Project,
                    "upstream" => ResearchMode::Upstream,
                    "compare" => ResearchMode::Compare,
                    "revision" => ResearchMode::Revision,
                    _ => return Err(invalid("unknown research mode")),
                },
                parent_context_id: r
                    .owned("parent_context_id")?
                    .map(TryInto::try_into)
                    .transpose()
                    .map_err(|e| invalid(format!("{e}")))?,
            };
            validate_context(&value)?;
            Ok(value)
        })
        .collect()
}

pub use super::publication::{decode as snapshots_from_batch, encode as snapshots};

/// # Errors
/// Arrow shape failures are propagated.
pub fn selections(rows: &[SnapshotSelection]) -> Result<RecordBatch, ArrowError> {
    batch(
        "catalog_selections",
        vec![
            required!(rows, "context_id", "ref:context", |r| r.context_id.as_str()),
            required!(rows, "snapshot_id", "ref:snapshot", |r| r
                .snapshot_id
                .as_str()),
            column(
                "generation",
                Arc::new(UInt64Array::from_iter_values(
                    rows.iter().map(|r| r.generation),
                )),
                false,
                "catalog-generation",
            ),
        ],
    )
}

/// # Errors
/// Invalid identity or generation domains are errors.
pub fn selections_from_batch(input: &RecordBatch) -> Result<Vec<SnapshotSelection>, ArrowError> {
    let columns = RowSet::batch(input)?;
    (0..input.num_rows())
        .map(|i| {
            let r = columns.row(i);
            Ok(SnapshotSelection {
                context_id: r
                    .text("context_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                snapshot_id: r
                    .text("snapshot_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                generation: r.number("generation")?,
            })
        })
        .collect()
}

/// # Errors
/// Attempt provenance uses the same typed producer projection as evidence snapshots.
pub fn attempts(rows: &[SnapshotAttempt]) -> Result<RecordBatch, ArrowError> {
    let runs: Vec<_> = rows.iter().map(|r| r.run.clone()).collect();
    let runs = super::producer_runs(&runs)?;
    let ids: Vec<_> = rows.iter().map(SnapshotAttempt::association_id).collect();
    let mut columns = vec![
        column(
            "acquisitions",
            super::acquisitions::array(
                &rows
                    .iter()
                    .map(|r| r.artifacts.as_slice())
                    .collect::<Vec<_>>(),
            )?,
            false,
            "attempt-acquisitions",
        ),
        column(
            "association_id",
            text(ids.iter().map(String::as_str)),
            false,
            "key:snapshot-attempt",
        ),
        required!(rows, "snapshot_id", "ref:snapshot", |r| r
            .snapshot_id
            .as_str()),
    ];
    columns.extend(
        runs.schema()
            .fields()
            .iter()
            .zip(runs.columns())
            .map(|(f, a)| (f.as_ref().clone(), a.clone())),
    );
    batch("catalog_attempts", columns)
}

/// # Errors
/// Invalid attempt association or producer identities are rejected.
pub fn attempts_from_batch(input: &RecordBatch) -> Result<Vec<SnapshotAttempt>, ArrowError> {
    let columns = RowSet::batch(input)?;
    let runs = super::producer_runs_from_batch(input)?;
    runs.into_iter()
        .enumerate()
        .map(|(i, run)| {
            let r = columns.row(i);
            let value = SnapshotAttempt {
                snapshot_id: r
                    .text("snapshot_id")?
                    .to_owned()
                    .try_into()
                    .map_err(|e| invalid(format!("{e}")))?,
                run,
                artifacts: super::acquisitions::decode(r)?,
            };
            if value.association_id() != r.text("association_id")? {
                return Err(invalid("invalid attempt association identity"));
            }
            Ok(value)
        })
        .collect()
}
