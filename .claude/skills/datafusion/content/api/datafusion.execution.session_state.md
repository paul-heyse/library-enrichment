# `datafusion::execution::session_state`

Crate `datafusion` · 3 public items · structured records in [`model/datafusion.execution.session_state.json`](../model/datafusion.execution.session_state.json)

## SessionState

`struct` · `datafusion::execution::session_state::SessionState`

Also reachable as `datafusion::execution::SessionState`, `datafusion::execution::context::SessionState`

```rust
struct SessionState
```

**Implements**: `datafusion_execution::task::TaskContextProvider`, `datafusion_expr::registry::FunctionRegistry`, `datafusion_optimizer::optimizer::OptimizerConfig`, `datafusion_session::physical_optimizer::PhysicalOptimizerContext`, `datafusion_session::session::Session`

**Derives**: Clone, Debug

**Methods** (54)

```rust
fn add_analyzer_rule(&mut self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> &Self
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
fn analyzer(&self) -> &Analyzer
fn cache_factory(&self) -> Option<&Arc<dyn CacheFactory>>
fn catalog_list(&self) -> &Arc<dyn CatalogProviderList>
fn config(&self) -> &SessionConfig
fn config_mut(&mut self) -> &mut SessionConfig
fn config_options(&self) -> &Arc<ConfigOptions>
fn create_logical_expr(&self, sql: &str, df_schema: &DFSchema) -> datafusion_common::Result<Expr>
fn create_logical_expr_from_sql_expr(&self, sql_expr: SQLExprWithAlias, df_schema: &DFSchema) -> datafusion_common::Result<Expr>
async fn create_logical_plan(&self, sql: &str) -> datafusion_common::Result<LogicalPlan>
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>>
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
fn default_table_options(&self) -> TableOptions
fn deregister_udtf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableFunctionImpl>>>
fn execution_props(&self) -> &ExecutionProps
fn execution_props_mut(&mut self) -> &mut ExecutionProps
fn expr_planners(&self) -> &[Arc<dyn ExprPlanner>]
fn function_factory(&self) -> Option<&Arc<dyn FunctionFactory>>
fn get_file_format_factory(&self, ext: &str) -> Option<Arc<dyn FileFormatFactory>>
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
fn mark_start_execution(&mut self)
fn optimize(&self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan>
fn optimizer(&self) -> &Optimizer
fn optimizers(&self) -> &[Arc<dyn OptimizerRule + Send + Sync>]
fn physical_optimizers(&self) -> &[Arc<dyn PhysicalOptimizerRule + Send + Sync>]
fn query_planner(&self) -> &Arc<dyn QueryPlanner + Send + Sync>
fn register_catalog_list(&mut self, catalog_list: Arc<dyn CatalogProviderList>)
fn register_file_format(&mut self, file_format: Arc<dyn FileFormatFactory>, overwrite: bool) -> Result<(), DataFusionError>
fn register_relation_planner(&mut self, planner: Arc<dyn RelationPlanner>) -> datafusion_common::Result<()>
fn register_table_options_extension<T: ConfigExtension>(&mut self, extension: T)
fn register_udtf(&mut self, name: &str, fun: Arc<dyn TableFunctionImpl>)
fn relation_planners(&self) -> &[Arc<dyn RelationPlanner>]
fn resolve_table_references(&self, statement: &Statement) -> datafusion_common::Result<Vec<TableReference>>
fn runtime_env(&self) -> &Arc<RuntimeEnv>
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
fn schema_for_ref(&self, table_ref: impl Into<TableReference>) -> datafusion_common::Result<Arc<dyn SchemaProvider>>
fn serializer_registry(&self) -> &Arc<dyn SerializerRegistry>
fn session_id(&self) -> &str
fn set_cache_factory(&mut self, cache_factory: Arc<dyn CacheFactory>)
fn set_function_factory(&mut self, function_factory: Arc<dyn FunctionFactory>)
fn sql_to_expr(&self, sql: &str, dialect: &Dialect) -> datafusion_common::Result<SQLExpr>
fn sql_to_expr_with_alias(&self, sql: &str, dialect: &Dialect) -> datafusion_common::Result<SQLExprWithAlias>
fn sql_to_statement(&self, sql: &str, dialect: &Dialect) -> datafusion_common::Result<Statement>
async fn statement_to_plan(&self, statement: Statement) -> datafusion_common::Result<LogicalPlan>
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
fn table_factories(&self) -> &HashMap<String, Arc<dyn TableProviderFactory>>
fn table_factories_mut(&mut self) -> &mut HashMap<String, Arc<dyn TableProviderFactory>>
fn table_functions(&self) -> &HashMap<String, Arc<TableFunction>>
fn table_options(&self) -> &TableOptions
fn table_options_mut(&mut self) -> &mut TableOptions
fn task_ctx(&self) -> Arc<TaskContext>
fn version(&self) -> &str
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

**via `datafusion_execution::task::TaskContextProvider`**

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

**via `datafusion_expr::registry::FunctionRegistry`**

```rust
fn deregister_higher_order_function(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<HigherOrderUDF>>>
fn deregister_udaf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<AggregateUDF>>>
fn deregister_udf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<ScalarUDF>>>
fn deregister_udwf(&mut self, name: &str) -> datafusion_common::Result<Option<Arc<WindowUDF>>>
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
fn higher_order_function(&self, name: &str) -> datafusion_common::Result<Arc<HigherOrderUDF>>
fn higher_order_function_names(&self) -> HashSet<String>
fn register_expr_planner(&mut self, expr_planner: Arc<dyn ExprPlanner>) -> datafusion_common::Result<()>
fn register_function_rewrite(&mut self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> datafusion_common::Result<()>
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> datafusion_common::Result<Option<Arc<HigherOrderUDF>>>
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> datafusion_common::Result<Option<Arc<AggregateUDF>>>
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> datafusion_common::Result<Option<Arc<ScalarUDF>>>
fn register_udwf(&mut self, udwf: Arc<WindowUDF>) -> datafusion_common::Result<Option<Arc<WindowUDF>>>
fn udaf(&self, name: &str) -> datafusion_common::Result<Arc<AggregateUDF>>
fn udafs(&self) -> HashSet<String>
fn udf(&self, name: &str) -> datafusion_common::Result<Arc<ScalarUDF>>
fn udfs(&self) -> HashSet<String>
fn udwf(&self, name: &str) -> datafusion_common::Result<Arc<WindowUDF>>
fn udwfs(&self) -> HashSet<String>
```

**via `datafusion_optimizer::optimizer::OptimizerConfig`**

```rust
fn alias_generator(&self) -> &Arc<AliasGenerator>
fn function_registry(&self) -> Option<&dyn FunctionRegistry>
fn options(&self) -> Arc<ConfigOptions>
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerContext`**

```rust
fn config_options(&self) -> &ConfigOptions
fn statistics_registry(&self) -> Option<&StatisticsRegistry>
```

**via `datafusion_session::session::Session`**

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
fn as_any(&self) -> &dyn Any
fn catalog_list(&self) -> Arc<dyn CatalogProviderList>
fn config(&self) -> &SessionConfig
fn create_physical_expr(&self, expr: Expr, df_schema: &DFSchema) -> datafusion_common::Result<Arc<dyn PhysicalExpr>>
async fn create_physical_plan(&self, logical_plan: &LogicalPlan) -> datafusion_common::Result<Arc<dyn ExecutionPlan>>
fn execution_props(&self) -> &ExecutionProps
fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
fn optimize(&self, plan: &LogicalPlan) -> datafusion_common::Result<LogicalPlan>
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

[Full member, field, variant and typed contracts](../operations/datafusion.execution.session_state.SessionState.md).


`SessionState` contains all the necessary state to plan and execute queries,
such as configuration, functions, and runtime environment. Please see the
documentation on [`SessionContext`] for more information.


# Example: `SessionState` from a [`SessionContext`]

```
use datafusion::prelude::*;
let ctx = SessionContext::new();
let state = ctx.state();
```

# Example: `SessionState` via [`SessionStateBuilder`]

You can also use [`SessionStateBuilder`] to build a `SessionState` object
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
the [`SessionConfig`] or [`RuntimeEnv`]. See [`SessionStateBuilder`] and
[`SessionContext`].

[`SessionContext`]: crate::execution::context::SessionContext

---

## SessionStateBuilder

`struct` · `datafusion::execution::session_state::SessionStateBuilder`

Also reachable as `datafusion::execution::SessionStateBuilder`

```rust
struct SessionStateBuilder
```

**Implements**: `core::convert::From`, `datafusion_spark::session_state::SessionStateBuilderSpark`

**Derives**: Clone, Debug, Default

**Methods** (62)

```rust
fn aggregate_functions(&mut self) -> &mut Option<Vec<Arc<AggregateUDF>>>
fn analyzer(&mut self) -> &mut Option<Analyzer>
fn analyzer_rules(&mut self) -> &mut Option<Vec<Arc<dyn AnalyzerRule + Send + Sync>>>
fn build(self) -> SessionState
fn cache_factory(&mut self) -> &mut Option<Arc<dyn CacheFactory>>
fn catalog_list(&mut self) -> &mut Option<Arc<dyn CatalogProviderList>>
fn config(&mut self) -> &mut Option<SessionConfig>
fn execution_props(&mut self) -> &mut Option<ExecutionProps>
fn expr_planners(&mut self) -> &mut Option<Vec<Arc<dyn ExprPlanner>>>
fn file_formats(&mut self) -> &mut Option<Vec<Arc<dyn FileFormatFactory>>>
fn function_factory(&mut self) -> &mut Option<Arc<dyn FunctionFactory>>
fn higher_order_functions(&mut self) -> &mut Option<Vec<Arc<HigherOrderUDF>>>
fn new() -> Self
fn new_from_existing(existing: SessionState) -> Self
fn new_with_default_features() -> Self
fn optimizer(&mut self) -> &mut Option<Optimizer>
fn optimizer_rules(&mut self) -> &mut Option<Vec<Arc<dyn OptimizerRule + Send + Sync>>>
fn physical_optimizer_rules(&mut self) -> &mut Option<Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>>
fn physical_optimizers(&mut self) -> &mut Option<PhysicalOptimizer>
fn query_planner(&mut self) -> &mut Option<Arc<dyn QueryPlanner + Send + Sync>>
fn relation_planners(&mut self) -> &mut Option<Vec<Arc<dyn RelationPlanner>>>
fn runtime_env(&mut self) -> &mut Option<Arc<RuntimeEnv>>
fn scalar_functions(&mut self) -> &mut Option<Vec<Arc<ScalarUDF>>>
fn serializer_registry(&mut self) -> &mut Option<Arc<dyn SerializerRegistry>>
fn session_id(&self) -> &Option<String>
fn table_factories(&mut self) -> &mut Option<HashMap<String, Arc<dyn TableProviderFactory>>>
fn table_functions(&mut self) -> &mut Option<HashMap<String, Arc<TableFunction>>>
fn table_options(&mut self) -> &mut Option<TableOptions>
fn type_planner(&mut self) -> &mut Option<Arc<dyn TypePlanner>>
fn window_functions(&mut self) -> &mut Option<Vec<Arc<WindowUDF>>>
fn with_aggregate_functions(self, aggregate_functions: Vec<Arc<AggregateUDF>>) -> Self
fn with_analyzer_rule(self, analyzer_rule: Arc<dyn AnalyzerRule + Send + Sync>) -> Self
fn with_analyzer_rules(self, rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self
fn with_cache_factory(self, cache_factory: Option<Arc<dyn CacheFactory>>) -> Self
fn with_catalog_list(self, catalog_list: Arc<dyn CatalogProviderList>) -> Self
fn with_config(self, config: SessionConfig) -> Self
fn with_default_features(self) -> Self
fn with_execution_props(self, execution_props: ExecutionProps) -> Self
fn with_expr_planners(self, expr_planners: Vec<Arc<dyn ExprPlanner>>) -> Self
fn with_extension_type_registry(self, registry: ExtensionTypeRegistryRef) -> Self
fn with_file_formats(self, file_formats: Vec<Arc<dyn FileFormatFactory>>) -> Self
fn with_function_factory(self, function_factory: Option<Arc<dyn FunctionFactory>>) -> Self
fn with_higher_order_functions(self, higher_order_functions: Vec<Arc<HigherOrderUDF>>) -> Self
fn with_object_store(self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Self
fn with_optimizer_rule(self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>) -> Self
fn with_optimizer_rules(self, rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self
fn with_physical_optimizer_rule(self, physical_optimizer_rule: Arc<dyn PhysicalOptimizerRule + Send + Sync>) -> Self
fn with_physical_optimizer_rules(self, physical_optimizers: Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>) -> Self
fn with_query_planner(self, query_planner: Arc<dyn QueryPlanner + Send + Sync>) -> Self
fn with_relation_planners(self, relation_planners: Vec<Arc<dyn RelationPlanner>>) -> Self
fn with_runtime_env(self, runtime_env: Arc<RuntimeEnv>) -> Self
fn with_scalar_functions(self, scalar_functions: Vec<Arc<ScalarUDF>>) -> Self
fn with_serializer_registry(self, serializer_registry: Arc<dyn SerializerRegistry>) -> Self
fn with_session_id(self, session_id: String) -> Self
fn with_statistics_registry(self, registry: StatisticsRegistry) -> Self
fn with_table_factories(self, table_factories: HashMap<String, Arc<dyn TableProviderFactory>>) -> Self
fn with_table_factory(self, key: String, table_factory: Arc<dyn TableProviderFactory>) -> Self
fn with_table_function_list(self, table_functions: Vec<Arc<TableFunction>>) -> Self
fn with_table_functions(self, table_functions: HashMap<String, Arc<TableFunction>>) -> Self
fn with_table_options(self, table_options: TableOptions) -> Self
fn with_type_planner(self, type_planner: Arc<dyn TypePlanner>) -> Self
fn with_window_functions(self, window_functions: Vec<Arc<WindowUDF>>) -> Self
```

**via `core::convert::From`**

```rust
fn from(state: SessionState) -> Self
fn from(session: SessionContext) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.session_state.SessionStateBuilder.md).


A builder to be used for building [`SessionState`]'s. Defaults will
be used for all values unless explicitly provided.

See example on [`SessionState`]

---

## CacheFactory

`trait` · `datafusion::execution::session_state::CacheFactory`

```rust
trait CacheFactory: Debug + Send + Sync
```

**Methods** (1)

```rust
fn create(&self, plan: LogicalPlan, session_state: &SessionState) -> datafusion_common::Result<LogicalPlan>
```

[Full member, field, variant and typed contracts](../operations/datafusion.execution.session_state.CacheFactory.md).


A [`CacheFactory`] can be registered via [`SessionState`]
to create a custom logical plan for [`crate::dataframe::DataFrame::cache`].
Additionally, a custom [`crate::physical_planner::ExtensionPlanner`]/[`QueryPlanner`]
may need to be implemented to handle such plans.

---
