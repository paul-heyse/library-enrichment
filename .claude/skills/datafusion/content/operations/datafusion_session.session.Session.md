# `datafusion_session::session::Session`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.session.Session.json).

<a id="op-75302dfa669885e17a5c9093"></a>
## Session

`trait` · `datafusion_session::session::Session` · datafusion-session 55.1.0

```rust
trait Session: Send + Sync
```

Source: `src/session.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Interface for accessing [`SessionState`] from the catalog and data source.

This trait provides access to the information needed to plan and execute
queries, such as configuration, functions, and runtime environment. See the
documentation on [`SessionState`] for more information.

Historically, the `SessionState` struct was passed directly to catalog
traits such as [`TableProvider`], which required a direct dependency on the
DataFusion core. The interface required is now defined by this trait. See
[#10782] for more details.

[#10782]: https://github.com/apache/datafusion/issues/10782

# Migration from `SessionState`

Using trait methods is preferred, as the implementation may change in future
versions. However, you can downcast a `Session` to a `SessionState` as shown
in the example below. If you find yourself needing to do this, please open
an issue on the DataFusion repository so we can extend the trait to provide
the required information.

```
# use datafusion_session::Session;
# use datafusion_common::{Result, exec_datafusion_err};
# struct SessionState {}
// Given a `Session` reference, get the concrete `SessionState` reference
// Note: this may stop working in future versions,
fn session_state_from_session(session: &dyn Session) -> Result<&SessionState> {
    session
        .as_any()
        .downcast_ref::<SessionState>()
        .ok_or_else(|| {
            exec_datafusion_err!("Failed to downcast Session to SessionState")
        })
}
```

[`SessionState`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html
[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

<a id="op-6864c3d5a9217ea4b216825e"></a>
## aggregate_functions

`function` · `datafusion_session::session::Session::aggregate_functions` · datafusion-session 55.1.0

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
```

Source: `src/session.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return reference to aggregate_functions

<a id="op-28dcd000dfe9b9cc49ed1c0e"></a>
## as_any

`function` · `datafusion_session::session::Session::as_any` · datafusion-session 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/session.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-421be28c58b70ba26def1a1f"></a>
## catalog_list

`function` · `datafusion_session::session::Session::catalog_list` · datafusion-session 55.1.0

```rust
fn catalog_list(&self) -> Arc<dyn CatalogProviderList>
```

Source: `src/session.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the catalogs registered with this session.

<a id="op-988730b2db63ed6a1d5f8829"></a>
## config

`function` · `datafusion_session::session::Session::config` · datafusion-session 55.1.0

```rust
fn config(&self) -> &SessionConfig
```

Source: `src/session.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08)

<a id="op-8529c4d4d8ba21f5d58f7072"></a>
## config_options

`function` · `datafusion_session::session::Session::config_options` · datafusion-session 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Source: `src/session.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

return the [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4)

<a id="op-ce46cad8a2c7c7d1ae10f45b"></a>
## create_physical_expr

`function` · `datafusion_session::session::Session::create_physical_expr` · datafusion-session 55.1.0

```rust
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/session.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Create a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) from an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) after applying type
coercion, and function rewrites.

Note: The expression is not simplified or otherwise optimized:  `a = 1
+ 2` will not be simplified to `a = 3` as this is a more involved process.
See the [expr_api] example for how to simplify expressions.

[expr_api]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/query_planning/expr_api.rs

<a id="op-f8a00c1a238fc9102cc1e8b7"></a>
## create_physical_plan

`function` · `datafusion_session::session::Session::create_physical_plan` · datafusion-session 55.1.0

```rust
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> Result<Arc<dyn ExecutionPlan>>
```

Source: `src/session.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Creates a physical [`ExecutionPlan`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-ac09436cd73f869917923673) plan from a [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

Note: this will optimize the provided plan first.

This function will error for [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s such as catalog DDL like
`CREATE TABLE`, which do not have corresponding physical plans and must
be handled by another layer, typically the `SessionContext`.

<a id="op-94617c2ccffe946a034523ac"></a>
## default_table_options

`function` · `datafusion_session::session::Session::default_table_options` · datafusion-session 55.1.0

```rust
fn default_table_options(&self) -> TableOptions
```

Source: `src/session.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

return the TableOptions options with its extensions

<a id="op-c5a7666452593efcd2f3271e"></a>
## execution_props

`function` · `datafusion_session::session::Session::execution_props` · datafusion-session 55.1.0

```rust
fn execution_props(&self) -> &ExecutionProps
```

Source: `src/session.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the execution properties

<a id="op-2a0df6cdf9b2c2f9d4694161"></a>
## extension_type_registry

`function` · `datafusion_session::session::Session::extension_type_registry` · datafusion-session 55.1.0

```rust
fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef
```

Source: `src/session.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return a reference to the extension type registry

<a id="op-91837a408b507749becfd15c"></a>
## higher_order_functions

`function` · `datafusion_session::session::Session::higher_order_functions` · datafusion-session 55.1.0

```rust
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
```

Source: `src/session.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return reference to higher_order_functions

<a id="op-f44d26653aa48c6d8006b502"></a>
## optimize

`function` · `datafusion_session::session::Session::optimize` · datafusion-session 55.1.0

```rust
fn optimize(&self, plan: &LogicalPlan) -> Result<LogicalPlan>
```

Source: `src/session.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Optimize a logical plan.

# Warning

The default implementation returns the plan **unchanged**, applying no
logical optimizations whatsoever. This is almost never what you want:
without optimization, queries execute in their naive, unoptimized form
and may be dramatically slower or fail to run at all. The default exists
only so this crate need not depend on the optimizer; any real session
should override this method (for example by delegating to
`SessionState::optimize`).

<a id="op-1b8ad5ef6b69d42d96c7adbe"></a>
## physical_optimizers

`function` · `datafusion_session::session::Session::physical_optimizers` · datafusion-session 55.1.0

```rust
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
```

Source: `src/session.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the physical optimizer rules for this session.

# Warning

The default implementation returns **no rules**. This is almost never
what you want: DataFusion relies on physical optimizer rules for
correctness-critical rewrites (such as inserting the repartitioning and
coalescing needed for parallel and multi-partition execution), so a
session with no rules will produce plans that are inefficient or that
fail to execute. The default exists only so this crate need not depend
on the optimizer; any real session should override this method (for
example by returning `SessionState::physical_optimizers`).

<a id="op-d9a5770e1d1190279fe94203"></a>
## query_planner

`function` · `datafusion_session::session::Session::query_planner` · datafusion-session 55.1.0

```rust
fn query_planner(&self) -> Arc<dyn QueryPlanner + Send + Sync>
```

Source: `src/session.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the query planner for this session.

# Warning

The default implementation returns an [`UnsupportedQueryPlanner`](../operations/datafusion_session.planner.UnsupportedQueryPlanner.md#op-3bc87131bbd1f7aa565e0e75), so
[`Session::create_physical_plan`](../operations/datafusion_session.session.Session.md#op-f8a00c1a238fc9102cc1e8b7) will fail. Sessions that support
physical planning should override this method (for example by returning
`SessionState::query_planner`).

<a id="op-c7fc0b9aeb7dd61b5e812286"></a>
## runtime_env

`function` · `datafusion_session::session::Session::runtime_env` · datafusion-session 55.1.0

```rust
fn runtime_env(&self) -> &Arc<RuntimeEnv>
```

Source: `src/session.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the runtime env

<a id="op-b4a552abb131ef3ab6667d27"></a>
## scalar_functions

`function` · `datafusion_session::session::Session::scalar_functions` · datafusion-session 55.1.0

```rust
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
```

Source: `src/session.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return reference to scalar_functions

<a id="op-db8333384155bf37ad2b0a12"></a>
## session_id

`function` · `datafusion_session::session::Session::session_id` · datafusion-session 55.1.0

```rust
fn session_id(&self) -> &str
```

Source: `src/session.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the session ID

<a id="op-0c5021fe52f5fee8f150e4d3"></a>
## statistics_registry

`function` · `datafusion_session::session::Session::statistics_registry` · datafusion-session 55.1.0

```rust
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

Source: `src/session.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the optional statistics registry used during physical optimization.

<a id="op-36b543d889732e51f3f7bcf4"></a>
## table_options

`function` · `datafusion_session::session::Session::table_options` · datafusion-session 55.1.0

```rust
fn table_options(&self) -> &TableOptions
```

Source: `src/session.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return the table options

<a id="op-3421caa9fd6f62f343aa5a55"></a>
## table_options_mut

`function` · `datafusion_session::session::Session::table_options_mut` · datafusion-session 55.1.0

```rust
fn table_options_mut(&mut self) -> &mut TableOptions
```

Source: `src/session.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Returns a mutable reference to [`TableOptions`](../operations/datafusion_common.config.TableOptions.md#op-0523542c8dd1656aea13ac4e)

<a id="op-86beddb384c16d2959a50795"></a>
## task_ctx

`function` · `datafusion_session::session::Session::task_ctx` · datafusion-session 55.1.0

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

Source: `src/session.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get a new TaskContext to run in this session

<a id="op-eea199d7dc08ee051316a1e1"></a>
## window_functions

`function` · `datafusion_session::session::Session::window_functions` · datafusion-session 55.1.0

```rust
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

Source: `src/session.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Return reference to window functions
