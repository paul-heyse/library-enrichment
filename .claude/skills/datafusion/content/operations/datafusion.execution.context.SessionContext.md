# `datafusion::execution::context::SessionContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.context.SessionContext.json).

<a id="op-640a08e4451b7f418c93b5ee"></a>
## SessionContext

`struct` · `datafusion::execution::context::SessionContext` · datafusion 55.1.0

```rust
struct SessionContext
```

Source: `src/execution/context/mod.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Main interface for executing queries with DataFusion. Maintains
the state of the connection between a user and an instance of the
DataFusion engine.

See examples below for how to use the `SessionContext` to execute queries
and how to configure the session.

# Overview

[`SessionContext`](../operations/datafusion.execution.context.SessionContext.md#op-640a08e4451b7f418c93b5ee) provides the following functionality:

* Create a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) from a CSV or Parquet data source.
* Register a CSV or Parquet data source as a table that can be referenced from a SQL query.
* Register a custom data source that can be referenced from a SQL query.
* Execute a SQL query

# Example: DataFrame API

The following example demonstrates how to use the context to execute a query against a CSV
data source using the [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) API:

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

The `SessionContext` can be configured by creating a [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) using
[`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb):

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

1. [`SessionContext`](../operations/datafusion.execution.context.SessionContext.md#op-640a08e4451b7f418c93b5ee): Most users should use a `SessionContext`. It contains
   all information required to execute queries including  high level APIs such
   as [`SessionContext::sql`](../operations/datafusion.execution.context.SessionContext.md#op-b4bfdbb6f399f7eb128c16c0). All queries run with the same `SessionContext`
   share the same configuration and resources (e.g. memory limits).

2. [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601): contains information required to plan and execute an
   individual query (e.g. creating a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) or [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673)).
   Each query is planned and executed using its own `SessionState`, which can
   be created with [`SessionContext::state`](../operations/datafusion.execution.context.SessionContext.md#op-c893e4a829d5f8655082532f). `SessionState` allows finer
   grained control over query execution, for example disallowing DDL operations
   such as `CREATE TABLE`.

3. [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) contains the state required for query execution (e.g.
   [`ExecutionPlan::execute`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-73dceb67a4da50c62af132d9)). It contains a subset of information in
   [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601). `TaskContext` allows executing [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673)s
   [`PhysicalExpr`]s without requiring a full [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601).

[`PhysicalExpr`]: crate::physical_expr::PhysicalExpr

<a id="op-755ee4265cd130e4bbbb557e"></a>
## add_analyzer_rule

`function` · `datafusion::execution::context::SessionContext::add_analyzer_rule` · datafusion 55.1.0

```rust
fn add_analyzer_rule(&self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Adds an analyzer rule to the end of the existing rules.

See [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) for more control of when the rule is applied.

<a id="op-b98999063e9dea4369d2bb58"></a>
## add_optimizer_rule

`function` · `datafusion::execution::context::SessionContext::add_optimizer_rule` · datafusion 55.1.0

```rust
fn add_optimizer_rule(&self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Adds an optimizer rule to the end of the existing rules.

See [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) for more control of when the rule is applied.

<a id="op-517c4cfd1fe8d0e5e4dbc3f2"></a>
## catalog

`function` · `datafusion::execution::context::SessionContext::catalog` · datafusion 55.1.0

```rust
fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1934`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieves a [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) instance by name

<a id="op-7e5ff624dc5327e64bb653ba"></a>
## catalog_names

`function` · `datafusion::execution::context::SessionContext::catalog_names` · datafusion 55.1.0

```rust
fn catalog_names(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1929`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieves the list of available catalog names.

<a id="op-29ab9c4c0a4aa8b8a6fd9e34"></a>
## clone

`function` · `datafusion::execution::context::SessionContext::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> SessionContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [292, 10], "end": [292, 15], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution/context/mod.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03988d64bc59d2fab73e65bf"></a>
## copied_config

`function` · `datafusion::execution::context::SessionContext::copied_config` · datafusion 55.1.0

```rust
fn copied_config(&self) -> SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:575`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a copied version of config for this Session

<a id="op-459fc83faeed845cf86ff364"></a>
## copied_table_options

`function` · `datafusion::execution::context::SessionContext::copied_table_options` · datafusion 55.1.0

```rust
fn copied_table_options(&self) -> TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a copied version of table options for this Session

<a id="op-11fb97c3cf205078f236f9c2"></a>
## create_physical_expr

`function` · `datafusion::execution::context::SessionContext::create_physical_expr` · datafusion 55.1.0

```rust
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:798`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) from an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) after applying type
coercion and function rewrites.

Note: The expression is not [simplified] or otherwise optimized:
`a = 1 + 2` will not be simplified to `a = 3` as this is a more involved process.
See the [expr_api] example for how to simplify expressions.

# Example
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion::prelude::*;
# use datafusion_common::DFSchema;
// a = 1 (i64)
let expr = col("a").eq(lit(1i64));
// provide type information that `a` is an Int32
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
// Create a PhysicalExpr. Note DataFusion automatically coerces (casts) `1i64` to `1i32`
let physical_expr = SessionContext::new()
  .create_physical_expr(expr, &df_schema).unwrap();
```
# See Also
* [`SessionState::create_physical_expr`](../operations/datafusion.execution.session_state.SessionState.md#op-bd782e590c997fe38ab9930b) for a lower level API

[simplified]: datafusion_optimizer::simplify_expressions
[expr_api]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/query_planning/expr_api.rs

<a id="op-3b81c1bfda0a8bcbec41bbc5"></a>
## default

`function` · `datafusion::execution::context::SessionContext::default` · datafusion 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 1], "end": [306, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/execution/context/mod.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-437764baf07301ec89a420cc"></a>
## deregister_higher_order_function

`function` · `datafusion::execution::context::SessionContext::deregister_higher_order_function` · datafusion 55.1.0

```rust
fn deregister_higher_order_function(&self, name: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1693`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters a higher-order function within this context.

<a id="op-f95d0d564746735504e2da3d"></a>
## deregister_object_store

`function` · `datafusion::execution::context::SessionContext::deregister_object_store` · datafusion 55.1.0

```rust
fn deregister_object_store(&self, url: &Url) -> Result<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:531`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) associated with the specific URL prefix.

See [`RuntimeEnv::deregister_object_store`] for more details.

Unresolved upstream links (retained, not inferred): ``RuntimeEnv::deregister_object_store``.

<a id="op-f5d07d9370f75307187850a6"></a>
## deregister_table

`function` · `datafusion::execution::context::SessionContext::deregister_table` · datafusion 55.1.0

```rust
fn deregister_table(&self, table_ref: impl Into<TableReference>) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1959`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters the given table.

Returns the registered provider, if any

<a id="op-c96aa3f4295d1d760c81fecd"></a>
## deregister_udaf

`function` · `datafusion::execution::context::SessionContext::deregister_udaf` · datafusion 55.1.0

```rust
fn deregister_udaf(&self, name: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1701`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters a UDAF within this context.

<a id="op-c85aae1bccaa7a35e6590770"></a>
## deregister_udf

`function` · `datafusion::execution::context::SessionContext::deregister_udf` · datafusion 55.1.0

```rust
fn deregister_udf(&self, name: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1688`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters a UDF within this context.

<a id="op-6ae0d03052ac3a27e2c25504"></a>
## deregister_udtf

`function` · `datafusion::execution::context::SessionContext::deregister_udtf` · datafusion 55.1.0

```rust
fn deregister_udtf(&self, name: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1711`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters a UDTF within this context.

<a id="op-6eb4cfeaf9085981361c7d2c"></a>
## deregister_udwf

`function` · `datafusion::execution::context::SessionContext::deregister_udwf` · datafusion 55.1.0

```rust
fn deregister_udwf(&self, name: &str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1706`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregisters a UDWF within this context.

<a id="op-4e36fafc43786ae63335de8a"></a>
## enable_ident_normalization

`function` · `datafusion::execution::context::SessionContext::enable_ident_normalization` · datafusion 55.1.0

```rust
fn enable_ident_normalization(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the `enable_ident_normalization` of this Session

<a id="op-546cdd8124bc850e0769a7bc"></a>
## enable_url_table

`function` · `datafusion::execution::context::SessionContext::enable_url_table` · datafusion 55.1.0

```rust
fn enable_url_table(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:414`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Enable querying local files as tables.

This feature is security sensitive and should only be enabled for
systems that wish to permit direct access to the file system from SQL.

When enabled, this feature permits direct access to arbitrary files via
SQL like

```sql
SELECT * from 'my_file.parquet'
```

See [DynamicFileCatalog](../operations/datafusion_catalog.dynamic_file.catalog.DynamicFileCatalog.md#op-5c92b657507bb03943531628) for more details

```
# use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new()
  .enable_url_table(); // permit local file access
let results = ctx
  .sql("SELECT a, MIN(b) FROM 'tests/data/example.csv' as example GROUP BY a LIMIT 100")
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

<a id="op-7501d393546810f2b8789dd6"></a>
## execute_logical_plan

`function` · `datafusion::execution::context::SessionContext::execute_logical_plan` · datafusion 55.1.0

```rust
async fn execute_logical_plan(&self, plan: LogicalPlan) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:686`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Execute the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da), return a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162). This API
is not featured limited (so all SQL such as `CREATE TABLE` and
`COPY` will be run).

If you wish to limit the type of plan that can be run from
SQL, see [`Self::sql_with_options`](../operations/datafusion.execution.context.SessionContext.md#op-7e09781b035fd8617b201286) and
[`SQLOptions::verify_plan`](../operations/datafusion.execution.context.SQLOptions.md#op-93354441f7a2ec957b5995ae).

<a id="op-0330e544348303a78b2857a6"></a>
## expr_planners

`function` · `datafusion::execution::context::SessionContext::expr_planners` · datafusion 55.1.0

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2134`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e817e6e8acd78346b720560"></a>
## from

`function` · `datafusion::execution::context::SessionContext::from` · datafusion 55.1.0

```rust
fn from(state: SessionState) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2171, 1], "end": [2175, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/execution/context/mod.rs:2172`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-359cc0ec67358af051d65d87"></a>
## higher_order_function

`function` · `datafusion::execution::context::SessionContext::higher_order_function` · datafusion 55.1.0

```rust
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2093`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3e5d1677c2fa36b9cc84308"></a>
## higher_order_function_names

`function` · `datafusion::execution::context::SessionContext::higher_order_function_names` · datafusion 55.1.0

```rust
fn higher_order_function_names(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2145`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fff4edf0caa0f9e7d4e0016f"></a>
## into_state_builder

`function` · `datafusion::execution::context::SessionContext::into_state_builder` · datafusion 55.1.0

```rust
fn into_state_builder(self) -> SessionStateBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Convert the current `SessionContext` into a [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb)

This is useful to switch back to `SessionState` with custom settings such as
[`Self::enable_url_table`](../operations/datafusion.execution.context.SessionContext.md#op-546cdd8124bc850e0769a7bc).

Avoids cloning the SessionState if possible.

# Example
```
# use std::sync::Arc;
# use datafusion::prelude::*;
# use datafusion::execution::SessionStateBuilder;
# use datafusion_optimizer::push_down_filter::PushDownFilter;
let my_rule = PushDownFilter {}; // pretend it is a new rule
                                 // Create a new builder with a custom optimizer rule
let context: SessionContext = SessionStateBuilder::new()
    .with_optimizer_rule(Arc::new(my_rule))
    .build()
    .into();
// Enable local file access and convert context back to a builder
let builder = context.enable_url_table().into_state_builder();
```

<a id="op-d67607d500dc3f34e5822b18"></a>
## new

`function` · `datafusion::execution::context::SessionContext::new` · datafusion 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new `SessionContext` using the default [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08).

<a id="op-2c1cebd79b1fd253b4e8026f"></a>
## new_with_config

`function` · `datafusion::execution::context::SessionContext::new_with_config` · datafusion 55.1.0

```rust
fn new_with_config(config: SessionConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new `SessionContext` using the provided
[`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08) and a new [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67).

See [`Self::new_with_config_rt`](../operations/datafusion.execution.context.SessionContext.md#op-7f3780563402b213f712b709) for more details on resource
limits.

<a id="op-7f3780563402b213f712b709"></a>
## new_with_config_rt

`function` · `datafusion::execution::context::SessionContext::new_with_config_rt` · datafusion 55.1.0

```rust
fn new_with_config_rt(config: SessionConfig, runtime: Arc<RuntimeEnv>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new `SessionContext` using the provided
[`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08) and a [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67).

# Resource Limits

By default, each new `SessionContext` creates a new
`RuntimeEnv`, and therefore will not enforce memory or disk
limits for queries run on different `SessionContext`s.

To enforce resource limits (e.g. to limit the total amount of
memory used) across all DataFusion queries in a process,
all `SessionContext`'s should be configured with the
same `RuntimeEnv`.

<a id="op-e14c41b4dc42a0cf61fc4af5"></a>
## new_with_state

`function` · `datafusion::execution::context::SessionContext::new_with_state` · datafusion 55.1.0

```rust
fn new_with_state(state: SessionState) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new `SessionContext` using the provided [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)

<a id="op-617768e8b4c5a1e77c005fbc"></a>
## parse_capacity_limit

`function` · `datafusion::execution::context::SessionContext::parse_capacity_limit` · datafusion 55.1.0

```rust
fn parse_capacity_limit(config_name: &str, limit: &str) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1331`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Parse capacity limit from string to number of bytes by allowing units: K, M and G.
Supports formats like '1.5G', '100M', '512K'. Capacity limit can be set to 0 with '0'.

# Examples
```
use datafusion::execution::context::SessionContext;

assert_eq!(
    SessionContext::parse_capacity_limit("datafusion.runtime.memory_limit", "1M").unwrap(),
    1024 * 1024
);
assert_eq!(
    SessionContext::parse_capacity_limit("datafusion.runtime.memory_limit", "1.5G").unwrap(),
    (1.5 * 1024.0 * 1024.0 * 1024.0) as usize
);
```

<a id="op-b9d3d118202df3a24a84cc26"></a>
## parse_memory_limit

`function` · `datafusion::execution::context::SessionContext::parse_memory_limit` · datafusion 55.1.0

```rust
fn parse_memory_limit(limit: &str) -> Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1289`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Parse memory limit from string to number of bytes
Supports formats like '1.5G', '100M', '512K'

# Examples
```
use datafusion::execution::context::SessionContext;

assert_eq!(
    SessionContext::parse_memory_limit("1M").unwrap(),
    1024 * 1024
);
assert_eq!(
    SessionContext::parse_memory_limit("1.5G").unwrap(),
    (1.5 * 1024.0 * 1024.0 * 1024.0) as usize
);
```

<a id="op-1bc0f926c708023cf129ad51"></a>
## parse_sql_expr

`function` · `datafusion::execution::context::SessionContext::parse_sql_expr` · datafusion 55.1.0

```rust
fn parse_sql_expr(&self, sql: &str, df_schema: &DFSchema) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates logical expressions from SQL query text.

# Example: Parsing SQL queries

```
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion::prelude::*;
# use datafusion_common::{DFSchema, Result};
# #[tokio::main]
# async fn main() -> Result<()> {
// datafusion will parse number as i64 first.
let sql = "a > 10";
let expected = col("a").gt(lit(10 as i64));
// provide type information that `a` is an Int32
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
let expr = SessionContext::new().parse_sql_expr(sql, &df_schema)?;
assert_eq!(expected, expr);
# Ok(())
# }
```

<a id="op-c7fcb69baf1f180a4c6b0d43"></a>
## read_arrow

`function` · `datafusion::execution::context::SessionContext::read_arrow` · datafusion 55.1.0

```rust
async fn read_arrow<P: DataFilePaths>(&self, table_paths: P, options: ArrowReadOptions<'_>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1772`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading an Arrow data source.

For more control such as reading multiple files, you can use
[`read_table`](Self::read_table) with a [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

For an example, see [`read_csv`](Self::read_csv)

<a id="op-62bc23af61d323278c5825d1"></a>
## read_avro

`function` · `datafusion::execution::context::SessionContext::read_avro` · datafusion 55.1.0

```rust
async fn read_avro<P: DataFilePaths>(&self, table_paths: P, options: AvroReadOptions<'_>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [61, 2], "filename": "src/execution/context/avro.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/avro.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading an Avro data source.

For more control such as reading multiple files, you can use
[`read_table`](Self::read_table) with a [`super::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

For an example, see [`read_csv`](Self::read_csv)

<a id="op-e5666dd7031c0e383c72e9a7"></a>
## read_batch

`function` · `datafusion::execution::context::SessionContext::read_batch` · datafusion 55.1.0

```rust
fn read_batch(&self, batch: RecordBatch) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1799`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

<a id="op-08c53e4873c2c1b1be7d9d55"></a>
## read_batches

`function` · `datafusion::execution::context::SessionContext::read_batches` · datafusion 55.1.0

```rust
fn read_batches(&self, batches: impl IntoIterator<Item = RecordBatch>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1812`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading a [`Vec[`RecordBatch`]`]

<a id="op-b76fcba763d91dfabf32a973"></a>
## read_csv

`function` · `datafusion::execution::context::SessionContext::read_csv` · datafusion 55.1.0

```rust
async fn read_csv<P: DataFilePaths>(&self, table_paths: P, options: CsvReadOptions<'_>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [94, 2], "filename": "src/execution/context/csv.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/csv.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading a CSV data source.

For more control such as reading multiple files, you can use
[`read_table`](Self::read_table) with a [`super::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

Example usage is given below:

```
use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
// You can read a single file using `read_csv`
let df = ctx
    .read_csv("tests/data/example.csv", CsvReadOptions::new())
    .await?;
// you can also read multiple files:
let df = ctx
    .read_csv(
        vec!["tests/data/example.csv", "tests/data/example.csv"],
        CsvReadOptions::new(),
    )
    .await?;
# Ok(())
# }
```

<a id="op-9565f47ace9363d227622f16"></a>
## read_empty

`function` · `datafusion::execution::context::SessionContext::read_empty` · datafusion 55.1.0

```rust
fn read_empty(&self) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1781`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates an empty DataFrame.

<a id="op-b1eeb48b59162f664cd65c5a"></a>
## read_json

`function` · `datafusion::execution::context::SessionContext::read_json` · datafusion 55.1.0

```rust
async fn read_json<P: DataFilePaths>(&self, table_paths: P, options: JsonReadOptions<'_>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [72, 2], "filename": "src/execution/context/json.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/json.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading an JSON data source.

For more control such as reading multiple files, you can use
[`read_table`](Self::read_table) with a [`super::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

For an example, see [`read_csv`](Self::read_csv)

<a id="op-f67a957dcf9eb2247139f644"></a>
## read_parquet

`function` · `datafusion::execution::context::SessionContext::read_parquet` · datafusion 55.1.0

```rust
async fn read_parquet<P: DataFilePaths>(&self, table_paths: P, options: ParquetReadOptions<'_>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [97, 2], "filename": "src/execution/context/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/parquet.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for reading a Parquet data source.

For more control such as reading multiple files, you can use
[`read_table`](Self::read_table) with a [`super::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a).

For an example, see [`read_csv`](Self::read_csv)

# Note: Statistics

NOTE: by default, statistics are collected when reading the Parquet
files This can slow down the initial DataFrame creation while
greatly accelerating queries with certain filters.

To disable statistics collection, set the [config option]
`datafusion.execution.collect_statistics` to `false`. See
[`ConfigOptions`] and [`ExecutionOptions::collect_statistics`] for more
details.

[config option]: https://datafusion.apache.org/user-guide/configs.html
[`ConfigOptions`]: crate::config::ConfigOptions
[`ExecutionOptions::collect_statistics`]: crate::config::ExecutionOptions::collect_statistics

<a id="op-231760ea5284214c3cf2b725"></a>
## read_table

`function` · `datafusion::execution::context::SessionContext::read_table` · datafusion 55.1.0

```rust
fn read_table(&self, provider: Arc<dyn TableProvider>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1790`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) for a [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) such as a
[`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) or a custom user defined provider.

<a id="op-894fce8289abc9c95591622d"></a>
## refresh_catalogs

`function` · `datafusion::execution::context::SessionContext::refresh_catalogs` · datafusion 55.1.0

```rust
async fn refresh_catalogs(&self) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Finds any [`ListingSchemaProvider`](../operations/datafusion_catalog.listing_schema.ListingSchemaProvider.md#op-2ea5d688cb1b249139f89be4)s and instructs them to reload tables from "disk"

<a id="op-4b695cbdb525859711d84546"></a>
## register_arrow

`function` · `datafusion::execution::context::SessionContext::register_arrow` · datafusion 55.1.0

```rust
async fn register_arrow(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ArrowReadOptions<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1891`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers an Arrow file as a table that can be referenced from
SQL statements executed against this context.

<a id="op-3c896ae8225730567a16d289"></a>
## register_avro

`function` · `datafusion::execution::context::SessionContext::register_avro` · datafusion 55.1.0

```rust
async fn register_avro(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: AvroReadOptions<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 1], "end": [61, 2], "filename": "src/execution/context/avro.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/avro.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers an Avro file as a table that can be referenced from
SQL statements executed against this context.

<a id="op-c892d9e97c7e178a0c639baa"></a>
## register_batch

`function` · `datafusion::execution::context::SessionContext::register_batch` · datafusion 55.1.0

```rust
fn register_batch(&self, table_ref: impl Into<TableReference>, batch: RecordBatch) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers the given [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) as the specified table reference.

<a id="op-e41f74a6c23bfbacd2b4c5c3"></a>
## register_catalog

`function` · `datafusion::execution::context::SessionContext::register_catalog` · datafusion 55.1.0

```rust
fn register_catalog(&self, name: impl Into<String>, catalog: Arc<dyn CatalogProvider>) -> Option<Arc<dyn CatalogProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1916`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a named catalog using a custom `CatalogProvider` so that
it can be referenced from SQL statements executed against this
context.

Returns the [`CatalogProvider`](../operations/datafusion_session.catalog.CatalogProvider.md#op-37a065b67403b669ccbe6bad) previously registered for this
name, if any

<a id="op-2e3cea13ccd0a04f5f7efd16"></a>
## register_catalog_list

`function` · `datafusion::execution::context::SessionContext::register_catalog_list` · datafusion 55.1.0

```rust
fn register_catalog_list(&self, catalog_list: Arc<dyn CatalogProviderList>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2071`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Register [`CatalogProviderList`](../operations/datafusion_session.catalog.CatalogProviderList.md#op-d1c9ece1dd28ba403a6492b6) in [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)

<a id="op-9032e0c39b3d8e852a34a7ad"></a>
## register_csv

`function` · `datafusion::execution::context::SessionContext::register_csv` · datafusion 55.1.0

```rust
async fn register_csv(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: CsvReadOptions<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [94, 2], "filename": "src/execution/context/csv.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/csv.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a CSV file as a table which can referenced from SQL
statements executed against this context.

<a id="op-e31282d2f7371727b176abc7"></a>
## register_expr_planner

`function` · `datafusion::execution::context::SessionContext::register_expr_planner` · datafusion 55.1.0

```rust
fn register_expr_planner(&mut self, expr_planner: Arc<dyn ExprPlanner>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2138`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57bd3074bec710b3948f61ca"></a>
## register_function_rewrite

`function` · `datafusion::execution::context::SessionContext::register_function_rewrite` · datafusion 55.1.0

```rust
fn register_function_rewrite(&mut self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2127`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21a1564d2b39dc23ea8aea7c"></a>
## register_higher_order_function

`function` · `datafusion::execution::context::SessionContext::register_higher_order_function` · datafusion 55.1.0

```rust
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2109`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6a3ede43c05f96fceffcb84"></a>
## register_higher_order_function

`function` · `datafusion::execution::context::SessionContext::register_higher_order_function` · datafusion 55.1.0

```rust
fn register_higher_order_function(&self, f: Arc<HigherOrderUDF>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1648`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a higher-order function within this context.

Note in SQL queries, function names are looked up using
lowercase unless the query uses quotes. For example,

- `SELECT MY_HIGHER_ORDER_FUNC(x)...` will look for a function named `"my_higher_order_func"`
- `SELECT "my_HIGHER_ORDER_FUNC"(x)` will look for a function named `"my_HIGHER_ORDER_FUNC"`

Any functions registered with the function name or its aliases will be overwritten with this new function

<a id="op-7404e9c179939b970364d2f1"></a>
## register_json

`function` · `datafusion::execution::context::SessionContext::register_json` · datafusion 55.1.0

```rust
async fn register_json(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: JsonReadOptions<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [72, 2], "filename": "src/execution/context/json.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/json.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a JSON file as a table that it can be referenced
from SQL statements executed against this context.

<a id="op-11b2c31eb811d4e8afedace8"></a>
## register_listing_table

`function` · `datafusion::execution::context::SessionContext::register_listing_table` · datafusion 55.1.0

```rust
async fn register_listing_table(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ListingOptions, provided_schema: Option<SchemaRef>, sql_definition: Option<String>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1841`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a) that can assemble multiple files
from locations in an [`ObjectStore`] instance into a single
table.

This method is `async` because it might need to resolve the schema.

[`ObjectStore`]: object_store::ObjectStore

<a id="op-c0b8a75bdfc12fe8a5ee5d8e"></a>
## register_object_store

`function` · `datafusion::execution::context::SessionContext::register_object_store` · datafusion 55.1.0

```rust
fn register_object_store(&self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:520`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers an [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) to be used with a specific URL prefix.

See [`RuntimeEnv::register_object_store`] for more details.

# Example: register a local object store for the "file://" URL prefix
```
# use std::sync::Arc;
# use datafusion::prelude::SessionContext;
# use datafusion_execution::object_store::ObjectStoreUrl;
let object_store_url = ObjectStoreUrl::parse("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
let ctx = SessionContext::new();
// All files with the file:// url prefix will be read from the local file system
ctx.register_object_store(object_store_url.as_ref(), Arc::new(object_store));
```

Unresolved upstream links (retained, not inferred): ``RuntimeEnv::register_object_store``.

<a id="op-ed9dd71702ab67adabc3f940"></a>
## register_parquet

`function` · `datafusion::execution::context::SessionContext::register_parquet` · datafusion 55.1.0

```rust
async fn register_parquet(&self, table_ref: impl Into<TableReference>, table_path: impl AsRef<str>, options: ParquetReadOptions<'_>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [97, 2], "filename": "src/execution/context/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/parquet.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a Parquet file as a table that can be referenced from SQL
statements executed against this context.

# Note: Statistics

Statistics are not collected by default. See  [`read_parquet`] for more
details and how to enable them.

[`read_parquet`]: Self::read_parquet

<a id="op-fa78bed857cfb6714fe4f92c"></a>
## register_relation_planner

`function` · `datafusion::execution::context::SessionContext::register_relation_planner` · datafusion 55.1.0

```rust
fn register_relation_planner(&self, planner: Arc<dyn RelationPlanner>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1680`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`RelationPlanner`](../operations/datafusion_expr.planner.RelationPlanner.md#op-1be858ef3f752319489e5fc4) to customize SQL table-factor planning.

Planners are invoked in reverse registration order, allowing newer
planners to take precedence over existing ones.

<a id="op-311a1b8767fde50f77b0e98a"></a>
## register_table

`function` · `datafusion::execution::context::SessionContext::register_table` · datafusion 55.1.0

```rust
fn register_table(&self, table_ref: impl Into<TableReference>, provider: Arc<dyn TableProvider>) -> Result<Option<Arc<dyn TableProvider>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1943`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) as a table that can be
referenced from SQL statements executed against this context.

If a table of the same name was already registered, returns "Table
already exists" error.

<a id="op-148bd823d50d96084a25ceb9"></a>
## register_table_options_extension

`function` · `datafusion::execution::context::SessionContext::register_table_options_extension` · datafusion 55.1.0

```rust
fn register_table_options_extension<T: ConfigExtension>(&self, extension: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2077`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c) as a table option extension that can be
referenced from SQL statements executed against this context.

<a id="op-cf2081744220282d3df184d0"></a>
## register_udaf

`function` · `datafusion::execution::context::SessionContext::register_udaf` · datafusion 55.1.0

```rust
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2116`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb2a7917a957a66f2adf8acf"></a>
## register_udaf

`function` · `datafusion::execution::context::SessionContext::register_udaf` · datafusion 55.1.0

```rust
fn register_udaf(&self, f: AggregateUDF)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1660`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers an aggregate UDF within this context.

Note in SQL queries, aggregate names are looked up using
lowercase unless the query uses quotes. For example,

- `SELECT MY_UDAF(x)...` will look for an aggregate named `"my_udaf"`
- `SELECT "my_UDAF"(x)` will look for an aggregate named `"my_UDAF"`

<a id="op-179ce6dbbb9d770c3a4c4d99"></a>
## register_udf

`function` · `datafusion::execution::context::SessionContext::register_udf` · datafusion 55.1.0

```rust
fn register_udf(&self, f: ScalarUDF)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1634`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a scalar UDF within this context.

Note in SQL queries, function names are looked up using
lowercase unless the query uses quotes. For example,

- `SELECT MY_FUNC(x)...` will look for a function named `"my_func"`
- `SELECT "my_FUNC"(x)` will look for a function named `"my_FUNC"`

Any functions registered with the udf name or its aliases will be overwritten with this new function

<a id="op-382d4f326db21d4ab9501b8b"></a>
## register_udf

`function` · `datafusion::execution::context::SessionContext::register_udf` · datafusion 55.1.0

```rust
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2105`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37fa5c45214385fa6b872c28"></a>
## register_udtf

`function` · `datafusion::execution::context::SessionContext::register_udtf` · datafusion 55.1.0

```rust
fn register_udtf(&self, name: &str, fun: Arc<dyn TableFunctionImpl>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1621`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Register a table UDF with this context

<a id="op-6f6f32c78e2ad5b9964c9673"></a>
## register_udwf

`function` · `datafusion::execution::context::SessionContext::register_udwf` · datafusion 55.1.0

```rust
fn register_udwf(&mut self, udwf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2123`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c8e4811ab5aa75ef71b7acb"></a>
## register_udwf

`function` · `datafusion::execution::context::SessionContext::register_udwf` · datafusion 55.1.0

```rust
fn register_udwf(&self, f: WindowUDF)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1671`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a window UDF within this context.

Note in SQL queries, window function names are looked up using
lowercase unless the query uses quotes. For example,

- `SELECT MY_UDWF(x)...` will look for a window function named `"my_udwf"`
- `SELECT "my_UDWF"(x)` will look for a window function named `"my_UDWF"`

<a id="op-63dbe47d24b968fede087704"></a>
## register_variable

`function` · `datafusion::execution::context::SessionContext::register_variable` · datafusion 55.1.0

```rust
fn register_variable(&self, variable_type: VarType, provider: Arc<dyn VarProvider + Send + Sync>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1609`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a variable provider within this context.

<a id="op-1289d03cbb76a79667816357"></a>
## remove_optimizer_rule

`function` · `datafusion::execution::context::SessionContext::remove_optimizer_rule` · datafusion 55.1.0

```rust
fn remove_optimizer_rule(&self, name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Removes an optimizer rule by name, returning `true` if it existed.

<a id="op-1c4a4a1b224035d421a4bd34"></a>
## runtime_env

`function` · `datafusion::execution::context::SessionContext::runtime_env` · datafusion 55.1.0

```rust
fn runtime_env(&self) -> Arc<RuntimeEnv>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:546`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the [RuntimeEnv](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) used to run queries with this `SessionContext`

<a id="op-80aa68aa5c659ecd92fe2eec"></a>
## session_id

`function` · `datafusion::execution::context::SessionContext::session_id` · datafusion 55.1.0

```rust
fn session_id(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:551`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns an id that uniquely identifies this `SessionContext`.

<a id="op-3d22bb6edfc29226b53b9734"></a>
## session_start_time

`function` · `datafusion::execution::context::SessionContext::session_start_time` · datafusion 55.1.0

```rust
fn session_start_time(&self) -> DateTime<Utc>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:470`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the time this `SessionContext` was created

<a id="op-b4bfdbb6f399f7eb128c16c0"></a>
## sql

`function` · `datafusion::execution::context::SessionContext::sql` · datafusion 55.1.0

```rust
async fn sql(&self, sql: &str) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:611`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) from SQL query text.

Note: This API implements DDL statements such as `CREATE TABLE` and
`CREATE VIEW` and DML statements such as `INSERT INTO` with in-memory
default implementations. See [`Self::sql_with_options`](../operations/datafusion.execution.context.SessionContext.md#op-7e09781b035fd8617b201286).

# Example: Running SQL queries

See the example on [`Self`](../operations/datafusion.execution.context.SessionContext.md#op-640a08e4451b7f418c93b5ee)

# Example: Creating a Table with SQL

```
use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
ctx.sql("CREATE TABLE foo (x INTEGER)")
    .await?
    .collect()
    .await?;
assert!(ctx.table_exist("foo").unwrap());
# Ok(())
# }
```

<a id="op-7e09781b035fd8617b201286"></a>
## sql_with_options

`function` · `datafusion::execution::context::SessionContext::sql_with_options` · datafusion 55.1.0

```rust
async fn sql_with_options(&self, sql: &str, options: SQLOptions) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) from SQL query text, first validating
that the queries are allowed by `options`

# Example: Preventing Creating a Table with SQL

If you want to avoid creating tables, or modifying data or the
session, set [`SQLOptions`](../operations/datafusion.execution.context.SQLOptions.md#op-45fe1731f80c6b691c6957b5) appropriately:

```
use datafusion::prelude::*;
# use datafusion::{error::Result};
# use datafusion::physical_plan::collect;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let options = SQLOptions::new().with_allow_ddl(false);
let err = ctx
    .sql_with_options("CREATE TABLE foo (x INTEGER)", options)
    .await
    .unwrap_err();
assert!(err
    .to_string()
    .starts_with("Error during planning: DDL not supported: CreateMemoryTable"));
# Ok(())
# }
```

<a id="op-c893e4a829d5f8655082532f"></a>
## state

`function` · `datafusion::execution::context::SessionContext::state` · datafusion 55.1.0

```rust
fn state(&self) -> SessionState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2054`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new  [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) suitable for executing a single query.

Notes:

1. `query_execution_start_time` is set to the current time for the
   returned state.

2. The returned state is not shared with the current session state
   and this changes to the returned `SessionState` such as changing
   [`ConfigOptions`] will not be reflected in this `SessionContext`.

[`ConfigOptions`]: crate::config::ConfigOptions

<a id="op-ce27662b6f50f072517f924a"></a>
## state_ref

`function` · `datafusion::execution::context::SessionContext::state_ref` · datafusion 55.1.0

```rust
fn state_ref(&self) -> Arc<RwLock<SessionState>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2061`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get reference to [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)

<a id="op-2a21b53ef6c3da7c37526622"></a>
## state_weak_ref

`function` · `datafusion::execution::context::SessionContext::state_weak_ref` · datafusion 55.1.0

```rust
fn state_weak_ref(&self) -> Weak<RwLock<SessionState>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2066`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get weak reference to [`SessionState`](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601)

<a id="op-216e2604d49b27d1e2dbf357"></a>
## table

`function` · `datafusion::execution::context::SessionContext::table` · datafusion 55.1.0

```rust
async fn table(&self, table_ref: impl Into<TableReference>) -> Result<DataFrame>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1997`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieves a [`DataFrame`](../operations/datafusion.dataframe.DataFrame.md#op-4dea21cb0a990412dd05d162) representing a table previously
registered by calling the [`register_table`] function.

Returns an error if no table has been registered with the
provided reference.

[`register_table`]: SessionContext::register_table

<a id="op-0d8af8c3b01025de9a3005be"></a>
## table_exist

`function` · `datafusion::execution::context::SessionContext::table_exist` · datafusion 55.1.0

```rust
fn table_exist(&self, table_ref: impl Into<TableReference>) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:1979`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return `true` if the specified table exists in the schema provider.

<a id="op-61faac47daf7e4216f25c5da"></a>
## table_factory

`function` · `datafusion::execution::context::SessionContext::table_factory` · datafusion 55.1.0

```rust
fn table_factory(&self, file_type: &str) -> Option<Arc<dyn TableProviderFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:557`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the [`TableProviderFactory`](../operations/datafusion_session.table.TableProviderFactory.md#op-69227d35dfbf2f4aef45aae7) that is registered for the
specified file type, if any.

<a id="op-0d90a2ca51764fab6834688a"></a>
## table_function

`function` · `datafusion::execution::context::SessionContext::table_function` · datafusion 55.1.0

```rust
fn table_function(&self, name: &str) -> Result<Arc<TableFunction>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2014`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieves a [`TableFunction`](../operations/datafusion_session.table.TableFunction.md#op-5f9bc702571844223c6d4a37) reference by name.

Returns an error if no table function has been registered with the provided name.

[`register_udtf`]: SessionContext::register_udtf

<a id="op-a65ca20b4c4beb85f89a5575"></a>
## table_provider

`function` · `datafusion::execution::context::SessionContext::table_provider` · datafusion 55.1.0

```rust
async fn table_provider(&self, table_ref: impl Into<TableReference>) -> Result<Arc<dyn TableProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2024`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a [`TableProvider`](../operations/datafusion_session.table.TableProvider.md#op-76e5c2e5b081ebf294e9493e) for the specified table.

<a id="op-a3343180187a7e33e5889f93"></a>
## task_ctx

`function` · `datafusion::execution::context::SessionContext::task_ctx` · datafusion 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2158, 1], "end": [2162, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_execution::task::TaskContextProvider", "path": "TaskContextProvider"}, "trait_path": "datafusion_execution::task::TaskContextProvider"}`

Source: `src/execution/context/mod.rs:2159`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8fad29d9271d24f6d2a02f4"></a>
## task_ctx

`function` · `datafusion::execution::context::SessionContext::task_ctx` · datafusion 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:2038`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get a new TaskContext to run in this session

<a id="op-c9854f3556a5c3be786f2019"></a>
## udaf

`function` · `datafusion::execution::context::SessionContext::udaf` · datafusion 55.1.0

```rust
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2097`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26f8fe93eafd3cd1bf003c6f"></a>
## udafs

`function` · `datafusion::execution::context::SessionContext::udafs` · datafusion 55.1.0

```rust
fn udafs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2149`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80e5728be3a0ed48a50cd933"></a>
## udf

`function` · `datafusion::execution::context::SessionContext::udf` · datafusion 55.1.0

```rust
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2089`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c321424d96dad902aca1e44"></a>
## udfs

`function` · `datafusion::execution::context::SessionContext::udfs` · datafusion 55.1.0

```rust
fn udfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2085`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-360a86674001be564bf4867f"></a>
## udwf

`function` · `datafusion::execution::context::SessionContext::udwf` · datafusion 55.1.0

```rust
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2101`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc6d29c387f3f2b3b3a01bfb"></a>
## udwfs

`function` · `datafusion::execution::context::SessionContext::udwfs` · datafusion 55.1.0

```rust
fn udwfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2084, 1], "end": [2156, 2], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/context/mod.rs:2153`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-048236e92e04cade4ce94339"></a>
## with_function_factory

`function` · `datafusion::execution::context::SessionContext::with_function_factory` · datafusion 55.1.0

```rust
fn with_function_factory(self, function_factory: Arc<dyn FunctionFactory>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [2082, 2], "filename": "src/execution/context/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/mod.rs:475`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`FunctionFactory`](../operations/datafusion.execution.context.FunctionFactory.md#op-4f2bfbb3e4aa340eb4071d97) to handle `CREATE FUNCTION` statements

<a id="op-ecb387d520fa8828076ef8ad"></a>
## write_csv

`function` · `datafusion::execution::context::SessionContext::write_csv` · datafusion 55.1.0

```rust
async fn write_csv(&self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [94, 2], "filename": "src/execution/context/csv.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/csv.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Executes a query and writes the results to a partitioned CSV file.

<a id="op-28b1fee09b19706d9802d0a1"></a>
## write_json

`function` · `datafusion::execution::context::SessionContext::write_json` · datafusion 55.1.0

```rust
async fn write_json(&self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [25, 1], "end": [72, 2], "filename": "src/execution/context/json.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/json.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Executes a query and writes the results to a partitioned JSON file.

<a id="op-009285a0287222c6d7be1426"></a>
## write_parquet

`function` · `datafusion::execution::context::SessionContext::write_parquet` · datafusion 55.1.0

```rust
async fn write_parquet(&self, plan: Arc<dyn ExecutionPlan>, path: impl AsRef<str>, writer_properties: Option<WriterProperties>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::SessionContext", "path": "super::SessionContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [97, 2], "filename": "src/execution/context/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/context/parquet.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Executes a query and writes the results to a partitioned Parquet file.
