# `datafusion_expr::execution_props::ExecutionProps`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.execution_props.ExecutionProps.json).

<a id="op-52e4deb424494aeff92bf188"></a>
## ExecutionProps

`struct` · `datafusion_expr::execution_props::ExecutionProps` · datafusion-expr 55.1.0

```rust
struct ExecutionProps
```

Source: `src/execution_props.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Holds properties and scratch state used while optimizing a [`LogicalPlan`]
and translating it into an executable physical plan, such as the statement
start time used during simplification.

An [`ExecutionProps`](../operations/datafusion_expr.execution_props.ExecutionProps.md#op-52e4deb424494aeff92bf188) is created each time a `LogicalPlan` is
prepared for execution (optimized). If the same plan is optimized
multiple times, a new `ExecutionProps` is created each time.

It is important that this structure be cheap to create as it is
done so during predicate pruning and expression simplification

# Relationship with [`TaskContext`]

[`ExecutionProps`](../operations/datafusion_expr.execution_props.ExecutionProps.md#op-52e4deb424494aeff92bf188) is intentionally distinct from [`TaskContext`].
It is used while optimizing a logical plan and constructing physical
expressions and physical plans, before physical operators are run.

[`TaskContext`] is the runtime context passed to physical operators during
physical-plan execution.

Keeping these structures separate avoids threading execution/runtime state
through planning APIs, and avoids making execution depend on planner-only
scratch state.

[`TaskContext`]: https://docs.rs/datafusion/latest/datafusion/execution/struct.TaskContext.html
[`LogicalPlan`]: crate::LogicalPlan

<a id="op-0242a55cddef6c4b1ffd3ac2"></a>
## add_var_provider

`function` · `datafusion_expr::execution_props::ExecutionProps::add_var_provider` · datafusion-expr 55.1.0

```rust
fn add_var_provider(&mut self, var_type: VarType, provider: Arc<dyn VarProvider + Send + Sync>) -> Option<Arc<dyn VarProvider + Send + Sync>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Registers a variable provider, returning the existing provider, if any

<a id="op-926ec49971dc42d68ac3d43e"></a>
## alias_generator

`struct_field` · `datafusion_expr::execution_props::ExecutionProps::alias_generator` · datafusion-expr 55.1.0

```rust
alias_generator: std::sync::Arc<datafusion_common::alias::AliasGenerator>
```

Source: `src/execution_props.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Alias generator used by subquery optimizer rules

<a id="op-e560ca4fe648344780819dd0"></a>
## clone

`function` · `datafusion_expr::execution_props::ExecutionProps::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ExecutionProps
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/execution_props.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution_props.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bf28768f2cb242000fb8a20"></a>
## config_options

`struct_field` · `datafusion_expr::execution_props::ExecutionProps::config_options` · datafusion-expr 55.1.0

```rust
config_options: Option<std::sync::Arc<datafusion_common::config::ConfigOptions>>
```

Source: `src/execution_props.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Snapshot of config options when the query started

<a id="op-9ec6b2fec1297b97d53fbc94"></a>
## config_options

`function` · `datafusion_expr::execution_props::ExecutionProps::config_options` · datafusion-expr 55.1.0

```rust
fn config_options(&self) -> Option<&Arc<ConfigOptions>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the configuration properties for this execution
if the execution has started

<a id="op-61c0d9b70075ac7683107df5"></a>
## default

`function` · `datafusion_expr::execution_props::ExecutionProps::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [68, 2], "filename": "src/execution_props.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/execution_props.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e7b8f3ef6c23e12d8b8655c"></a>
## fmt

`function` · `datafusion_expr::execution_props::ExecutionProps::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 22], "filename": "src/execution_props.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution_props.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80939806e814c6a985aa0c2d"></a>
## get_var_provider

`function` · `datafusion_expr::execution_props::ExecutionProps::get_var_provider` · datafusion-expr 55.1.0

```rust
fn get_var_provider(&self, var_type: VarType) -> Option<Arc<dyn VarProvider + Send + Sync>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the provider for the `var_type`, if any

<a id="op-89ebe94a5b2b0b96a53ff557"></a>
## mark_start_execution

`function` · `datafusion_expr::execution_props::ExecutionProps::mark_start_execution` · datafusion-expr 55.1.0

```rust
fn mark_start_execution(&mut self, config_options: Arc<ConfigOptions>) -> &Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Marks the execution of query started timestamp.
This also instantiates a new alias generator.

<a id="op-3e2f982c44bf644cd3b418ff"></a>
## new

`function` · `datafusion_expr::execution_props::ExecutionProps::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new execution props

<a id="op-0773cbf0fcf43a5ad22eece7"></a>
## query_execution_start_time

`struct_field` · `datafusion_expr::execution_props::ExecutionProps::query_execution_start_time` · datafusion-expr 55.1.0

```rust
query_execution_start_time: Option<chrono::DateTime<chrono::Utc>>
```

Source: `src/execution_props.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The time at which the query execution started. If `None`,
functions like `now()` will not be simplified during optimization.

<a id="op-d737e3fd46a898b717890f57"></a>
## start_execution

`function` · `datafusion_expr::execution_props::ExecutionProps::start_execution` · datafusion-expr 55.1.0

```rust
fn start_execution(&mut self) -> &Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48e99d97492f623b21ad7b19"></a>
## var_providers

`struct_field` · `datafusion_expr::execution_props::ExecutionProps::var_providers` · datafusion-expr 55.1.0

```rust
var_providers: Option<datafusion_common::HashMap<var_provider::VarType, std::sync::Arc<dyn VarProvider + Send + Sync>>>
```

Source: `src/execution_props.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Providers for scalar variables

<a id="op-eaf9e5682a934f7fd6b2dbb5"></a>
## with_query_execution_start_time

`function` · `datafusion_expr::execution_props::ExecutionProps::with_query_execution_start_time` · datafusion-expr 55.1.0

```rust
fn with_query_execution_start_time(self, query_execution_start_time: DateTime<Utc>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::execution_props::ExecutionProps", "path": "ExecutionProps"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [136, 2], "filename": "src/execution_props.rs"}, "trait": null, "trait_path": null}`

Source: `src/execution_props.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the query execution start time to use
