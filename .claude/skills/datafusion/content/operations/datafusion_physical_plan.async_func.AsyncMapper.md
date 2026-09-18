# `datafusion_physical_plan::async_func::AsyncMapper`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.async_func.AsyncMapper.json).

<a id="op-2482ca1bec1c70ac561d60c5"></a>
## AsyncMapper

`struct` · `datafusion_physical_plan::async_func::AsyncMapper` · datafusion-physical-plan 55.1.0

```rust
struct AsyncMapper
```

Source: `src/async_func.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Maps async_expressions to new columns

The output of the async functions are appended, in order, to the end of the input schema

<a id="op-7f7bbd78d2184b19571938d2"></a>
## async_exprs

`struct_field` · `datafusion_physical_plan::async_func::AsyncMapper::async_exprs` · datafusion-physical-plan 55.1.0

```rust
async_exprs: Vec<std::sync::Arc<datafusion_physical_expr::async_scalar_function::AsyncFuncExpr>>
```

Source: `src/async_func.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

the expressions to map

<a id="op-08e26b547933ac664750ab4a"></a>
## find_references

`function` · `datafusion_physical_plan::async_func::AsyncMapper::find_references` · datafusion-physical-plan 55.1.0

```rust
fn find_references(&mut self, physical_expr: &Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [502, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Finds any references to async functions in the expression and adds them to the map

<a id="op-e0f4f043b8c51093fd246ea9"></a>
## fmt

`function` · `datafusion_physical_plan::async_func::AsyncMapper::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 10], "end": [423, 15], "filename": "src/async_func.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/async_func.rs:423`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f27b345bb0f818080401318"></a>
## is_empty

`function` · `datafusion_physical_plan::async_func::AsyncMapper::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [502, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ef23316607a83bbb40069cf"></a>
## map_expr

`function` · `datafusion_physical_plan::async_func::AsyncMapper::map_expr` · datafusion-physical-plan 55.1.0

```rust
fn map_expr(&self, expr: Arc<dyn PhysicalExpr>) -> Transformed<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [502, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

If the expression matches any of the async functions, return the new column

<a id="op-8e8a6240eeb7548e9a998b1e"></a>
## new

`function` · `datafusion_physical_plan::async_func::AsyncMapper::new` · datafusion-physical-plan 55.1.0

```rust
fn new(num_input_columns: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [502, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:434`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c91a460ca764cea6c2a3244"></a>
## next_column_name

`function` · `datafusion_physical_plan::async_func::AsyncMapper::next_column_name` · datafusion-physical-plan 55.1.0

```rust
fn next_column_name(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [502, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:445`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f878c24d8ea4be9a4125a592"></a>
## output_column

`function` · `datafusion_physical_plan::async_func::AsyncMapper::output_column` · datafusion-physical-plan 55.1.0

```rust
fn output_column(&self, idx: usize) -> Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::async_func::AsyncMapper", "path": "AsyncMapper"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [433, 1], "end": [502, 2], "filename": "src/async_func.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_func.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

return the output column for the async function at index idx
