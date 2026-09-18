# `datafusion_execution::task`

Crate `datafusion-execution` · 2 public items · structured records in [`model/datafusion_execution.task.json`](../model/datafusion_execution.task.json)

## TaskContext

`struct` · `datafusion_execution::task::TaskContext`

Also reachable as `datafusion::execution::TaskContext`, `datafusion::execution::context::TaskContext`, `datafusion_execution::TaskContext`

```rust
struct TaskContext
```

**Implements**: `core::convert::From`, `datafusion_expr::registry::FunctionRegistry`

**Derives**: Debug, Default

**Methods** (13)

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
fn memory_pool(&self) -> &Arc<dyn MemoryPool>
fn new(task_id: Option<String>, session_id: String, session_config: SessionConfig, scalar_functions: HashMap<String, Arc<ScalarUDF>>, higher_order_functions: HashMap<String, Arc<HigherOrderUDF>>, aggregate_functions: HashMap<String, Arc<AggregateUDF>>, window_functions: HashMap<String, Arc<WindowUDF>>, runtime: Arc<RuntimeEnv>) -> Self
fn runtime_env(&self) -> Arc<RuntimeEnv>
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
fn session_config(&self) -> &SessionConfig
fn session_id(&self) -> String
fn task_id(&self) -> Option<String>
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
fn with_runtime(self, runtime: Arc<RuntimeEnv>) -> Self
fn with_session_config(self, session_config: SessionConfig) -> Self
fn with_task_id(self, task_id: String) -> Self
```

**via `datafusion_expr::registry::FunctionRegistry`**

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
fn higher_order_function_names(&self) -> HashSet<String>
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

[Full member, field, variant and typed contracts](../operations/datafusion_execution.task.TaskContext.md).


Task Execution Context

A [`TaskContext`] contains the state required during a single query's
execution. Please see the documentation on [`SessionContext`] for more
information.

# Relationship with [`ExecutionProps`]

[`TaskContext`] is intentionally distinct from [`ExecutionProps`].
[`ExecutionProps`] is state used while optimizing a logical
plan and constructing a physical plan.

[`TaskContext`] is the runtime context passed to physical operators when
executing a physical plan. It carries runtime services and session state
needed at that stage, such as [`RuntimeEnv`], memory-pool access, session
configuration, and function lookup.

Keeping these structures separate avoids threading execution/runtime state
through planning APIs, and avoids making execution depend on planner-only
scratch state.

[`SessionContext`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html
[`ExecutionProps`]: datafusion_expr::execution_props::ExecutionProps

---

## TaskContextProvider

`trait` · `datafusion_execution::task::TaskContextProvider`

Also reachable as `datafusion::execution::TaskContextProvider`, `datafusion_execution::TaskContextProvider`

```rust
trait TaskContextProvider
```

**Implementors** (2)

- `datafusion::execution::context::SessionContext`
- `datafusion::execution::session_state::SessionState`

**Methods** (1)

```rust
fn task_ctx(&self) -> Arc<TaskContext>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.task.TaskContextProvider.md).


Produce the [`TaskContext`].

---
