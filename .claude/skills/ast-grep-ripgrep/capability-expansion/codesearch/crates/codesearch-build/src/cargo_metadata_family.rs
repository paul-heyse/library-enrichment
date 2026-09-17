//! The `cargo-metadata` family: §3.1, and the first fed by an adapter rather than by a skill.
//!
//! §6.2 puts this pass first -- "A must precede B" -- and it is the cheapest of the eight families
//! because `cargo metadata` compiles nothing. Its input is a producer root written by
//! `producers/cargo-metadata-adapter`, read through the same `index.rs` the skill goes through.
//!
//! # What this family cannot know
//!
//! Evidence 06 is blunt: this family "answers *nothing whatsoever about code*". It sees manifests.
//! [`NEVER_FILLS`] forbids the columns that describe anything else, and `Columns::never_fills`
//! makes writing one a build error rather than a plausible-looking null.
//!
//! # It owns `program.package` now
//!
//! `library-api` used to mint these from the skill's `pins.crates`, which is a bare
//! `name -> version` map -- so the key degraded to `pkg:<name>@<version>` and `source` was
//! never-filled. A resolve recovers both, so the table moves here and §1.2's full
//! `pkg:<registry>/<name>@<version>` is writable. That is a migration: every package key changed,
//! and `compare` between the two snapshots is what checks it.

use std::collections::{BTreeMap, BTreeSet};

use arrow::array::RecordBatch;
use codesearch_bridge::identity;

use crate::catalog::{CatalogError, Columns, Common, RunContext, push_common};
use crate::index::Row;

/// Columns this family must leave NULL, because a manifest does not contain them.
const NEVER_FILLS: &[&str] = &["evidence_id"];

/// The batches this family writes.
pub struct CargoMetadataBatches {
    pub package: RecordBatch,
    pub crate_unit: RecordBatch,
    pub feature_declaration: RecordBatch,
    pub dependency_edge: RecordBatch,
}

/// Build all four, in dependency order: packages first, because everything else references one.
pub fn build(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    resolve_present: bool,
    run: RunContext<'_>,
) -> Result<(CargoMetadataBatches, BTreeSet<String>), CatalogError> {
    let (package, keys) = build_packages(indexes, run)?;
    Ok((
        CargoMetadataBatches {
            package,
            crate_unit: build_crate_units(indexes, &keys, run)?,
            feature_declaration: build_features(indexes, &keys, run)?,
            dependency_edge: build_edges(indexes, &keys, resolve_present, run)?,
        },
        keys,
    ))
}

/// The package key a row references, checked against the keys this family minted.
///
/// Deliberately not a `filter_map`: a row naming a package nothing holds means the adapter's four
/// files disagree with each other, and dropping it silently would hide exactly that.
fn referenced(row: &Row, keys: &BTreeSet<String>, what: &str) -> Result<String, CatalogError> {
    let key = row.get("package_key").cloned().unwrap_or_default();
    if keys.contains(&key) {
        Ok(key)
    } else {
        Err(CatalogError::Build(format!(
            "{what} references `{key}`, which no `program.package` row holds. The adapter's index \
             files disagree with each other -- re-run it rather than relaxing this check."
        )))
    }
}

fn build_packages(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<(RecordBatch, BTreeSet<String>), CatalogError> {
    let schema = codesearch_model::schema::program_package();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    let mut keys = BTreeSet::new();
    for row in indexes.get("packages").into_iter().flatten() {
        let name = row.get("name").cloned().unwrap_or_default();
        let version = row.get("version").cloned().unwrap_or_default();
        let registry = row.get("registry").cloned().unwrap_or_default();
        let key = identity::package_key(&registry, &name, &version);

        // The adapter emits the key as well as its parts. Minting it here and comparing is what
        // keeps §1.2's grammar in one place: if the adapter ever spells a key differently, this
        // fails at build time rather than producing two namespaces nobody notices.
        let emitted = row.get("package_key").cloned().unwrap_or_default();
        if emitted != key {
            return Err(CatalogError::Build(format!(
                "the adapter emitted `{emitted}` where §1.2's grammar mints `{key}`. The key \
                 grammar lives in `bridge::identity` and nowhere else; fix the adapter."
            )));
        }
        if !keys.insert(key.clone()) {
            return Err(CatalogError::Build(format!(
                "`{key}` appears twice in the resolve, which cannot happen for one registry, one \
                 name and one version."
            )));
        }

        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                // Read from a manifest a resolve chose, not observed by running anything.
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("name", name);
        cols.str_col("version", version);
        cols.str_col("source", registry);
        cols.str_col("repository", optional(row, "repository"));
        cols.str_col("edition", optional(row, "edition"));
        cols.str_col("rust_version", optional(row, "rust_version"));
        cols.str_col("license", optional(row, "license"));
        cols.bool_col(
            "in_subject_pins",
            row.get("in_subject_pins")
                .map(|v| v == "true")
                .unwrap_or(false),
        );
        cols.end_row();
    }
    Ok((cols.build(&schema)?, keys))
}

/// An empty cell is absent, not an empty string. The distinction is the one §5.1 insists on.
fn optional(row: &Row, column: &str) -> Option<String> {
    row.get(column).filter(|v| !v.is_empty()).cloned()
}

fn build_crate_units(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    keys: &BTreeSet<String>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::program_crate_unit();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    for row in indexes.get("crate_units").into_iter().flatten() {
        let package_key = referenced(row, keys, "a crate unit")?;
        let name = row.get("target_name").cloned().unwrap_or_default();
        let kind = row.get("target_kind").cloned().unwrap_or_default();
        // §1.2: `crate:<pkg-key>#<target-name>/<target-kind>`. The kind is in the key because a
        // package can declare a lib and an example under one name.
        let key = format!("crate:{package_key}#{name}/{kind}");
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("package_key", package_key.clone());
        cols.str_col(
            "package_id",
            identity::entity_id(&package_key, run.snapshot_id),
        );
        cols.str_col("target_name", name);
        cols.str_col("target_kind", kind);
        cols.str_col("src_path", row.get("src_path").cloned().unwrap_or_default());
        cols.str_col("edition", row.get("edition").cloned().unwrap_or_default());
        cols.end_row();
    }
    cols.build(&schema)
}

fn build_features(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    keys: &BTreeSet<String>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::program_feature_declaration();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    for row in indexes.get("features").into_iter().flatten() {
        let package_key = referenced(row, keys, "a feature declaration")?;
        let feature = row.get("feature_name").cloned().unwrap_or_default();
        let key = format!("{package_key}#feature:{feature}");
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("package_key", package_key.clone());
        cols.str_col(
            "package_id",
            identity::entity_id(&package_key, run.snapshot_id),
        );
        cols.str_col("feature_name", feature);
        // An EMPTY list is a feature that implies nothing -- a different fact from a null one,
        // which would be "nobody looked". The adapter always knows, so it is never null.
        let implies = row
            .get("implies")
            .filter(|v| !v.is_empty())
            .map(|v| v.split(';').map(str::to_string).collect())
            .unwrap_or_default();
        cols.list_col("implies", implies);
        cols.end_row();
    }
    cols.build(&schema)
}

fn build_edges(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    keys: &BTreeSet<String>,
    resolve_present: bool,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::program_dependency_edge();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    for row in indexes.get("dep_edges").into_iter().flatten() {
        let package_key = referenced(row, keys, "a dependency edge")?;
        let dep_key = row.get("dep_package_key").cloned().unwrap_or_default();
        if !dep_key.is_empty() && !keys.contains(&dep_key) {
            return Err(CatalogError::Build(format!(
                "a dependency edge points at `{dep_key}`, which no `program.package` row holds. A \
                 resolved edge whose target is missing is a broken graph, not a gap."
            )));
        }
        let kind = row.get("dep_kind").cloned().unwrap_or_default();
        let cfg = optional(row, "dep_target_cfg");
        // Both endpoints, the kind, and the target cfg. A crate can be a `normal` dependency on
        // one target and a `dev` one on another, and those are two edges rather than one.
        let key = format!(
            "{package_key}#dep:{kind}/{dep_key}@{}",
            cfg.clone().unwrap_or_else(|| "any".to_string())
        );
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("package_key", package_key.clone());
        cols.str_col(
            "package_id",
            identity::entity_id(&package_key, run.snapshot_id),
        );
        cols.str_col(
            "dep_package_key",
            if dep_key.is_empty() {
                None
            } else {
                Some(dep_key)
            },
        );
        cols.str_col("dep_name", row.get("dep_name").cloned().unwrap_or_default());
        cols.str_col("dep_req", optional(row, "dep_req"));
        cols.str_col("dep_kind", kind);
        cols.str_col("dep_target_cfg", cfg);
        cols.bool_col("resolve_present", resolve_present);
        cols.end_row();
    }
    cols.build(&schema)
}
