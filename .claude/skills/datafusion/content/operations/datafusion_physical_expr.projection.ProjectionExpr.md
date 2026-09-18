# `datafusion_physical_expr::projection::ProjectionExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.projection.ProjectionExpr.json).

<a id="op-77aa0dab554890daa533d155"></a>
## ProjectionExpr

`struct` · `datafusion_physical_expr::projection::ProjectionExpr` · datafusion-physical-expr 55.1.0

```rust
struct ProjectionExpr
```

Source: `src/projection.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

An expression used by projection operations.

The expression is evaluated and the result is stored in a column
with the name specified by `alias`.

For example, the SQL expression `a + b AS sum_ab` would be represented
as a `ProjectionExpr` where `expr` is the expression `a + b`
and `alias` is the string `sum_ab`.

See [`ProjectionExprs`](../operations/datafusion_physical_expr.projection.ProjectionExprs.md#op-9af61cebb79ab505edeb3c19) for a collection of projection expressions.

<a id="op-e108f7002d87a7b05dd12c42"></a>
## alias

`struct_field` · `datafusion_physical_expr::projection::ProjectionExpr::alias` · datafusion-physical-expr 55.1.0

```rust
alias: String
```

Source: `src/projection.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The name of the output column for use an output schema.

<a id="op-e05d99111f5ad43f10db76b3"></a>
## as_ref

`function` · `datafusion_physical_expr::projection::ProjectionExpr::as_ref` · datafusion-physical-expr 55.1.0

```rust
fn as_ref(&self) -> &Arc<dyn PhysicalExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [78, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/projection.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3585fee26a484b6d7309e0a4"></a>
## clone

`function` · `datafusion_physical_expr::projection::ProjectionExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ProjectionExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 17], "end": [55, 22], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/projection.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a25e0189da544227d3534ac"></a>
## eq

`function` · `datafusion_physical_expr::projection::ProjectionExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [68, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/projection.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dc24ad609ca9720925058af"></a>
## expr

`struct_field` · `datafusion_physical_expr::projection::ProjectionExpr::expr` · datafusion-physical-expr 55.1.0

```rust
expr: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/projection.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The expression that will be evaluated.

<a id="op-4dd02e235c90299793d23540"></a>
## fmt

`function` · `datafusion_physical_expr::projection::ProjectionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [88, 2], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/projection.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5d194a51d42ec7d529f9cb7"></a>
## fmt

`function` · `datafusion_physical_expr::projection::ProjectionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/projection.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/projection.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01dd1479fb923569f9325e4b"></a>
## from

`function` · `datafusion_physical_expr::projection::ProjectionExpr::from` · datafusion-physical-expr 55.1.0

```rust
fn from(value: &(Arc<dyn PhysicalExpr>, String)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [120, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}, {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}]}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/projection.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95647d8865e85ccdd7e67b53"></a>
## from

`function` · `datafusion_physical_expr::projection::ProjectionExpr::from` · datafusion-physical-expr 55.1.0

```rust
fn from(value: (Arc<dyn PhysicalExpr>, String)) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [114, 2], "filename": "src/projection.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}, {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/projection.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90f4657e9ab18553f6df5f85"></a>
## new

`function` · `datafusion_physical_expr::projection::ProjectionExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, alias: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [108, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new projection expression

<a id="op-64fb0ff35bc0558ff8395be3"></a>
## new_from_expression

`function` · `datafusion_physical_expr::projection::ProjectionExpr::new_from_expression` · datafusion-physical-expr 55.1.0

```rust
fn new_from_expression(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::projection::ProjectionExpr", "path": "ProjectionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [108, 2], "filename": "src/projection.rs"}, "trait": null, "trait_path": null}`

Source: `src/projection.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new projection expression from an expression and a schema using the expression's output field name as alias.
