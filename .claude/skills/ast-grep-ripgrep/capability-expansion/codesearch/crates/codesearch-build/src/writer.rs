//! Writing the assembled batches into Delta tables.
//!
//! # Why a scoped overwrite, and what the scope is
//!
//! Every write replaces exactly the rows the current run wrote before -- `SaveMode::Overwrite`
//! with `with_replace_where("run_id = '<this run>'")`. A run id is `run:<snapshot>/<family>/0`, so
//! one predicate over one existing column expresses both halves of the scope the design asks for:
//! this family, this snapshot, and nothing else.
//!
//! **This is a `WriteBuilder` method, not a `MergeBuilder` one.** Evidence 05 §2 recorded that
//! correctly in round 1; the plan later compressed it onto the wrong type, and PB17 (evidence 15)
//! is where that was caught. `MergeBuilder` has no target-narrowing predicate at the pinned rev.
//!
//! Three properties PB17 measured, each of which this code depends on:
//!
//! 1. **A scoped write preserves other runs' rows**, and the probe's control -- the same write
//!    without the predicate -- destroys them. That control is why the predicate is known to be
//!    doing work: `with_replace_where` is *silently ignored* outside `Overwrite` mode.
//! 2. **Every row written must satisfy the predicate**, enforced per row by delta-rs, or the write
//!    fails with `Invalid data found`. So a batch carrying a foreign `run_id` is rejected rather
//!    than quietly widening what this write replaces. That is a guard, and it is free.
//! 3. **A re-run is idempotent in content but not in version.** The rows are unchanged; the table
//!    still advances a version, because `WriteBuilder` has no empty-actions guard. Anything
//!    deciding whether a build did something must compare rows, not versions.
//!
//! The predicate is plain column algebra by necessity. PB07 (evidence 12) measured that any Delta
//! expression naming a *relation* -- a CHECK constraint, a merge predicate, a `replaceWhere` --
//! reaches `DeltaContextProvider::get_table_source`, which is `unimplemented!()`, and **aborts the
//! process**. No table qualification, no subquery, no UDF.
//!
//! # What [`assert_single_writer`] guards now
//!
//! It still refuses a table claimed by two families in one build, but the reason has changed. It
//! used to be what made whole-table overwrite defensible. Now that writes are scoped by run, two
//! families writing one table would not destroy each other's rows -- they would each replace their
//! own slice. The check stays because the tool does not yet *mean* to share a table between
//! families, and a table that acquired a second writer by accident would be a silent change to
//! what every projection over it returns.
//!
//! # Tables are created with their constraints, before any data
//!
//! Probe PB13 measured the two orders. Writing first and adding the constraint afterwards leaves a
//! violating row committed *and* no constraint recorded; creating the constraint first rejects the
//! write and nothing lands. So creation and population are separate steps here, in that order.

use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

use arrow::array::RecordBatch;
use codesearch_model::ddl;
use datafusion::execution::SessionState;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::SessionFallbackPolicy;
use deltalake::protocol::SaveMode;

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("preparing `{0}`: {1}")]
    Ddl(String, String),
    #[error("writing `{0}`: {1}")]
    Write(String, String),
    #[error(
        "`{table}` is claimed by more than one extraction family in this build. Writes are scoped to one run, so the two would not overwrite each other -- but no table is meant to have two writers yet, and a second one appearing by accident silently changes what every projection over that table returns. Give the table one writer, or make the sharing deliberate and say so here."
    )]
    MultipleWriters { table: String },
}

/// Guard the precondition that makes overwrite sound.
pub fn assert_single_writer(targets: &[&str]) -> Result<(), WriteError> {
    let mut seen = BTreeSet::new();
    for t in targets {
        if !seen.insert(*t) {
            return Err(WriteError::MultipleWriters {
                table: (*t).to_string(),
            });
        }
    }
    Ok(())
}

/// Tables written whole rather than scoped to one run, each for its own reason.
///
/// `catalog.lattice` has no `run_id` to scope by: rank-to-label is a reference mapping, not an
/// observation about a snapshot, and it is exempt from the run *filter* for the same reason.
///
/// `snapshot.extraction_run` is rewritten across runs on purpose. Reaping flips an earlier run
/// from `running` to `abandoned`, so a build legitimately rewrites rows it did not write, and
/// `runs::merged` hands over the whole table. Scoping that write to the current run would be
/// rejected outright by delta-rs's conformance check -- the batch carries other runs' ids -- which
/// is a pleasant way to find out that the exemption is real rather than decorative.
pub const UNSCOPED: &[&str] = &["catalog.lattice", "snapshot.extraction_run"];

/// Create the table if needed, then replace the rows `run_id` wrote before with `batch`.
///
/// The run id is the scope, not merely provenance. See the module docs for what that buys and what
/// PB17 measured about it.
pub async fn write_table(
    root: &Path,
    table_name: &str,
    batch: RecordBatch,
    run_id: &str,
    session: &Arc<SessionState>,
) -> Result<url::Url, WriteError> {
    // The predicate is interpolated, which is safe only because run ids are minted by this process
    // from a blake3 digest and a fixed shape. Stated as an assertion rather than left implied: a
    // quote here would not be an injection so much as a predicate that silently parses as
    // something else.
    debug_assert!(
        !run_id.contains('\'') && !run_id.is_empty(),
        "a run id must be quote-free and non-empty to appear in a Delta predicate: {run_id:?}"
    );
    let spec = ddl::all_specs()
        .into_iter()
        .find(|s| s.name == table_name)
        .ok_or_else(|| WriteError::Ddl(table_name.into(), "no spec for this table".into()))?;

    let url = ddl::ensure_table(root, &spec, session)
        .await
        .map_err(|e| WriteError::Ddl(table_name.into(), e.to_string()))?;

    let log_store = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .build_storage()
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let table = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .load()
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let snapshot = table.snapshot().ok().map(|s| s.snapshot().clone());

    let mut write = deltalake::operations::write::WriteBuilder::new(log_store, snapshot)
        .with_input_batches(vec![batch])
        .with_save_mode(SaveMode::Overwrite)
        .with_session_state(Arc::clone(session) as Arc<dyn datafusion::catalog::Session>)
        // PB05: the default policy discards a supplied session with only a log line, taking the
        // UDF registry and extension types with it.
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState);
    if !UNSCOPED.contains(&table_name) {
        write = write.with_replace_where(format!("run_id = '{run_id}'"));
    }
    write
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;

    Ok(url)
}

/// Remove every row of one snapshot from one table.
///
/// The mechanism PB17 measured: `SaveMode::Overwrite` with a `replace_where` predicate and an
/// input carrying **no rows**. The matched files are removed and nothing is added, in one commit.
///
/// Two findings from that probe shape this:
///
/// * `with_input_batches(vec![])` is refused with `MissingData` -- an empty *iterator* leaves the
///   builder with no input at all. A vec holding one zero-row batch is not empty, and is what
///   carries the schema.
/// * A write always commits, even with nothing to do. So pruning a snapshot that is not there
///   advances the version without changing a row, which is why the caller reports rows removed
///   rather than versions written.
pub async fn prune_snapshot(
    root: &Path,
    table_name: &str,
    snapshot_id: &str,
    session: &Arc<SessionState>,
) -> Result<(), WriteError> {
    debug_assert!(
        !snapshot_id.contains('\'') && !snapshot_id.is_empty(),
        "a snapshot id must be quote-free and non-empty to appear in a Delta predicate"
    );
    let location = root.join(table_name.replace('.', "__"));
    if !location.exists() {
        return Ok(());
    }
    let schema = codesearch_model::schema::all_tables()
        .into_iter()
        .find(|(n, _)| *n == table_name)
        .map(|(_, s)| s)
        .ok_or_else(|| WriteError::Ddl(table_name.into(), "no schema for this table".into()))?;
    let empty = RecordBatch::new_empty(schema);

    let url = url::Url::from_directory_path(&location)
        .map_err(|_| WriteError::Write(table_name.into(), "not an absolute path".into()))?;
    let log_store = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .build_storage()
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let table = DeltaTableBuilder::from_url(url)
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .load()
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let snapshot = table.snapshot().ok().map(|s| s.snapshot().clone());

    deltalake::operations::write::WriteBuilder::new(log_store, snapshot)
        .with_input_batches(vec![empty])
        .with_save_mode(SaveMode::Overwrite)
        .with_replace_where(format!("snapshot_id = '{snapshot_id}'"))
        .with_session_state(Arc::clone(session) as Arc<dyn datafusion::catalog::Session>)
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    Ok(())
}

/// Read a table's current contents, or `None` if it has never been written.
///
/// The build is otherwise write-only, and this is the single exception: `snapshot.extraction_run`
/// has to carry earlier runs forward, so it must first know what they were. Absence is `None`
/// rather than an error, because the first build of a fresh catalog home is the ordinary case.
pub async fn read_table(
    root: &Path,
    table_name: &str,
    ctx: &datafusion::prelude::SessionContext,
) -> Result<Option<RecordBatch>, WriteError> {
    let location = root.join(table_name.replace('.', "__"));
    if !location.exists() {
        return Ok(None);
    }
    let url = url::Url::from_directory_path(&location)
        .map_err(|_| WriteError::Write(table_name.into(), "not an absolute path".into()))?;
    let log_store = DeltaTableBuilder::from_url(url.clone())
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .build_storage()
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let table = DeltaTableBuilder::from_url(url)
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .load()
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let snapshot = table
        .snapshot()
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .snapshot()
        .clone();
    let provider = deltalake::delta_datafusion::TableProviderBuilder::default()
        .with_log_store(log_store)
        .with_eager_snapshot(snapshot)
        .build()
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;

    // A unique registration name, because this runs inside a long-lived context and a build may
    // read the same table twice in one process -- once to carry runs forward, once to flip the
    // status.
    let alias = format!("__read_{}", table_name.replace('.', "__"));
    let _ = ctx.deregister_table(alias.as_str());
    ctx.register_table(alias.as_str(), std::sync::Arc::new(provider))
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let batches = ctx
        .sql(&format!("SELECT * FROM {alias}"))
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?
        .collect()
        .await
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))?;
    let _ = ctx.deregister_table(alias.as_str());

    let schema = codesearch_model::schema::all_tables()
        .into_iter()
        .find(|(n, _)| *n == table_name)
        .map(|(_, s)| s)
        .ok_or_else(|| WriteError::Ddl(table_name.into(), "no schema for this table".into()))?;

    // Delta's read path returns `Utf8View` for string columns even though the stored type is
    // `Utf8` (PB05's `schema_force_view_types` finding). Casting back to the AUTHORED schema is
    // what makes the batch usable as input to a write of the same table -- without it the write
    // fails on a type mismatch that names neither the read path nor the cause.
    let restored: Vec<RecordBatch> = batches
        .iter()
        .map(|b| cast_to(b, &schema))
        .collect::<Result<_, _>>()?;
    if restored.iter().all(|b| b.num_rows() == 0) {
        return Ok(None);
    }
    arrow::compute::concat_batches(&schema, restored.iter())
        .map(Some)
        .map_err(|e| WriteError::Write(table_name.into(), e.to_string()))
}

fn cast_to(
    batch: &RecordBatch,
    schema: &arrow_schema::SchemaRef,
) -> Result<RecordBatch, WriteError> {
    let mut columns = Vec::with_capacity(schema.fields().len());
    for field in schema.fields() {
        let column = batch
            .column_by_name(field.name())
            .ok_or_else(|| WriteError::Write(field.name().clone(), "column absent".into()))?;
        let cast = arrow::compute::cast(column, field.data_type())
            .map_err(|e| WriteError::Write(field.name().clone(), e.to_string()))?;
        columns.push(cast);
    }
    RecordBatch::try_new(std::sync::Arc::clone(schema), columns)
        .map_err(|e| WriteError::Write("<batch>".into(), e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_second_writer_for_one_table_is_refused() {
        assert!(assert_single_writer(&["catalog.mechanism", "catalog.surface_entry"]).is_ok());
        let err = assert_single_writer(&["catalog.mechanism", "catalog.mechanism"])
            .expect_err("a repeated target must be refused");
        assert!(matches!(err, WriteError::MultipleWriters { .. }));
    }
}
