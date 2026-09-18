# `datafusion::dataframe`

Crate `datafusion` · 2 public items · structured records in [`model/datafusion.dataframe.json`](../model/datafusion.dataframe.json)

## DataFrame

`struct` · `datafusion::dataframe::DataFrame`

Also reachable as `datafusion::prelude::DataFrame`

```rust
struct DataFrame
```

**Derives**: Clone, Debug

**Methods** (61)

```rust
fn aggregate(self, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>) -> Result<DataFrame>
fn alias(self, alias: &str) -> Result<DataFrame>
async fn cache(self) -> Result<DataFrame>
async fn collect(self) -> Result<Vec<RecordBatch>>
async fn collect_partitioned(self) -> Result<Vec<Vec<RecordBatch>>>
async fn count(self) -> Result<usize>
async fn create_physical_plan(&self) -> Result<Arc<dyn ExecutionPlan>>
async fn describe(self) -> Result<Self>
fn distinct(self) -> Result<DataFrame>
fn distinct_on(self, on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>) -> Result<DataFrame>
fn drop_columns<T>(self, columns: &[T]) -> Result<DataFrame> where T: Into<Column> + Clone
fn except(self, dataframe: DataFrame) -> Result<DataFrame>
fn except_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
async fn execute_stream(self) -> Result<SendableRecordBatchStream>
async fn execute_stream_partitioned(self) -> Result<Vec<SendableRecordBatchStream>>
fn explain(self, verbose: bool, analyze: bool) -> Result<DataFrame>
fn explain_with_options(self, explain_option: ExplainOption) -> Result<DataFrame>
fn fill_nan(&self, value: &ScalarValue, columns: &[&str]) -> Result<DataFrame>
fn fill_null(&self, value: &ScalarValue, columns: &[&str]) -> Result<DataFrame>
fn filter(self, predicate: Expr) -> Result<DataFrame>
fn find_qualified_columns(&self, names: &[&str]) -> Result<Vec<(Option<&TableReference>, &FieldRef)>>
fn from_columns(columns: Vec<(&str, ArrayRef)>) -> Result<Self>
fn intersect(self, dataframe: DataFrame) -> Result<DataFrame>
fn intersect_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
fn into_optimized_plan(self) -> Result<LogicalPlan>
fn into_parts(self) -> (SessionState, LogicalPlan)
fn into_temporary_view(self) -> Arc<dyn TableProvider>
fn into_unoptimized_plan(self) -> LogicalPlan
fn into_view(self) -> Arc<dyn TableProvider>
fn join(self, right: DataFrame, join_type: JoinType, left_cols: &[&str], right_cols: &[&str], filter: Option<Expr>) -> Result<DataFrame>
fn join_on(self, right: DataFrame, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<DataFrame>
fn limit(self, skip: usize, fetch: Option<usize>) -> Result<DataFrame>
fn logical_plan(&self) -> &LogicalPlan
fn new(session_state: SessionState, plan: LogicalPlan) -> Self
fn parse_sql_expr(&self, sql: &str) -> Result<Expr>
fn registry(&self) -> &dyn FunctionRegistry
fn repartition(self, partitioning_scheme: Partitioning) -> Result<DataFrame>
fn schema(&self) -> &DFSchema
fn select(self, expr_list: impl IntoIterator<Item = impl Into<SelectExpr>>) -> Result<DataFrame>
fn select_columns(self, columns: &[&str]) -> Result<DataFrame>
fn select_exprs(self, exprs: &[&str]) -> Result<DataFrame>
async fn show(self) -> Result<()>
async fn show_limit(self, num: usize) -> Result<()>
fn sort(self, expr: Vec<SortExpr>) -> Result<DataFrame>
fn sort_by(self, expr: Vec<Expr>) -> Result<DataFrame>
fn task_ctx(&self) -> TaskContext
async fn to_string(self) -> Result<String>
fn union(self, dataframe: DataFrame) -> Result<DataFrame>
fn union_by_name(self, dataframe: DataFrame) -> Result<DataFrame>
fn union_by_name_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
fn union_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
fn unnest_columns(self, columns: &[&str]) -> Result<DataFrame>
fn unnest_columns_with_options(self, columns: &[&str], options: UnnestOptions) -> Result<DataFrame>
fn window(self, window_exprs: Vec<Expr>) -> Result<DataFrame>
fn with_column(self, name: &str, expr: Expr) -> Result<DataFrame>
fn with_column_renamed(self, old_name: impl Into<String>, new_name: &str) -> Result<DataFrame>
fn with_param_values(self, query_values: impl Into<ParamValues>) -> Result<Self>
async fn write_csv(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<CsvOptions>) -> Result<Vec<RecordBatch>, DataFusionError>
async fn write_json(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<JsonOptions>) -> Result<Vec<RecordBatch>, DataFusionError>
async fn write_parquet(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<TableParquetOptions>) -> Result<Vec<RecordBatch>, DataFusionError>
async fn write_table(self, table_name: &str, write_options: DataFrameWriteOptions) -> Result<Vec<RecordBatch>, DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion.dataframe.DataFrame.md).


Represents a logical set of rows with the same named columns.

Similar to a [Pandas DataFrame] or [Spark DataFrame], a DataFusion DataFrame
represents a 2 dimensional table of rows and columns.

The typical workflow using DataFrames looks like

1. Create a DataFrame via methods on [SessionContext], such as [`read_csv`]
   and [`read_parquet`].

2. Build a desired calculation by calling methods such as [`filter`],
   [`select`], [`aggregate`], and [`limit`]

3. Execute into [`RecordBatch`]es by calling [`collect`]

A `DataFrame` is a wrapper around a [`LogicalPlan`] and the [`SessionState`]
   required for execution.

DataFrames are "lazy" in the sense that most methods do not actually compute
anything, they just build up a plan. Calling [`collect`] executes the plan
using the same DataFusion planning and execution process used to execute SQL
and other queries.

[Pandas DataFrame]: https://pandas.pydata.org/pandas-docs/stable/reference/api/pandas.DataFrame.html
[Spark DataFrame]: https://spark.apache.org/docs/latest/sql-programming-guide.html
[`read_csv`]: SessionContext::read_csv
[`read_parquet`]: SessionContext::read_parquet
[`filter`]: DataFrame::filter
[`select`]: DataFrame::select
[`aggregate`]: DataFrame::aggregate
[`limit`]: DataFrame::limit
[`collect`]: DataFrame::collect

# Example
```
# use std::sync::Arc;
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion::functions_aggregate::expr_fn::min;
# use datafusion::arrow::array::{Int32Array, RecordBatch, StringArray};
# use datafusion::arrow::datatypes::{DataType, Field, Schema};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
// Read the data from a csv file
let df = ctx.read_csv("tests/data/example.csv", CsvReadOptions::new()).await?;
// create a new dataframe that computes the equivalent of
// `SELECT a, MIN(b) FROM df WHERE a <= b GROUP BY a LIMIT 100;`
let df = df.filter(col("a").lt_eq(col("b")))?
           .aggregate(vec![col("a")], vec![min(col("b"))])?
           .limit(0, Some(100))?;
// Perform the actual computation
let results = df.collect();

// Create a new dataframe with in-memory data
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, true),
    Field::new("name", DataType::Utf8, true),
]);
let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![
        Arc::new(Int32Array::from(vec![1, 2, 3])),
        Arc::new(StringArray::from(vec!["foo", "bar", "baz"])),
    ],
)?;
let df = ctx.read_batch(batch)?;
df.show().await?;

// Create a new dataframe with in-memory data using macro
let df = dataframe!(
    "id" => [1, 2, 3],
    "name" => ["foo", "bar", "baz"]
 )?;
df.show().await?;
# Ok(())
# }
```

---

## DataFrameWriteOptions

`struct` · `datafusion::dataframe::DataFrameWriteOptions`

```rust
struct DataFrameWriteOptions
```

**Derives**: Default

**Methods** (5)

```rust
fn new() -> Self
fn with_insert_operation(self, insert_op: InsertOp) -> Self
fn with_partition_by(self, partition_by: Vec<String>) -> Self
fn with_single_file_output(self, single_file_output: bool) -> Self
fn with_sort_by(self, sort_by: Vec<SortExpr>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion.dataframe.DataFrameWriteOptions.md).


Contains options that control how data is
written out from a DataFrame

---
