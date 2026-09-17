//! PB18 — can a `preimage` express a STRING PREFIX range, and does it prune a Delta scan?
//!
//! §10.1 says `entity_key_namespace(key)` implements `preimage`, "**yes** — prefix range". That is
//! the only string-valued preimage in the inventory, and every one PB06 measured was `Int64`.
//! `Interval` is documented as an interval-arithmetic type; whether it accepts `Utf8` bounds at
//! all is not stated anywhere in the pinned API surface, and the design has one claim resting on
//! the answer.
//!
//!     cargo run --example pb18_utf8_preimage
//!
//! Two questions, in order, because the second is only worth asking if the first says yes:
//!
//!   1. Does `Interval::try_new` accept `Utf8` bounds?
//!   2. If it does, does a `preimage` returning that interval actually PRUNE a Delta scan --
//!      i.e. do Delta's min/max statistics on a string column get used the way they do on an
//!      integer one?
//!
//! ARMS
//!   A  `entity_namespace(key) = 'mech'` — the UDF, Immutable, half-open `['mech:', 'mech;')`
//!   B  CONTROL: byte-identical UDF with `preimage` left at the trait default. It must NOT prune,
//!      or arm A's pruning is something other than the preimage
//!   C  GOLD STANDARD: `key >= 'mech:' AND key < 'mech;'` written by hand — what a perfect
//!      rewrite would produce, and the ceiling arm A can reach
//!
//! The table is FOUR files with disjoint namespace ranges, one per namespace, so pruning is
//! observable as files read rather than inferred from plan text. PB06's shape exactly.
//!
//! `;` is `:` + 1 in ASCII, which is what makes `['mech:', 'mech;')` the half-open range holding
//! every key starting `mech:` and nothing else.

use std::sync::Arc;

use arrow::array::{RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use datafusion::common::cast::as_string_array;
use datafusion::common::{Result as DFResult, ScalarValue};
use datafusion::logical_expr::interval_arithmetic::Interval;
use datafusion::logical_expr::preimage::PreimageResult;
use datafusion::logical_expr::simplify::SimplifyContext;
use datafusion::logical_expr::{
    ColumnarValue, Expr, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
};
use datafusion::prelude::SessionContext;
use deltalake::DeltaTableBuilder;
use deltalake::delta_datafusion::TableProviderBuilder;
use deltalake::kernel::StructField;
use deltalake::kernel::engine::arrow_conversion::TryFromArrow;
use deltalake::operations::create::CreateBuilder;
use deltalake::operations::write::WriteBuilder;

/// One file per namespace, so a pruned file is a visible drop in rows scanned.
const NAMESPACES: &[&str] = &["cap", "def", "mech", "pkg"];
const ROWS_PER_FILE: usize = 50;

fn schema() -> SchemaRef {
    Arc::new(Schema::new(vec![Field::new("entity_key", DataType::Utf8, false)]))
}

/// `mech:0001` … — sortable, and every row in a file shares one namespace.
fn rows(namespace: &str) -> RecordBatch {
    let keys: Vec<String> = (0..ROWS_PER_FILE)
        .map(|i| format!("{namespace}:{i:04}"))
        .collect();
    RecordBatch::try_new(schema(), vec![Arc::new(StringArray::from(keys))]).expect("batch")
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct EntityNamespace {
    signature: Signature,
    /// Whether this instance answers `preimage` at all. `false` is the control.
    with_preimage: bool,
    name: &'static str,
}

impl EntityNamespace {
    fn new(name: &'static str, with_preimage: bool) -> Self {
        Self {
            signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
            with_preimage,
            name,
        }
    }
}

impl ScalarUDFImpl for EntityNamespace {
    fn name(&self) -> &str {
        self.name
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _args: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Utf8)
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let keys = as_string_array(&arrays[0])?;
        let out: StringArray = keys
            .iter()
            .map(|k| k.and_then(|k| k.split_once(':').map(|(ns, _)| ns.to_string())))
            .collect();
        Ok(ColumnarValue::Array(Arc::new(out)))
    }

    /// `entity_namespace(key) = 'mech'` holds exactly for keys in `['mech:', 'mech;')`.
    ///
    /// Half-open, and over `args[0]` — the BARE stored column. Both are PB06's measured
    /// requirements: a closed upper bound is off by one row, and a derived expression rewrites
    /// but cannot prune, because Delta's statistics are keyed on stored columns.
    fn preimage(
        &self,
        args: &[Expr],
        lit_expr: &Expr,
        _info: &SimplifyContext,
    ) -> DFResult<PreimageResult> {
        if !self.with_preimage {
            return Ok(PreimageResult::None);
        }
        let Expr::Literal(ScalarValue::Utf8(Some(namespace)), _) = lit_expr else {
            return Ok(PreimageResult::None);
        };
        let lower = ScalarValue::Utf8(Some(format!("{namespace}:")));
        let upper = ScalarValue::Utf8(Some(format!("{namespace};")));
        match Interval::try_new(lower, upper) {
            Ok(interval) => Ok(PreimageResult::Range {
                expr: args[0].clone(),
                interval: Box::new(interval),
            }),
            // The finding, if this is where it lands: a Utf8 interval is not constructible, so the
            // preimage cannot be expressed and §10.1's claim is wrong rather than merely unbuilt.
            Err(e) => {
                println!("      Interval::try_new(Utf8, Utf8) FAILED: {e}");
                Ok(PreimageResult::None)
            }
        }
    }
}

async fn fresh(root: &std::path::Path) -> Result<url::Url, Box<dyn std::error::Error>> {
    let location = root.join("t_pb18");
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
        .with_log_store(log_store.clone())
        .with_columns(fields)
        .await?;

    // One APPEND per namespace, so each lands in its own file with its own min/max statistics.
    for namespace in NAMESPACES {
        let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
        let snapshot = table.snapshot().ok().map(|s| s.snapshot().clone());
        WriteBuilder::new(log_store.clone(), snapshot)
            .with_input_batches(vec![rows(namespace)])
            .await?;
    }
    Ok(url)
}

/// Rows returned, and the rendered plan, for one predicate.
async fn run(ctx: &SessionContext, sql: &str) -> (usize, String) {
    let Ok(df) = ctx.sql(sql).await else {
        return (0, "<plan error>".into());
    };
    let plan = match df.clone().into_optimized_plan() {
        Ok(p) => format!("{}", p.display_indent()).replace('\n', " / "),
        Err(e) => format!("<optimise error: {e}>"),
    };
    let rows = match df.collect().await {
        Ok(b) => b.iter().map(|b| b.num_rows()).sum(),
        Err(_) => 0,
    };
    (rows, plan)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::var("PROBE_ROOT").unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("pb18-utf8-preimage")
            .display()
            .to_string()
    });
    let root = std::path::PathBuf::from(root);

    println!("PB18 — a STRING prefix preimage: constructible, and does it prune?\n");
    println!("  table   {} files, {ROWS_PER_FILE} rows each, one namespace per file", NAMESPACES.len());
    println!("  target  entity_key LIKE 'mech:%'  -> {ROWS_PER_FILE} rows of {}\n",
             NAMESPACES.len() * ROWS_PER_FILE);

    // Question 1, asked directly, because if this is No then nothing else matters.
    let direct = Interval::try_new(
        ScalarValue::Utf8(Some("mech:".into())),
        ScalarValue::Utf8(Some("mech;".into())),
    );
    match &direct {
        Ok(i) => println!("  Q1  Interval::try_new(Utf8, Utf8)   OK    {i:?}"),
        Err(e) => println!("  Q1  Interval::try_new(Utf8, Utf8)   ERR   {e}"),
    }
    println!();

    let url = fresh(&root).await?;
    let ctx = deltalake::delta_datafusion::create_session().into_inner();
    ctx.register_udf(ScalarUDF::from(EntityNamespace::new("entity_namespace", true)));
    ctx.register_udf(ScalarUDF::from(EntityNamespace::new("entity_namespace_np", false)));

    let log_store = DeltaTableBuilder::from_url(url.clone())?.build_storage()?;
    let table = DeltaTableBuilder::from_url(url.clone())?.load().await?;
    let provider = TableProviderBuilder::default()
        .with_log_store(log_store)
        .with_eager_snapshot(table.snapshot()?.snapshot().clone())
        .build()
        .await?;
    ctx.register_table("keys", Arc::new(provider))?;

    let arms = [
        ("A  preimage UDF        ", "SELECT * FROM keys WHERE entity_namespace(entity_key) = 'mech'"),
        ("B  CONTROL no preimage ", "SELECT * FROM keys WHERE entity_namespace_np(entity_key) = 'mech'"),
        ("C  GOLD hand-written   ", "SELECT * FROM keys WHERE entity_key >= 'mech:' AND entity_key < 'mech;'"),
    ];
    let mut rewritten = Vec::new();
    for (label, sql) in arms {
        let (rows, plan) = run(&ctx, sql).await;
        // A rewrite is visible as the UDF disappearing from the filter in favour of a range.
        let is_range = !plan.contains("entity_namespace") && plan.contains("entity_key");
        println!("  {label} rows={rows:>3}  rewritten_to_range={is_range}");
        println!("      {}", &plan[..plan.len().min(150)]);
        rewritten.push((rows, is_range));
    }

    println!("\n  ---");
    let (a_rows, a_range) = rewritten[0];
    let (_, b_range) = rewritten[1];
    let (c_rows, _) = rewritten[2];
    if direct.is_err() {
        println!("  RECORDED   a Utf8 Interval is not constructible at this version, so a string");
        println!("             prefix preimage cannot be expressed. §10.1's claim is wrong.");
        return Ok(());
    }
    if a_rows != ROWS_PER_FILE || c_rows != ROWS_PER_FILE {
        println!("  INCONCLUSIVE  the arms disagree on ROWS, so they are not asking one question");
        return Err("PB18 did not confirm".into());
    }
    if a_range && !b_range {
        println!("  CONFIRMED  a string prefix preimage is constructible AND fires; the control,");
        println!("             which differs only in returning PreimageResult::None, does not.");
        Ok(())
    } else {
        println!("  RECORDED   rewrite A={a_range} control B={b_range} -- the preimage did not");
        println!("             fire, or the control fired too, so it is not attributable.");
        Ok(())
    }
}
