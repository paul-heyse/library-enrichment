# `datafusion_expr::execution_props`

Crate `datafusion-expr` · 1 public items · structured records in [`model/datafusion_expr.execution_props.json`](../model/datafusion_expr.execution_props.json)

## ExecutionProps

`struct` · `datafusion_expr::execution_props::ExecutionProps`

Also reachable as `datafusion::execution::context::ExecutionProps`, `datafusion_physical_expr::execution_props::ExecutionProps`

```rust
struct ExecutionProps
```

**Fields**: `query_execution_start_time`, `alias_generator`, `config_options`, `var_providers`

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn add_var_provider(&mut self, var_type: VarType, provider: Arc<dyn VarProvider + Send + Sync>) -> Option<Arc<dyn VarProvider + Send + Sync>>
fn config_options(&self) -> Option<&Arc<ConfigOptions>>
fn get_var_provider(&self, var_type: VarType) -> Option<Arc<dyn VarProvider + Send + Sync>>
fn mark_start_execution(&mut self, config_options: Arc<ConfigOptions>) -> &Self
fn new() -> Self
fn start_execution(&mut self) -> &Self
fn with_query_execution_start_time(self, query_execution_start_time: DateTime<Utc>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.execution_props.ExecutionProps.md).


Holds properties and scratch state used while optimizing a [`LogicalPlan`]
and translating it into an executable physical plan, such as the statement
start time used during simplification.

An [`ExecutionProps`] is created each time a `LogicalPlan` is
prepared for execution (optimized). If the same plan is optimized
multiple times, a new `ExecutionProps` is created each time.

It is important that this structure be cheap to create as it is
done so during predicate pruning and expression simplification

# Relationship with [`TaskContext`]

[`ExecutionProps`] is intentionally distinct from [`TaskContext`].
It is used while optimizing a logical plan and constructing physical
expressions and physical plans, before physical operators are run.

[`TaskContext`] is the runtime context passed to physical operators during
physical-plan execution.

Keeping these structures separate avoids threading execution/runtime state
through planning APIs, and avoids making execution depend on planner-only
scratch state.

[`TaskContext`]: https://docs.rs/datafusion/latest/datafusion/execution/struct.TaskContext.html
[`LogicalPlan`]: crate::LogicalPlan

---
