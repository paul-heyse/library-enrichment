//! PB02 — does a wrapping `TableProvider` that supplies `constraints()` actually change the plan?
//!
//! The plan's §2.3 proposes `bridge::provider::CanonicalTable`, a wrapper around delta-rs's
//! `DeltaScan` that fills in the seven `TableProvider` methods delta-rs leaves at their trait
//! defaults (evidence 05 §1). Two of those — `constraints()` and `statistics()` — are the ones
//! that justify the effort, because they are what feed the optimizer. If DataFusion plans the
//! same query identically with and without them, the wrapper is ceremony and should be dropped.
//!
//! This probe does not assert which optimisation fires. It runs a set of candidate queries
//! against the *same* Delta table through two providers that differ in exactly one respect, and
//! diffs the optimised plans. Whatever differs, differs.
//!
//!     cargo run --example pb02_constraints
//!
//! Control: the wrapper is used in BOTH arms, returning `None` in one and `Some` in the other.
//! That isolates the constraint declaration rather than the fact of being wrapped — a plain
//! `DeltaScan` in one arm would confound the two.

use std::sync::Arc;

use arrow::array::{Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use async_trait::async_trait;
use datafusion::catalog::{Session, TableProvider};
use datafusion::common::{Constraint, Constraints, Statistics};
use datafusion::datasource::TableType;
use datafusion::error::Result as DFResult;
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown};
use datafusion::physical_plan::ExecutionPlan;
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::TableProviderBuilder;
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;
use deltalake::operations::write::WriteBuilder;

/// The wrapper the plan proposes, reduced to the one variable under test.
#[derive(Debug)]
struct CanonicalTable {
    inner: Arc<dyn TableProvider>,
    constraints: Option<Constraints>,
    statistics: Option<Statistics>,
}

#[async_trait]
impl TableProvider for CanonicalTable {
    fn schema(&self) -> SchemaRef {
        self.inner.schema()
    }
    fn table_type(&self) -> TableType {
        self.inner.table_type()
    }

    // The variable under test. delta-rs leaves this at the trait default (None).
    fn constraints(&self) -> Option<&Constraints> {
        self.constraints.as_ref()
    }

    // Secondary: delta-rs also leaves this at None.
    fn statistics(&self) -> Option<Statistics> {
        self.statistics.clone()
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> DFResult<Vec<TableProviderFilterPushDown>> {
        self.inner.supports_filters_pushdown(filters)
    }

    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> DFResult<Arc<dyn ExecutionPlan>> {
        self.inner.scan(state, projection, filters, limit).await
    }
}

/// Queries chosen because `datafusion-optimizer` reads functional dependencies in exactly three
/// files: `replace_distinct_aggregate.rs`, `eliminate_join.rs` and `mod.rs`. Each query below
/// targets one of those paths; the rest are there to show what does NOT change.
const QUERIES: &[(&str, &str)] = &[
    (
        "distinct on the key alone",
        "SELECT DISTINCT entity_id FROM t",
    ),
    (
        "distinct on key + dependent column",
        "SELECT DISTINCT entity_id, kind FROM t",
    ),
    (
        "distinct on a non-key column",
        "SELECT DISTINCT kind FROM t",
    ),
    (
        "group by the key",
        "SELECT entity_id, count(*) FROM t GROUP BY entity_id",
    ),
    (
        "count(distinct key)",
        "SELECT count(DISTINCT entity_id) FROM t",
    ),
    (
        "self-join on the key",
        "SELECT a.entity_id FROM t a JOIN t b ON a.entity_id = b.entity_id",
    ),
    (
        "plain filter (should not change)",
        "SELECT kind FROM t WHERE entity_id = 'e1'",
    ),
];

async fn optimised_plan(ctx: &SessionContext, sql: &str) -> String {
    match ctx.sql(sql).await {
        Ok(df) => match df.into_optimized_plan() {
            Ok(plan) => format!("{}", plan.display_indent()),
            Err(e) => format!("<optimise error: {e}>"),
        },
        Err(e) => format!("<plan error: {e}>"),
    }
}

/// The physical plan rendered WITH statistics. `datafusion.explain.show_statistics` governs
/// `EXPLAIN` output only; the programmatic display needs `set_show_statistics(true)`. Getting
/// this wrong makes an unobserved result look like a negative one.
async fn physical_plan(ctx: &SessionContext, sql: &str) -> String {
    match ctx.sql(sql).await {
        Ok(df) => match df.create_physical_plan().await {
            Ok(plan) => format!(
                "{}",
                datafusion::physical_plan::displayable(plan.as_ref())
                    .set_show_statistics(true)
                    .indent(true)
            ),
            Err(e) => format!("<physical error: {e}>"),
        },
        Err(e) => format!("<plan error: {e}>"),
    }
}

/// The unambiguous channel: ask the physical root what it thinks its cardinality is.
async fn root_rows(ctx: &SessionContext, sql: &str) -> String {
    match ctx.sql(sql).await {
        Ok(df) => match df.create_physical_plan().await {
            Ok(plan) => {
                let all = plan
                    .partition_statistics(None)
                    .map(|s| format!("{:?}", s.num_rows))
                    .unwrap_or_else(|e| format!("<err {e}>"));
                let p0 = plan
                    .partition_statistics(Some(0))
                    .map(|s| format!("{:?}", s.num_rows))
                    .unwrap_or_else(|e| format!("<err {e}>"));
                format!("all={all} part0={p0}")
            }
            Err(e) => format!("<physical error: {e}>"),
        },
        Err(e) => format!("<plan error: {e}>"),
    }
}

async fn build_context(
    table: Arc<dyn TableProvider>,
    constraints: Option<Constraints>,
    statistics: Option<Statistics>,
) -> DFResult<SessionContext> {
    let ctx = SessionContext::new();
    ctx.register_table(
        "t",
        Arc::new(CanonicalTable {
            inner: table,
            constraints,
            statistics,
        }),
    )?;
    Ok(ctx)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let location = std::path::PathBuf::from(root).join("t_pb02");
    let _ = std::fs::remove_dir_all(&location);
    std::fs::create_dir_all(&location)?;

    println!("PB02 — does supplying `constraints()` change the optimised plan?\n");
    println!("  Both arms use the SAME wrapper over the SAME Delta table.");
    println!("  Arm A: constraints() -> None            (what delta-rs does today)");
    println!("  Arm B: constraints() -> PrimaryKey([0]) (what the plan's wrapper would add)\n");

    // ---- one Delta table, three columns, entity_id at index 0 -------------------------
    let url = url::Url::from_directory_path(&location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let log_store2 = log_store.clone();

    let arrow_schema: SchemaRef = Arc::new(Schema::new(vec![
        Field::new("entity_id", DataType::Utf8, false),
        Field::new("kind", DataType::Utf8, true),
        Field::new("n", DataType::Int64, true),
    ]));
    let delta_fields: Vec<StructField> = arrow_schema
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
        .with_columns(delta_fields)
        .await?;

    let batch = RecordBatch::try_new(
        arrow_schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["e1", "e2", "e3"])),
            Arc::new(StringArray::from(vec![
                Some("fn"),
                Some("struct"),
                Some("fn"),
            ])),
            Arc::new(Int64Array::from(vec![1, 2, 3])),
        ],
    )?;
    let _ = WriteBuilder::new(log_store, table.state.map(|s| s.snapshot().clone()))
        .with_input_batches(vec![batch])
        .await?;

    // `DeltaTable` is NOT a `TableProvider` at this rev. The provider is built separately —
    // `deltalake::delta_datafusion::TableProviderBuilder` -> `DeltaScan`.
    let mut delta = DeltaTableBuilder::from_url(url)?.load().await?;
    delta.load().await?;
    let snapshot = delta.snapshot()?.snapshot().clone();
    let scan = TableProviderBuilder::default()
        .with_log_store(log_store2)
        .with_eager_snapshot(snapshot)
        .build()
        .await?;
    let provider: Arc<dyn TableProvider> = Arc::new(scan);

    // ---- PB02a: constraints -----------------------------------------------------------
    let without = build_context(provider.clone(), None, None).await?;
    let with = build_context(
        provider.clone(),
        Some(Constraints::new_unverified(vec![Constraint::PrimaryKey(
            vec![0],
        )])),
        None,
    )
    .await?;

    println!("== PB02a · constraints() ==\n");
    let mut changed = 0usize;
    for (label, sql) in QUERIES {
        let a = optimised_plan(&without, sql).await;
        let b = optimised_plan(&with, sql).await;
        if a == b {
            println!("  [same]    {label}");
        } else {
            changed += 1;
            println!("  [CHANGED] {label}");
            println!("            sql: {sql}");
            for line in a.lines() {
                println!("              A(None) | {line}");
            }
            for line in b.lines() {
                println!("              B(PK)   | {line}");
            }
        }
    }
    println!(
        "\n  {changed} of {} queries planned differently.\n",
        QUERIES.len()
    );

    // ---- PB02b: statistics ------------------------------------------------------------
    //
    // The first version of this sub-probe supplied `Statistics::new_unknown`, whose every field
    // is `Precision::Absent`. That carries no information, so "the plan did not change" was the
    // expected and uninformative answer — a probe that could not have failed. It is replaced
    // here with statistics that actually say something, and the observation moves to the
    // PHYSICAL plan with `datafusion.explain.show_statistics` on, because that is where
    // cardinality is visible at all.
    println!("== PB02b · statistics() ==\n");

    let with_stats_cfg = || {
        datafusion::prelude::SessionConfig::new()
            .set_bool("datafusion.explain.show_statistics", true)
    };

    let mut real = Statistics::new_unknown(&arrow_schema);
    real.num_rows = datafusion::common::stats::Precision::Exact(3);
    real.total_byte_size = datafusion::common::stats::Precision::Exact(96);

    let ctx_none = SessionContext::new_with_config(with_stats_cfg());
    ctx_none.register_table(
        "t",
        Arc::new(CanonicalTable {
            inner: provider.clone(),
            constraints: None,
            statistics: None,
        }),
    )?;
    let ctx_stats = SessionContext::new_with_config(with_stats_cfg());
    ctx_stats.register_table(
        "t",
        Arc::new(CanonicalTable {
            inner: provider.clone(),
            constraints: None,
            statistics: Some(real.clone()),
        }),
    )?;

    for (label, sql) in [
        ("scan", "SELECT entity_id FROM t"),
        (
            "self-join",
            "SELECT a.entity_id FROM t a JOIN t b ON a.entity_id = b.entity_id",
        ),
    ] {
        let a = physical_plan(&ctx_none, sql).await;
        let b = physical_plan(&ctx_stats, sql).await;
        let ra = root_rows(&ctx_none, sql).await;
        let rb = root_rows(&ctx_stats, sql).await;
        println!(
            "  {label:<10} plan: {:<8}  root num_rows  A(None)={ra}  B(Exact 3)={rb}",
            if a == b { "same" } else { "CHANGED" }
        );
        // Show the rendered plan regardless of the verdict. If no statistics text appears
        // at all, the observation channel is broken and "same" means "not observed" rather
        // than "not used" -- two results that must never be confused.
        for line in b.lines().take(4) {
            println!("      rendered | {line}");
        }
        println!(
            "      channel  | statistics rendered in plan text: {}",
            b.contains("statistics=")
        );
    }
    println!();

    // ---- what the schema actually carries ---------------------------------------------
    println!("== what reached the plan ==\n");
    let df = with.sql("SELECT entity_id, kind, n FROM t").await?;
    println!(
        "  functional dependencies on the scan schema (arm B): {:?}",
        df.schema().functional_dependencies()
    );
    let df0 = without.sql("SELECT entity_id, kind, n FROM t").await?;
    println!(
        "  functional dependencies on the scan schema (arm A): {:?}",
        df0.schema().functional_dependencies()
    );
    Ok(())
}
