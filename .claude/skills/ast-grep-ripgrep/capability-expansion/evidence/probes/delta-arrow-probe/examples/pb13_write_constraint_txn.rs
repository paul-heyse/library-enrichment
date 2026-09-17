//! PB13 — does `WriteBuilder::with_input_plan` share a transaction with a subsequent
//! `ConstraintBuilder` call?
//!
//! Evidence 05 §4 question 4, the last open question from the round-1 Delta dossier. Plan v2 §6.4
//! sidesteps it by ordering — constraints are added in a DDL phase before any data is written, so
//! the interleaving never occurs — but "we avoid the question" is only a sound design if the
//! answer would otherwise have hurt. This measures whether it would.
//!
//!     cargo run --example pb13_write_constraint_txn
//!
//! Three things are observable and all three matter:
//!   1. COMMIT GRANULARITY   — does the table version advance once or twice?
//!   2. FAILURE ATOMICITY    — if the constraint add fails because existing data violates it,
//!                             does the earlier write survive, or is it rolled back?
//!   3. WHETHER DDL-FIRST HELPS — v2 §6.4 claims ordering makes the question moot. Arms C and D
//!                             test that claim rather than assuming it.
//!
//! ARMS
//!   A  create -> write CONFORMING -> add constraint        both should succeed
//!   B  create -> write VIOLATING  -> add constraint        add should fail; what happened to
//!                                                          the write?
//!   C  create -> add constraint   -> write VIOLATING       the v2 §6.4 order; write should fail
//!   D  create -> add constraint   -> write CONFORMING      CONTROL: must succeed, or arm C's
//!                                                          failure proves nothing
//!
//! Arm D is load-bearing. Without it, "the write failed in arm C" could mean the constraint broke
//! writing altogether rather than rejecting the specific rows — the same confound that made the
//! first PB07 run meaningless.

use std::sync::Arc;

use arrow::array::{Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::logical_expr::LogicalPlan;
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::{SessionFallbackPolicy, create_session};
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;

const CONSTRAINT: &str = "off >= 0";

fn schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("entity_id", DataType::Utf8, false),
        Field::new("off", DataType::Int64, true),
    ]))
}

fn batch(ids: Vec<&str>, offs: Vec<i64>) -> Result<RecordBatch, Box<dyn std::error::Error>> {
    Ok(RecordBatch::try_new(
        schema(),
        vec![
            Arc::new(StringArray::from(ids)),
            Arc::new(Int64Array::from(offs)),
        ],
    )?)
}

/// A `LogicalPlan` source, which is what the question is actually about — not `with_input_batches`.
fn plan_for(
    ctx: &SessionContext,
    b: RecordBatch,
) -> Result<LogicalPlan, Box<dyn std::error::Error>> {
    Ok(ctx.read_batch(b)?.into_unoptimized_plan())
}

fn verdict<T, E: std::fmt::Display>(r: &Result<T, E>) -> String {
    match r {
        Ok(_) => "OK".into(),
        Err(e) => {
            let msg = e.to_string();
            let first = msg.lines().next().unwrap_or("").trim();
            format!("ERR: {}", &first[..first.len().min(110)])
        }
    }
}

/// Version, row count and constraint presence, read from a FRESH load so nothing in-process is
/// being trusted. `None` for rows means the table could not be scanned.
async fn observe(url: &url::Url) -> (String, String, String) {
    let Ok(builder) = DeltaTableBuilder::from_url(url.clone()) else {
        return ("<bad url>".into(), "-".into(), "-".into());
    };
    let Ok(table) = builder.load().await else {
        return ("<load failed>".into(), "-".into(), "-".into());
    };
    let version = table
        .version()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "<none>".into());
    let constraints = match table.snapshot() {
        Ok(s) => {
            let found: Vec<String> = s
                .metadata()
                .configuration()
                .iter()
                .filter(|(k, _)| k.starts_with("delta.constraints."))
                .map(|(k, v)| format!("{}={v}", k.trim_start_matches("delta.constraints.")))
                .collect();
            if found.is_empty() {
                "none".into()
            } else {
                found.join(",")
            }
        }
        Err(_) => "<no snapshot>".into(),
    };
    let files = table
        .get_file_uris()
        .map(|it| it.count().to_string())
        .unwrap_or_else(|_| "-".into());
    (version, files, constraints)
}

async fn fresh(root: &std::path::Path, name: &str) -> Result<url::Url, Box<dyn std::error::Error>> {
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
    CreateBuilder::new()
        .with_log_store(log_store)
        .with_columns(fields)
        .await?;
    Ok(url)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let root = std::path::PathBuf::from(root);

    println!("PB13 — write and constraint: one transaction, or two?\n");
    println!("  constraint under test: `{CONSTRAINT}`");
    println!("  conforming rows: off = 10, 20      violating row: off = -5\n");

    // Per PB05/PB07: delta writes need delta-rs's own session, and the fallback policy must be
    // explicit or a non-SessionState session is silently discarded.
    let session_ctx = create_session().into_inner();
    let session = Arc::new(session_ctx.state());

    for (label, ddl_first, conforming) in [
        ("A  write CONFORMING then constraint ", false, true),
        ("B  write VIOLATING  then constraint ", false, false),
        ("C  constraint then write VIOLATING  ", true, false),
        ("D  constraint then write CONFORMING ", true, true),
    ] {
        let name = format!(
            "t_pb13{}",
            label.chars().next().unwrap().to_ascii_lowercase()
        );
        let url = fresh(&root, &name).await?;
        let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;

        let rows = if conforming {
            batch(vec!["e1", "e2"], vec![10, 20])?
        } else {
            batch(vec!["e1", "e2"], vec![10, -5])?
        };

        println!("  {label}");
        let (v, f, c) = observe(&url).await;
        println!("      after create              version={v} files={f} constraints={c}");

        if ddl_first {
            let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
            let r = table
                .add_constraint()
                .with_constraint("off_nonneg", CONSTRAINT)
                .with_session_state(session.clone())
                .await;
            let (v, f, c) = observe(&url).await;
            println!(
                "      after add constraint      version={v} files={f} constraints={c}   {}",
                verdict(&r)
            );

            let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
            let snapshot = table.snapshot().ok().map(|s| s.snapshot().clone());
            let r = deltalake::operations::write::WriteBuilder::new(log_store.clone(), snapshot)
                .with_input_plan(plan_for(&session_ctx, rows)?)
                .with_session_state(session.clone())
                .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
                .await;
            let (v, f, c) = observe(&url).await;
            println!(
                "      after write (input_plan)  version={v} files={f} constraints={c}   {}",
                verdict(&r)
            );
        } else {
            let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
            let snapshot = table.snapshot().ok().map(|s| s.snapshot().clone());
            let r = deltalake::operations::write::WriteBuilder::new(log_store.clone(), snapshot)
                .with_input_plan(plan_for(&session_ctx, rows)?)
                .with_session_state(session.clone())
                .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
                .await;
            let (v, f, c) = observe(&url).await;
            println!(
                "      after write (input_plan)  version={v} files={f} constraints={c}   {}",
                verdict(&r)
            );

            let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
            let r = table
                .add_constraint()
                .with_constraint("off_nonneg", CONSTRAINT)
                .with_session_state(session.clone())
                .await;
            let (v, f, c) = observe(&url).await;
            println!(
                "      after add constraint      version={v} files={f} constraints={c}   {}",
                verdict(&r)
            );
        }
        println!();
    }

    println!("  Reading the table: each successful operation advances `version` by one if it is");
    println!("  its own commit. A constraint add that FAILS should leave version and files as");
    println!("  they were -- if it instead removed the data, arm B would show files dropping.\n");
    Ok(())
}
