use std::sync::{Arc, Mutex};

use arrow_array::cast::AsArray;
use arrow_array::types::Int32Type;
use arrow_array::{ArrayRef, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema, SchemaRef};
use async_trait::async_trait;
use datafusion::catalog::Session;
use datafusion::common::{Result, ToDFSchema};
use datafusion::datasource::{MemTable, TableProvider};
use datafusion::execution::memory_pool::GreedyMemoryPool;
use datafusion::execution::runtime_env::RuntimeEnvBuilder;
use datafusion::logical_expr::simplify::SimplifyContext;
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown, TableType};
use datafusion::optimizer::simplify_expressions::ExprSimplifier;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::prelude::*;
use futures::TryStreamExt;

#[derive(Debug)]
struct ObservedScan {
    projection: Option<Vec<usize>>,
    filters: usize,
    limit: Option<usize>,
}

#[derive(Debug)]
struct FixtureProvider {
    batch: RecordBatch,
    exact: bool,
    calls: Arc<Mutex<Vec<ObservedScan>>>,
}

#[async_trait]
impl TableProvider for FixtureProvider {
    fn schema(&self) -> SchemaRef {
        self.batch.schema()
    }

    fn table_type(&self) -> TableType {
        TableType::Base
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>> {
        Ok(vec![
            if self.exact {
                TableProviderFilterPushDown::Exact
            } else {
                TableProviderFilterPushDown::Inexact
            };
            filters.len()
        ])
    }

    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        self.calls.lock().unwrap().push(ObservedScan {
            projection: projection.cloned(),
            filters: filters.len(),
            limit,
        });
        let mut batch = self.batch.clone();
        if self.exact {
            let schema = self.schema().to_dfschema_ref()?;
            for filter in filters {
                let predicate = state
                    .create_physical_expr(filter.clone(), &schema)?
                    .evaluate(&batch)?
                    .into_array(batch.num_rows())?;
                batch = arrow_select::filter::filter_record_batch(&batch, predicate.as_boolean())?;
            }
        }
        // The Inexact fixture deliberately keeps every row: a conservative candidate superset.
        if let Some(limit) = limit {
            assert!(self.exact || filters.is_empty());
            batch = batch.slice(0, limit.min(batch.num_rows()));
        }
        MemTable::try_new(self.schema(), vec![vec![batch]])?
            .scan(state, projection, &[], None)
            .await
    }
}

fn fixture() -> RecordBatch {
    RecordBatch::try_from_iter(vec![
        (
            "a",
            Arc::new(Int32Array::from(vec![7, 8, 8, 9, 10])) as ArrayRef,
        ),
        (
            "b",
            Arc::new(Int32Array::from(vec![
                Some(0),
                Some(6),
                Some(6),
                None,
                Some(9),
            ])) as ArrayRef,
        ),
    ])
    .unwrap()
}

fn ints(batches: &[RecordBatch], column: usize) -> Vec<Option<i32>> {
    batches
        .iter()
        .flat_map(|b| b.column(column).as_primitive::<Int32Type>().iter())
        .collect()
}

#[tokio::test]
async fn provider_exact_inexact_projection_limit_match_reference() -> Result<()> {
    for exact in [true, false] {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let ctx = SessionContext::new();
        ctx.register_table(
            "p",
            Arc::new(FixtureProvider {
                batch: fixture(),
                exact,
                calls: calls.clone(),
            }),
        )?;
        ctx.register_batch("reference", fixture())?;
        for (projection, suffix) in [("a", ""), ("a", " LIMIT 2"), ("a, b", "")] {
            let query = format!("SELECT {projection} FROM p WHERE b > 5{suffix}");
            let df = ctx.sql(&query).await?;
            let plan = df
                .clone()
                .into_optimized_plan()?
                .display_indent()
                .to_string();
            assert_eq!(plan.contains("Filter:"), !exact, "{plan}");
            let actual = df.collect().await?;
            let expected = ctx
                .sql(&query.replace("FROM p ", "FROM reference "))
                .await?
                .collect()
                .await?;
            assert_eq!(ints(&actual, 0), ints(&expected, 0));
            if projection == "a, b" {
                assert_eq!(ints(&actual, 1), ints(&expected, 1));
            }
            assert!(ints(&actual, 0).starts_with(&[Some(8), Some(8)]));
            println!("provider exact={exact} query={query} plan={plan}");
        }
        let observed = calls.lock().unwrap();
        assert!(observed.iter().all(|s| s.filters > 0));
        if exact {
            assert!(observed.iter().any(|s| s.projection == Some(vec![0])));
            assert!(observed.iter().any(|s| s.limit == Some(2)));
        } else {
            assert!(observed.iter().all(|s| s.limit.is_none()));
            // The residual filter still needs b, even though final output only contains a.
            assert!(
                observed
                    .iter()
                    .all(|s| s.projection.as_ref().is_none_or(|p| p.contains(&1)))
            );
        }
        println!("scan observations: {observed:?}");
    }
    Ok(())
}

#[tokio::test]
async fn expressions_require_coercion_before_direct_simplification() -> Result<()> {
    let ctx = SessionContext::new();
    let result = ctx
        .sql("SELECT CAST(1 AS INT) + CAST(2 AS BIGINT) AS n")
        .await?
        .collect()
        .await?;
    assert_eq!(result[0].schema().field(0).data_type(), &DataType::Int64);
    let schema = Arc::new(Schema::empty()).to_dfschema_ref()?;
    let simplifier = ExprSimplifier::new(
        SimplifyContext::builder()
            .with_schema(schema.clone())
            .build(),
    );
    let expr = lit(1i32) + lit(2i64);
    let direct = simplifier.simplify(expr.clone())?;
    println!("uncoerced simplification: {direct:?}");
    assert_ne!(direct, lit(3i64));
    let coerced = simplifier.coerce(expr, &schema)?;
    assert_eq!(simplifier.simplify(coerced)?, lit(3i64));
    assert!(simplifier.coerce(lit(true) + lit(2i32), &schema).is_err());
    Ok(())
}

#[tokio::test]
async fn joins_sets_and_window_frames_have_distinct_cardinality() -> Result<()> {
    let ctx = SessionContext::new();
    let a = "(VALUES (1), (1), (CAST(NULL AS INT)))";
    let b = "(VALUES (1), (CAST(NULL AS INT)))";
    for (predicate, expected) in [("a.x = b.x", 2), ("a.x IS NOT DISTINCT FROM b.x", 3)] {
        let rows = ctx
            .sql(&format!(
                "SELECT a.x FROM {a} a(x) JOIN {b} b(x) ON {predicate}"
            ))
            .await?
            .collect()
            .await?;
        assert_eq!(
            rows.iter().map(RecordBatch::num_rows).sum::<usize>(),
            expected
        );
    }
    for (operator, expected) in [
        ("UNION ALL", 5),
        ("UNION", 2),
        ("INTERSECT", 2),
        ("EXCEPT", 0),
    ] {
        let rows = ctx
            .sql(&format!(
                "SELECT * FROM {a} a(x) {operator} SELECT * FROM {b} b(x)"
            ))
            .await?
            .collect()
            .await?;
        assert_eq!(
            rows.iter().map(RecordBatch::num_rows).sum::<usize>(),
            expected
        );
    }
    let rows = ctx.sql("SELECT x, count(*) OVER (ORDER BY x) AS peers, count(*) OVER (ORDER BY x ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS explicit_rows FROM (VALUES (1), (1), (2)) t(x) ORDER BY x, explicit_rows")
        .await?.collect().await?;
    println!("window frame control: {rows:?}");
    let pretty = datafusion::arrow::util::pretty::pretty_format_batches(&rows)?.to_string();
    assert!(pretty.contains("1 | 2     | 1"), "{pretty}");
    Ok(())
}

#[tokio::test]
async fn stream_collect_cache_and_sort_resource_error() -> Result<()> {
    let ctx = SessionContext::new();
    ctx.register_batch("t", fixture())?;
    let df = ctx.sql("SELECT a FROM t ORDER BY a DESC").await?;
    let stream: Vec<_> = df.clone().execute_stream().await?.try_collect().await?;
    let collected = df.clone().collect().await?;
    assert_eq!(ints(&stream, 0), ints(&collected, 0));
    let cached = df.cache().await?;
    assert_eq!(
        ints(&cached.clone().collect().await?, 0),
        ints(&cached.collect().await?, 0)
    );

    let runtime = RuntimeEnvBuilder::new()
        .with_memory_pool(Arc::new(GreedyMemoryPool::new(1)))
        .build_arc()?;
    let limited = SessionContext::new_with_config_rt(SessionConfig::new(), runtime);
    let values = RecordBatch::try_new(
        Arc::new(Schema::new(vec![Field::new("x", DataType::Int32, false)])),
        vec![Arc::new(Int32Array::from(
            (0..10_000).rev().collect::<Vec<_>>(),
        ))],
    )?;
    limited.register_batch("t", values)?;
    let df = limited.sql("SELECT x FROM t ORDER BY x").await?;
    let physical = df.clone().create_physical_plan().await?;
    println!(
        "restricted sort plan: {}",
        datafusion::physical_plan::displayable(physical.as_ref()).indent(true)
    );
    let outcome = match df.execute_stream().await {
        Ok(stream) => stream.try_collect::<Vec<_>>().await,
        Err(error) => Err(error),
    };
    let error = outcome.expect_err("streaming does not remove the sort's resource requirement");
    assert!(error.to_string().contains("Resources exhausted"), "{error}");
    println!("pool control: {error}");
    Ok(())
}

#[tokio::test]
async fn parquet_listing_source_executes_query() -> Result<()> {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(format!("listing-{}", std::process::id()));
    std::fs::create_dir_all(&directory)?;
    let file = directory.join("part.parquet");
    let mut writer = parquet::arrow::ArrowWriter::try_new(
        std::fs::File::create(&file)?,
        fixture().schema(),
        None,
    )?;
    writer.write(&fixture())?;
    writer.close()?;
    let ctx = SessionContext::new();
    ctx.register_parquet(
        "files",
        directory.to_str().unwrap(),
        ParquetReadOptions::default(),
    )
    .await?;
    let df = ctx
        .sql("SELECT a FROM files WHERE b > 5 ORDER BY a")
        .await?;
    let plan = df.clone().create_physical_plan().await?;
    println!(
        "listing plan: {}",
        datafusion::physical_plan::displayable(plan.as_ref()).indent(true)
    );
    assert_eq!(
        ints(&df.collect().await?, 0),
        vec![Some(8), Some(8), Some(10)]
    );
    std::fs::remove_dir_all(&directory)?;
    Ok(())
}

#[tokio::test]
async fn coalesce_uses_planning_and_preserves_nonnull_fallback() -> Result<()> {
    let ctx = SessionContext::new();
    ctx.register_batch("t", fixture())?;
    let df = ctx.sql("SELECT coalesce(b, a) AS c FROM t").await?;
    let plan = df
        .clone()
        .into_optimized_plan()?
        .display_indent()
        .to_string();
    assert!(plan.contains("CASE"), "{plan}");
    let rows = df.collect().await?;
    assert_eq!(
        ints(&rows, 0),
        vec![Some(0), Some(6), Some(6), Some(9), Some(9)]
    );
    assert!(!rows[0].schema().field(0).is_nullable());
    let rows = ctx
        .sql("SELECT coalesce(CAST(NULL AS INT), CAST(7 AS BIGINT)) AS c")
        .await?
        .collect()
        .await?;
    assert_eq!(rows[0].schema().field(0).data_type(), &DataType::Int64);
    let nulls = ctx
        .sql("SELECT coalesce(CAST(NULL AS INT), CAST(NULL AS INT)) AS c")
        .await?
        .collect()
        .await?;
    assert_eq!(nulls[0].column(0).null_count(), 1);
    assert_eq!(nulls[0].schema().field(0).data_type(), &DataType::Int32);
    let invalid = match ctx.sql("SELECT coalesce()").await {
        Ok(df) => df.collect().await,
        Err(error) => Err(error),
    };
    assert!(invalid.is_err());
    println!("coalesce optimized plan: {plan}");
    Ok(())
}
