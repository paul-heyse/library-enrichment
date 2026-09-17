//! `snapshot.extraction_run` — the run row, its lifecycle, and the reaping of runs that stopped.
//!
//! # Why the row is written before the rows it accounts for
//!
//! A build writes this row at `running`, then the family's tables, then rewrites it at `complete`.
//! Nothing about that order is incidental.
//!
//! Written last, a process killed mid-build would leave rows in the canonical tables and no run
//! row at all — indistinguishable from a build that never started, with the half-written rows
//! fully visible to every reader. Written first, the same crash leaves rows that are *attributable*
//! to a run which is recorded as not having finished, and `projection.visible_run` hides them
//! without anyone having to notice.
//!
//! This is the same argument PB13 settled for constraints: when two writes cannot share a
//! transaction, the order is the entire mitigation.
//!
//! # Reaping
//!
//! A run row still at `running` when a *later* build starts did not finish, because the process
//! that would have finished it is gone. The later build rewrites it to `abandoned` — the bottom of
//! the `completion` lattice — and keeps it. Deleting it instead would destroy the only record that
//! the rows it wrote are unaccounted for.
//!
//! Reaping is deliberately **not** time-based. A heartbeat and a timeout would make the verdict
//! depend on a clock skew nobody measured; "a different process started, and this one had not
//! finished" is decidable without one.

use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, BooleanArray, Int8Array, Int32Array, Int64Array, ListArray, RecordBatch,
    StringArray, StructArray, TimestampMicrosecondArray,
};
use arrow::buffer::OffsetBuffer;
use arrow_schema::{DataType, SchemaRef};
use codesearch_bridge::{identity, lattice};

/// The `completion` values this module writes. Named rather than spelled at each call site, so a
/// typo is a compile error instead of a lattice lookup that fails at runtime.
pub const RUNNING: &str = "running";
pub const COMPLETE: &str = "complete";
pub const PARTIAL: &str = "partial";
pub const ABANDONED: &str = "abandoned";

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("`{0}` is not a value of the `completion` lattice")]
    UnknownCompletion(String),
    #[error("building the extraction_run batch: {0}")]
    Build(String),
    #[error(
        "the previous `snapshot.extraction_run` has no `{0}` column. The table on disk was written by a different schema version; delete the catalog home and rebuild."
    )]
    MissingColumn(&'static str),
    #[error(
        "`snapshot.extraction_run` declares a column `{0}` this builder does not fill. Every schema column must be populated -- an omitted one would silently become null."
    )]
    UnfilledColumn(String),
}

/// One extraction run.
#[derive(Debug, Clone)]
pub struct RunRecord {
    /// The run's `entity_key`, which is also its `run_id`. Every canonical row this run writes
    /// carries this string in `run_id`, and that is the join `visible_run` uses.
    pub key: String,
    pub snapshot_id: String,
    pub family: String,
    pub extractor_identity: String,
    pub context_id: String,
    pub skill_root: String,
    pub index_files: i32,
    pub index_rows: i64,
    pub completion: &'static str,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub input_artifact_ids: Vec<String>,
    pub failed_pass: Option<String>,
}

/// Microseconds since the Unix epoch, UTC.
pub fn now_micros() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

fn rank(label: &str) -> Result<i8, RunError> {
    lattice::rank_of("completion", label).ok_or_else(|| RunError::UnknownCompletion(label.into()))
}

/// Build the one-row batch for a run.
///
/// The arrays are assembled by field **name** against the authored schema rather than pushed in
/// positional order, for the same reason `catalog::Columns` does: two columns of the same type
/// swapped positionally produce a plausible wrong answer and no error anywhere.
pub fn batch(schema: &SchemaRef, run: &RunRecord) -> Result<RecordBatch, RunError> {
    let completion = rank(run.completion)?;
    let mut arrays: Vec<ArrayRef> = Vec::with_capacity(schema.fields().len());
    for field in schema.fields() {
        let a: ArrayRef = match field.name().as_str() {
            "entity_id" => Arc::new(StringArray::from(vec![identity::entity_id(
                &run.key,
                &run.snapshot_id,
            )])),
            "entity_key" => Arc::new(StringArray::from(vec![run.key.clone()])),
            "snapshot_id" => Arc::new(StringArray::from(vec![run.snapshot_id.clone()])),
            // A run row's `run_id` is its own key. Self-reference is correct here and it is also
            // load-bearing: it means `visible_run` needs no special case for the run table itself.
            "run_id" => Arc::new(StringArray::from(vec![run.key.clone()])),
            "evidence_id" => Arc::new(StringArray::from(vec![None::<String>])),
            // A run row is an exact, recorded observation of this process's own behaviour: the
            // build watched itself start and stop. It is not `confirmed`, because nothing controls
            // it.
            "precision_rank" => Arc::new(Int8Array::from(vec![
                lattice::rank_of("precision", "exact").unwrap_or(0),
            ])),
            "observed_rank" => Arc::new(Int8Array::from(vec![
                lattice::rank_of("observed", "recorded").unwrap_or(0),
            ])),
            "coverage_status" => Arc::new(StringArray::from(vec!["characterised"])),
            "ord" => Arc::new(Int32Array::from(vec![None::<i32>])),
            "first_seen_version" => Arc::new(Int64Array::from(vec![0i64])),
            "last_seen_version" => Arc::new(Int64Array::from(vec![None::<i64>])),
            "retracted" => Arc::new(BooleanArray::from(vec![false])),
            "family" => Arc::new(StringArray::from(vec![run.family.clone()])),
            "extractor_identity" => {
                Arc::new(StringArray::from(vec![run.extractor_identity.clone()]))
            }
            "context_id" => Arc::new(StringArray::from(vec![run.context_id.clone()])),
            "scope" => {
                let DataType::Struct(children) = field.data_type() else {
                    return Err(RunError::Build("`scope` is not a struct".into()));
                };
                Arc::new(StructArray::new(
                    children.clone(),
                    vec![
                        Arc::new(StringArray::from(vec![run.skill_root.clone()])) as ArrayRef,
                        Arc::new(Int32Array::from(vec![run.index_files])) as ArrayRef,
                        Arc::new(Int64Array::from(vec![run.index_rows])) as ArrayRef,
                    ],
                    None,
                ))
            }
            "completion_rank" => Arc::new(Int8Array::from(vec![completion])),
            "started_at" => {
                Arc::new(TimestampMicrosecondArray::from(vec![run.started_at]).with_timezone("UTC"))
            }
            "finished_at" => Arc::new(
                TimestampMicrosecondArray::from(vec![run.finished_at]).with_timezone("UTC"),
            ),
            "input_artifact_ids" => {
                let DataType::List(child) = field.data_type() else {
                    return Err(RunError::Build("`input_artifact_ids` is not a list".into()));
                };
                let values = run.input_artifact_ids.clone();
                let offsets: Vec<i32> = vec![0, values.len() as i32];
                Arc::new(ListArray::new(
                    Arc::clone(child),
                    OffsetBuffer::new(offsets.into()),
                    Arc::new(StringArray::from(values)),
                    None,
                ))
            }
            "failed_pass" => Arc::new(StringArray::from(vec![run.failed_pass.clone()])),
            // Derived from every other column, so it is filled once they all exist. The exhaustive
            // match below is why this needs an arm at all: a column nobody fills is an error here,
            // not a silent null, and that is the property worth keeping.
            codesearch_bridge::identity::PAYLOAD_DIGEST => Arc::new(StringArray::new_null(1)),
            other => return Err(RunError::UnfilledColumn(other.to_string())),
        };
        arrays.push(a);
    }
    with_digests(schema, arrays, 1)
}

/// Fill the derived digest column and assemble the batch.
///
/// Shared by [`batch`] and [`carry_forward`] because reaping rewrites `completion_rank` on a
/// carried row, which changes that row's content -- so its digest has to move with it. A digest
/// left stale after a reap would make `compare` report an abandoned run as unchanged, which is the
/// one thing it must not say about a run that stopped.
fn with_digests(
    schema: &SchemaRef,
    columns: Vec<ArrayRef>,
    rows: usize,
) -> Result<RecordBatch, RunError> {
    codesearch_bridge::identity::batch_with_digest(schema, columns, rows)
        .map_err(|e| RunError::Build(e.to_string()))
}

/// Carry earlier runs forward, reaping any that never finished.
///
/// Rows belonging to `current_key` are dropped, because the caller is about to write that run's
/// row afresh — this is what makes writing `running` and then `complete` idempotent rather than
/// accumulating two rows for one run.
pub fn carry_forward(
    previous: &RecordBatch,
    current_key: &str,
) -> Result<Option<RecordBatch>, RunError> {
    let keys = previous
        .column_by_name("entity_key")
        .and_then(|c| c.as_any().downcast_ref::<StringArray>())
        .ok_or(RunError::MissingColumn("entity_key"))?;
    let keep: Vec<bool> = (0..previous.num_rows())
        .map(|i| keys.value(i) != current_key)
        .collect();
    if !keep.iter().any(|k| *k) {
        return Ok(None);
    }

    let running = rank(RUNNING)?;
    let abandoned = rank(ABANDONED)?;
    let ranks = previous
        .column_by_name("completion_rank")
        .and_then(|c| c.as_any().downcast_ref::<Int8Array>())
        .ok_or(RunError::MissingColumn("completion_rank"))?;
    let reaped: Vec<Option<i8>> = (0..previous.num_rows())
        .map(|i| {
            if ranks.is_null(i) {
                None
            } else if ranks.value(i) == running {
                Some(abandoned)
            } else {
                Some(ranks.value(i))
            }
        })
        .collect();

    let index = previous
        .schema()
        .index_of("completion_rank")
        .map_err(|e| RunError::Build(e.to_string()))?;
    let mut columns: Vec<ArrayRef> = previous.columns().to_vec();
    columns[index] = Arc::new(Int8Array::from(reaped));
    let rewritten = with_digests(&previous.schema(), columns, previous.num_rows())?;

    let mask = BooleanArray::from(keep);
    let filtered = arrow::compute::filter_record_batch(&rewritten, &mask)
        .map_err(|e| RunError::Build(e.to_string()))?;
    Ok(Some(filtered))
}

/// The run table's contents after this build's row is added to whatever came before.
pub fn merged(
    schema: &SchemaRef,
    previous: Option<&RecordBatch>,
    run: &RunRecord,
) -> Result<RecordBatch, RunError> {
    let current = batch(schema, run)?;
    let carried = match previous {
        Some(p) => carry_forward(p, &run.key)?,
        None => None,
    };
    match carried {
        None => Ok(current),
        Some(c) => arrow::compute::concat_batches(schema, [&c, &current])
            .map_err(|e| RunError::Build(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(key: &str, completion: &'static str) -> RunRecord {
        RunRecord {
            key: key.into(),
            snapshot_id: "snap:test".into(),
            family: "catalog".into(),
            extractor_identity: "codesearch-build@test".into(),
            context_id: "ctx:test".into(),
            skill_root: "/nowhere".into(),
            index_files: 3,
            index_rows: 42,
            completion,
            started_at: 1_000_000,
            finished_at: None,
            input_artifact_ids: vec!["a.tsv".into(), "b.tsv".into()],
            failed_pass: None,
        }
    }

    fn schema() -> SchemaRef {
        codesearch_model::schema::snapshot_extraction_run()
    }

    fn completions(b: &RecordBatch) -> Vec<i8> {
        let a = b
            .column_by_name("completion_rank")
            .and_then(|c| c.as_any().downcast_ref::<Int8Array>())
            .expect("completion_rank");
        (0..b.num_rows()).map(|i| a.value(i)).collect()
    }

    /// The rule this module exists for.
    #[test]
    fn a_previous_run_left_running_is_reaped_to_abandoned() {
        let s = schema();
        let first = batch(&s, &record("run:one", RUNNING)).expect("first run row");
        let after = merged(&s, Some(&first), &record("run:two", RUNNING)).expect("merged");

        assert_eq!(
            after.num_rows(),
            2,
            "the earlier run must be kept, not deleted"
        );
        assert_eq!(
            completions(&after),
            vec![rank(ABANDONED).unwrap(), rank(RUNNING).unwrap()],
            "the earlier run is reaped; the current one is left alone"
        );

        // The control: a previous run that FINISHED must not be touched, or "reaped" would just
        // mean "rewritten", and the test above would pass for the wrong reason.
        let done = batch(&s, &record("run:one", COMPLETE)).expect("finished run row");
        let after = merged(&s, Some(&done), &record("run:two", RUNNING)).expect("merged");
        assert_eq!(
            completions(&after),
            vec![rank(COMPLETE).unwrap(), rank(RUNNING).unwrap()],
            "a complete run must survive a later build unchanged"
        );
    }

    /// Writing `running` then `complete` for one run leaves ONE row, not two.
    #[test]
    fn the_status_flip_replaces_the_row_rather_than_appending_one() {
        let s = schema();
        let started = batch(&s, &record("run:one", RUNNING)).expect("start");
        let mut done = record("run:one", COMPLETE);
        done.finished_at = Some(2_000_000);
        let after = merged(&s, Some(&started), &done).expect("flip");
        assert_eq!(after.num_rows(), 1);
        assert_eq!(completions(&after), vec![rank(COMPLETE).unwrap()]);
    }

    #[test]
    fn a_run_row_is_a_fixed_point_of_its_own_schema() {
        let s = schema();
        let b = batch(&s, &record("run:one", RUNNING)).expect("row");
        assert_eq!(
            b.schema(),
            s,
            "the built batch must match the authored schema"
        );
        assert_eq!(b.num_rows(), 1);
    }
}
