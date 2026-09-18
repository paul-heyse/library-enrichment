# `datafusion::execution::context::RegisterFunction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.context.RegisterFunction.json).

<a id="op-9f71b0e97897512ace3c0cb9"></a>
## RegisterFunction

`enum` · `datafusion::execution::context::RegisterFunction` · datafusion 55.1.0

```rust
enum RegisterFunction
```

Source: `src/execution/context/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

The result of processing a [`CreateFunction`](../operations/datafusion_expr.logical_plan.ddl.CreateFunction.md#op-3668c6565f5901a6967c489f) statement with [`FunctionFactory`](../operations/datafusion.execution.context.FunctionFactory.md#op-4f2bfbb3e4aa340eb4071d97).

<a id="op-aeed0f32f0e483ca6f11aa95"></a>
## Aggregate

`variant` · `datafusion::execution::context::RegisterFunction::Aggregate` · datafusion 55.1.0

```rust
Aggregate
```

Source: `src/execution/context/mod.rs:2240`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Aggregate user defined function

<a id="op-08c7c5f710a90b105a6864bc"></a>
## HigherOrder

`variant` · `datafusion::execution::context::RegisterFunction::HigherOrder` · datafusion 55.1.0

```rust
HigherOrder
```

Source: `src/execution/context/mod.rs:2244`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Higher-order user defined function

<a id="op-b693400936222392600d2d41"></a>
## Scalar

`variant` · `datafusion::execution::context::RegisterFunction::Scalar` · datafusion 55.1.0

```rust
Scalar
```

Source: `src/execution/context/mod.rs:2238`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Scalar user defined function

<a id="op-00c83be12a6a228a540247ef"></a>
## Table

`variant` · `datafusion::execution::context::RegisterFunction::Table` · datafusion 55.1.0

```rust
Table
```

Source: `src/execution/context/mod.rs:2246`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Table user defined function

<a id="op-ef504c7a1c69b10539aedea2"></a>
## Window

`variant` · `datafusion::execution::context::RegisterFunction::Window` · datafusion 55.1.0

```rust
Window
```

Source: `src/execution/context/mod.rs:2242`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Window user defined function

<a id="op-98dacaeab86378b777e6bf76"></a>
## clone

`function` · `datafusion::execution::context::RegisterFunction::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> RegisterFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::RegisterFunction", "path": "RegisterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2235, 17], "end": [2235, 22], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/execution/context/mod.rs:2235`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4ebc42c2fdd4b61a4aa5a86"></a>
## fmt

`function` · `datafusion::execution::context::RegisterFunction::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::execution::context::RegisterFunction", "path": "RegisterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2235, 10], "end": [2235, 15], "filename": "src/execution/context/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/execution/context/mod.rs:2235`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
