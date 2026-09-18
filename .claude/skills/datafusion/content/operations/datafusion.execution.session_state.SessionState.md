# `datafusion::execution::session_state::SessionState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.session_state.SessionState.json).

<a id="op-3ba80ad5c25fa63e8340a601"></a>
## SessionState

`struct` · `datafusion::execution::session_state::SessionState` · datafusion 55.1.0

```rust
struct SessionState
```

Source: `src/execution/session_state.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

`SessionState` contains all the necessary state to plan and execute queries,
such as configuration, functions, and runtime environment. Please see the
documentation on [`SessionContext`] for more information.


# Example: `SessionState` from a [`SessionContext`]

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
let state = ctx.state();
```

# Example: `SessionState` via [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb)

You can also use [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb) to build a `SessionState` object
directly:

```
use datafusion::prelude::*;
# use datafusion::{error::Result, assert_batches_eq};
# use datafusion::execution::session_state::SessionStateBuilder;
# use datafusion_execution::runtime_env::RuntimeEnv;
# use std::sync::Arc;
# #[tokio::main]
# async fn main() -> Result<()> {
let state = SessionStateBuilder::new()
    .with_config(SessionConfig::new())
    .with_runtime_env(Arc::new(RuntimeEnv::default()))
    .with_default_features()
    .build();
Ok(())
# }
```

Note that there is no `Default` or `new()` for SessionState,
to avoid accidentally running queries or other operations without passing through
the [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08) or [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67). See [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb) and
[`SessionContext`].

[`SessionContext`]: crate::execution::context::SessionContext

<a id="op-0821feb505c28cdf5270cd72"></a>
## add_analyzer_rule

`function` · `datafusion::execution::session_state::SessionState::add_analyzer_rule` · datafusion 55.1.0

```rust
fn add_analyzer_rule(&mut self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> &Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Add `analyzer_rule` to the end of the list of
[`AnalyzerRule`](../operations/datafusion_optimizer.analyzer.AnalyzerRule.md#op-cff859d3fc5687654af4ac1d)s used to rewrite queries.

<a id="op-4124c4769553c5205807a198"></a>
## aggregate_functions

`function` · `datafusion::execution::session_state::SessionState::aggregate_functions` · datafusion 55.1.0

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fdfaa976ad7dd6e78a31ea2"></a>
## aggregate_functions

`function` · `datafusion::execution::session_state::SessionState::aggregate_functions` · datafusion 55.1.0

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:972`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return reference to aggregate_functions

<a id="op-1b5872210394b0739f04b728"></a>
## alias_generator

`function` · `datafusion::execution::session_state::SessionState::alias_generator` · datafusion 55.1.0

```rust
fn alias_generator(&self) -> &Arc<AliasGenerator>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2297, 1], "end": [2313, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/execution/session_state.rs:2302`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6634439361695600b4507651"></a>
## analyzer

`function` · `datafusion::execution::session_state::SessionState::analyzer` · datafusion 55.1.0

```rust
fn analyzer(&self) -> &Analyzer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:651`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the [`Analyzer`](../operations/datafusion_optimizer.analyzer.Analyzer.md#op-f305ce03fd0d8b4faab403e4) for this session

<a id="op-f2ff42eef441b1bc4a0dc29f"></a>
## as_any

`function` · `datafusion::execution::session_state::SessionState::as_any` · datafusion 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:337`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f874161a1f3520b9acaf7655"></a>
## cache_factory

`function` · `datafusion::execution::session_state::SessionState::cache_factory` · datafusion 55.1.0

```rust
fn cache_factory(&self) -> Option<&Arc<dyn CacheFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get the cache factory

<a id="op-1aa6b8f8dcb1f42b57a9f86f"></a>
## catalog_list

`function` · `datafusion::execution::session_state::SessionState::catalog_list` · datafusion 55.1.0

```rust
fn catalog_list(&self) -> Arc<dyn CatalogProviderList>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f257ea2d7b3ebfe9194d2ffd"></a>
## catalog_list

`function` · `datafusion::execution::session_state::SessionState::catalog_list` · datafusion 55.1.0

```rust
fn catalog_list(&self) -> &Arc<dyn CatalogProviderList>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:952`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return catalog list

<a id="op-e28325c5f6e1af02b869d915"></a>
## clone

`function` · `datafusion::execution::session_state::SessionState::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> SessionState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 10], "end": [140, 15], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution/session_state.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79851e3621bbb750715b2550"></a>
## config

`function` · `datafusion::execution::session_state::SessionState::config` · datafusion 55.1.0

```rust
fn config(&self) -> &SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:267`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac096c70d450c74120681990"></a>
## config

`function` · `datafusion::execution::session_state::SessionState::config` · datafusion 55.1.0

```rust
fn config(&self) -> &SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:853`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08)

<a id="op-654102fbe480599cb39a8b63"></a>
## config_mut

`function` · `datafusion::execution::session_state::SessionState::config_mut` · datafusion 55.1.0

```rust
fn config_mut(&mut self) -> &mut SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:858`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the mutable [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08).

<a id="op-2193d0783fab4d7132426286"></a>
## config_options

`function` · `datafusion::execution::session_state::SessionState::config_options` · datafusion 55.1.0

```rust
fn config_options(&self) -> &Arc<ConfigOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:873`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

return the configuration options

<a id="op-521bb3ff6f873a2edc57833a"></a>
## config_options

`function` · `datafusion::execution::session_state::SessionState::config_options` · datafusion 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [220, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerContext", "path": "PhysicalOptimizerContext"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerContext"}`

Source: `src/execution/session_state.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c00e0a757ac28ef4d52fb39"></a>
## create_logical_expr

`function` · `datafusion::execution::session_state::SessionState::create_logical_expr` · datafusion 55.1.0

```rust
fn create_logical_expr(&self, sql: &str, df_schema: &DFSchema) -> datafusion_common::Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:622`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a datafusion style AST [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) from a SQL string.

See example on  [SessionContext::parse_sql_expr](crate::execution::context::SessionContext::parse_sql_expr)

<a id="op-875e2d91869afa144b10ac94"></a>
## create_logical_expr_from_sql_expr

`function` · `datafusion::execution::session_state::SessionState::create_logical_expr_from_sql_expr` · datafusion 55.1.0

```rust
fn create_logical_expr_from_sql_expr(&self, sql_expr: SQLExprWithAlias, df_schema: &DFSchema) -> datafusion_common::Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:636`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a datafusion style AST [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) from a SQL expression.

<a id="op-f9b55c1ba43bed593f026265"></a>
## create_logical_plan

`function` · `datafusion::execution::session_state::SessionState::create_logical_plan` · datafusion 55.1.0

```rust
async fn create_logical_plan(&self, sql: &str) -> datafusion_common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:608`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) from the provided SQL string. This
interface will plan any SQL DataFusion supports, including DML
like `CREATE TABLE`, and `COPY` (which can write to local
files.

See [`SessionContext::sql`] and
[`SessionContext::sql_with_options`] for a higher-level
interface that handles DDL and verification of allowed
statements.

[`SessionContext::sql`]: crate::execution::context::SessionContext::sql
[`SessionContext::sql_with_options`]: crate::execution::context::SessionContext::sql_with_options

<a id="op-770adddfd78dfd1c0082c3ea"></a>
## create_physical_expr

`function` · `datafusion::execution::session_state::SessionState::create_physical_expr` · datafusion 55.1.0

```rust
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd782e590c997fe38ab9930b"></a>
## create_physical_expr

`function` · `datafusion::execution::session_state::SessionState::create_physical_expr` · datafusion 55.1.0

```rust
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:801`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) from an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) after applying type
coercion, and function rewrites.

Note: The expression is not [simplified] or otherwise optimized:
`a = 1 + 2` will not be simplified to `a = 3` as this is a more involved process.
See the [expr_api] example for how to simplify expressions.

# See Also:
* [`SessionContext::create_physical_expr`] for a higher-level API
* [`create_physical_expr`](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104) for a lower-level API

[simplified]: datafusion_optimizer::simplify_expressions
[expr_api]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/query_planning/expr_api.rs
[`SessionContext::create_physical_expr`]: crate::execution::context::SessionContext::create_physical_expr

<a id="op-155ba659cafcb614fa000a55"></a>
## create_physical_plan

`function` · `datafusion::execution::session_state::SessionState::create_physical_plan` · datafusion 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:777`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a physical [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) plan from a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

Note: this first calls [`Self::optimize`](../operations/datafusion.execution.session_state.SessionState.md#op-c0e81141eadec0b9af4f9b08) on the provided
plan.

This function will error for [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s such as catalog DDL like
`CREATE TABLE`, which do not have corresponding physical plans and must
be handled by another layer, typically [`SessionContext`].

[`SessionContext`]: crate::execution::context::SessionContext

<a id="op-45bc046bb27ac2c99f2026df"></a>
## create_physical_plan

`function` · `datafusion::execution::session_state::SessionState::create_physical_plan` · datafusion 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f29f95bf004332c553b513f0"></a>
## default_table_options

`function` · `datafusion::execution::session_state::SessionState::default_table_options` · datafusion 55.1.0

```rust
fn default_table_options(&self) -> TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:897`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

return the TableOptions options with its extensions

<a id="op-6a79ccb491482f03703ad750"></a>
## deregister_higher_order_function

`function` · `datafusion::execution::session_state::SessionState::deregister_higher_order_function` · datafusion 55.1.0

```rust
fn deregister_higher_order_function(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<HigherOrderUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2219`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23e63494828742133fd2210f"></a>
## deregister_udaf

`function` · `datafusion::execution::session_state::SessionState::deregister_udaf` · datafusion 55.1.0

```rust
fn deregister_udaf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<AggregateUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2232`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a538321fcde1b856762ff1f7"></a>
## deregister_udf

`function` · `datafusion::execution::session_state::SessionState::deregister_udf` · datafusion 55.1.0

```rust
fn deregister_udf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<ScalarUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2206`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bb7d0d78e7f24ddbb19a768"></a>
## deregister_udtf

`function` · `datafusion::execution::session_state::SessionState::deregister_udtf` · datafusion 55.1.0

```rust
fn deregister_udtf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableFunctionImpl>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:1005`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Deregister a user defined table function

<a id="op-046802672a549a1e6124e630"></a>
## deregister_udwf

`function` · `datafusion::execution::session_state::SessionState::deregister_udwf` · datafusion 55.1.0

```rust
fn deregister_udwf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<WindowUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2245`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f694b0c4cee27fa37673ec4"></a>
## execution_props

`function` · `datafusion::execution::session_state::SessionState::execution_props` · datafusion 55.1.0

```rust
fn execution_props(&self) -> &ExecutionProps
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:843`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the execution properties

<a id="op-92ad92c36e5abd31fc803d1f"></a>
## execution_props

`function` · `datafusion::execution::session_state::SessionState::execution_props` · datafusion 55.1.0

```rust
fn execution_props(&self) -> &ExecutionProps
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03a4340cfe39d1e6a8ce39a3"></a>
## execution_props_mut

`function` · `datafusion::execution::session_state::SessionState::execution_props_mut` · datafusion 55.1.0

```rust
fn execution_props_mut(&mut self) -> &mut ExecutionProps
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:848`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return mutable execution properties

<a id="op-9b0b3eb2cd591f00cd01593a"></a>
## expr_planners

`function` · `datafusion::execution::session_state::SessionState::expr_planners` · datafusion 55.1.0

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2266`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e760e19333bb1d8dff4783ea"></a>
## expr_planners

`function` · `datafusion::execution::session_state::SessionState::expr_planners` · datafusion 55.1.0

```rust
fn expr_planners(&self) -> &[Arc<dyn ExprPlanner>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the [`ExprPlanner`](../operations/datafusion_expr.planner.ExprPlanner.md#op-c0ce2d948f3fc34627ff15dd)s for this session

<a id="op-9dfce521e2ba04a8de548b08"></a>
## extension_type_registry

`function` · `datafusion::execution::session_state::SessionState::extension_type_registry` · datafusion 55.1.0

```rust
fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b51cb3f52c00c7254ddc9a04"></a>
## fmt

`function` · `datafusion::execution::session_state::SessionState::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 1], "end": [259, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/session_state.rs:225`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Prefer having short fields at the top and long vector fields near the end
Group fields by

<a id="op-789cd4291e2cfb45804d05b2"></a>
## function_factory

`function` · `datafusion::execution::session_state::SessionState::function_factory` · datafusion 55.1.0

```rust
fn function_factory(&self) -> Option<&Arc<dyn FunctionFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get the function factory

<a id="op-b7210effaeda2c9d2606666e"></a>
## function_registry

`function` · `datafusion::execution::session_state::SessionState::function_registry` · datafusion 55.1.0

```rust
fn function_registry(&self) -> Option<&dyn FunctionRegistry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2297, 1], "end": [2313, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/execution/session_state.rs:2310`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e0cf74c0e131e92e3d971c7"></a>
## get_file_format_factory

`function` · `datafusion::execution::session_state::SessionState::get_file_format_factory` · datafusion 55.1.0

```rust
fn get_file_format_factory(&self, ext: &str) -> Option<Arc<dyn FileFormatFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:939`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieves a [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973) based on file extension which has been registered
via SessionContext::register_file_format. Extensions are not case sensitive.

<a id="op-a8f011e6cc927bd5a1263846"></a>
## higher_order_function

`function` · `datafusion::execution::session_state::SessionState::higher_order_function` · datafusion 55.1.0

```rust
fn higher_order_function(&self, name: &str) -> datafusion_common::Result<Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2134`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82e534f97ad7e435659ca0e"></a>
## higher_order_function_names

`function` · `datafusion::execution::session_state::SessionState::higher_order_function_names` · datafusion 55.1.0

```rust
fn higher_order_function_names(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2278`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3272f0901e87ac041f63f924"></a>
## higher_order_functions

`function` · `datafusion::execution::session_state::SessionState::higher_order_functions` · datafusion 55.1.0

```rust
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:967`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return reference to higher_order_functions

<a id="op-a0bc9a160ecdc9b6ac75a505"></a>
## higher_order_functions

`function` · `datafusion::execution::session_state::SessionState::higher_order_functions` · datafusion 55.1.0

```rust
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-195965fb1dd933f0c331312d"></a>
## mark_start_execution

`function` · `datafusion::execution::session_state::SessionState::mark_start_execution` · datafusion 55.1.0

```rust
fn mark_start_execution(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:886`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Mark the start of the execution

<a id="op-624a0c36797f5248fa2fdeb1"></a>
## optimize

`function` · `datafusion::execution::session_state::SessionState::optimize` · datafusion 55.1.0

```rust
fn optimize(&self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0e81141eadec0b9af4f9b08"></a>
## optimize

`function` · `datafusion::execution::session_state::SessionState::optimize` · datafusion 55.1.0

```rust
fn optimize(&self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Optimizes the logical plan by applying optimizer rules.

<a id="op-77dede5ab7e98011921c7dcd"></a>
## optimizer

`function` · `datafusion::execution::session_state::SessionState::optimizer` · datafusion 55.1.0

```rust
fn optimizer(&self) -> &Optimizer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:656`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the [`Optimizer`](../operations/datafusion_optimizer.optimizer.Optimizer.md#op-111f177db94ba9d05d0d598a) for this session

<a id="op-54feefb6d76ebb72fec2e2b1"></a>
## optimizers

`function` · `datafusion::execution::session_state::SessionState::optimizers` · datafusion 55.1.0

```rust
fn optimizers(&self) -> &[Arc<dyn OptimizerRule + Send + Sync>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:863`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the logical optimizers

<a id="op-1912de8e0445df05f6c83c4b"></a>
## options

`function` · `datafusion::execution::session_state::SessionState::options` · datafusion 55.1.0

```rust
fn options(&self) -> Arc<ConfigOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2297, 1], "end": [2313, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/execution/session_state.rs:2306`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e254b08cca358d582d0e6a4"></a>
## physical_optimizers

`function` · `datafusion::execution::session_state::SessionState::physical_optimizers` · datafusion 55.1.0

```rust
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:868`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the physical optimizers

<a id="op-8a46128ad7931914e35a52b9"></a>
## physical_optimizers

`function` · `datafusion::execution::session_state::SessionState::physical_optimizers` · datafusion 55.1.0

```rust
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f56d2ab4fb4459e58c8e0c4"></a>
## query_execution_start_time

`function` · `datafusion::execution::session_state::SessionState::query_execution_start_time` · datafusion 55.1.0

```rust
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2297, 1], "end": [2313, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/execution/session_state.rs:2298`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58f493e777416e80d5a493bf"></a>
## query_planner

`function` · `datafusion::execution::session_state::SessionState::query_planner` · datafusion 55.1.0

```rust
fn query_planner(&self) -> &Arc<dyn QueryPlanner + Send + Sync>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:684`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the [`QueryPlanner`](../operations/datafusion_session.planner.QueryPlanner.md#op-d105e63a68841dd69dcdcc42) for this session

<a id="op-abf2ebca7c5ddc3c6497f2de"></a>
## query_planner

`function` · `datafusion::execution::session_state::SessionState::query_planner` · datafusion 55.1.0

```rust
fn query_planner(&self) -> Arc<dyn QueryPlanner + Send + Sync>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2da57b2222621c45fddd0bc1"></a>
## register_catalog_list

`function` · `datafusion::execution::session_state::SessionState::register_catalog_list` · datafusion 55.1.0

```rust
fn register_catalog_list(&mut self, catalog_list: Arc<dyn CatalogProviderList>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:957`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Set the catalog list

<a id="op-8f0813fb9d6b162bbdbc6973"></a>
## register_expr_planner

`function` · `datafusion::execution::session_state::SessionState::register_expr_planner` · datafusion 55.1.0

```rust
fn register_expr_planner(&mut self, expr_planner: Arc<dyn ExprPlanner>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2270`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f1a0845c7a6a6b17b02a481"></a>
## register_file_format

`function` · `datafusion::execution::session_state::SessionState::register_file_format` · datafusion 55.1.0

```rust
fn register_file_format(&mut self, file_format: Arc<dyn FileFormatFactory>, overwrite: bool) -> Result<(), DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:915`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Adds or updates a [FileFormatFactory](../operations/datafusion_datasource.file_format.FileFormatFactory.md#op-f967009d6b2c0c52e0070973) which can be used with COPY TO or
CREATE EXTERNAL TABLE statements for reading and writing files of custom
formats.

<a id="op-691bbc7021de46561ec423c0"></a>
## register_function_rewrite

`function` · `datafusion::execution::session_state::SessionState::register_function_rewrite` · datafusion 55.1.0

```rust
fn register_function_rewrite(&mut self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2258`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5b5a10b2524d438dcf83b52"></a>
## register_higher_order_function

`function` · `datafusion::execution::session_state::SessionState::register_higher_order_function` · datafusion 55.1.0

```rust
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> datafusion_common::Result<Option<Arc<HigherOrderUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2171`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb88202e9712daa1d6360e72"></a>
## register_relation_planner

`function` · `datafusion::execution::session_state::SessionState::register_relation_planner` · datafusion 55.1.0

```rust
fn register_relation_planner(&mut self, planner: Arc<dyn RelationPlanner>) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`RelationPlanner`](../operations/datafusion_expr.planner.RelationPlanner.md#op-1be858ef3f752319489e5fc4) to customize SQL relation planning.

Newly registered planners are given higher priority than existing ones.

<a id="op-46b4bd0b65a4550371f7de60"></a>
## register_table_options_extension

`function` · `datafusion::execution::session_state::SessionState::register_table_options_extension` · datafusion 55.1.0

```rust
fn register_table_options_extension<T: ConfigExtension>(&mut self, extension: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:908`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`ConfigExtension`](../operations/datafusion_common.config.ConfigExtension.md#op-ef20648fc6d3eb8f6cc2530c) as a table option extension that can be
referenced from SQL statements executed against this context.

<a id="op-456c30d3c1635f7361431979"></a>
## register_udaf

`function` · `datafusion::execution::session_state::SessionState::register_udaf` · datafusion 55.1.0

```rust
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> datafusion_common::Result<Option<Arc<AggregateUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2184`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-503fbe5353a103e6200eaefe"></a>
## register_udf

`function` · `datafusion::execution::session_state::SessionState::register_udf` · datafusion 55.1.0

```rust
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> datafusion_common::Result<Option<Arc<ScalarUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2160`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64d716860e5df94831a68b80"></a>
## register_udtf

`function` · `datafusion::execution::session_state::SessionState::register_udtf` · datafusion 55.1.0

```rust
fn register_udtf(&mut self, name: &str, fun: Arc<dyn TableFunctionImpl>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:997`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Register a user defined table function

<a id="op-f64cd70e846e00c3571041b3"></a>
## register_udwf

`function` · `datafusion::execution::session_state::SessionState::register_udwf` · datafusion 55.1.0

```rust
fn register_udwf(&mut self, udwf: Arc<WindowUDF>) -> datafusion_common::Result<Option<Arc<WindowUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2195`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92553e05ede13fea89aba963"></a>
## relation_planners

`function` · `datafusion::execution::session_state::SessionState::relation_planners` · datafusion 55.1.0

```rust
fn relation_planners(&self) -> &[Arc<dyn RelationPlanner>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:667`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the registered relation planners in priority order.

<a id="op-fe21a1f78f0a6dc8805a9ed8"></a>
## resolve_table_references

`function` · `datafusion::execution::session_state::SessionState::resolve_table_references` · datafusion 55.1.0

```rust
fn resolve_table_references(&self, statement: &Statement) -> datafusion_common::Result<Vec<TableReference>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:534`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Resolve all table references in the SQL statement. Does not include CTE references.

See [`datafusion_sql::resolve::resolve_table_references`] for more information.

[`datafusion_sql::resolve::resolve_table_references`]: datafusion_sql::resolve::resolve_table_references

<a id="op-64b9e56d6cba02dc6e1311c6"></a>
## runtime_env

`function` · `datafusion::execution::session_state::SessionState::runtime_env` · datafusion 55.1.0

```rust
fn runtime_env(&self) -> &Arc<RuntimeEnv>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af11a86698b87ea3db6b9197"></a>
## runtime_env

`function` · `datafusion::execution::session_state::SessionState::runtime_env` · datafusion 55.1.0

```rust
fn runtime_env(&self) -> &Arc<RuntimeEnv>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:838`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the runtime env

<a id="op-25a7ddf8e699878347177717"></a>
## scalar_functions

`function` · `datafusion::execution::session_state::SessionState::scalar_functions` · datafusion 55.1.0

```rust
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7217e9deef5ad1504d02344c"></a>
## scalar_functions

`function` · `datafusion::execution::session_state::SessionState::scalar_functions` · datafusion 55.1.0

```rust
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:962`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return reference to scalar_functions

<a id="op-9fae2f11b936a61001f5d583"></a>
## schema_for_ref

`function` · `datafusion::execution::session_state::SessionState::schema_for_ref` · datafusion 55.1.0

```rust
fn schema_for_ref(&self, table_ref: impl Into<TableReference>) -> datafusion_common::Result<Arc<dyn SchemaProvider>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Retrieve the [`SchemaProvider`](../operations/datafusion_session.schema.SchemaProvider.md#op-009a61a5d6d9122859b6d788) for a specific [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0), if it
exists.

<a id="op-95acdbbbbcb89568ff481e97"></a>
## serializer_registry

`function` · `datafusion::execution::session_state::SessionState::serializer_registry` · datafusion 55.1.0

```rust
fn serializer_registry(&self) -> &Arc<dyn SerializerRegistry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:987`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return [SerializerRegistry](../operations/datafusion_expr.registry.SerializerRegistry.md#op-815a1f89b969d5eb430d928d) for extensions

<a id="op-b1ad3b9fd846f2aaae063d75"></a>
## session_id

`function` · `datafusion::execution::session_state::SessionState::session_id` · datafusion 55.1.0

```rust
fn session_id(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee8f93861d30572e8673df38"></a>
## session_id

`function` · `datafusion::execution::session_state::SessionState::session_id` · datafusion 55.1.0

```rust
fn session_id(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:833`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the session ID

<a id="op-61fccdac75a2a40f30ffccc0"></a>
## set_cache_factory

`function` · `datafusion::execution::session_state::SessionState::set_cache_factory` · datafusion 55.1.0

```rust
fn set_cache_factory(&mut self, cache_factory: Arc<dyn CacheFactory>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Register a [`CacheFactory`](../operations/datafusion.execution.session_state.CacheFactory.md#op-05346466af099911c46feb7a) for custom caching strategy

<a id="op-1bc98875297aac422e9c146e"></a>
## set_function_factory

`function` · `datafusion::execution::session_state::SessionState::set_function_factory` · datafusion 55.1.0

```rust
fn set_function_factory(&mut self, function_factory: Arc<dyn FunctionFactory>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Registers a [`FunctionFactory`](../operations/datafusion.execution.context.FunctionFactory.md#op-4f2bfbb3e4aa340eb4071d97) to handle `CREATE FUNCTION` statements

<a id="op-963183c3ce0c3b4ebc52fbe3"></a>
## sql_to_expr

`function` · `datafusion::execution::session_state::SessionState::sql_to_expr` · datafusion 55.1.0

```rust
fn sql_to_expr(&self, sql: &str, dialect: &Dialect) -> datafusion_common::Result<SQLExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

parse a sql string into a sqlparser-rs AST [`SQLExpr`](../operations/sqlparser.ast.Expr.md#op-60bb4348f2010610c575bb63).

See [`Self::create_logical_expr`](../operations/datafusion.execution.session_state.SessionState.md#op-6c00e0a757ac28ef4d52fb39) for parsing sql to [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc).

<a id="op-2d5c171cfaab2f90d72335c3"></a>
## sql_to_expr_with_alias

`function` · `datafusion::execution::session_state::SessionState::sql_to_expr_with_alias` · datafusion 55.1.0

```rust
fn sql_to_expr_with_alias(&self, sql: &str, dialect: &Dialect) -> datafusion_common::Result<SQLExprWithAlias>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

parse a sql string into a sqlparser-rs AST [`SQLExprWithAlias`](../operations/sqlparser.ast.query.ExprWithAlias.md#op-6ce26adcdcea6e1157831034).

See [`Self::create_logical_expr`](../operations/datafusion.execution.session_state.SessionState.md#op-6c00e0a757ac28ef4d52fb39) for parsing sql to [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc).

<a id="op-492f7bab9abde07d2ea3f578"></a>
## sql_to_statement

`function` · `datafusion::execution::session_state::SessionState::sql_to_statement` · datafusion 55.1.0

```rust
fn sql_to_statement(&self, sql: &str, dialect: &Dialect) -> datafusion_common::Result<Statement>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:458`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Parse an SQL string into an DataFusion specific AST
[`Statement`](../operations/datafusion_sql.parser.Statement.md#op-c2acea2bc56287d845c8d8f4). See [`SessionContext::sql`] for running queries.

[`SessionContext::sql`]: crate::execution::context::SessionContext::sql

<a id="op-a32eb42d3e085a929ba04cfa"></a>
## statement_to_plan

`function` · `datafusion::execution::session_state::SessionState::statement_to_plan` · datafusion 55.1.0

```rust
async fn statement_to_plan(&self, statement: Statement) -> datafusion_common::Result<LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:549`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Convert an AST Statement into a LogicalPlan

<a id="op-16e0d3194f41f4a3150796b0"></a>
## statistics_registry

`function` · `datafusion::execution::session_state::SessionState::statistics_registry` · datafusion 55.1.0

```rust
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28deef0142a5f08ea79aa35c"></a>
## statistics_registry

`function` · `datafusion::execution::session_state::SessionState::statistics_registry` · datafusion 55.1.0

```rust
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [212, 1], "end": [220, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerContext", "path": "PhysicalOptimizerContext"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerContext"}`

Source: `src/execution/session_state.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7b762219b19c447cb55ee65"></a>
## statistics_registry

`function` · `datafusion::execution::session_state::SessionState::statistics_registry` · datafusion 55.1.0

```rust
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:881`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns the statistics registry if one is configured.

The registry provides pluggable statistics providers for enhanced
cardinality estimation (e.g., NDV overrides, histograms).

<a id="op-2c7a51cbbe2060f74c40d571"></a>
## table_factories

`function` · `datafusion::execution::session_state::SessionState::table_factories` · datafusion 55.1.0

```rust
fn table_factories(&self) -> &HashMap<String, Arc<dyn TableProviderFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get the table factories

<a id="op-401bb60fcbc0785283bc5743"></a>
## table_factories_mut

`function` · `datafusion::execution::session_state::SessionState::table_factories_mut` · datafusion 55.1.0

```rust
fn table_factories_mut(&mut self) -> &mut HashMap<String, Arc<dyn TableProviderFactory>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get the table factories

<a id="op-fd1ec9d036c9069c93dd438f"></a>
## table_functions

`function` · `datafusion::execution::session_state::SessionState::table_functions` · datafusion 55.1.0

```rust
fn table_functions(&self) -> &HashMap<String, Arc<TableFunction>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:982`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return reference to table_functions

<a id="op-8553d808f7de8861c13427bc"></a>
## table_options

`function` · `datafusion::execution::session_state::SessionState::table_options` · datafusion 55.1.0

```rust
fn table_options(&self) -> &TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:892`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return the table options

<a id="op-d79462242807ef4dde0071e3"></a>
## table_options

`function` · `datafusion::execution::session_state::SessionState::table_options` · datafusion 55.1.0

```rust
fn table_options(&self) -> &TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-468720fcab1cc8e32fe27e91"></a>
## table_options_mut

`function` · `datafusion::execution::session_state::SessionState::table_options_mut` · datafusion 55.1.0

```rust
fn table_options_mut(&mut self) -> &mut TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:902`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a mutable reference to [`TableOptions`](../operations/datafusion_common.config.TableOptions.md#op-0523542c8dd1656aea13ac4e)

<a id="op-95234699ba908f77d492fecc"></a>
## table_options_mut

`function` · `datafusion::execution::session_state::SessionState::table_options_mut` · datafusion 55.1.0

```rust
fn table_options_mut(&mut self) -> &mut TableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28283e3d99eb53893c288540"></a>
## task_ctx

`function` · `datafusion::execution::session_state::SessionState::task_ctx` · datafusion 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-431b4ec051398e6da00b0d23"></a>
## task_ctx

`function` · `datafusion::execution::session_state::SessionState::task_ctx` · datafusion 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:947`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Get a new TaskContext to run in this session

<a id="op-8ef0139868dc247f720b6ba3"></a>
## task_ctx

`function` · `datafusion::execution::session_state::SessionState::task_ctx` · datafusion 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2291, 1], "end": [2295, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_execution::task::TaskContextProvider", "path": "TaskContextProvider"}, "trait_path": "datafusion_execution::task::TaskContextProvider"}`

Source: `src/execution/session_state.rs:2292`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1e1c79abf39469b3f3b2f9"></a>
## udaf

`function` · `datafusion::execution::session_state::SessionState::udaf` · datafusion 55.1.0

```rust
fn udaf(&self, name: &str) -> datafusion_common::Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2144`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5195d1ef6c6a468d6f79660b"></a>
## udafs

`function` · `datafusion::execution::session_state::SessionState::udafs` · datafusion 55.1.0

```rust
fn udafs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2282`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7abc601b828d85909612aa76"></a>
## udf

`function` · `datafusion::execution::session_state::SessionState::udf` · datafusion 55.1.0

```rust
fn udf(&self, name: &str) -> datafusion_common::Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2126`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02969d2e88e643c597d24add"></a>
## udfs

`function` · `datafusion::execution::session_state::SessionState::udfs` · datafusion 55.1.0

```rust
fn udfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2122`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c028fbc73f549736562d8080"></a>
## udwf

`function` · `datafusion::execution::session_state::SessionState::udwf` · datafusion 55.1.0

```rust
fn udwf(&self, name: &str) -> datafusion_common::Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2152`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9139c34280b8ba39a18b1f0f"></a>
## udwfs

`function` · `datafusion::execution::session_state::SessionState::udwfs` · datafusion 55.1.0

```rust
fn udwfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2121, 1], "end": [2289, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/execution/session_state.rs:2286`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c166e5ac8c6077bdcb9d4d46"></a>
## version

`function` · `datafusion::execution::session_state::SessionState::version` · datafusion 55.1.0

```rust
fn version(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:992`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return version of the cargo package that produced this query

<a id="op-6a9570c68f50f57459d0efea"></a>
## window_functions

`function` · `datafusion::execution::session_state::SessionState::window_functions` · datafusion 55.1.0

```rust
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [1046, 2], "filename": "src/execution/session_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution/session_state.rs:977`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return reference to window functions

<a id="op-bb943909cf0840cc57392a9a"></a>
## window_functions

`function` · `datafusion::execution::session_state::SessionState::window_functions` · datafusion 55.1.0

```rust
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::session_state::SessionState", "path": "SessionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [262, 1], "end": [352, 2], "filename": "src/execution/session_state.rs"}, "trait": {"args": null, "id": "datafusion_session::session::Session", "path": "Session"}, "trait_path": "datafusion_session::session::Session"}`

Source: `src/execution/session_state.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
