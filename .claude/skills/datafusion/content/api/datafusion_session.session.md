# `datafusion_session::session`

Crate `datafusion-session` · 2 public items · structured records in [`model/datafusion_session.session.json`](../model/datafusion_session.session.json)

## SessionStore

`struct` · `datafusion_session::session::SessionStore`

Also reachable as `datafusion_session::SessionStore`

```rust
struct SessionStore
```

**Derives**: Debug, Default

**Methods** (3)

```rust
fn get_session(&self) -> Weak<RwLock<dyn Session>>
fn new() -> Self
fn with_state(&self, state: Weak<RwLock<dyn Session>>)
```

The state store that stores the reference of the runtime session state.

---

## Session

`trait` · `datafusion_session::session::Session`

Also reachable as `datafusion::catalog::Session`, `datafusion_catalog::Session`, `datafusion_session::Session`

```rust
trait Session: Send + Sync
```

**Implementors** (2)

- `datafusion::execution::session_state::SessionState`
- `datafusion_ffi::session::ForeignSession`

**Methods** (22)

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
fn as_any(&self) -> &dyn Any
fn catalog_list(&self) -> Arc<dyn CatalogProviderList>
fn config(&self) -> &SessionConfig
fn config_options(&self) -> &ConfigOptions
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> Result<Arc<dyn PhysicalExpr>>
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> Result<Arc<dyn ExecutionPlan>>
fn default_table_options(&self) -> TableOptions
fn execution_props(&self) -> &ExecutionProps
fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
fn optimize(&self, plan: &LogicalPlan) -> Result<LogicalPlan>
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
fn query_planner(&self) -> Arc<dyn QueryPlanner + Send + Sync>
fn runtime_env(&self) -> &Arc<RuntimeEnv>
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
fn session_id(&self) -> &str
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
fn table_options(&self) -> &TableOptions
fn table_options_mut(&mut self) -> &mut TableOptions
fn task_ctx(&self) -> Arc<TaskContext>
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

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

---
