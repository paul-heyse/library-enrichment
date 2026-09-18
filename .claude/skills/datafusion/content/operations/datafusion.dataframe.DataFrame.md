# `datafusion::dataframe::DataFrame`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.dataframe.DataFrame.json).

<a id="op-4dea21cb0a990412dd05d162"></a>
## DataFrame

`struct` · `datafusion::dataframe::DataFrame` · datafusion 55.1.0

```rust
struct DataFrame
```

Source: `src/dataframe/mod.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Represents a logical set of rows with the same named columns.

Similar to a [Pandas DataFrame] or [Spark DataFrame], a DataFusion DataFrame
represents a 2 dimensional table of rows and columns.

The typical workflow using DataFrames looks like

1. Create a DataFrame via methods on [SessionContext](../operations/datafusion.execution.context.SessionContext.md#op-640a08e4451b7f418c93b5ee), such as [`read_csv`]
   and [`read_parquet`].

2. Build a desired calculation by calling methods such as [`filter`],
   [`select`], [`aggregate`], and [`limit`]

3. Execute into [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es by calling [`collect`]

A `DataFrame` is a wrapper around a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) and the [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)
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

<a id="op-07465b5b7fe15661df64605b"></a>
## aggregate

`function` · `datafusion::dataframe::DataFrame::aggregate` · datafusion 55.1.0

```rust
fn aggregate(self, group_expr: Vec<Expr>, aggr_expr: Vec<Expr>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:645`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new `DataFrame` that aggregates the rows of the current
`DataFrame`, first optionally grouping by the given expressions.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion::functions_aggregate::expr_fn::min;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;

// The following use is the equivalent of "SELECT MIN(b) GROUP BY a"
let df1 = df.clone().aggregate(vec![col("a")], vec![min(col("b"))])?;
let expected1 = vec![
    "+---+----------------+",
    "| a | min(?table?.b) |",
    "+---+----------------+",
    "| 1 | 2              |",
    "| 4 | 5              |",
    "| 7 | 8              |",
    "+---+----------------+",
];
assert_batches_sorted_eq!(expected1, &df1.collect().await?);
// The following use is the equivalent of "SELECT MIN(b)"
let df2 = df.aggregate(vec![], vec![min(col("b"))])?;
let expected2 = vec![
    "+----------------+",
    "| min(?table?.b) |",
    "+----------------+",
    "| 2              |",
    "+----------------+",
];
# assert_batches_sorted_eq!(expected2, &df2.collect().await?);
# Ok(())
# }
```

<a id="op-c2d1983fe009d03d738cf381"></a>
## alias

`function` · `datafusion::dataframe::DataFrame::alias` · datafusion 55.1.0

```rust
fn alias(self, alias: &str) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2432`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Apply an alias to the DataFrame.

This method replaces the qualifiers of output columns with the given alias.

<a id="op-46de08d8309fcb4f0782d08e"></a>
## cache

`function` · `datafusion::dataframe::DataFrame::cache` · datafusion 55.1.0

```rust
async fn cache(self) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2412`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Cache DataFrame as a memory table.

Default behavior could be changed using
a [`crate::execution::session_state::CacheFactory`](../operations/datafusion.execution.session_state.CacheFactory.md#op-05346466af099911c46feb7a)
configured via [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601).

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df.cache().await?;
# Ok(())
# }
```

<a id="op-f4a5c4464e2236ff036c1ad5"></a>
## clone

`function` · `datafusion::dataframe::DataFrame::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> DataFrame
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 17], "end": [228, 22], "filename": "src/dataframe/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dataframe/mod.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2a2c9a0147637ad4b86374e"></a>
## collect

`function` · `datafusion::dataframe::DataFrame::collect` · datafusion 55.1.0

```rust
async fn collect(self) -> Result<Vec<RecordBatch>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1480`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute this `DataFrame` and buffer all resulting `RecordBatch`es  into memory.

Prior to calling `collect`, modifying a DataFrame simply updates a plan
(no actual computation is performed). `collect` triggers the computation.

See [`Self::execute_stream`](../operations/datafusion.dataframe.DataFrame.md#op-acdcda8e3349a26da03fd615) to execute a DataFrame without buffering.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let batches = df.collect().await?;
# Ok(())
# }
```

<a id="op-4c19efb63c32a62bc31e3abf"></a>
## collect_partitioned

`function` · `datafusion::dataframe::DataFrame::collect_partitioned` · datafusion 55.1.0

```rust
async fn collect_partitioned(self) -> Result<Vec<Vec<RecordBatch>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1624`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Executes this DataFrame and collects all results into a vector of vector of RecordBatch
maintaining the input partitioning.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let batches = df.collect_partitioned().await?;
# Ok(())
# }
```

<a id="op-f5e978b71d60cee83bca7616"></a>
## count

`function` · `datafusion::dataframe::DataFrame::count` · datafusion 55.1.0

```rust
async fn count(self) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1440`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the total number of rows in this `DataFrame`.

Note that this method will actually run a plan to calculate the count,
which may be slow for large or complicated DataFrames.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let count = df.count().await?; // 1
# assert_eq!(count, 1);
# Ok(())
# }
```

<a id="op-4eb9d78c488bf998f7a1391d"></a>
## create_physical_plan

`function` · `datafusion::dataframe::DataFrame::create_physical_plan` · datafusion 55.1.0

```rust
async fn create_physical_plan(&self) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a physical plan from this DataFrame.

The `DataFrame` remains accessible after this call, so you can inspect
the plan and still call [`DataFrame::collect`](../operations/datafusion.dataframe.DataFrame.md#op-c2a2c9a0147637ad4b86374e) or other execution methods.

<a id="op-f79dbd8b8eec158ba0fc10b0"></a>
## describe

`function` · `datafusion::dataframe::DataFrame::describe` · datafusion 55.1.0

```rust
async fn describe(self) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1017`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new `DataFrame` that has statistics for a DataFrame.

The summary contains the `count`, `null_count`, `mean`, `std`, `min`,
`max`, and `median` of each column. `count` and `null_count` are
computed for every column; `min` and `max` for every column except
`Boolean`; and `mean`, `std`, and `median` only for numeric columns
(other columns report `null` for these). `min`/`max` of binary columns
(`Binary`, `LargeBinary`, `BinaryView`, `FixedSizeBinary`) are rendered
as lowercase hex. The output format is modeled after pandas

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use arrow::util::pretty;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/tpch-csv/customer.csv", CsvReadOptions::new()).await?;
let stat = df.describe().await?;
# // some output column are ignored
let expected = vec![
    "+------------+--------------------+--------------------+------------------------------------+--------------------+-----------------+--------------------+--------------+----------------------------------------------------------------------------------------------------------+",
    "| describe   | c_custkey          | c_name             | c_address                          | c_nationkey        | c_phone         | c_acctbal          | c_mktsegment | c_comment                                                                                                |",
    "+------------+--------------------+--------------------+------------------------------------+--------------------+-----------------+--------------------+--------------+----------------------------------------------------------------------------------------------------------+",
    "| count      | 9.0                | 9                  | 9                                  | 9.0                | 9               | 9.0                | 9            | 9                                                                                                        |",
    "| max        | 10.0               | Customer#000000010 | xKiAFTjUsCuxfeleNqefumTrjS         | 20.0               | 30-114-968-4951 | 9561.95            | MACHINERY    | tions. even deposits boost according to the slyly bold packages. final accounts cajole requests. furious |",
    "| mean       | 6.0                | null               | null                               | 9.88888888888889   | null            | 5153.2155555555555 | null         | null                                                                                                     |",
    "| median     | 6.0                | null               | null                               | 8.0                | null            | 6819.74            | null         | null                                                                                                     |",
    "| min        | 2.0                | Customer#000000002 | 6LrEaV6KR6PLVcgl2ArL Q3rqzLzcT1 v2 | 1.0                | 11-719-748-3364 | 121.65             | AUTOMOBILE   |  deposits eat slyly ironic, even instructions. express foxes detect slyly. blithely even accounts abov   |",
    "| null_count | 0.0                | 0                  | 0                                  | 0.0                | 0               | 0.0                | 0            | 0                                                                                                        |",
    "| std        | 2.7386127875258306 | null               | null                               | 7.2188026092359046 | null            | 3522.169804254585  | null         | null                                                                                                     |",
    "+------------+--------------------+--------------------+------------------------------------+--------------------+-----------------+--------------------+--------------+----------------------------------------------------------------------------------------------------------+"];
assert_batches_sorted_eq!(expected, &stat.collect().await?);
# Ok(())
# }
```

<a id="op-6a36e90dcfafdf5272a269a0"></a>
## distinct

`function` · `datafusion::dataframe::DataFrame::distinct` · datafusion 55.1.0

```rust
fn distinct(self) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:926`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new `DataFrame` with all duplicated rows removed.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df.distinct()?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-783a18779752fcde2821af0f"></a>
## distinct_on

`function` · `datafusion::dataframe::DataFrame::distinct_on` · datafusion 55.1.0

```rust
fn distinct_on(self, on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:963`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new `DataFrame` with duplicated rows removed as per the specified expression list
according to the provided sorting expressions grouped by the `DISTINCT ON` clause
expressions.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    // Return a single row (a, b) for each distinct value of a
    .distinct_on(vec![col("a")], vec![col("a"), col("b")], None)?;
let expected = vec![
    "+---+---+",
    "| a | b |",
    "+---+---+",
    "| 1 | 2 |",
    "+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-5778e73279dc17a86fcc387f"></a>
## drop_columns

`function` · `datafusion::dataframe::DataFrame::drop_columns` · datafusion 55.1.0

```rust
fn drop_columns<T>(self, columns: &[T]) -> Result<DataFrame> where T: Into<Column> + Clone
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a new DataFrame containing all columns except the specified columns.

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
// +----+----+----+
// | a  | b  | c  |
// +----+----+----+
// | 1  | 2  | 3  |
// +----+----+----+
let df = df.drop_columns(&["a"])?;
let expected = vec![
    "+---+---+",
    "| b | c |",
    "+---+---+",
    "| 2 | 3 |",
    "+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-dbd92e8ee9b929d68d189748"></a>
## except

`function` · `datafusion::dataframe::DataFrame::except` · datafusion 55.1.0

```rust
fn except(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1938`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the exception of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s.  The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s must have exactly the same schema

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let d2 = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let result = df.except(d2)?;
// those columns are not in example.csv, but in example_long.csv
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &result.collect().await?);
# Ok(())
# }
```

<a id="op-e00df3507717fabf8c8c8108"></a>
## except_distinct

`function` · `datafusion::dataframe::DataFrame::except_distinct` · datafusion 55.1.0

```rust
fn except_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1978`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the distinct exception of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s.  The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s must have exactly the same schema

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let d2 = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let result = df.except_distinct(d2)?;
// those columns are not in example.csv, but in example_long.csv
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &result.collect().await?);
# Ok(())
# }
```

<a id="op-acdcda8e3349a26da03fd615"></a>
## execute_stream

`function` · `datafusion::dataframe::DataFrame::execute_stream` · datafusion 55.1.0

```rust
async fn execute_stream(self) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1601`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Executes this DataFrame and returns a stream over a single partition

See [Self::collect](../operations/datafusion.dataframe.DataFrame.md#op-c2a2c9a0147637ad4b86374e) to buffer the `RecordBatch`es in memory.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let stream = df.execute_stream().await?;
# Ok(())
# }
```

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources

<a id="op-73ff2f187bcd18e356825de9"></a>
## execute_stream_partitioned

`function` · `datafusion::dataframe::DataFrame::execute_stream_partitioned` · datafusion 55.1.0

```rust
async fn execute_stream_partitioned(self) -> Result<Vec<SendableRecordBatchStream>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1650`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Executes this DataFrame and returns one stream per partition.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let batches = df.execute_stream_partitioned().await?;
# Ok(())
# }
```
# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources

<a id="op-bbce822bf4cc02d77df597ea"></a>
## explain

`function` · `datafusion::dataframe::DataFrame::explain` · datafusion 55.1.0

```rust
fn explain(self, verbose: bool, analyze: bool) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1761`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a DataFrame with the explanation of its plan so far.

if `analyze` is specified, runs the plan and reports metrics
if `verbose` is true, prints out additional details.
The default format is Indent format.

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let batches = df
    .limit(0, Some(100))?
    .explain(false, false)?
    .collect()
    .await?;
# Ok(())
# }
```

<a id="op-6d0eead211d83db619311b0b"></a>
## explain_with_options

`function` · `datafusion::dataframe::DataFrame::explain_with_options` · datafusion 55.1.0

```rust
fn explain_with_options(self, explain_option: ExplainOption) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1795`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a DataFrame with the explanation of its plan so far.

`opt` is used to specify the options for the explain operation.
Details of the options can be found in [`ExplainOption`](../operations/datafusion_expr.logical_plan.plan.ExplainOption.md#op-784cf352b14885c813b5fb5d).
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
use datafusion_expr::{Explain, ExplainOption};
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let batches = df
    .limit(0, Some(100))?
    .explain_with_options(
        ExplainOption::default()
            .with_verbose(false)
            .with_analyze(false),
    )?
    .collect()
    .await?;
# Ok(())
# }
```

<a id="op-9fd18a56835f9be6b5e8b5ff"></a>
## fill_nan

`function` · `datafusion::dataframe::DataFrame::fill_nan` · datafusion 55.1.0

```rust
fn fill_nan(&self, value: &ScalarValue, columns: &[&str]) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2513`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Fill NaN values in specified floating-point columns with a given value
If no columns are specified (empty slice), applies to all columns
Only floating-point columns are affected; other columns are left unchanged
Only fills if the value can be cast to the column's type

# Arguments
* `value` - Value to fill NaNs with
* `columns` - List of column names to fill. If empty, fills all columns.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::ScalarValue;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
// Fill NaN in only columns "a" and "c":
let df = df.fill_nan(&ScalarValue::from(0.0), &["a", "c"])?;
// Fill NaN across all columns:
let df = df.fill_nan(&ScalarValue::from(0.0), &[])?;
# Ok(())
# }
```

<a id="op-a830d05696ece5f77d126f53"></a>
## fill_null

`function` · `datafusion::dataframe::DataFrame::fill_null` · datafusion 55.1.0

```rust
fn fill_null(&self, value: &ScalarValue, columns: &[&str]) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2467`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Fill null values in specified columns with a given value
If no columns are specified (empty slice), applies to all columns
Only fills if the value can be cast to the column's type

# Arguments
* `value` - Value to fill nulls with
* `columns` - List of column names to fill. If empty, fills all columns.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::ScalarValue;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
// Fill nulls in only columns "a" and "c":
let df = df.fill_null(&ScalarValue::from(0), &["a", "c"])?;
// Fill nulls across all columns:
let df = df.fill_null(&ScalarValue::from(0), &[])?;
# Ok(())
# }
```

<a id="op-a952f26bc03a3cd2a4cb765b"></a>
## filter

`function` · `datafusion::dataframe::DataFrame::filter` · datafusion 55.1.0

```rust
fn filter(self, predicate: Expr) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:593`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a DataFrame with only rows for which `predicate` evaluates to
`true`.

Rows for which `predicate` evaluates to `false` or `null`
are filtered out.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let df = df.filter(col("a").lt_eq(col("b")))?;
// all rows where a <= b are returned
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-7962b7aeae43e29cfd83cc2e"></a>
## find_qualified_columns

`function` · `datafusion::dataframe::DataFrame::find_qualified_columns` · datafusion 55.1.0

```rust
fn find_qualified_columns(&self, names: &[&str]) -> Result<Vec<(Option<&TableReference>, &FieldRef)>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2587`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Find qualified columns for this dataframe from names

# Arguments
* `names` - Unqualified names to find.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::ScalarValue;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
ctx.register_csv("first_table", "tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = ctx.table("first_table").await?;
ctx.register_csv("second_table", "tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df2 = ctx.table("second_table").await?;
let join_expr = df.find_qualified_columns(&["a"])?.iter()
    .zip(df2.find_qualified_columns(&["a"])?.iter())
    .map(|(col1, col2)| col(*col1).eq(col(*col2)))
    .collect::<Vec<Expr>>();
let df3 = df.join_on(df2, JoinType::Inner, join_expr)?;
# Ok(())
# }
```

<a id="op-7e4443c8f0401534019e5be5"></a>
## fmt

`function` · `datafusion::dataframe::DataFrame::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 10], "end": [228, 15], "filename": "src/dataframe/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dataframe/mod.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9963a857c05795cc6c4e3ff3"></a>
## from_columns

`function` · `datafusion::dataframe::DataFrame::from_columns` · datafusion 55.1.0

```rust
fn from_columns(columns: Vec<(&str, ArrayRef)>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2619`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Helper for creating DataFrame.
# Example
```
use arrow::array::{ArrayRef, Int32Array, StringArray};
use datafusion::prelude::DataFrame;
use std::sync::Arc;
let id: ArrayRef = Arc::new(Int32Array::from(vec![1, 2, 3]));
let name: ArrayRef = Arc::new(StringArray::from(vec!["foo", "bar", "baz"]));
let df = DataFrame::from_columns(vec![("id", id), ("name", name)]).unwrap();
// +----+------+,
// | id | name |,
// +----+------+,
// | 1  | foo  |,
// | 2  | bar  |,
// | 3  | baz  |,
// +----+------+,
```

<a id="op-6711a3af98d7542d1cec7903"></a>
## intersect

`function` · `datafusion::dataframe::DataFrame::intersect` · datafusion 55.1.0

```rust
fn intersect(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1860`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the intersection of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s.  The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s must have exactly the same schema

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let d2 = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let df = df.intersect(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-43ec383958572e5fe6436894"></a>
## intersect_distinct

`function` · `datafusion::dataframe::DataFrame::intersect_distinct` · datafusion 55.1.0

```rust
fn intersect_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1898`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the distinct intersection of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s.  The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s must have exactly the same schema

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let d2 = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let df = df.intersect_distinct(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-8fb954562aaa180227b40644"></a>
## into_optimized_plan

`function` · `datafusion::dataframe::DataFrame::into_optimized_plan` · datafusion 55.1.0

```rust
fn into_optimized_plan(self) -> Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1712`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the optimized [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) represented by this DataFrame.

Note: This method should not be used outside testing -- see
[`Self::into_unoptimized_plan`](../operations/datafusion.dataframe.DataFrame.md#op-88c3675dd1b58c8e6f964f13) for more details.

<a id="op-9ab87adf40dfec5f6ea47021"></a>
## into_parts

`function` · `datafusion::dataframe::DataFrame::into_parts` · datafusion 55.1.0

```rust
fn into_parts(self) -> (SessionState, LogicalPlan)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1690`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns both the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) and [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) that comprise this [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)

<a id="op-b28995590d422f4a9e9e6e72"></a>
## into_temporary_view

`function` · `datafusion::dataframe::DataFrame::into_temporary_view` · datafusion 55.1.0

```rust
fn into_temporary_view(self) -> Arc<dyn TableProvider>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1731`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

See [`Self::into_view`](../operations/datafusion.dataframe.DataFrame.md#op-ce2618e84e6ecfd888a3b153). The returned [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) will
create a transient table.

<a id="op-88c3675dd1b58c8e6f964f13"></a>
## into_unoptimized_plan

`function` · `datafusion::dataframe::DataFrame::into_unoptimized_plan` · datafusion 55.1.0

```rust
fn into_unoptimized_plan(self) -> LogicalPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1704`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) represented by this DataFrame without running
any optimizers

Note: This method should not be used outside testing, as it loses the
snapshot of the [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) attached to this [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) and
consequently subsequent operations may take place against a different
state (e.g. a different value of `now()`)

See [`Self::into_parts`](../operations/datafusion.dataframe.DataFrame.md#op-9ab87adf40dfec5f6ea47021) to retrieve the owned [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) and
corresponding [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601).

<a id="op-ce2618e84e6ecfd888a3b153"></a>
## into_view

`function` · `datafusion::dataframe::DataFrame::into_view` · datafusion 55.1.0

```rust
fn into_view(self) -> Arc<dyn TableProvider>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1722`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Converts this [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) into a [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) that can be registered
as a table view using [`SessionContext::register_table`](../operations/datafusion.execution.context.SessionContext.md#op-311a1b8767fde50f77b0e98a).

Note: This discards the [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) associated with this
[`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) in favour of the one passed to [`TableProvider::scan`](../operations/datafusion_session.table.TableProvider.md#op-116d00acf0401874b7f0d9f0)

<a id="op-8991ac8bbaecfd6a6972ed32"></a>
## join

`function` · `datafusion::dataframe::DataFrame::join` · datafusion 55.1.0

```rust
fn join(self, right: DataFrame, join_type: JoinType, left_cols: &[&str], right_cols: &[&str], filter: Option<Expr>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1299`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Join this `DataFrame` with another `DataFrame` using explicitly specified
columns and an optional filter expression.

See [`join_on`](Self::join_on) for a more concise way to specify the
join condition. Since DataFusion will automatically identify and
optimize equality predicates there is no performance difference between
this function and `join_on`

`left_cols` and `right_cols` are used to form "equijoin" predicates (see
example below), which are then combined with the optional `filter`
expression. If `left_cols` and `right_cols` contain ambiguous column
references, they will be disambiguated by prioritizing the left relation
for `left_cols` and the right relation for `right_cols`.

Note that in case of outer join, the `filter` is applied to only matched rows.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let left = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let right = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    .select(vec![
        col("a").alias("a2"),
        col("b").alias("b2"),
        col("c").alias("c2"),
    ])?;
// Perform the equivalent of `left INNER JOIN right ON (a = a2 AND b = b2)`
// finding all pairs of rows from `left` and `right` where `a = a2` and `b = b2`.
let join = left.join(right, JoinType::Inner, &["a", "b"], &["a2", "b2"], None)?;
let expected = vec![
    "+---+---+---+----+----+----+",
    "| a | b | c | a2 | b2 | c2 |",
    "+---+---+---+----+----+----+",
    "| 1 | 2 | 3 | 1  | 2  | 3  |",
    "+---+---+---+----+----+----+",
];
assert_batches_sorted_eq!(expected, &join.collect().await?);
# Ok(())
# }
```

<a id="op-bf2e40975b5857c34b3f7997"></a>
## join_on

`function` · `datafusion::dataframe::DataFrame::join_on` · datafusion 55.1.0

```rust
fn join_on(self, right: DataFrame, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1366`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Join this `DataFrame` with another `DataFrame` using the specified
expressions.

Note that DataFusion automatically optimizes joins, including
identifying and optimizing equality predicates.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let left = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let right = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    .select(vec![
        col("a").alias("a2"),
        col("b").alias("b2"),
        col("c").alias("c2"),
    ])?;

// Perform the equivalent of `left INNER JOIN right ON (a != a2 AND b != b2)`
// finding all pairs of rows from `left` and `right` where
// where `a != a2` and `b != b2`.
let join_on = left.join_on(
    right,
    JoinType::Inner,
    [col("a").not_eq(col("a2")), col("b").not_eq(col("b2"))],
)?;
let expected = vec![
    "+---+---+---+----+----+----+",
    "| a | b | c | a2 | b2 | c2 |",
    "+---+---+---+----+----+----+",
    "+---+---+---+----+----+----+",
];
# assert_batches_sorted_eq!(expected, &join_on.collect().await?);
# Ok(())
# }
```

<a id="op-340a92023d14345723815167"></a>
## limit

`function` · `datafusion::dataframe::DataFrame::limit` · datafusion 55.1.0

```rust
fn limit(self, skip: usize, fetch: Option<usize>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a new `DataFrame` with a limited number of rows.

# Arguments
`skip` - Number of rows to skip before fetch any row
`fetch` - Maximum number of rows to return, after skipping `skip` rows.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let df = df.limit(1, Some(2))?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-cfd0c5a27a88bd8e26fc3d5a"></a>
## logical_plan

`function` · `datafusion::dataframe::DataFrame::logical_plan` · datafusion 55.1.0

```rust
fn logical_plan(&self) -> &LogicalPlan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1685`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a reference to the unoptimized [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) that comprises
this DataFrame.

See [`Self::into_unoptimized_plan`](../operations/datafusion.dataframe.DataFrame.md#op-88c3675dd1b58c8e6f964f13) for more details.

<a id="op-13fb5e41567b47427b495840"></a>
## new

`function` · `datafusion::dataframe::DataFrame::new` · datafusion 55.1.0

```rust
fn new(session_state: SessionState, plan: LogicalPlan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a new `DataFrame ` based on an existing `LogicalPlan`

This is a low-level method and is not typically used by end users. See
[`SessionContext::read_csv`](../operations/datafusion.execution.context.SessionContext.md#op-b76fcba763d91dfabf32a973) and other methods for creating a
`DataFrame` from an existing datasource.

<a id="op-9c1456783d113f9b23c38187"></a>
## parse_sql_expr

`function` · `datafusion::dataframe::DataFrame::parse_sql_expr` · datafusion 55.1.0

```rust
fn parse_sql_expr(&self, sql: &str) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates logical expression from a SQL query text.
The expression is created and processed against the current schema.

# Example: Parsing SQL queries
```
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion::prelude::*;
# use datafusion_common::{DFSchema, Result};
# #[tokio::main]
# async fn main() -> Result<()> {
// datafusion will parse number as i64 first.
let sql = "a > 1 and b in (1, 10)";
let expected = col("a")
    .gt(lit(1 as i64))
    .and(col("b").in_list(vec![lit(1 as i64), lit(10 as i64)], false));
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let expr = df.parse_sql_expr(sql)?;
assert_eq!(expected, expr);
# Ok(())
# }
```

<a id="op-9c4fa029c8b139b78d3af53b"></a>
## registry

`function` · `datafusion::dataframe::DataFrame::registry` · datafusion 55.1.0

```rust
fn registry(&self) -> &dyn FunctionRegistry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1829`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a `FunctionRegistry` used to plan udf's calls

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let f = df.registry();
// use f.udf("name", vec![...]) to use the udf
# Ok(())
# }
```

<a id="op-82194a0d94ad9638b2866707"></a>
## repartition

`function` · `datafusion::dataframe::DataFrame::repartition` · datafusion 55.1.0

```rust
fn repartition(self, partitioning_scheme: Partitioning) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1409`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Repartition a DataFrame based on a logical partitioning scheme.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let df1 = df.repartition(Partitioning::RoundRobinBatch(4))?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df1.collect().await?);
# Ok(())
# }
```

<a id="op-f88bd90cac963f19984e4c39"></a>
## schema

`function` · `datafusion::dataframe::DataFrame::schema` · datafusion 55.1.0

```rust
fn schema(&self) -> &DFSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1677`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the `DFSchema` describing the output of this DataFrame.

The output `DFSchema` contains information on the name, data type, and
nullability for each column.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let schema = df.schema();
# Ok(())
# }
```

<a id="op-f765f70d2feb697e9c649c7b"></a>
## select

`function` · `datafusion::dataframe::DataFrame::select` · datafusion 55.1.0

```rust
fn select(self, expr_list: impl IntoIterator<Item = impl Into<SelectExpr>>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:410`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Project arbitrary expressions (like SQL SELECT expressions) into a new
`DataFrame`.

The output `DataFrame` has one column for each element in `expr_list`.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df.select(vec![col("a"), col("b") * col("c")])?;
let expected = vec![
    "+---+-----------------------+",
    "| a | ?table?.b * ?table?.c |",
    "+---+-----------------------+",
    "| 1 | 6                     |",
    "+---+-----------------------+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-f63530a3ee3fcfb573ae5656"></a>
## select_columns

`function` · `datafusion::dataframe::DataFrame::select_columns` · datafusion 55.1.0

```rust
fn select_columns(self, columns: &[&str]) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Filter the DataFrame by column. Returns a new DataFrame only containing the
specified columns.

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df.select_columns(&["a", "b"])?;
let expected = vec![
    "+---+---+",
    "| a | b |",
    "+---+---+",
    "| 1 | 2 |",
    "+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-3e268e8aca174c3205324798"></a>
## select_exprs

`function` · `datafusion::dataframe::DataFrame::select_exprs` · datafusion 55.1.0

```rust
fn select_exprs(self, exprs: &[&str]) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:373`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Project arbitrary list of expression strings into a new `DataFrame`.
Method will parse string expressions into logical plan expressions.

The output `DataFrame` has one column for each element in `exprs`.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df: DataFrame = df.select_exprs(&["a * b", "c"])?;
# Ok(())
# }
```

<a id="op-c7be59f08097fbe0555e49de"></a>
## show

`function` · `datafusion::dataframe::DataFrame::show` · datafusion 55.1.0

```rust
async fn show(self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1502`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the `DataFrame` and print the results to the console.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
df.show().await?;
# Ok(())
# }
```

<a id="op-56f0f7285edad6dfda2bbfb7"></a>
## show_limit

`function` · `datafusion::dataframe::DataFrame::show_limit` · datafusion 55.1.0

```rust
async fn show_limit(self, num: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1568`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the `DataFrame` and print only the first `num` rows of the
result to the console.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
df.show_limit(10).await?;
# Ok(())
# }
```

<a id="op-493144c438bbc7012a81f330"></a>
## sort

`function` · `datafusion::dataframe::DataFrame::sort` · datafusion 55.1.0

```rust
fn sort(self, expr: Vec<SortExpr>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1241`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Sort the DataFrame by the specified sorting expressions.

Note that any expression can be turned into
a sort expression by calling its [sort](Expr::sort) method.

# Example

```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example_long.csv", CsvReadOptions::new())
    .await?;
let df = df.sort(vec![
    col("a").sort(false, true), // a DESC, nulls first
    col("b").sort(true, false), // b ASC, nulls last
])?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 4 | 5 | 6 |",
    "| 7 | 8 | 9 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

Unresolved upstream links (retained, not inferred): `Expr::sort`.

<a id="op-954dcd288325ef98f1e8fbff"></a>
## sort_by

`function` · `datafusion::dataframe::DataFrame::sort_by` · datafusion 55.1.0

```rust
fn sort_by(self, expr: Vec<Expr>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1199`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Apply a sort by provided expressions with default direction

<a id="op-48beb73e78cdca86650a9e90"></a>
## task_ctx

`function` · `datafusion::dataframe::DataFrame::task_ctx` · datafusion 55.1.0

```rust
fn task_ctx(&self) -> TaskContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1574`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) which would be used to execute this DataFrame

<a id="op-23194f0f609ae93c34b1c54a"></a>
## to_string

`function` · `datafusion::dataframe::DataFrame::to_string` · datafusion 55.1.0

```rust
async fn to_string(self) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1535`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the `DataFrame` and return a string representation of the results.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion::execution::SessionStateBuilder;

# #[tokio::main]
# async fn main() -> Result<()> {
let cfg = SessionConfig::new()
    .set_str("datafusion.format.null", "no-value");
let session_state = SessionStateBuilder::new()
    .with_config(cfg)
    .with_default_features()
    .build();
let ctx = SessionContext::new_with_state(session_state);
let df = ctx.sql("select null as 'null-column'").await?;
let result = df.to_string().await?;
assert_eq!(result,
"+-------------+
| null-column |
+-------------+
| no-value    |
+-------------+"
);
# Ok(())
# }

<a id="op-ef61c631381b9b5946811db6"></a>
## union

`function` · `datafusion::dataframe::DataFrame::union` · datafusion 55.1.0

```rust
fn union(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:763`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the union of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s, preserving duplicate rows.

The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s must have exactly the same schema

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let d2 = df.clone();
let df = df.union(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "| 1 | 2 | 3 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-6738f7d16fc3d7217c31b0bb"></a>
## union_by_name

`function` · `datafusion::dataframe::DataFrame::union_by_name` · datafusion 55.1.0

```rust
fn union_by_name(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:808`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the union of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s using column names, preserving duplicate rows.

The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s are combined using column names rather than position,
filling missing columns with null.


# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let d2 = df
    .clone()
    .select_columns(&["b", "c", "a"])?
    .with_column("d", lit("77"))?;
let df = df.union_by_name(d2)?;
let expected = vec![
    "+---+---+---+----+",
    "| a | b | c | d  |",
    "+---+---+---+----+",
    "| 1 | 2 | 3 |    |",
    "| 1 | 2 | 3 | 77 |",
    "+---+---+---+----+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-83c24fbccb4e6ac64c155de3"></a>
## union_by_name_distinct

`function` · `datafusion::dataframe::DataFrame::union_by_name_distinct` · datafusion 55.1.0

```rust
fn union_by_name_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:890`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the union of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s using column names with all duplicated rows removed.

The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s are combined using column names rather than position,
filling missing columns with null.


# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let d2 = df.clone().select_columns(&["b", "c", "a"])?;
let df = df.union_by_name_distinct(d2)?;
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-26f79a38ddbe6096a2d66b5b"></a>
## union_distinct

`function` · `datafusion::dataframe::DataFrame::union_distinct` · datafusion 55.1.0

```rust
fn union_distinct(self, dataframe: DataFrame) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:849`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Calculate the distinct union of two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s.

The two [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162)s must have exactly the same schema. Any duplicate
rows are discarded.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let d2 = df.clone();
let df = df.union_distinct(d2)?;
// df2 are duplicate of df
let expected = vec![
    "+---+---+---+",
    "| a | b | c |",
    "+---+---+---+",
    "| 1 | 2 | 3 |",
    "+---+---+---+",
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-a62714b7d02a6f39683e377c"></a>
## unnest_columns

`function` · `datafusion::dataframe::DataFrame::unnest_columns` · datafusion 55.1.0

```rust
fn unnest_columns(self, columns: &[&str]) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Expand multiple list/struct columns into a set of rows and new columns.

See also: [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a) documentation for the behavior of `unnest`

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::assert_batches_sorted_eq;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_json("tests/data/unnest.json", JsonReadOptions::default()).await?;
// expand into multiple columns if it's json array, flatten field name if it's nested structure
let df = df.unnest_columns(&["b","c","d"])?;
let expected = vec![
    "+---+------+-------+-----+-----+",
    "| a | b    | c     | d.e | d.f |",
    "+---+------+-------+-----+-----+",
    "| 1 | 2.0  | false | 1   | 2   |",
    "| 1 | 1.3  | true  | 1   | 2   |",
    "| 1 | -6.1 |       | 1   | 2   |",
    "| 2 | 3.0  | false |     |     |",
    "| 2 | 2.3  | true  |     |     |",
    "| 2 | -7.1 |       |     |     |",
    "+---+------+-------+-----+-----+"
];
# assert_batches_sorted_eq!(expected, &df.collect().await?);
# Ok(())
# }
```

<a id="op-d58166773c6174898e7ac8cb"></a>
## unnest_columns_with_options

`function` · `datafusion::dataframe::DataFrame::unnest_columns_with_options` · datafusion 55.1.0

```rust
fn unnest_columns_with_options(self, columns: &[&str], options: UnnestOptions) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:545`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Expand multiple list columns into a set of rows, with
behavior controlled by [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a).

Please see the documentation on [`UnnestOptions`](../operations/datafusion_common.unnest.UnnestOptions.md#op-fc8c0e778d2849560cc4457a) for more
details about the meaning of unnest.

<a id="op-55e521dccb19829cfd3ced1d"></a>
## window

`function` · `datafusion::dataframe::DataFrame::window` · datafusion 55.1.0

```rust
fn window(self, window_exprs: Vec<Expr>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new DataFrame that adds the result of evaluating one or more
window functions ([`Expr::WindowFunction`](../operations/datafusion_expr.expr.Expr.md#op-1ab47cd303f22d10dd7f4cec)) to the existing columns

<a id="op-fdab0d558d59d418b19de16b"></a>
## with_column

`function` · `datafusion::dataframe::DataFrame::with_column` · datafusion 55.1.0

```rust
fn with_column(self, name: &str, expr: Expr) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2197`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Add or replace a column in the DataFrame.

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df.with_column("ab_sum", col("a") + col("b"))?;
# Ok(())
# }
```

<a id="op-7c8da872cbeef847ca9579ac"></a>
## with_column_renamed

`function` · `datafusion::dataframe::DataFrame::with_column_renamed` · datafusion 55.1.0

```rust
fn with_column_renamed(self, old_name: impl Into<String>, new_name: &str) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2274`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Rename one column by applying a new projection. This is a no-op if the column to be
renamed does not exist.

The method supports case sensitive rename with wrapping column name into one of following symbols (  "  or  '  or  `  )

Alternatively setting DataFusion param `datafusion.sql_parser.enable_ident_normalization` to `false` will enable
case sensitive rename without need to wrap column name into special symbols

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df.with_column_renamed("ab_sum", "total")?;

# Ok(())
# }
```

<a id="op-2f2c1fefdb02572c82e74540"></a>
## with_param_values

`function` · `datafusion::dataframe::DataFrame::with_param_values` · datafusion 55.1.0

```rust
fn with_param_values(self, query_values: impl Into<ParamValues>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2384`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Replace all parameters in logical plan with the specified
values, in preparation for execution.

# Example

```
use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
# use datafusion_common::ScalarValue;
let ctx = SessionContext::new();
# ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new()).await?;
let results = ctx
  .sql("SELECT a FROM example WHERE b = $1")
  .await?
   // replace $1 with value 2
  .with_param_values(vec![
     // value at index 0 --> $1
     ScalarValue::from(2i64)
   ])?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+",
   "| a |",
   "+---+",
   "| 1 |",
   "+---+",
 ],
 &results
);
// Note you can also provide named parameters
let results = ctx
  .sql("SELECT a FROM example WHERE b = $my_param")
  .await?
   // replace $my_param with value 2
   // Note you can also use a HashMap as well
  .with_param_values(vec![
      ("my_param", ScalarValue::from(2i64))
   ])?
  .collect()
  .await?;
assert_batches_eq!(
 &[
   "+---+",
   "| a |",
   "+---+",
   "| 1 |",
   "+---+",
 ],
 &results
);
# Ok(())
# }
```

<a id="op-c7d6aa1dd3f12656d0d73109"></a>
## write_csv

`function` · `datafusion::dataframe::DataFrame::write_csv` · datafusion 55.1.0

```rust
async fn write_csv(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<CsvOptions>) -> Result<Vec<RecordBatch>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2062`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the `DataFrame` and write the results to CSV file(s).

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use std::fs;
# #[tokio::main]
# async fn main() -> Result<()> {
use datafusion::dataframe::DataFrameWriteOptions;
let ctx = SessionContext::new();
// Sort the data by column "b" and write it to a new location
ctx.read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    .sort(vec![col("b").sort(true, true)])? // sort by b asc, nulls first
    .write_csv(
        "output.csv",
        DataFrameWriteOptions::new(),
        None, // can also specify CSV writing options here
    )
    .await?;
# fs::remove_file("output.csv")?;
# Ok(())
# }
```

<a id="op-becc9cbaa9f133fe33c29cb9"></a>
## write_json

`function` · `datafusion::dataframe::DataFrame::write_json` · datafusion 55.1.0

```rust
async fn write_json(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<JsonOptions>) -> Result<Vec<RecordBatch>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:2132`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the `DataFrame` and write the results to JSON file(s).

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use std::fs;
# #[tokio::main]
# async fn main() -> Result<()> {
use datafusion::dataframe::DataFrameWriteOptions;
let ctx = SessionContext::new();
// Sort the data by column "b" and write it to a new location
ctx.read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    .sort(vec![col("b").sort(true, true)])? // sort by b asc, nulls first
    .write_json("output.json", DataFrameWriteOptions::new(), None)
    .await?;
# fs::remove_file("output.json")?;
# Ok(())
# }
```

<a id="op-3c9ca113f3c5829bfb6ac6f0"></a>
## write_parquet

`function` · `datafusion::dataframe::DataFrame::write_parquet` · datafusion 55.1.0

```rust
async fn write_parquet(self, path: &str, options: DataFrameWriteOptions, writer_options: Option<TableParquetOptions>) -> Result<Vec<RecordBatch>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "super::DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [105, 2], "filename": "src/dataframe/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/parquet.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the `DataFrame` and write the results to Parquet file(s).

# Example
```
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use std::fs;
# #[tokio::main]
# async fn main() -> Result<()> {
use datafusion::dataframe::DataFrameWriteOptions;
let ctx = SessionContext::new();
// Sort the data by column "b" and write it to a new location
ctx.read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?
    .sort(vec![col("b").sort(true, true)])? // sort by b asc, nulls first
    .write_parquet(
        "output.parquet",
        DataFrameWriteOptions::new(),
        None, // can also specify parquet writing options here
    )
    .await?;
# fs::remove_file("output.parquet")?;
# Ok(())
# }
```

<a id="op-03b06a16dfccd7287df86167"></a>
## write_table

`function` · `datafusion::dataframe::DataFrame::write_table` · datafusion 55.1.0

```rust
async fn write_table(self, table_name: &str, write_options: DataFrameWriteOptions) -> Result<Vec<RecordBatch>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::dataframe::DataFrame", "path": "DataFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [251, 1], "end": [2636, 2], "filename": "src/dataframe/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/dataframe/mod.rs:1998`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute this `DataFrame` and write the results to `table_name`.

Returns a single [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) containing a single column and
row representing the count of total rows written.

Unlike most other `DataFrame` methods, this method executes eagerly.
Data is written to the table using the [`TableProvider::insert_into`](../operations/datafusion_session.table.TableProvider.md#op-f6f850e68c1fe795e5e5fac4)
method. This is the same underlying implementation used by SQL `INSERT
INTO` statements.
