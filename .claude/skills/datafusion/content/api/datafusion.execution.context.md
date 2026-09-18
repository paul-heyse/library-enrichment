# `datafusion::execution::context`

Crate `datafusion` · 6 public items · structured records in [`model/datafusion.execution.context.json`](../model/datafusion.execution.context.json)

## RegisterFunction

`enum` · `datafusion::execution::context::RegisterFunction`

```rust
enum RegisterFunction
```

**Variants**: `Scalar`, `Aggregate`, `Window`, `HigherOrder`, `Table`

**Derives**: Clone, Debug

[Full member, field, variant and typed contracts](../operations/datafusion.execution.context.RegisterFunction.md).


The result of processing a [`CreateFunction`] statement with [`FunctionFactory`].

---

## EmptySerializerRegistry

`struct` · `datafusion::execution::context::EmptySerializerRegistry`

```rust
struct EmptySerializerRegistry
```

**Implements**: `datafusion_expr::registry::SerializerRegistry`

**Derives**: Debug

**via `datafusion_expr::registry::SerializerRegistry`**

```rust
fn deserialize_logical_plan(&self, name: &str, _bytes: &[u8]) -> Result<Arc<dyn UserDefinedLogicalNode>>
fn serialize_logical_plan(&self, node: &dyn UserDefinedLogicalNode) -> Result<Vec<u8>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.context.EmptySerializerRegistry.md).


Default implementation of [SerializerRegistry] that throws unimplemented error
for all requests.

---

## SQLOptions

`struct` · `datafusion::execution::context::SQLOptions`

Also reachable as `datafusion::prelude::SQLOptions`

```rust
struct SQLOptions
```

**Derives**: Clone, Copy, Debug, Default

**Methods** (5)

```rust
fn new() -> Self
fn verify_plan(&self, plan: &LogicalPlan) -> Result<()>
fn with_allow_ddl(self, allow: bool) -> Self
fn with_allow_dml(self, allow: bool) -> Self
fn with_allow_statements(self, allow: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.context.SQLOptions.md).


Describes which SQL statements can be run.

See [`SessionContext::sql_with_options`] for more details.

---

## SessionContext

`struct` · `datafusion::execution::context::SessionContext`

Also reachable as `datafusion::prelude::SessionContext`

```rust
struct SessionContext
```

**Implements**: `core::convert::From`, `datafusion_execution::task::TaskContextProvider`, `datafusion_expr::registry::FunctionRegistry`

**Derives**: Clone, Default

**Methods** (73)

```rust
fn add_analyzer_rule(&self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>)
fn add_optimizer_rule(&self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>)
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
fn catalog_names(&self) -> Vec<String>
fn copied_config(&self) -> SessionConfig
fn copied_table_options(&self) -> TableOptions
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>>
fn deregister_higher_order_function(&self, name: &str)
fn deregister_object_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
fn deregister_table(&self, table_ref: impl Into<TableReference>) -> Result<Option<Arc<dyn TableProvider>>>
fn deregister_udaf(&self, name: &str)
fn deregister_udf(&self, name: &str)
fn deregister_udtf(&self, name: &str)
fn deregister_udwf(&self, name: &str)
fn enable_ident_normalization(&self) -> bool
fn enable_url_table(self) -> Self
async fn execute_logical_plan(&self, plan: LogicalPlan) -> Result<DataFrame>
fn into_state_builder(self) -> SessionStateBuilder
fn new() -> Self
fn new_with_config(config: SessionConfig) -> Self
fn new_with_config_rt(config: SessionConfig, runtime: Arc<RuntimeEnv>) -> Self
fn new_with_state(state: SessionState) -> Self
fn parse_capacity_limit(config_name: &str, limit: &str) -> Result<usize>
fn parse_memory_limit(limit: &str) -> Result<usize>
fn parse_sql_expr(&self, sql: &str, df_schema: &DFSchema) -> Result<Expr>
async fn read_arrow<P: DataFilePaths>(&self, table_paths: P, options: ArrowReadOptions<'_>) -> Result<DataFrame>
async fn read_avro<P: DataFilePaths>(&self, table_paths: P, options: AvroReadOptions<'_>) -> Result<DataFrame>
fn read_batch(&self, batch: RecordBatch) -> Result<DataFrame>
fn read_batches(&self, batches: impl IntoIterator<Item = RecordBatch>) -> Result<DataFrame>
async fn read_csv<P: DataFilePaths>(&self, table_paths: P, options: CsvReadOptions<'_>) -> Result<DataFrame>
fn read_empty(&self) -> Result<DataFrame>
async fn read_json<P: DataFilePaths>(&self, table_paths: P, options: JsonReadOptions<'_>) -> Result<DataFrame>
async fn read_parquet<P: DataFilePaths>(&self, table_paths: P, options: ParquetReadOptions<'_>) -> Result<DataFrame>
fn read_table(&self, provider: Arc<dyn TableProvider>) -> Result<DataFrame>
async fn refresh_catalogs(&self) -> Result<()>
async fn register_arrow(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ArrowReadOptions<'_>) -> Result<()>
async fn register_avro(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: AvroReadOptions<'_>) -> Result<()>
fn register_batch(&self, table_ref: impl Into<TableReference>, batch: RecordBatch) -> Result<Option<Arc<dyn TableProvider>>>
fn register_catalog(&self, name: impl Into<String>, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
fn register_catalog_list(&self, catalog_list: Arc<dyn CatalogProviderList>)
async fn register_csv(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: CsvReadOptions<'_>) -> Result<()>
fn register_higher_order_function(&self, f: Arc<HigherOrderUDF>)
async fn register_json(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: JsonReadOptions<'_>) -> Result<()>
async fn register_listing_table(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ListingOptions, provided_schema: Option<SchemaRef>, sql_definition: Option<String>) -> Result<()>
fn register_object_store(&self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
async fn register_parquet(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ParquetReadOptions<'_>) -> Result<()>
fn register_relation_planner(&self, planner: Arc<dyn RelationPlanner>) -> Result<()>
fn register_table(&self, table_ref: impl Into<TableReference>, provider: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
fn register_table_options_extension<T: ConfigExtension>(&self, extension: T)
fn register_udaf(&self, f: AggregateUDF)
fn register_udf(&self, f: ScalarUDF)
fn register_udtf(&self, name: &str, fun: Arc<dyn TableFunctionImpl>)
fn register_udwf(&self, f: WindowUDF)
fn register_variable(&self, variable_type: VarType, provider: Arc<dyn VarProvider + Send + Sync>)
fn remove_optimizer_rule(&self, name: &str) -> bool
fn runtime_env(&self) -> Arc<RuntimeEnv>
fn session_id(&self) -> String
fn session_start_time(&self) -> DateTime<Utc>
async fn sql(&self, sql: &str) -> Result<DataFrame>
async fn sql_with_options(&self, sql: &str, options: SQLOptions) -> Result<DataFrame>
fn state(&self) -> SessionState
fn state_ref(&self) -> Arc<RwLock<SessionState>>
fn state_weak_ref(&self) -> Weak<RwLock<SessionState>>
async fn table(&self, table_ref: impl Into<TableReference>) -> Result<DataFrame>
fn table_exist(&self, table_ref: impl Into<TableReference>) -> Result<bool>
fn table_factory(&self, file_type: &str) -> Option<Arc<dyn TableProviderFactory>>
fn table_function(&self, name: &str) -> Result<Arc<TableFunction>>
async fn table_provider(&self, table_ref: impl Into<TableReference>) -> Result<Arc<dyn TableProvider>>
fn task_ctx(&self) -> Arc<TaskContext>
fn with_function_factory(self, function_factory: Arc<dyn FunctionFactory>) -> Self
async fn write_csv(&self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> Result<()>
async fn write_json(&self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> Result<()>
async fn write_parquet(&self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>, writer_properties: Option<WriterProperties>) -> Result<()>
```

**via `core::convert::From`**

```rust
fn from(state: SessionState) -> Self
```

**via `datafusion_execution::task::TaskContextProvider`**

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

**via `datafusion_expr::registry::FunctionRegistry`**

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
fn higher_order_function_names(&self) -> HashSet<String>
fn register_expr_planner(&mut self, expr_planner: Arc<dyn ExprPlanner>) -> Result<()>
fn register_function_rewrite(&mut self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> Result<()>
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
fn register_udwf(&mut self, udwf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
fn udafs(&self) -> HashSet<String>
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
fn udfs(&self) -> HashSet<String>
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
fn udwfs(&self) -> HashSet<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.context.SessionContext.md).


Main interface for executing queries with DataFusion. Maintains
the state of the connection between a user and an instance of the
DataFusion engine.

See examples below for how to use the `SessionContext` to execute queries
and how to configure the session.

# Overview

[`SessionContext`] provides the following functionality:

* Create a [`DataFrame`] from a CSV or Parquet data source.
* Register a CSV or Parquet data source as a table that can be referenced from a SQL query.
* Register a custom data source that can be referenced from a SQL query.
* Execute a SQL query

# Example: DataFrame API

The following example demonstrates how to use the context to execute a query against a CSV
data source using the [`DataFrame`] API:

```
use datafusion::prelude::*;
# use datafusion::functions_aggregate::expr_fn::min;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
let df = df
    .filter(col("a").lt_eq(col("b")))?
    .aggregate(vec![col("a")], vec![min(col("b"))])?
    .limit(0, Some(100))?;
let results = df.collect().await?;
assert_batches_eq!(
    &[
        "+---+----------------+",
        "| a | min(?table?.b) |",
        "+---+----------------+",
        "| 1 | 2              |",
        "+---+----------------+",
    ],
    &results
);
# Ok(())
# }
```

# Example: SQL API

The following example demonstrates how to execute the same query using SQL:

```
use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new())
    .await?;
let results = ctx
    .sql("SELECT a, min(b) FROM example GROUP BY a LIMIT 100")
    .await?
    .collect()
    .await?;
assert_batches_eq!(
    &[
        "+---+----------------+",
        "| a | min(example.b) |",
        "+---+----------------+",
        "| 1 | 2              |",
        "+---+----------------+",
    ],
    &results
);
# Ok(())
# }
```

# Example: Configuring `SessionContext`

The `SessionContext` can be configured by creating a [`SessionState`] using
[`SessionStateBuilder`]:

```
# use std::sync::Arc;
# use datafusion::prelude::*;
# use datafusion::execution::SessionStateBuilder;
# use datafusion_execution::runtime_env::RuntimeEnvBuilder;
// Configure a 4k batch size
let config = SessionConfig::new().with_batch_size(4 * 1024);

// configure a memory limit of 1GB with 20%  slop
let runtime_env = RuntimeEnvBuilder::new()
    .with_memory_limit(1024 * 1024 * 1024, 0.80)
    .build_arc()
    .unwrap();

// Create a SessionState using the config and runtime_env
let state = SessionStateBuilder::new()
    .with_config(config)
    .with_runtime_env(runtime_env)
    // include support for built-in functions and configurations
    .with_default_features()
    .build();

// Create a SessionContext
let ctx = SessionContext::from(state);
```

# Relationship between `SessionContext`, `SessionState`, and `TaskContext`

The state required to optimize, and evaluate queries is
broken into three levels to allow tailoring

The objects are:

1. [`SessionContext`]: Most users should use a `SessionContext`. It contains
   all information required to execute queries including  high level APIs such
   as [`SessionContext::sql`]. All queries run with the same `SessionContext`
   share the same configuration and resources (e.g. memory limits).

2. [`SessionState`]: contains information required to plan and execute an
   individual query (e.g. creating a [`LogicalPlan`] or [`ExecutionPlan`]).
   Each query is planned and executed using its own `SessionState`, which can
   be created with [`SessionContext::state`]. `SessionState` allows finer
   grained control over query execution, for example disallowing DDL operations
   such as `CREATE TABLE`.

3. [`TaskContext`] contains the state required for query execution (e.g.
   [`ExecutionPlan::execute`]). It contains a subset of information in
   [`SessionState`]. `TaskContext` allows executing [`ExecutionPlan`]s
   [`PhysicalExpr`]s without requiring a full [`SessionState`].

[`PhysicalExpr`]: crate::physical_expr::PhysicalExpr

---

## DataFilePaths

`trait` · `datafusion::execution::context::DataFilePaths`

```rust
trait DataFilePaths
```

**Implementors** (2)

- `alloc::string::String`
- `alloc::vec::Vec`

**Methods** (1)

```rust
fn to_urls(self) -> Result<Vec<ListingTableUrl>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.context.DataFilePaths.md).


DataFilePaths adds a method to convert strings and vector of strings to vector of [`ListingTableUrl`] URLs.
This allows methods such [`SessionContext::read_csv`] and [`SessionContext::read_avro`]
to take either a single file or multiple files.

---

## FunctionFactory

`trait` · `datafusion::execution::context::FunctionFactory`

```rust
trait FunctionFactory: Debug + Sync + Send
```

**Methods** (1)

```rust
async fn create(&self, state: &SessionState, statement: CreateFunction) -> Result<RegisterFunction>
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.context.FunctionFactory.md).


Interface for handling `CREATE FUNCTION` statements and interacting with
[SessionState] to create and register functions ([`ScalarUDF`],
[`AggregateUDF`], [`WindowUDF`], and [`TableFunctionImpl`]) dynamically.

Implement this trait to create user-defined functions in a custom way, such
as loading from external libraries or defining them programmatically.
DataFusion will parse `CREATE FUNCTION` statements into [`CreateFunction`]
structs and pass them to the [`create`](Self::create) method.

Note there is no default implementation of this trait provided in DataFusion,
because the implementation and requirements vary widely. Please see
[function_factory example] for a reference implementation.

[function_factory example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/builtin_functions/function_factory.rs

# Examples of syntax that can be supported

```sql
CREATE FUNCTION f1(BIGINT)
  RETURNS BIGINT
  RETURN $1 + 1;
```
or
```sql
CREATE FUNCTION to_miles(DOUBLE)
RETURNS DOUBLE
LANGUAGE PYTHON
AS '
import pyarrow.compute as pc

conversation_rate_multiplier = 0.62137119

def to_miles(km_data):
    return pc.multiply(km_data, conversation_rate_multiplier)
'
```

---
