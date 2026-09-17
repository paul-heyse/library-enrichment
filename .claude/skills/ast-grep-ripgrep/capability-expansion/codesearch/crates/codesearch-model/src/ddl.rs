//! Creating the Delta tables, and proving the schema survives the trip.
//!
//! # Constraints go on at creation, never afterwards
//!
//! Probe PB13 measured both orders against the same failure. A write followed by a constraint-add
//! that fails validation leaves the violating row **committed at v1** and **no constraint
//! recorded** -- a table containing data that breaks an invariant, with nothing in its metadata
//! saying the invariant was ever intended. Adding the constraint first rejects the write and
//! nothing lands.
//!
//! They are always two commits (create `v0`, write `v1`, constraint `v2`), so there is no
//! transaction to lean on. Ordering is the whole mitigation, which is why [`create_table`] takes
//! its constraints as an argument rather than offering an `add_constraint` afterwards.
//!
//! # What a constraint may contain
//!
//! Single-row, single-table, plain column algebra. Nothing else, because probe PB07 measured the
//! two failure modes:
//!
//! - a constraint naming **another table** panics the process outright
//!   (`DeltaContextProvider::get_table_source` is `unimplemented!()`), so constraint text must
//!   never be reachable from input;
//! - a constraint calling a **UDF** is accepted, stored as SQL text, and re-parsed by every future
//!   writer -- so the table becomes unwritable by any session that does not register that UDF.
//!
//! Cross-row and cross-table invariants live in `projection.violations_*` views instead.

use std::collections::HashMap;

use arrow_schema::{Schema, SchemaRef};
use codesearch_bridge::lattice;
use deltalake::DeltaTableBuilder;
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;

use crate::schema;

/// A table's creation recipe: its Arrow schema, its partition columns and its CHECK constraints.
pub struct TableSpec {
    pub name: &'static str,
    pub arrow_schema: SchemaRef,
    pub partition_columns: Vec<String>,
    /// `(constraint_name, sql_expression)`. Plain column algebra only -- see the module docs.
    pub constraints: Vec<(String, String)>,
}

/// The CHECK constraints implied by the schema itself.
///
/// Every lattice column gets `BETWEEN 0 AND max_rank`, derived from the lattice definition rather
/// than written out per table, so adding a value to a lattice cannot leave a stale bound behind.
/// This is also strictly better than the `IN ('a','b','c')` form the label encoding would have
/// needed: an integer range is cheaper and cannot drift from the ordering.
fn derived_constraints(arrow_schema: &Schema) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for field in arrow_schema.fields() {
        let Some(stem) = field.name().strip_suffix("_rank") else {
            continue;
        };
        let Some(def) = lattice::lattice(stem) else {
            continue;
        };
        out.push((
            format!("{stem}_rank_in_range"),
            format!("{} BETWEEN 0 AND {}", field.name(), def.max_rank()),
        ));
    }
    out
}

/// Partition columns per table. PB10 measured that the change feed narrows **only** on partition
/// columns -- a partition filter cut three files to two while a data-column filter read all three
/// and still answered correctly. So partitioning is part of the execution design, not a physical
/// layout footnote.
fn partition_columns(name: &str) -> Vec<String> {
    match name {
        "catalog.lattice" => Vec::new(),
        "evidence.behavior_assertion" => vec!["subject_kind".to_string()],
        "catalog.surface_entry" => vec!["source_table".to_string()],
        _ => Vec::new(),
    }
}

/// Every table's creation recipe.
pub fn all_specs() -> Vec<TableSpec> {
    schema::all_tables()
        .into_iter()
        .map(|(name, arrow_schema)| {
            let mut constraints = derived_constraints(&arrow_schema);
            if name == "snapshot.source_anchor" {
                // Single-row plain column algebra: legal as a CHECK, and the proposal's
                // "anchor_is_ordered" invariant.
                constraints.push((
                    "anchor_is_ordered".to_string(),
                    "start_byte <= end_byte".to_string(),
                ));
            }
            TableSpec {
                partition_columns: partition_columns(name),
                name,
                arrow_schema,
                constraints,
            }
        })
        .collect()
}

/// Convert an Arrow schema to Delta struct fields, preserving field metadata.
///
/// Metadata is passed in **one** `with_metadata` call. PB01's first run lost exactly one key per
/// field because `StructField::with_metadata` replaces rather than extends, so a per-entry loop
/// keeps only whichever entry came last -- and the control is what caught it.
pub fn delta_fields(arrow_schema: &Schema) -> Result<Vec<StructField>, DdlError> {
    arrow_schema
        .fields()
        .iter()
        .map(|f| {
            let dt = deltalake::kernel::DataType::try_from_arrow(f.data_type())
                .map_err(|e| DdlError::Conversion(f.name().clone(), e.to_string()))?;
            let mut sf = StructField::new(f.name().clone(), dt, f.is_nullable());
            if !f.metadata().is_empty() {
                let entries: HashMap<String, String> = f
                    .metadata()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                sf = sf.with_metadata(entries);
            }
            Ok(sf)
        })
        .collect()
}

/// Ensure the table exists with the authored schema, creating it if it does not.
///
/// **Create-if-absent, verify-if-present.** A rebuild must be idempotent, so a second run finds
/// the table and must not fail -- but it must also not proceed blindly. If the stored schema has
/// drifted from the authored one, that is a real problem (a code change altered the model without
/// a migration) and it is reported rather than written over.
///
/// The constraints are applied to the freshly created, still-empty table. That is the order PB13
/// measured as correct: a violating write is then rejected and nothing lands, whereas the reverse
/// order commits the violating row and fails to record the constraint. "At creation" in the design
/// means "before any data", and this is what implements it.
pub async fn ensure_table(
    root: &std::path::Path,
    spec: &TableSpec,
    session: &std::sync::Arc<datafusion::execution::SessionState>,
) -> Result<url::Url, DdlError> {
    let location = root.join(spec.name.replace('.', "__"));
    std::fs::create_dir_all(&location).map_err(|e| DdlError::Create(e.to_string()))?;
    let url = url::Url::from_directory_path(&location)
        .map_err(|_| DdlError::Create(format!("not an absolute path: {}", location.display())))?;

    let log_store = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| DdlError::Create(e.to_string()))?
        .build_storage()
        .map_err(|e| DdlError::Create(e.to_string()))?;

    // Already there? Verify rather than recreate. `load()` failing is how absence presents.
    if let Ok(existing) = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| DdlError::Create(e.to_string()))?
        .load()
        .await
    {
        let actual = existing
            .snapshot()
            .map_err(|e| DdlError::Open(e.to_string()))?
            .snapshot()
            .arrow_schema();
        assert_fixed_point(spec.name, &spec.arrow_schema, &actual)?;
        return Ok(url);
    }

    let mut create = deltalake::operations::create::CreateBuilder::new()
        .with_log_store(log_store)
        .with_columns(delta_fields(&spec.arrow_schema)?);
    if !spec.partition_columns.is_empty() {
        create = create.with_partition_columns(spec.partition_columns.clone());
    }
    let table = create
        .await
        .map_err(|e| DdlError::Create(format!("{}: {e}", spec.name)))?;

    // Constraints go on now, while the table is empty, and never again.
    let mut table = table;
    for (name, expr) in &spec.constraints {
        table = table
            .add_constraint()
            .with_constraint(name.clone(), expr.clone())
            .with_session_state(
                std::sync::Arc::clone(session) as std::sync::Arc<dyn datafusion::catalog::Session>
            )
            .await
            .map_err(|e| DdlError::Create(format!("{}: constraint {name}: {e}", spec.name)))?;
    }
    Ok(url)
}

/// Read a table's Arrow schema back from storage.
pub async fn read_back_schema(url: &url::Url) -> Result<SchemaRef, DdlError> {
    let table = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| DdlError::Open(e.to_string()))?
        .load()
        .await
        .map_err(|e| DdlError::Open(e.to_string()))?;
    let snapshot = table
        .snapshot()
        .map_err(|e| DdlError::Open(e.to_string()))?;
    Ok(snapshot.snapshot().arrow_schema())
}

/// What went wrong creating or checking a table.
#[derive(Debug, thiserror::Error)]
pub enum DdlError {
    #[error("converting column `{0}` to a Delta type: {1}")]
    Conversion(String, String),
    #[error("opening table: {0}")]
    Open(String),
    #[error("creating table: {0}")]
    Create(String),
    #[error(
        "schema is not a fixed point of the Delta conversion for `{table}`:\n  authored:  {authored}\n  read back: {actual}\nDelta narrows types silently (PB03); author what comes back."
    )]
    NotAFixedPoint {
        table: String,
        authored: String,
        actual: String,
    },
}

/// Compare an authored schema against what storage returned, field by field.
///
/// Names, types and nullability must all match. This is the check that would have caught
/// `UInt32` byte offsets truncating above `i32::MAX`, and it is the reason the schema is authored
/// in Delta's narrowed vocabulary rather than in the one that reads most naturally.
pub fn assert_fixed_point(table: &str, authored: &Schema, actual: &Schema) -> Result<(), DdlError> {
    let describe = |s: &Schema| {
        s.fields()
            .iter()
            .map(|f| {
                format!(
                    "{}:{:?}{}",
                    f.name(),
                    f.data_type(),
                    if f.is_nullable() { "?" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let a = describe(authored);
    let b = describe(actual);
    if a != b {
        return Err(DdlError::NotAFixedPoint {
            table: table.to_string(),
            authored: a,
            actual: b,
        });
    }
    Ok(())
}
