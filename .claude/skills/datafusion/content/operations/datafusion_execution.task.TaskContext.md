# `datafusion_execution::task::TaskContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.task.TaskContext.json).

<a id="op-ab706f4ad3fa6be4a08c30b0"></a>
## TaskContext

`struct` · `datafusion_execution::task::TaskContext` · datafusion-execution 55.1.0

```rust
struct TaskContext
```

Source: `src/task.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Task Execution Context

A [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) contains the state required during a single query's
execution. Please see the documentation on [`SessionContext`] for more
information.

# Relationship with [`ExecutionProps`]

[`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) is intentionally distinct from [`ExecutionProps`].
[`ExecutionProps`] is state used while optimizing a logical
plan and constructing a physical plan.

[`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) is the runtime context passed to physical operators when
executing a physical plan. It carries runtime services and session state
needed at that stage, such as [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67), memory-pool access, session
configuration, and function lookup.

Keeping these structures separate avoids threading execution/runtime state
through planning APIs, and avoids making execution depend on planner-only
scratch state.

[`SessionContext`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html
[`ExecutionProps`]: datafusion_expr::execution_props::ExecutionProps

<a id="op-1bc56fa29430f36b99afba21"></a>
## aggregate_functions

`function` · `datafusion_execution::task::TaskContext::aggregate_functions` · datafusion-execution 55.1.0

```rust
fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0298dbca98ec2b00cce4ee70"></a>
## default

`function` · `datafusion_execution::task::TaskContext::default` · datafusion-execution 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [87, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/task.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6f3296e9930ee1b1f4f3308"></a>
## expr_planners

`function` · `datafusion_execution::task::TaskContext::expr_planners` · datafusion-execution 55.1.0

```rust
fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27741f1b0a3ebeb879fff382"></a>
## fmt

`function` · `datafusion_execution::task::TaskContext::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/task.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/task.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbe74a96ab5913f42d42fb9b"></a>
## higher_order_function

`function` · `datafusion_execution::task::TaskContext::higher_order_function` · datafusion-execution 55.1.0

```rust
fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7667ff256b293dc9a9d7670"></a>
## higher_order_function_names

`function` · `datafusion_execution::task::TaskContext::higher_order_function_names` · datafusion-execution 55.1.0

```rust
fn higher_order_function_names(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f8a7fe2cfe2b7edfdc41ac9"></a>
## higher_order_functions

`function` · `datafusion_execution::task::TaskContext::higher_order_functions` · datafusion-execution 55.1.0

```rust
fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67707580ae4238a6e3ec6224"></a>
## memory_pool

`function` · `datafusion_execution::task::TaskContext::memory_pool` · datafusion-execution 55.1.0

```rust
fn memory_pool(&self) -> &Arc<dyn MemoryPool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the [`MemoryPool`](../operations/datafusion_execution.memory_pool.MemoryPool.md#op-05e08c54432abb360395d4af) associated with this [TaskContext](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0)

<a id="op-7ea18a6a3f8ea8729c0e6516"></a>
## new

`function` · `datafusion_execution::task::TaskContext::new` · datafusion-execution 55.1.0

```rust
fn new(task_id: Option<String>, session_id: String, session_config: SessionConfig, scalar_functions: HashMap<String, Arc<ScalarUDF>>, higher_order_functions: HashMap<String, Arc<HigherOrderUDF>>, aggregate_functions: HashMap<String, Arc<AggregateUDF>>, window_functions: HashMap<String, Arc<WindowUDF>>, runtime: Arc<RuntimeEnv>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Create a new [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0) instance.

Most users will use [`SessionContext::task_ctx`] to create [`TaskContext`](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0)s

[`SessionContext::task_ctx`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.task_ctx

<a id="op-639219092a824f053d78c20c"></a>
## register_higher_order_function

`function` · `datafusion_execution::task::TaskContext::register_higher_order_function` · datafusion-execution 55.1.0

```rust
fn register_higher_order_function(&mut self, function: Arc<HigherOrderUDF>) -> Result<Option<Arc<HigherOrderUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e67b20cbc379e843da80e558"></a>
## register_udaf

`function` · `datafusion_execution::task::TaskContext::register_udaf` · datafusion-execution 55.1.0

```rust
fn register_udaf(&mut self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e552aee41294094f48a251ce"></a>
## register_udf

`function` · `datafusion_execution::task::TaskContext::register_udf` · datafusion-execution 55.1.0

```rust
fn register_udf(&mut self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2fa0bc324cb473ce9186f09"></a>
## register_udwf

`function` · `datafusion_execution::task::TaskContext::register_udwf` · datafusion-execution 55.1.0

```rust
fn register_udwf(&mut self, udwf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9cd877c1f7477327fdf7a03"></a>
## runtime_env

`function` · `datafusion_execution::task::TaskContext::runtime_env` · datafusion-execution 55.1.0

```rust
fn runtime_env(&self) -> Arc<RuntimeEnv>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the [RuntimeEnv](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67) associated with this [TaskContext](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0)

<a id="op-47ba1667ce8795f8cba02030"></a>
## scalar_functions

`function` · `datafusion_execution::task::TaskContext::scalar_functions` · datafusion-execution 55.1.0

```rust
fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c889b445938641206c65ae9f"></a>
## session_config

`function` · `datafusion_execution::task::TaskContext::session_config` · datafusion-execution 55.1.0

```rust
fn session_config(&self) -> &SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the SessionConfig associated with this [TaskContext](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0)

<a id="op-fa95c558c77a04b3f97d1a6a"></a>
## session_id

`function` · `datafusion_execution::task::TaskContext::session_id` · datafusion-execution 55.1.0

```rust
fn session_id(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the `session_id` of this [TaskContext](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0)

<a id="op-1d19b6810d82235de9ea0453"></a>
## task_id

`function` · `datafusion_execution::task::TaskContext::task_id` · datafusion-execution 55.1.0

```rust
fn task_id(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the `task_id` of this [TaskContext](../operations/datafusion_execution.task.TaskContext.md#op-ab706f4ad3fa6be4a08c30b0)

<a id="op-10987e006a49c44e866f9371"></a>
## udaf

`function` · `datafusion_execution::task::TaskContext::udaf` · datafusion-execution 55.1.0

```rust
fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87aca44470de1601c1291365"></a>
## udafs

`function` · `datafusion_execution::task::TaskContext::udafs` · datafusion-execution 55.1.0

```rust
fn udafs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e84b41a4f277757fa0e4be4"></a>
## udf

`function` · `datafusion_execution::task::TaskContext::udf` · datafusion-execution 55.1.0

```rust
fn udf(&self, name: &str) -> Result<Arc<ScalarUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71320fab74f55ec16f33141c"></a>
## udfs

`function` · `datafusion_execution::task::TaskContext::udfs` · datafusion-execution 55.1.0

```rust
fn udfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3eeef2547edfdca1b7fb9195"></a>
## udwf

`function` · `datafusion_execution::task::TaskContext::udwf` · datafusion-execution 55.1.0

```rust
fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9ac3949d04c986ac9b0c838"></a>
## udwfs

`function` · `datafusion_execution::task::TaskContext::udwfs` · datafusion-execution 55.1.0

```rust
fn udwfs(&self) -> HashSet<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 1], "end": [271, 2], "filename": "src/task.rs"}, "trait": {"args": null, "id": "datafusion_expr::registry::FunctionRegistry", "path": "FunctionRegistry"}, "trait_path": "datafusion_expr::registry::FunctionRegistry"}`

Source: `src/task.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fd10fa1373823fbc2229ff"></a>
## window_functions

`function` · `datafusion_execution::task::TaskContext::window_functions` · datafusion-execution 55.1.0

```rust
fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dfe7a110d3d21d072906db5"></a>
## with_runtime

`function` · `datafusion_execution::task::TaskContext::with_runtime` · datafusion-execution 55.1.0

```rust
fn with_runtime(self, runtime: Arc<RuntimeEnv>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Update the [`RuntimeEnv`](../operations/datafusion_execution.runtime_env.RuntimeEnv.md#op-c598f4df4cf51824ae4b9a67)

<a id="op-1088b25132c3a26be3f33a81"></a>
## with_session_config

`function` · `datafusion_execution::task::TaskContext::with_session_config` · datafusion-execution 55.1.0

```rust
fn with_session_config(self, session_config: SessionConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Update the [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08)

<a id="op-bce1cd699b979b45d2522925"></a>
## with_task_id

`function` · `datafusion_execution::task::TaskContext::with_task_id` · datafusion-execution 55.1.0

```rust
fn with_task_id(self, task_id: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::task::TaskContext", "path": "TaskContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [176, 2], "filename": "src/task.rs"}, "trait": null, "trait_path": null}`

Source: `src/task.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Update the `task_id`
