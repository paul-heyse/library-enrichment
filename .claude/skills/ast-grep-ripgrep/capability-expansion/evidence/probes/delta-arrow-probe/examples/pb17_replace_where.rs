//! PB17 — does a run-scoped `replace_where` overwrite behave on a table shaped like ours?
//!
//! Plan v2 §12.3 item 2 asked whether `MergeBuilder::with_replace_where` commits in one
//! transaction. Reading the pinned rev answers a prior question: **that method does not exist on
//! `MergeBuilder`**. `with_replace_where` is `WriteBuilder`'s alone (`write/mod.rs:233`), which is
//! what evidence 05 §2 recorded in round 1 before the plan compressed it onto the wrong type.
//!
//!     cargo run --example pb17_replace_where
//!
//! Reading also settles, from upstream source and upstream tests, that a data-column predicate is
//! accepted on an unpartitioned table, that the written batch must satisfy the predicate, and that
//! the whole thing is one commit. So this probe does NOT re-ask those. It asks the thing reading
//! cannot settle: whether it works against a table shaped like a canonical `codesearch` one —
//! CHECK constraint, Arrow extension metadata, `Utf8` strings — because the scoped path unions the
//! caller's batch with a **scan of the existing Delta files** (`write/plan.rs:500-517`), and that
//! scan returns `Utf8View` where the stored type is `Utf8`. That mismatch has already cost this
//! codebase two cycles and is why `writer::read_table` carries a `cast_to` helper.
//!
//! ARMS
//!   A  run1 scoped, then run2 scoped        run1 must SURVIVE. Also: does write-into-empty work
//!                                            at all with a predicate, which every fresh catalog
//!                                            hits on its first write?
//!   B  run1 scoped, then run2 UNSCOPED      CONTROL: run1 must be DESTROYED
//!   C  batch carrying a foreign run_id      must be REJECTED, on a constrained table
//!   D  re-run arm A's second write          rows unchanged; version advances anyway
//!
//! Arm B is load-bearing and not decoratively so: `with_replace_where` is SILENTLY IGNORED unless
//! the save mode is `Overwrite` (`write/plan.rs:441-445` returns a passthrough plan, no error). A
//! green arm A on its own is therefore consistent with a predicate that did nothing at all.
//!
//! The predicate is plain column algebra by necessity, not by style. PB07 (evidence 12) measured
//! that any Delta expression naming a RELATION — a CHECK constraint, a merge predicate, a
//! `replaceWhere` — reaches `DeltaContextProvider::get_table_source`, which is `unimplemented!()`,
//! and ABORTS THE PROCESS. So: no table qualification, no subquery, no UDF.

use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{Int8Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::{SessionFallbackPolicy, create_session};
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;
use deltalake::protocol::SaveMode;

/// The same shape `model/src/ddl.rs::derived_constraints` puts on every lattice-bearing table.
const CONSTRAINT: &str = "precision_rank BETWEEN 0 AND 3";

/// A canonical table in miniature: two extension-typed identity columns, a lattice rank under a
/// CHECK constraint, the `run_id` the predicate will name, and a plain nullable payload column.
///
/// Every one of those is load-bearing for the question. A bare `(id, value)` table would answer a
/// question nobody asked.
fn schema() -> SchemaRef {
    let mut id_meta = HashMap::new();
    id_meta.insert(
        "ARROW:extension:name".to_string(),
        "codesearch.entity_id".to_string(),
    );
    let mut key_meta = HashMap::new();
    key_meta.insert(
        "ARROW:extension:name".to_string(),
        "codesearch.entity_key".to_string(),
    );
    let mut lattice_meta = HashMap::new();
    lattice_meta.insert(
        "ARROW:extension:name".to_string(),
        "codesearch.lattice".to_string(),
    );

    Arc::new(Schema::new(vec![
        Field::new("entity_id", DataType::Utf8, false).with_metadata(id_meta),
        Field::new("entity_key", DataType::Utf8, false).with_metadata(key_meta),
        Field::new("run_id", DataType::Utf8, false),
        Field::new("precision_rank", DataType::Int8, false).with_metadata(lattice_meta),
        Field::new("summary", DataType::Utf8, true),
    ]))
}

/// Rows attributed to `run`, keyed `k1..kn`. `runs` lets one row be misattributed for arm C.
fn batch(runs: Vec<&str>, keys: Vec<&str>) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    let ids: Vec<String> = keys.iter().map(|k| format!("id-{k}")).collect();
    let ranks: Vec<i8> = keys.iter().map(|_| 1i8).collect();
    let summaries: Vec<Option<String>> = keys.iter().map(|k| Some(format!("about {k}"))).collect();
    Ok(RecordBatch::try_new(
        schema(),
        vec![
            Arc::new(StringArray::from(ids)),
            Arc::new(StringArray::from(keys)),
            Arc::new(StringArray::from(runs)),
            Arc::new(Int8Array::from(ranks)),
            Arc::new(StringArray::from(summaries)),
        ],
    )?)
}

fn verdict<T, E: std::fmt::Display>(r: &Result<T, E>) -> String {
    match r {
        Ok(_) => "OK".into(),
        Err(e) => {
            let msg = e.to_string();
            let first = msg
                .lines()
                .find(|l| !l.trim().is_empty())
                .unwrap_or("")
                .trim();
            format!("ERR: {}", &first[..first.len().min(96)])
        }
    }
}

/// Rows per `run_id`, read from a FRESH load through a real scan, so nothing in-process is being
/// trusted and the read path's `Utf8View` coercion is actually exercised.
async fn observe(ctx: &SessionContext, url: &url::Url, tag: &str) -> (String, String) {
    let Ok(builder) = DeltaTableBuilder::from_url(url.clone()) else {
        return ("<bad url>".into(), "-".into());
    };
    let Ok(table) = builder.load().await else {
        return ("<load failed>".into(), "-".into());
    };
    let version = table
        .version()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "<none>".into());

    let Ok(log_store) = DeltaTableBuilder::from_url(url.clone()).and_then(|b| b.build_storage())
    else {
        return (version, "<no log store>".into());
    };
    let Ok(snapshot) = table.snapshot().map(|s| s.snapshot().clone()) else {
        return (version, "<no snapshot>".into());
    };
    let provider = match deltalake::delta_datafusion::TableProviderBuilder::default()
        .with_log_store(log_store)
        .with_eager_snapshot(snapshot)
        .build()
        .await
    {
        Ok(p) => p,
        Err(e) => return (version, format!("<provider: {e}>")),
    };

    let alias = format!("t_{tag}");
    let _ = ctx.deregister_table(alias.as_str());
    if ctx.register_table(alias.as_str(), Arc::new(provider)).is_err() {
        return (version, "<register failed>".into());
    }
    let sql = format!("SELECT run_id, count(*) AS n FROM {alias} GROUP BY run_id ORDER BY run_id");
    let rows = match ctx.sql(&sql).await {
        Ok(f) => match f.collect().await {
            Ok(b) => b,
            Err(e) => return (version, format!("<scan: {e}>")),
        },
        Err(e) => return (version, format!("<plan: {e}>")),
    };
    let _ = ctx.deregister_table(alias.as_str());

    let mut parts: Vec<String> = Vec::new();
    for b in &rows {
        // The stored type is `Utf8`; delta-rs hands back `Utf8View`. Casting rather than
        // downcasting to the authored type is the PB05 lesson, restated here because a probe that
        // panicked on the read path would look like a finding about the write path.
        let run = arrow::compute::cast(b.column(0), &DataType::Utf8).expect("cast run_id");
        let run = run
            .as_any()
            .downcast_ref::<StringArray>()
            .expect("run_id is a string");
        let n = b
            .column(1)
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .expect("count is i64");
        for i in 0..b.num_rows() {
            parts.push(format!("{}={}", run.value(i), n.value(i)));
        }
    }
    if parts.is_empty() {
        parts.push("<no rows>".into());
    }
    (version, parts.join(" "))
}

/// A fresh table carrying the CHECK constraint, created empty. Constraint first, per PB13.
async fn fresh(
    root: &std::path::Path,
    name: &str,
    session: &Arc<datafusion::execution::SessionState>,
) -> Result<url::Url, Box<dyn std::error::Error>> {
    let location = root.join(name);
    let _ = std::fs::remove_dir_all(&location);
    std::fs::create_dir_all(&location)?;
    let url = url::Url::from_directory_path(&location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let fields: Vec<StructField> = schema()
        .fields()
        .iter()
        .map(|f| {
            let dt = deltalake::kernel::DataType::try_from_arrow(f.data_type())?;
            Ok::<StructField, Box<dyn std::error::Error>>(StructField::new(
                f.name().clone(),
                dt,
                f.is_nullable(),
            ))
        })
        .collect::<Result<_, _>>()?;
    let table = CreateBuilder::new()
        .with_log_store(log_store)
        .with_columns(fields)
        .await?;
    table
        .add_constraint()
        .with_constraint("precision_rank_in_range", CONSTRAINT)
        .with_session_state(
            Arc::clone(session) as Arc<dyn datafusion::catalog::Session>
        )
        .await?;
    Ok(url)
}

/// One scoped (or deliberately unscoped) write, exactly as `writer::write_table` will perform it.
async fn write(
    url: &url::Url,
    rows: RecordBatch,
    scope: Option<&str>,
    session: &Arc<datafusion::execution::SessionState>,
) -> Result<(), deltalake::DeltaTableError> {
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
    let snapshot = table.snapshot().ok().map(|s| s.snapshot().clone());

    let mut builder = deltalake::operations::write::WriteBuilder::new(log_store, snapshot)
        .with_input_batches(vec![rows])
        .with_save_mode(SaveMode::Overwrite)
        .with_session_state(Arc::clone(session) as Arc<dyn datafusion::catalog::Session>)
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState);
    if let Some(predicate) = scope {
        builder = builder.with_replace_where(predicate);
    }
    builder.await.map(|_| ())
}

fn scope_for(run: &str) -> String {
    format!("run_id = '{run}'")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("pb17-replace-where")
            .display()
            .to_string()
    });
    let root = std::path::PathBuf::from(root);

    println!("PB17 — a run-scoped overwrite, on a table shaped like a canonical one\n");
    println!("  table     entity_id/entity_key/precision_rank carry ARROW:extension:name");
    println!("  constraint `{CONSTRAINT}`");
    println!("  predicate  {}\n", scope_for("run2"));

    // PB05/PB07: a Delta write needs delta-rs's own session, and the fallback policy must be
    // explicit or a supplied session is discarded with only a log line.
    let session_ctx = create_session().into_inner();
    let session = Arc::new(session_ctx.state());

    let mut failures: Vec<&str> = Vec::new();

    // ---- A: scoped, then scoped. run1 must survive. ----------------------------------------
    println!("  A  run1 scoped -> run2 scoped        run1 must SURVIVE");
    let url = fresh(&root, "t_pb17a", &session).await?;
    let r1 = write(
        &url,
        batch(vec!["run1", "run1"], vec!["k1", "k2"])?,
        Some(&scope_for("run1")),
        &session,
    )
    .await;
    let (v, rows) = observe(&session_ctx, &url, "a1").await;
    println!(
        "      write run1 (into EMPTY)   version={v}  {rows:<28}  {}",
        verdict(&r1)
    );
    let r2 = write(
        &url,
        batch(vec!["run2", "run2"], vec!["k3", "k4"])?,
        Some(&scope_for("run2")),
        &session,
    )
    .await;
    let (v, rows_a) = observe(&session_ctx, &url, "a2").await;
    println!(
        "      write run2 scoped         version={v}  {rows_a:<28}  {}",
        verdict(&r2)
    );
    if !rows_a.contains("run1=2") || !rows_a.contains("run2=2") {
        failures.push("A: a scoped write did not preserve the other run's rows");
    }

    // ---- B: CONTROL. Unscoped, so run1 must be destroyed. -----------------------------------
    println!("\n  B  run1 scoped -> run2 UNSCOPED      CONTROL: run1 must be DESTROYED");
    let url = fresh(&root, "t_pb17b", &session).await?;
    write(
        &url,
        batch(vec!["run1", "run1"], vec!["k1", "k2"])?,
        Some(&scope_for("run1")),
        &session,
    )
    .await?;
    let r = write(
        &url,
        batch(vec!["run2", "run2"], vec!["k3", "k4"])?,
        None,
        &session,
    )
    .await;
    let (v, rows_b) = observe(&session_ctx, &url, "b").await;
    println!(
        "      write run2 unscoped       version={v}  {rows_b:<28}  {}",
        verdict(&r)
    );
    if rows_b.contains("run1") {
        failures.push("B: the CONTROL kept run1, so arm A proves nothing about the predicate");
    }

    // ---- C: a misattributed row must be rejected. -------------------------------------------
    println!("\n  C  batch carries a foreign run_id    must be REJECTED");
    let url = fresh(&root, "t_pb17c", &session).await?;
    write(
        &url,
        batch(vec!["run1", "run1"], vec!["k1", "k2"])?,
        Some(&scope_for("run1")),
        &session,
    )
    .await?;
    let r = write(
        &url,
        batch(vec!["run2", "run1"], vec!["k3", "k4"])?,
        Some(&scope_for("run2")),
        &session,
    )
    .await;
    let (v, rows_c) = observe(&session_ctx, &url, "c").await;
    println!(
        "      write mixed run_ids       version={v}  {rows_c:<28}  {}",
        verdict(&r)
    );
    if r.is_ok() {
        failures.push("C: a batch violating its own predicate was accepted");
    }
    if !rows_c.contains("run1=2") {
        failures.push("C: the rejected write still disturbed the existing rows");
    }

    // ---- D: re-running a scoped write is idempotent in content. -----------------------------
    println!("\n  D  re-run arm A's second write       rows unchanged, version advances");
    let url = fresh(&root, "t_pb17d", &session).await?;
    write(
        &url,
        batch(vec!["run1", "run1"], vec!["k1", "k2"])?,
        Some(&scope_for("run1")),
        &session,
    )
    .await?;
    write(
        &url,
        batch(vec!["run2", "run2"], vec!["k3", "k4"])?,
        Some(&scope_for("run2")),
        &session,
    )
    .await?;
    let (v_before, rows_before) = observe(&session_ctx, &url, "d1").await;
    let r = write(
        &url,
        batch(vec!["run2", "run2"], vec!["k3", "k4"])?,
        Some(&scope_for("run2")),
        &session,
    )
    .await;
    let (v_after, rows_after) = observe(&session_ctx, &url, "d2").await;
    println!("      before re-run             version={v_before}  {rows_before}");
    println!(
        "      after re-run              version={v_after}  {rows_after:<28}  {}",
        verdict(&r)
    );
    if rows_before != rows_after {
        failures.push("D: re-running the same scoped write changed the rows");
    }

    println!("\n  ---");
    if failures.is_empty() {
        println!("  CONFIRMED  a run-scoped overwrite preserves other runs, rejects a");
        println!("             misattributed batch, and is idempotent in content.");
        println!("             The control destroyed what the treatment preserved.");
        Ok(())
    } else {
        for f in &failures {
            println!("  FAILED     {f}");
        }
        Err("PB17 did not confirm".into())
    }
}
