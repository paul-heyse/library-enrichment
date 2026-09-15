# `datafusion::execution::session_state_defaults`

Crate `datafusion` · 1 public items · structured records in [`model/datafusion.execution.session_state_defaults.json`](../model/datafusion.execution.session_state_defaults.json)

## SessionStateDefaults

`struct` · `datafusion::execution::session_state_defaults::SessionStateDefaults`

Also reachable as `datafusion::execution::SessionStateDefaults`

```rust
struct SessionStateDefaults
```

**Methods** (16)

```rust
fn default_aggregate_functions() -> Vec<Arc<AggregateUDF>>
fn default_catalog(config: &SessionConfig, table_factories: &HashMap<String, Arc<dyn TableProviderFactory>>, runtime: &Arc<RuntimeEnv>) -> MemoryCatalogProvider
fn default_expr_planners() -> Vec<Arc<dyn ExprPlanner>>
fn default_extension_types() -> Vec<ExtensionTypeRegistrationRef>
fn default_file_formats() -> Vec<Arc<dyn FileFormatFactory>>
fn default_higher_order_functions() -> Vec<Arc<HigherOrderUDF>>
fn default_scalar_functions() -> Vec<Arc<ScalarUDF>>
fn default_table_factories() -> HashMap<String, Arc<dyn TableProviderFactory>>
fn default_table_functions() -> Vec<Arc<TableFunction>>
fn default_window_functions() -> Vec<Arc<WindowUDF>>
fn register_aggregate_functions(state: &mut SessionState)
fn register_array_functions(state: &mut SessionState)
fn register_builtin_functions(state: &mut SessionState)
fn register_default_file_formats(state: &mut SessionState)
fn register_default_schema(config: &SessionConfig, table_factories: &HashMap<String, Arc<dyn TableProviderFactory>>, runtime: &Arc<RuntimeEnv>, default_catalog: &MemoryCatalogProvider)
fn register_scalar_functions(state: &mut SessionState)
```

Defaults that are used as part of creating a SessionState such as table providers,
file formats, registering of builtin functions, etc.

---
