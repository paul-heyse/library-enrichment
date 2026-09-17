//! PB06 — does `ScalarUDFImpl::preimage` fire for a Delta-backed scan, and does the rewritten
//! predicate actually prune files?
//!
//! The highest-value probe in the backlog. Plan §5.1 requires every domain UDF that may appear in
//! a filter to implement `preimage`, because that is what turns `f(col) = lit` into a range on
//! `col` that the Delta transaction log can use to skip files. If it does not fire, domain UDFs
//! are barred from hot filters and the design falls back to precomputed `GeneratedColumns`.
//!
//! Read from the pinned source before writing this, because the mechanism constrains the probe:
//! `datafusion-optimizer-55.1.0/src/simplify_expressions/expr_simplifier.rs:2111` — `get_preimage`
//! returns `None` unless the left side is an `Expr::ScalarFunction`, the right side is a literal,
//! AND `func.signature().volatility == Volatility::Immutable`. A `Volatile` UDF is silently
//! skipped with no diagnostic, which is a trap worth demonstrating rather than describing.
//!
//! The table is written as FOUR separate files with disjoint `off` ranges, one per bucket, so
//! that pruning is observable as a row count rather than inferred from plan shape.
//!
//!     cargo run --example pb06_preimage
//!
//! ARMS
//!   1. `offset_bucket(off) = 2`    — the domain UDF, Immutable, implements `preimage`
//!   2. `offset_bucket_np(off) = 2` — CONTROL: byte-identical, `preimage` left at trait default
//!   3. `offset_bucket_vol(off) = 2`— CONTROL: implements `preimage` but declares Volatile
//!   4. `off >= 8192 AND off <= 12287` — GOLD STANDARD: what a perfect rewrite would produce
//!   5. `floor(off / 4096.0) = 2.0` — CONTROL: a BUILT-IN that ships `preimage`, proving the
//!      machinery is reachable from a Delta scan at all, independent of any UDF this file defines
//!
//! Also folded in: PB11, the unexplained disagreement recorded in evidence 11 between
//! `ExecutionPlan::partition_statistics` and the rendered plan text. Here it is asked of a
//! MemTable as well as a Delta table, which is what distinguishes "delta-rs does not populate
//! that channel" from "that channel is not populated by anyone".

use std::sync::Arc;

use arrow::array::{Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::common::cast::as_int64_array;
use datafusion::common::{Result as DFResult, ScalarValue};
use datafusion::logical_expr::interval_arithmetic::Interval;
use datafusion::logical_expr::preimage::PreimageResult;
use datafusion::logical_expr::simplify::SimplifyContext;
use datafusion::logical_expr::{
    ColumnarValue, Expr, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
};
use datafusion::physical_plan::statistics::{StatisticsArgs, StatisticsContext};
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::TableProviderBuilder;
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;
use deltalake::operations::write::WriteBuilder;

const BUCKET: i64 = 4096;
const ROWS_PER_FILE: i64 = 100;
const FILES: i64 = 4;

/// How this instance answers `preimage`. The distinction between `HalfOpen` and `Closed` is the
/// whole point of arms 1 and 2: `datafusion-optimizer-55.1.0/src/simplify_expressions/
/// udf_preimage.rs:47` rewrites an `Eq` comparison into `expr GtEq lower AND expr Lt upper`, so
/// the interval is consumed as HALF-OPEN even though `Interval` is nominally a closed-interval
/// type. A `preimage` returning the inclusive upper bound therefore drops the last value of every
/// bucket, with no error and no warning.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PreimageMode {
    /// upper = (k + 1) * BUCKET -- matches what the rewrite expects.
    HalfOpen,
    /// upper = (k + 1) * BUCKET - 1 -- the natural-looking mistake.
    Closed,
    /// `preimage` left at its trait default.
    None,
}

/// `offset_bucket(off)` buckets the plan's `Int64` byte offsets by 4096 (plan §3.1 forces
/// `Int64`; `UInt32` silently truncates above `i32::MAX`).
///
/// `mode` and `volatility` are the two variables under test. Everything else is identical across
/// the instances, so a difference in plan is caused by one of them.
#[derive(Debug, PartialEq, Eq, Hash)]
struct OffsetBucket {
    signature: Signature,
    name: String,
    mode: PreimageMode,
}

impl OffsetBucket {
    fn new(name: &str, mode: PreimageMode, volatility: Volatility) -> Self {
        Self {
            signature: Signature::exact(vec![DataType::Int64], volatility),
            name: name.to_string(),
            mode,
        }
    }
}

impl ScalarUDFImpl for OffsetBucket {
    fn name(&self) -> &str {
        &self.name
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _arg_types: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Int64)
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let input = as_int64_array(&arrays[0])?;
        let out: Int64Array = input.iter().map(|v| v.map(|v| v / BUCKET)).collect();
        Ok(ColumnarValue::Array(Arc::new(out)))
    }

    /// The solution set of `offset_bucket(off) = k` runs from `k*4096` up to but NOT including
    /// `(k+1)*4096`.
    ///
    /// NOTE the returned `expr`: it is `args[0]`, the BARE COLUMN. Arm 6 shows that a rewrite
    /// onto a derived expression (which is what the built-in `floor` produces) is still
    /// rewritten but cannot prune, because Delta's min/max statistics are keyed on stored
    /// columns.
    fn preimage(
        &self,
        args: &[Expr],
        lit_expr: &Expr,
        _info: &SimplifyContext,
    ) -> DFResult<PreimageResult> {
        let upper_adjust = match self.mode {
            PreimageMode::None => return Ok(PreimageResult::None),
            PreimageMode::HalfOpen => 0,
            PreimageMode::Closed => -1,
        };
        let Expr::Literal(ScalarValue::Int64(Some(k)), _) = lit_expr else {
            return Ok(PreimageResult::None);
        };
        let lower = ScalarValue::Int64(Some(k * BUCKET));
        let upper = ScalarValue::Int64(Some((k + 1) * BUCKET + upper_adjust));
        Ok(PreimageResult::Range {
            expr: args[0].clone(),
            interval: Box::new(Interval::try_new(lower, upper)?),
        })
    }
}

async fn logical(ctx: &SessionContext, sql: &str) -> String {
    match ctx.sql(sql).await {
        Ok(df) => match df.into_optimized_plan() {
            Ok(plan) => format!("{}", plan.display_indent()),
            Err(e) => format!("<optimise error: {e}>"),
        },
        Err(e) => format!("<plan error: {e}>"),
    }
}

async fn physical(ctx: &SessionContext, sql: &str) -> String {
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

/// The pruning observable. `DataSourceExec`'s `Rows=Exact(n)` after pruning is the number of rows
/// in the files that survived file skipping — so a successful rewrite shows 100, not 400.
fn scanned_rows(plan_text: &str) -> Vec<String> {
    plan_text
        .lines()
        .filter(|l| l.contains("DataSourceExec") || l.contains("DeltaScanExec"))
        .map(|l| {
            let tag = if l.contains("DataSourceExec") {
                "DataSourceExec"
            } else {
                "DeltaScanExec"
            };
            let stats = l
                .split("statistics=")
                .nth(1)
                .map(|s| s.split(']').next().unwrap_or(s).to_string())
                .unwrap_or_else(|| "<none>".into());
            format!("{tag} statistics={stats}]")
        })
        .collect()
}

async fn count(ctx: &SessionContext, sql: &str) -> String {
    match ctx.sql(sql).await {
        Ok(df) => match df.collect().await {
            Ok(batches) => batches
                .first()
                .and_then(|b| as_int64_array(b.column(0)).ok().map(|a| a.value(0)))
                .map(|v| v.to_string())
                .unwrap_or_else(|| "<no rows>".into()),
            Err(e) => format!("<exec error: {e}>"),
        },
        Err(e) => format!("<plan error: {e}>"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| "/tmp/delta-arrow-probe".into());
    let location = std::path::PathBuf::from(root).join("t_pb06");
    let _ = std::fs::remove_dir_all(&location);
    std::fs::create_dir_all(&location)?;

    println!("PB06 — does `preimage` fire for a Delta-backed scan, and does it prune files?\n");
    println!(
        "  {FILES} files x {ROWS_PER_FILE} rows; file i holds {} rows from i*{BUCKET} plus one at i*{BUCKET}+{}.",
        ROWS_PER_FILE - 1,
        BUCKET - 1
    );
    println!(
        "  A successful rewrite prunes to ONE file: Rows=Exact({ROWS_PER_FILE}), not Exact({}).\n",
        FILES * ROWS_PER_FILE
    );

    // ---- one Delta table, four files ---------------------------------------------------
    let url = url::Url::from_directory_path(&location).map_err(|_| "bad path")?;
    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;

    let arrow_schema: SchemaRef = Arc::new(Schema::new(vec![
        Field::new("off", DataType::Int64, false),
        Field::new("payload", DataType::Utf8, true),
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

    let mut table = CreateBuilder::new()
        .with_log_store(log_store.clone())
        .with_columns(delta_fields)
        .await?;

    // Separate writes, so each bucket lands in its own file and pruning is measurable.
    for file in 0..FILES {
        // ROWS_PER_FILE - 1 rows at the bottom of the bucket, plus ONE row at the very top
        // (`off = file*BUCKET + BUCKET - 1`). That last row is what distinguishes a correct
        // half-open preimage from a closed one; without it both arms return the same count and
        // the off-by-one is invisible. The first run of this probe had no such row, and passed.
        let mut offs: Vec<i64> = (0..ROWS_PER_FILE - 1).map(|r| file * BUCKET + r).collect();
        offs.push(file * BUCKET + BUCKET - 1);
        let payloads: Vec<String> = offs.iter().map(|o| format!("row-{o}")).collect();
        let batch = RecordBatch::try_new(
            arrow_schema.clone(),
            vec![
                Arc::new(Int64Array::from(offs)),
                Arc::new(StringArray::from(payloads)),
            ],
        )?;
        table = WriteBuilder::new(
            log_store.clone(),
            Some(table.snapshot()?.snapshot().clone()),
        )
        .with_input_batches(vec![batch])
        .await?;
    }
    println!("  wrote {} files\n", table.get_file_uris()?.count());

    let mut delta = DeltaTableBuilder::from_url(url)?.load().await?;
    delta.load().await?;
    let snapshot = delta.snapshot()?.snapshot().clone();
    let provider = TableProviderBuilder::default()
        .with_log_store(log_store.clone())
        .with_eager_snapshot(snapshot)
        .build()
        .await?;

    let ctx = SessionContext::new();
    ctx.register_table("t", Arc::new(provider))?;
    ctx.register_udf(ScalarUDF::from(OffsetBucket::new(
        "offset_bucket",
        PreimageMode::HalfOpen,
        Volatility::Immutable,
    )));
    ctx.register_udf(ScalarUDF::from(OffsetBucket::new(
        "offset_bucket_closed",
        PreimageMode::Closed,
        Volatility::Immutable,
    )));
    ctx.register_udf(ScalarUDF::from(OffsetBucket::new(
        "offset_bucket_np",
        PreimageMode::None,
        Volatility::Immutable,
    )));
    ctx.register_udf(ScalarUDF::from(OffsetBucket::new(
        "offset_bucket_vol",
        PreimageMode::HalfOpen,
        Volatility::Volatile,
    )));

    let arms: &[(&str, &str)] = &[
        (
            "1 UDF preimage HALF-OPEN  ",
            "SELECT count(*) FROM t WHERE offset_bucket(off) = 2",
        ),
        (
            "2 UDF preimage CLOSED     ",
            "SELECT count(*) FROM t WHERE offset_bucket_closed(off) = 2",
        ),
        (
            "3 CONTROL no preimage     ",
            "SELECT count(*) FROM t WHERE offset_bucket_np(off) = 2",
        ),
        (
            "4 CONTROL volatile        ",
            "SELECT count(*) FROM t WHERE offset_bucket_vol(off) = 2",
        ),
        (
            "5 GOLD hand-written range ",
            "SELECT count(*) FROM t WHERE off >= 8192 AND off <= 12287",
        ),
        (
            "6 CONTROL built-in floor  ",
            "SELECT count(*) FROM t WHERE floor(off / 4096.0) = 2.0",
        ),
    ];

    println!("== rewrite and pruning ==\n");
    for (label, sql) in arms {
        let lp = logical(&ctx, sql).await;
        let pp = physical(&ctx, sql).await;
        let n = count(&ctx, sql).await;
        // Did the UDF survive into the optimised plan? If it is gone, it was rewritten away.
        let udf_name = sql
            .split("WHERE ")
            .nth(1)
            .and_then(|s| s.split('(').next())
            .unwrap_or("")
            .trim();
        let rewritten = !udf_name.is_empty() && !lp.contains(udf_name);
        println!("  {label} count={n}");
        println!("      sql:       {sql}");
        println!(
            "      rewritten: {}  (UDF `{udf_name}` {} in optimised plan)",
            if rewritten { "YES" } else { "no " },
            if rewritten { "absent" } else { "present" }
        );
        for line in lp.lines() {
            println!("      logical  | {line}");
        }
        for line in scanned_rows(&pp) {
            println!("      pruning  | {line}");
        }
        println!();
    }

    // ---- PB11: which statistics channel is authoritative? ------------------------------
    // Evidence 11 recorded `partition_statistics` returning Absent at a plan root whose rendered
    // text said Rows=Exact(n), and left it unexplained. The compiler answered it: building this
    // probe emits
    //
    //     warning: use of deprecated method `ExecutionPlan::partition_statistics`:
    //              Use StatisticsContext::compute instead
    //
    // So the disagreement was never between two peers -- evidence 11 read a DEPRECATED channel.
    // The deprecation warning is deliberately NOT suppressed here: it is part of the finding.
    // All three channels are asked side by side, of a Delta table AND of a MemTable, so that
    // "delta-rs does not populate it" is distinguishable from "nobody populates it".
    println!("== PB11 · three statistics channels, Delta and MemTable ==\n");
    println!("  partition_statistics  -- DEPRECATED (this is what evidence 11 read)");
    println!("  StatisticsContext     -- the replacement the deprecation names");
    println!("  rendered plan text    -- what EXPLAIN shows\n");

    let mem_batch = RecordBatch::try_new(
        arrow_schema.clone(),
        vec![
            Arc::new(Int64Array::from(vec![0_i64, 1, 2])),
            Arc::new(StringArray::from(vec!["a", "b", "c"])),
        ],
    )?;
    let mem =
        datafusion::datasource::MemTable::try_new(arrow_schema.clone(), vec![vec![mem_batch]])?;
    let memctx = SessionContext::new();
    memctx.register_table("m", Arc::new(mem))?;

    for (label, ctxref, sql) in [
        ("delta scan     ", &ctx, "SELECT off FROM t"),
        (
            "delta filtered ",
            &ctx,
            "SELECT off FROM t WHERE off >= 8192 AND off <= 12287",
        ),
        ("memtable scan  ", &memctx, "SELECT off FROM m"),
    ] {
        let plan = ctxref.sql(sql).await?.create_physical_plan().await?;
        let all = plan
            .partition_statistics(None)
            .map(|s| format!("{:?}", s.num_rows))
            .unwrap_or_else(|e| format!("<err {e}>"));
        let p0 = plan
            .partition_statistics(Some(0))
            .map(|s| format!("{:?}", s.num_rows))
            .unwrap_or_else(|e| format!("<err {e}>"));
        // The replacement the deprecation names. `StatisticsContext` holds an `Rc<RefCell<_>>`
        // and is therefore !Send, so each use is confined to a block with no await inside it.
        let ctx_all = {
            let sc = StatisticsContext::new();
            sc.compute(plan.as_ref(), &StatisticsArgs::new())
                .map(|s| format!("{:?}", s.num_rows))
                .unwrap_or_else(|e| format!("<err {e}>"))
        };
        let ctx_p0 = {
            let sc = StatisticsContext::new();
            sc.compute(
                plan.as_ref(),
                &StatisticsArgs::new().with_partition(Some(0)),
            )
            .map(|s| format!("{:?}", s.num_rows))
            .unwrap_or_else(|e| format!("<err {e}>"))
        };
        let rendered = format!(
            "{}",
            datafusion::physical_plan::displayable(plan.as_ref())
                .set_show_statistics(true)
                .indent(true)
        );
        let root_line = rendered.lines().next().unwrap_or("").trim().to_string();
        println!("  {label}");
        println!("      DEPRECATED partition_statistics  None={all}  Some(0)={p0}");
        println!("      StatisticsContext::compute       None={ctx_all}  Some(0)={ctx_p0}");
        println!("      rendered root | {root_line}");
    }
    println!();
    Ok(())
}
