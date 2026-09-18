# `datafusion_physical_plan::windows::WindowUDFExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.windows.WindowUDFExpr.json).

<a id="op-4acda1be924c3f2333f79721"></a>
## WindowUDFExpr

`struct` · `datafusion_physical_plan::windows::WindowUDFExpr` · datafusion-physical-plan 55.1.0

```rust
struct WindowUDFExpr
```

Source: `src/windows/mod.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Implements [`StandardWindowFunctionExpr`](../operations/datafusion_physical_expr.window.standard_window_function_expr.StandardWindowFunctionExpr.md#op-5882bec2eb25cba3cfad5983) for [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65)

<a id="op-ded3bf7d97483c9dbe3a4e27"></a>
## args

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::args` · datafusion-physical-plan 55.1.0

```rust
fn args(&self) -> &[Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [243, 2], "filename": "src/windows/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/mod.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns all arguments passed to this window function.

Unlike [`StandardWindowFunctionExpr::expressions`](../operations/datafusion_physical_expr.window.standard_window_function_expr.StandardWindowFunctionExpr.md#op-db755a623dada2fe6238c02c), which returns
only the expressions that need batch evaluation (and may filter out
literal offset/default args like those for `lead`/`lag`), this
method returns the complete, unfiltered argument list. This is
needed for serialization so that all arguments survive a
protobuf round-trip.

<a id="op-0c93d8d8dd85b1c06b9bfd56"></a>
## as_any

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::as_any` · datafusion-physical-plan 55.1.0

```rust
fn as_any(&self) -> &dyn std::any::Any
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:246`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed5f83a1be61f3e1ec997154"></a>
## clone

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> WindowUDFExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 10], "end": [211, 15], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/windows/mod.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee03bc975017e0f99f8feb01"></a>
## create_evaluator

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::create_evaluator` · datafusion-physical-plan 55.1.0

```rust
fn create_evaluator(&self) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6caf00abc46a85a1e999dae"></a>
## expressions

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::expressions` · datafusion-physical-plan 55.1.0

```rust
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9449fa6bbb0237eabf48253c"></a>
## field

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::field` · datafusion-physical-plan 55.1.0

```rust
fn field(&self) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-857c540e7c1a46350ac59db7"></a>
## fmt

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [211, 17], "end": [211, 22], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/windows/mod.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2210bb8a98c5a7c5dc74a7ee"></a>
## fun

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::fun` · datafusion-physical-plan 55.1.0

```rust
fn fun(&self) -> &Arc<WindowUDF>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [243, 2], "filename": "src/windows/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/windows/mod.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da05633e9e8e7c16847fce67"></a>
## get_result_ordering

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::get_result_ordering` · datafusion-physical-plan 55.1.0

```rust
fn get_result_ordering(&self, schema: &SchemaRef) -> Option<PhysicalSortExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-385b98a58b321052338355d6"></a>
## limit_effect

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::limit_effect` · datafusion-physical-plan 55.1.0

```rust
fn limit_effect(&self) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ebc994450eb1dba0c111589"></a>
## name

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:270`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a456c3cf4c13c016f705d3c2"></a>
## reverse_expr

`function` · `datafusion_physical_plan::windows::WindowUDFExpr::reverse_expr` · datafusion-physical-plan 55.1.0

```rust
fn reverse_expr(&self) -> Option<Arc<dyn StandardWindowFunctionExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::windows::WindowUDFExpr", "path": "WindowUDFExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [302, 2], "filename": "src/windows/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr", "path": "StandardWindowFunctionExpr"}, "trait_path": "datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr"}`

Source: `src/windows/mod.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
