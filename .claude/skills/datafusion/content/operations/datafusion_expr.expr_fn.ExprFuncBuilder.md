# `datafusion_expr::expr_fn::ExprFuncBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.ExprFuncBuilder.json).

<a id="op-072708c533d8ed0378d04174"></a>
## ExprFuncBuilder

`struct` · `datafusion_expr::expr_fn::ExprFuncBuilder` · datafusion-expr 55.1.0

```rust
struct ExprFuncBuilder
```

Source: `src/expr_fn.rs:812`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Implementation of [`ExprFunctionExt`](../operations/datafusion_expr.expr_fn.ExprFunctionExt.md#op-99506f92b515415a11439278).

See [`ExprFunctionExt`](../operations/datafusion_expr.expr_fn.ExprFunctionExt.md#op-99506f92b515415a11439278) for usage and examples

<a id="op-003ea2ed88c6e5b8d3d38366"></a>
## build

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::build` · datafusion-expr 55.1.0

```rust
fn build(self) -> Result<Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [822, 1], "end": [882, 2], "filename": "src/expr_fn.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_fn.rs:842`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Updates and returns the in progress [`Expr::AggregateFunction`](../operations/datafusion_expr.expr.Expr.md#op-98c26e3c882c5c14cacb82a2) or [`Expr::WindowFunction`](../operations/datafusion_expr.expr.Expr.md#op-1ab47cd303f22d10dd7f4cec)

# Errors:

Returns an error if this builder  [`ExprFunctionExt`](../operations/datafusion_expr.expr_fn.ExprFunctionExt.md#op-99506f92b515415a11439278) was used with an
`Expr` variant other than [`Expr::AggregateFunction`](../operations/datafusion_expr.expr.Expr.md#op-98c26e3c882c5c14cacb82a2) or [`Expr::WindowFunction`](../operations/datafusion_expr.expr.Expr.md#op-1ab47cd303f22d10dd7f4cec)

<a id="op-18dfda2b54a96688bc97bfee"></a>
## clone

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [811, 17], "end": [811, 22], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr_fn.rs:811`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaeb25277ce98b8d54312a67"></a>
## distinct

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::distinct` · datafusion-expr 55.1.0

```rust
fn distinct(self) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [921, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:898`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `DISTINCT`

<a id="op-cc2a22659ff783c0c73ac129"></a>
## filter

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::filter` · datafusion-expr 55.1.0

```rust
fn filter(self, filter: Expr) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [921, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:892`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `FILTER <filter>`

<a id="op-df43320d45f7bb84b202d1ed"></a>
## fmt

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [811, 10], "end": [811, 15], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr_fn.rs:811`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8d77587a5b8e7117212478f"></a>
## null_treatment

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::null_treatment` · datafusion-expr 55.1.0

```rust
fn null_treatment(self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [921, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:904`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `RESPECT NULLS` or `IGNORE NULLS`

<a id="op-a27f7845ba47ee1d30d5e752"></a>
## order_by

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::order_by` · datafusion-expr 55.1.0

```rust
fn order_by(self, order_by: Vec<Sort>) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [921, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:886`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Add `ORDER BY <order_by>`

<a id="op-798f7f8a77bd6cf827969f2d"></a>
## partition_by

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::partition_by` · datafusion-expr 55.1.0

```rust
fn partition_by(self, partition_by: Vec<Expr>) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [921, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:912`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb55713de2b040a4026c2afa"></a>
## window_frame

`function` · `datafusion_expr::expr_fn::ExprFuncBuilder::window_frame` · datafusion-expr 55.1.0

```rust
fn window_frame(self, window_frame: WindowFrame) -> ExprFuncBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::ExprFuncBuilder", "path": "ExprFuncBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [921, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::expr_fn::ExprFunctionExt", "path": "ExprFunctionExt"}, "trait_path": "datafusion_expr::expr_fn::ExprFunctionExt"}`

Source: `src/expr_fn.rs:917`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
