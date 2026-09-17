//! Two snapshots in one catalog, and the control that proves the scope is doing the work.
//!
//! These write real Delta tables into a temp directory -- never into a repository, per the
//! boundary rule, and the same `tempfile::tempdir()` shape `model/tests/fixed_point.rs` uses.
//!
//! PB17 (evidence 15) measured the delta-rs behaviour these rest on. What they add is the same
//! question asked of *this* writer, against the authored schemas, through the code the build
//! actually runs -- because the probe proved the library does the right thing when asked
//! correctly, and nothing yet proved that this code asks correctly.

use arrow::array::{Array, RecordBatch, StringArray};

use crate::catalog::Columns;
use crate::writer;

const TABLE: &str = "program.export_path";
const RUN_TABLE: &str = "snapshot.extraction_run";

/// One row of `program.export_path`, attributed to `run_id`, with `path` as its only content.
///
/// Small on purpose: this is a test about the write scope, not about the program model, and a
/// table with four extra columns keeps the subject visible.
fn row(snapshot: &str, run_id: &str, path: &str) -> RecordBatch {
    let key = format!("export:{path}");
    let mut cols = Columns::default();
    cols.str_col(
        "entity_id",
        codesearch_bridge::identity::entity_id(&key, snapshot),
    );
    cols.str_col("entity_key", key);
    cols.str_col("snapshot_id", snapshot.to_string());
    cols.str_col("run_id", run_id.to_string());
    cols.str_col("evidence_id", None);
    cols.rank_col("precision_rank", 0);
    cols.rank_col("observed_rank", 0);
    cols.str_col("coverage_status", "characterised".to_string());
    cols.i32_col("ord", None);
    cols.int_col("first_seen_version", Some(0));
    cols.int_col("last_seen_version", None);
    cols.bool_col("retracted", false);
    cols.str_col("access_path", path.to_string());
    cols.str_col("canonical_path", path.to_string());
    cols.str_col("item_kind", None);
    cols.str_col("definition_id", None);
    cols.end_row();
    cols.build(&codesearch_model::schema::program_export_path())
        .expect("the fixture row fills every column")
}

/// Which `run_id`s the table holds, and how many rows each wrote -- read back through a real scan
/// from a fresh open, so nothing in-process is trusted.
async fn runs_present(root: &std::path::Path) -> Vec<(String, usize)> {
    let (ctx, _session) = codesearch_bridge::session::build_session().expect("a session");
    let batch = writer::read_table(root, TABLE, &ctx)
        .await
        .expect("the table reads back")
        .expect("the table has rows");
    let runs = batch
        .column_by_name("run_id")
        .expect("run_id is a column")
        .as_any()
        .downcast_ref::<StringArray>()
        .expect("run_id is a string column after the authored-schema cast");
    let mut counted: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for i in 0..runs.len() {
        *counted.entry(runs.value(i).to_string()).or_default() += 1;
    }
    counted.into_iter().collect()
}

fn run_of(snapshot: &str) -> String {
    format!("run:{snapshot}/library-api/0")
}

/// A completed run record for `snapshot`. Only the control uses it.
fn record(snapshot: &str) -> crate::runs::RunRecord {
    crate::runs::RunRecord {
        key: run_of(snapshot),
        snapshot_id: snapshot.to_string(),
        family: "library-api".to_string(),
        extractor_identity: "writer-tests".to_string(),
        context_id: "ctx:test".to_string(),
        skill_root: "/nowhere".to_string(),
        index_files: 1,
        index_rows: 1,
        completion: crate::runs::COMPLETE,
        started_at: crate::runs::now_micros(),
        finished_at: Some(crate::runs::now_micros()),
        input_artifact_ids: Vec::new(),
        failed_pass: None,
    }
}

/// The treatment: a second snapshot leaves the first intact.
#[tokio::test]
async fn a_second_snapshot_does_not_destroy_the_first() {
    let scratch = tempfile::tempdir().expect("temp dir");
    let root = scratch.path();
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("a session");

    let (a, b) = ("snap:aaaa", "snap:bbbb");
    writer::write_table(root, TABLE, row(a, &run_of(a), "one"), &run_of(a), &session)
        .await
        .expect("the first snapshot writes");
    writer::write_table(root, TABLE, row(b, &run_of(b), "two"), &run_of(b), &session)
        .await
        .expect("the second snapshot writes");

    assert_eq!(
        runs_present(root).await,
        vec![(run_of(a), 1), (run_of(b), 1)],
        "a run-scoped write must leave the other run's rows alone"
    );
}

/// The CONTROL, and the reason to believe the test above.
///
/// `with_replace_where` is silently ignored outside `Overwrite` mode, so a passing treatment is on
/// its own consistent with a predicate that did nothing at all. The control has to run the
/// *unscoped* path through this same code, and [`writer::UNSCOPED`] names a real table that takes
/// it: `snapshot.extraction_run`.
///
/// Two run rows written straight to it, without [`crate::runs::merged`] carrying the first
/// forward, and the first is gone. That is the treatment's failure mode, reproduced on demand --
/// and it is also why the run table is the one place a read-modify-write is the operation rather
/// than a shortcut.
#[tokio::test]
async fn the_unscoped_path_loses_what_the_scoped_one_keeps() {
    assert!(
        writer::UNSCOPED.contains(&RUN_TABLE),
        "this control depends on {RUN_TABLE} being written whole"
    );

    let scratch = tempfile::tempdir().expect("temp dir");
    let root = scratch.path();
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("a session");
    let schema = codesearch_model::schema::snapshot_extraction_run();

    let (a, b) = ("snap:aaaa", "snap:bbbb");
    for snapshot in [a, b] {
        // `None` for `previous`: the point is what happens when nothing carries earlier rows
        // forward, which is exactly what an unscoped overwrite does to them.
        let batch = crate::runs::merged(&schema, None, &record(snapshot)).expect("a run row");
        writer::write_table(root, RUN_TABLE, batch, &run_of(snapshot), &session)
            .await
            .expect("the run row writes");
    }

    let (ctx, _s) = codesearch_bridge::session::build_session().expect("a session");
    let held = writer::read_table(root, RUN_TABLE, &ctx)
        .await
        .expect("the run table reads back")
        .expect("the run table has rows");
    assert_eq!(
        held.num_rows(),
        1,
        "the CONTROL must lose the earlier run -- if it does not, the scoped test proves nothing"
    );

    // And the compensating mechanism, so the control does not read as a defect in the run table.
    let previous = writer::read_table(root, RUN_TABLE, &ctx)
        .await
        .expect("read back")
        .expect("rows");
    let carried = crate::runs::merged(&schema, Some(&previous), &record(a)).expect("merged");
    assert_eq!(
        carried.num_rows(),
        2,
        "`merged` is what keeps the run ledger whole across an unscoped write"
    );
}

/// Re-running the same scoped write changes no rows.
///
/// `just catalog` claims to be idempotent and this is the part of that claim the writer owns.
/// Note what is NOT asserted: the table version. PB17 measured that a scoped write with nothing to
/// do still commits, because `WriteBuilder` has no empty-actions guard.
#[tokio::test]
async fn re_running_a_scoped_write_changes_no_rows() {
    let scratch = tempfile::tempdir().expect("temp dir");
    let root = scratch.path();
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("a session");

    let a = "snap:aaaa";
    for _ in 0..2 {
        writer::write_table(root, TABLE, row(a, &run_of(a), "one"), &run_of(a), &session)
            .await
            .expect("the write succeeds twice");
    }
    assert_eq!(runs_present(root).await, vec![(run_of(a), 1)]);
}

/// A batch carrying a foreign `run_id` is refused, and leaves the table as it was.
///
/// This is delta-rs's own per-row conformance check reaching this code: the writer is made to
/// prove its own scope rather than being trusted to have got it right.
#[tokio::test]
async fn a_batch_that_escapes_its_own_scope_is_refused() {
    let scratch = tempfile::tempdir().expect("temp dir");
    let root = scratch.path();
    let (_ctx, session) = codesearch_bridge::session::build_session().expect("a session");

    let (a, b) = ("snap:aaaa", "snap:bbbb");
    writer::write_table(root, TABLE, row(a, &run_of(a), "one"), &run_of(a), &session)
        .await
        .expect("the first snapshot writes");

    // Rows attributed to run A, written under run B's predicate.
    let err = writer::write_table(root, TABLE, row(a, &run_of(a), "two"), &run_of(b), &session)
        .await
        .expect_err("a batch outside its own predicate must be refused");
    assert!(
        err.to_string().contains("failed validation"),
        "expected delta-rs's conformance check, got: {err}"
    );

    assert_eq!(
        runs_present(root).await,
        vec![(run_of(a), 1)],
        "a refused write must leave the table exactly as it was"
    );
}
