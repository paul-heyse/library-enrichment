//! PB14 — does a `UNION`-flavoured recursive CTE (`is_distinct=true`) terminate on a cyclic graph?
//!
//! PB09 measured only `UNION ALL` and found that an outer bound is not pushed into the recursive
//! term, so a cyclic graph never terminates. PLAN.md had claimed that `is_distinct` "gives cycle
//! tolerance without a written visited-set"; round 2 WITHDREW that claim rather than relying on
//! it, because PB09 had not tested it. This closes the gap.
//!
//!     cargo run --example pb14_recursive_distinct
//!
//! Read from the pinned source first, because it predicts the shape of the answer:
//! `datafusion-physical-plan-55.1.0/src/recursive_query.rs:318` builds a `DistinctDeduplicator`
//! over `Arc::clone(&schema)` -- the FULL output schema. So deduplication is on the whole output
//! tuple, not on a node identity. If the projection carries a `depth` column, every row is unique
//! by construction (depth increments each round), the deduplicator never fires, and `UNION` is no
//! safer than `UNION ALL`. If the projection is the bare node, the domain is finite and it should
//! terminate.
//!
//! That prediction matters because it would make the two mitigations MUTUALLY EXCLUSIVE: the
//! depth column is exactly what a bound needs, and it is exactly what defeats the deduplicator.
//!
//! ARMS  (graph: a->b, b->c, c->a  [CYCLE],  c->d)
//!   A  UNION ALL, projects (node, depth), unbounded    CONTROL: replicates PB09; must NOT finish,
//!                                                      proving the graph really is cyclic
//!   B  UNION,     projects (node, depth), unbounded    the prediction says: also unbounded
//!   C  UNION,     projects (node),        unbounded    the prediction says: terminates
//!   D  UNION ALL, projects (node, depth), BOUNDED      CONTROL: must finish, gives the reference
//!                                                      answer the others are compared against
//!
//! The observation channel is checked before any verdict is read: the physical plan must actually
//! say `is_distinct=true` for arms B and C, or "it behaved like UNION ALL" would mean the flag
//! never arrived rather than that it did not help.

use std::sync::Arc;
use std::time::Duration;

use arrow::array::{RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::TableProviderBuilder;
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;

const TIMEOUT: Duration = Duration::from_secs(15);

fn edge_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("parent", DataType::Utf8, false),
        Field::new("child", DataType::Utf8, false),
    ]))
}

/// `is_distinct` as the PHYSICAL plan reports it. `RecursiveQueryExec`'s `DisplayAs` prints
/// `is_distinct={bool}`, so this reads the flag that execution will actually use rather than the
/// one the SQL text implies.
async fn distinct_flag(ctx: &SessionContext, sql: &str) -> String {
    match ctx.sql(sql).await {
        Ok(df) => match df.create_physical_plan().await {
            Ok(plan) => {
                let text = format!(
                    "{}",
                    datafusion::physical_plan::displayable(plan.as_ref()).indent(true)
                );
                text.lines()
                    .find(|l| l.contains("RecursiveQueryExec"))
                    .map(|l| l.trim().to_string())
                    .unwrap_or_else(|| "<no RecursiveQueryExec in plan>".into())
            }
            Err(e) => format!("<physical error: {e}>"),
        },
        Err(e) => format!("<plan error: {e}>"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let location = std::path::PathBuf::from(root).join("t_pb14");
    let _ = std::fs::remove_dir_all(&location);
    std::fs::create_dir_all(&location)?;

    println!("PB14 — does `is_distinct` tame a cyclic recursive CTE?\n");
    println!("  graph: a->b, b->c, c->a (CYCLE), c->d");
    println!("  timeout: {TIMEOUT:?} per arm; a timeout IS the result\n");

    let url = url::Url::from_directory_path(&location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let fields: Vec<StructField> = edge_schema()
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
        .with_log_store(log_store.clone())
        .with_columns(fields)
        .await?;
    let edges = RecordBatch::try_new(
        edge_schema(),
        vec![
            Arc::new(StringArray::from(vec!["a", "b", "c", "c"])),
            Arc::new(StringArray::from(vec!["b", "c", "a", "d"])),
        ],
    )?;
    let table = table.write(vec![edges]).await?;

    let provider = TableProviderBuilder::default()
        .with_log_store(log_store)
        .with_eager_snapshot(table.snapshot()?.snapshot().clone())
        .build()
        .await?;
    let ctx = SessionContext::new();
    ctx.register_table("edge", Arc::new(provider))?;

    // (label, sql, expectation-in-words)
    let arms: &[(&str, String, &str)] = &[
        (
            "A CONTROL  UNION ALL, (node,depth), unbounded",
            "WITH RECURSIVE closure(node, depth) AS ( \
                SELECT child, 1 FROM edge WHERE parent = 'a' \
                UNION ALL \
                SELECT e.child, c.depth + 1 FROM closure c JOIN edge e ON e.parent = c.node \
             ) SELECT count(*) FROM closure"
                .to_string(),
            "must NOT finish -- proves the graph is cyclic",
        ),
        (
            "B          UNION,     (node,depth), unbounded",
            "WITH RECURSIVE closure(node, depth) AS ( \
                SELECT child, 1 FROM edge WHERE parent = 'a' \
                UNION \
                SELECT e.child, c.depth + 1 FROM closure c JOIN edge e ON e.parent = c.node \
             ) SELECT count(*) FROM closure"
                .to_string(),
            "predicted unbounded: depth makes every tuple unique",
        ),
        (
            "C          UNION,     (node),       unbounded",
            "WITH RECURSIVE closure(node) AS ( \
                SELECT child FROM edge WHERE parent = 'a' \
                UNION \
                SELECT e.child FROM closure c JOIN edge e ON e.parent = c.node \
             ) SELECT count(*) FROM closure"
                .to_string(),
            "predicted terminating: finite node domain",
        ),
        (
            "D CONTROL  UNION ALL, (node,depth), BOUNDED  ",
            "WITH RECURSIVE closure(node, depth) AS ( \
                SELECT child, 1 FROM edge WHERE parent = 'a' \
                UNION ALL \
                SELECT e.child, c.depth + 1 FROM closure c JOIN edge e ON e.parent = c.node \
                  WHERE c.depth < 3 \
             ) SELECT count(*) FROM closure"
                .to_string(),
            "must finish -- the reference answer",
        ),
    ];

    for (label, sql, expectation) in arms {
        println!("  {label}");
        println!("      expectation: {expectation}");
        // Observation channel FIRST. If the flag did not reach the plan, no verdict is readable.
        println!("      plan node:   {}", distinct_flag(&ctx, sql).await);

        let started = std::time::Instant::now();
        match ctx.sql(sql).await {
            Ok(df) => match tokio::time::timeout(TIMEOUT, df.collect()).await {
                Ok(Ok(batches)) => {
                    let n = batches
                        .first()
                        .and_then(|b| {
                            datafusion::common::cast::as_int64_array(b.column(0))
                                .ok()
                                .map(|a| a.value(0))
                        })
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "<no rows>".into());
                    println!(
                        "      RESULT:      TERMINATED count={n} in {:?}",
                        started.elapsed()
                    );
                }
                Ok(Err(e)) => println!(
                    "      RESULT:      ERROR {}",
                    e.to_string().lines().next().unwrap_or("").trim()
                ),
                Err(_) => println!("      RESULT:      TIMED OUT after {TIMEOUT:?} — UNBOUNDED"),
            },
            Err(e) => println!(
                "      RESULT:      <plan error: {}>",
                e.to_string().lines().next().unwrap_or("").trim()
            ),
        }
        println!();
    }
    Ok(())
}
