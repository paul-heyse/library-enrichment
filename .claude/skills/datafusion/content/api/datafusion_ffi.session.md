# `datafusion_ffi::session`

Crate `datafusion-ffi` · 1 public items · structured records in [`model/datafusion_ffi.session.json`](../model/datafusion_ffi.session.json)

## ForeignSession

`struct` · `datafusion_ffi::session::ForeignSession`

```rust
struct ForeignSession
```

**Implements**: `datafusion_session::session::Session`

**Derives**: Debug, Send, Sync

**via `datafusion_session::session::Session`**

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
fn as_any(&self) -> &dyn Any
fn catalog_list(&self) -> Arc<dyn CatalogProviderList>
fn config(&self) -> &SessionConfig
fn config_options(&self) -> &ConfigOptions
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>>
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
fn default_table_options(&self) -> TableOptions
fn execution_props(&self) -> &ExecutionProps
fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
fn optimize(&self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan>
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
fn query_planner(&self) -> Arc<dyn QueryPlanner + Send + Sync>
fn runtime_env(&self) -> &Arc<RuntimeEnv>
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
fn session_id(&self) -> &str
fn table_options(&self) -> &TableOptions
fn table_options_mut(&mut self) -> &mut TableOptions
fn task_ctx(&self) -> Arc<TaskContext>
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.session.ForeignSession.md).


This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must use only the stable function pointers in
`FFI_SessionRef` to interact with the foreign session.

# Query planner delegation

If the session owner installed the current foreign query planner,
[`Session::create_physical_plan`] dispatches back to that planner and
[`Session::query_planner`] returns that planner. The planner must retain and
invoke the session owner's previous planner instead of using either method to
delegate back to the session. Otherwise, repeated delegation exhausts the
stack. See [`crate::query_planner`] for details.

---
