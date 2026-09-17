//! PB09 + PB10 — the two questions that decide how closure queries and incremental
//! re-derivation are written.
//!
//! PB09  Does recursive-CTE execution push an outer predicate INTO the recursive term?
//!       Plan §6.4 replaces a graph database with `LogicalPlan::RecursiveQuery` for six closure
//!       questions. If an outer `WHERE depth <= k` is not pushed down, the recursion runs to
//!       fixpoint first and the bound is decoration — which on a cyclic graph means it never
//!       terminates. The plan's mitigation ("explicit depth column and bound") becomes mandatory
//!       rather than optional.
//!
//! PB10  Does `DeltaCdfTableProvider` support filter pushdown, and does a pushed filter actually
//!       narrow what is read? Plan §6.3 makes incremental re-derivation a query against the
//!       change feed.
//!
//! Read from the pinned source first, because it predicts a result that a naive probe would
//! misread as a success:
//!
//! - `delta_datafusion/cdf/scan.rs:116` — `supports_filters_pushdown` returns
//!   `TableProviderFilterPushDown::Exact` for EVERY filter, unconditionally, with the source
//!   comment `// maybe exact`. DataFusion therefore removes the filter from the plan above the
//!   scan. Seeing "the filter disappeared" is NOT evidence that it narrowed anything.
//! - `operations/load_cdf.rs:68` — the builder's own doc: the predicate is "used ONLY to prune
//!   files by their partition values. This is never applied as a row-level filter, so any
//!   non-partition conjuncts are ignored here". Correctness is restored by a `FilterExec` the
//!   provider wraps around its own plan.
//!
//! So the real question is not "is it accepted" (it always is) but "does it narrow", and the two
//! arms that separate those are a PARTITION-column filter against a DATA-column filter.
//!
//!     cargo run --example pb09_pb10_recursion_cdf

use std::sync::Arc;
use std::time::Duration;

use arrow::array::{Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::{DeltaCdfTableProvider, TableProviderBuilder};
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;
use deltalake::table::config::TableProperty;

/// How long an unbounded recursion is allowed to run before the probe calls it unbounded.
/// A timeout IS a result here, and a probe that could hang is a probe that cannot be re-run.
const RECURSION_TIMEOUT: Duration = Duration::from_secs(20);

fn edge_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("parent", DataType::Utf8, false),
        Field::new("child", DataType::Utf8, false),
    ]))
}

fn cdf_schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("family", DataType::Utf8, false),
        Field::new("entity_id", DataType::Utf8, false),
        Field::new("n", DataType::Int64, true),
    ]))
}

fn delta_fields(schema: &SchemaRef) -> Result<Vec<StructField>, Box<dyn std::error::Error>> {
    schema
        .fields()
        .iter()
        .map(|f| {
            let dt = deltalake::kernel::DataType::try_from_arrow(f.data_type())?;
            Ok(StructField::new(f.name().clone(), dt, f.is_nullable()))
        })
        .collect()
}

fn fresh(root: &std::path::Path, name: &str) -> Result<url::Url, Box<dyn std::error::Error>> {
    let location = root.join(name);
    let _ = std::fs::remove_dir_all(&location);
    std::fs::create_dir_all(&location)?;
    Ok(url::Url::from_directory_path(&location).map_err(|_| "bad path")?)
}

/// Is the DEPTH BOUND inside the recursive term?
///
/// An earlier version of this asked only whether any `Filter:` appeared below `RecursiveQuery`,
/// and answered `true` for both arms -- because the base term carries `Filter: edge.parent = 'a'`,
/// which is inside the node but is not the bound. That contradicted the execution result, and a
/// plan-shape observable that disagrees with execution is worse than no observable at all. The
/// question is specifically whether a filter ON THE DEPTH COLUMN sits in the recursive subtree.
fn depth_bound_inside_recursive_term(plan_text: &str) -> bool {
    let mut seen_recursive = false;
    for line in plan_text.lines() {
        if line.contains("RecursiveQuery") {
            seen_recursive = true;
            continue;
        }
        let trimmed = line.trim_start();
        if seen_recursive && trimmed.starts_with("Filter:") && trimmed.contains("depth") {
            return true;
        }
    }
    false
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let root = std::path::PathBuf::from(root);

    // ================= PB09 — recursive CTEs =================
    println!("PB09 — is an outer predicate pushed into the recursive term?\n");

    // A graph with a CYCLE: a -> b -> c -> a, plus a tail d. Without a bound inside the
    // recursive term, the closure never reaches a fixpoint. That is what makes "not pushed"
    // observable as a timeout rather than as an opinion about plan shape.
    let url = fresh(&root, "t_pb09")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let table = CreateBuilder::new()
        .with_log_store(log_store.clone())
        .with_columns(delta_fields(&edge_schema())?)
        .await?;
    let edges = RecordBatch::try_new(
        edge_schema(),
        vec![
            Arc::new(StringArray::from(vec!["a", "b", "c", "c"])),
            Arc::new(StringArray::from(vec!["b", "c", "a", "d"])),
        ],
    )?;
    let table = table.write(vec![edges]).await?;
    println!("  graph: a->b, b->c, c->a (CYCLE), c->d\n");

    let snapshot = table.snapshot()?.snapshot().clone();
    let provider = TableProviderBuilder::default()
        .with_log_store(log_store.clone())
        .with_eager_snapshot(snapshot)
        .build()
        .await?;
    let ctx = SessionContext::new();
    ctx.register_table("edge", Arc::new(provider))?;

    // Arm A: the bound lives OUTSIDE the CTE. If it is pushed into the recursive term, this
    // terminates; if not, it recurses forever on the cycle.
    let outer = "WITH RECURSIVE closure(node, depth) AS ( \
                    SELECT child, 1 FROM edge WHERE parent = 'a' \
                    UNION ALL \
                    SELECT e.child, c.depth + 1 FROM closure c JOIN edge e ON e.parent = c.node \
                 ) SELECT count(*) FROM closure WHERE depth <= 3";

    // Arm B: the bound lives INSIDE the recursive term. Must terminate. This is the control —
    // if B also hangs, the probe is measuring something other than pushdown.
    let inner = "WITH RECURSIVE closure(node, depth) AS ( \
                    SELECT child, 1 FROM edge WHERE parent = 'a' \
                    UNION ALL \
                    SELECT e.child, c.depth + 1 FROM closure c JOIN edge e ON e.parent = c.node \
                      WHERE c.depth < 3 \
                 ) SELECT count(*) FROM closure";

    for (label, sql) in [
        ("A bound OUTSIDE the CTE", outer),
        ("B bound INSIDE  (CONTROL)", inner),
    ] {
        match ctx.sql(sql).await {
            Ok(df) => {
                let plan = df.clone().into_optimized_plan()?;
                let text = format!("{}", plan.display_indent());
                let pushed = depth_bound_inside_recursive_term(&text);
                println!("  {label}");
                println!("      depth bound inside recursive term (plan): {pushed}");
                let started = std::time::Instant::now();
                match tokio::time::timeout(RECURSION_TIMEOUT, df.collect()).await {
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
                        println!("      executed: count={n} in {:?}", started.elapsed());
                    }
                    Ok(Err(e)) => {
                        let msg = e.to_string();
                        println!(
                            "      executed: ERROR {}",
                            msg.lines().next().unwrap_or("").trim()
                        );
                    }
                    Err(_) => {
                        println!(
                            "      executed: TIMED OUT after {RECURSION_TIMEOUT:?} — UNBOUNDED"
                        );
                    }
                }
                for line in text.lines() {
                    println!("      plan | {line}");
                }
            }
            Err(e) => println!("  {label}\n      <plan error: {e}>"),
        }
        println!();
    }

    // ================= PB10 — the change feed as a table =================
    println!("PB10 — does a filter on the CDF provider narrow what is read?\n");

    let url = fresh(&root, "t_pb10")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let table = CreateBuilder::new()
        .with_log_store(log_store.clone())
        .with_columns(delta_fields(&cdf_schema())?)
        .with_partition_columns(vec!["family"])
        .with_configuration_property(TableProperty::EnableChangeDataFeed, Some("true"))
        .await?;
    println!("  table partitioned by `family`, CDF enabled\n");

    // Three commits across two partitions, so there is a change range with more than one
    // version and more than one partition to narrow to.
    let mut table = table;
    for (version, family, ids) in [
        (0, "hir", vec!["h1", "h2"]),
        (1, "mir", vec!["m1", "m2"]),
        (2, "hir", vec!["h3", "h4"]),
    ] {
        let n: Vec<i64> = (0..ids.len() as i64).map(|i| version * 10 + i).collect();
        let families: Vec<&str> = ids.iter().map(|_| family).collect();
        let batch = RecordBatch::try_new(
            cdf_schema(),
            vec![
                Arc::new(StringArray::from(families)),
                Arc::new(StringArray::from(ids)),
                Arc::new(Int64Array::from(n)),
            ],
        )?;
        table = table.write(vec![batch]).await?;
    }
    println!("  wrote 3 commits: hir, mir, hir\n");

    let reloaded = DeltaTableBuilder::from_url(url.clone())?.load().await?;
    let cdf_builder = reloaded.scan_cdf().with_starting_version(0);
    let cdf_provider = DeltaCdfTableProvider::try_new(cdf_builder)?;

    let cctx = SessionContext::new();
    cctx.register_table("changes", Arc::new(cdf_provider))?;

    println!(
        "  CDF schema: {:?}\n",
        cctx.table("changes")
            .await?
            .schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect::<Vec<_>>()
    );

    for (label, sql) in [
        ("no filter          ", "SELECT count(*) FROM changes"),
        (
            "PARTITION col      ",
            "SELECT count(*) FROM changes WHERE family = 'hir'",
        ),
        (
            "DATA col           ",
            "SELECT count(*) FROM changes WHERE n >= 20",
        ),
        (
            "CDF metadata col   ",
            "SELECT count(*) FROM changes WHERE _commit_version >= 2",
        ),
    ] {
        match cctx.sql(sql).await {
            Ok(df) => {
                let plan = df.clone().create_physical_plan().await?;
                let rendered = format!(
                    "{}",
                    datafusion::physical_plan::displayable(plan.as_ref())
                        .set_show_statistics(true)
                        .indent(true)
                );
                // The narrowing observable: how many rows the leaf reports reading. If the
                // predicate only prunes partitions, a data-column filter leaves this at the full
                // change-range size while still producing the right answer via FilterExec.
                // The SCAN-level nodes, not the aggregates above them. An earlier version
                // matched every line containing `statistics=`, which selected the four
                // aggregate nodes and showed nothing about how much was read -- so "narrowing"
                // would have been reported without evidence for it.
                let leaf = rendered
                    .lines()
                    .filter(|l| {
                        l.contains("DataSourceExec")
                            || l.contains("DeltaCdf")
                            || l.contains("FilterExec")
                            || l.contains("file_groups")
                    })
                    .map(|l| l.trim().to_string())
                    .collect::<Vec<_>>();
                let n = match df.collect().await {
                    Ok(batches) => batches
                        .first()
                        .and_then(|b| {
                            datafusion::common::cast::as_int64_array(b.column(0))
                                .ok()
                                .map(|a| a.value(0))
                        })
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "<no rows>".into()),
                    Err(e) => format!(
                        "<exec error: {}>",
                        e.to_string().lines().next().unwrap_or("")
                    ),
                };
                let has_filter_exec = rendered.contains("FilterExec");
                println!("  {label} count={n}  FilterExec present={has_filter_exec}");
                if leaf.is_empty() {
                    println!("      plan | <no scan-level node matched -- channel not observed>");
                }
                for line in leaf.iter().take(8) {
                    println!("      plan | {line}");
                }
            }
            Err(e) => println!(
                "  {label} <plan error: {}>",
                e.to_string().lines().next().unwrap_or("")
            ),
        }
        println!();
    }
    Ok(())
}
