# `datafusion_expr_common::placement::ExpressionPlacement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.placement.ExpressionPlacement.json).

<a id="op-92d2b127b1e1a43010d90d42"></a>
## ExpressionPlacement

`enum` · `datafusion_expr_common::placement::ExpressionPlacement` · datafusion-expr-common 55.1.0

```rust
enum ExpressionPlacement
```

Source: `src/placement.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Describes where an expression should be placed in the query plan for
optimal execution. This is used by optimizers to make decisions about
expression placement, such as whether to push expressions down through
projections.

<a id="op-447fdee2195f5434074c79e2"></a>
## Column

`variant` · `datafusion_expr_common::placement::ExpressionPlacement::Column` · datafusion-expr-common 55.1.0

```rust
Column
```

Source: `src/placement.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A simple column reference.

<a id="op-600eef0885718f7165bfa8f5"></a>
## KeepInPlace

`variant` · `datafusion_expr_common::placement::ExpressionPlacement::KeepInPlace` · datafusion-expr-common 55.1.0

```rust
KeepInPlace
```

Source: `src/placement.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An expensive expression that should stay where it is in the plan.
Examples include complex scalar functions or UDFs.

<a id="op-ee12feb49aec95294190fb4c"></a>
## Literal

`variant` · `datafusion_expr_common::placement::ExpressionPlacement::Literal` · datafusion-expr-common 55.1.0

```rust
Literal
```

Source: `src/placement.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A constant literal value.

<a id="op-3f7da67a9c55715af50a0a77"></a>
## MoveTowardsLeafNodes

`variant` · `datafusion_expr_common::placement::ExpressionPlacement::MoveTowardsLeafNodes` · datafusion-expr-common 55.1.0

```rust
MoveTowardsLeafNodes
```

Source: `src/placement.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A cheap expression that can be pushed to leaf nodes in the plan.
Examples include `get_field` for struct field access.
Pushing these expressions down in the plan can reduce data early
at low compute cost.
See [`ExpressionPlacement::should_push_to_leaves`](../operations/datafusion_expr_common.placement.ExpressionPlacement.md#op-308011e7f40edd91ff4475d6) for details.

<a id="op-ff407507692d0fb972bf9501"></a>
## clone

`function` · `datafusion_expr_common::placement::ExpressionPlacement::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::placement::ExpressionPlacement", "path": "ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 17], "end": [24, 22], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/placement.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37a4aa4e4a3c28a40edb294b"></a>
## eq

`function` · `datafusion_expr_common::placement::ExpressionPlacement::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &ExpressionPlacement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::placement::ExpressionPlacement", "path": "ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 30], "end": [24, 39], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/placement.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-215ef2352cbad2002482f09c"></a>
## fmt

`function` · `datafusion_expr_common::placement::ExpressionPlacement::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::placement::ExpressionPlacement", "path": "ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/placement.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fb0e24d93aa161f9caf4567"></a>
## hash

`function` · `datafusion_expr_common::placement::ExpressionPlacement::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::placement::ExpressionPlacement", "path": "ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 45], "end": [24, 49], "filename": "src/placement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/placement.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-308011e7f40edd91ff4475d6"></a>
## should_push_to_leaves

`function` · `datafusion_expr_common::placement::ExpressionPlacement::should_push_to_leaves` · datafusion-expr-common 55.1.0

```rust
fn should_push_to_leaves(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::placement::ExpressionPlacement", "path": "ExpressionPlacement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [62, 2], "filename": "src/placement.rs"}, "trait": null, "trait_path": null}`

Source: `src/placement.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns true if the expression can be pushed down to leaf nodes
in the query plan.

This returns true for:
- [`ExpressionPlacement::Column`](../operations/datafusion_expr_common.placement.ExpressionPlacement.md#op-447fdee2195f5434074c79e2): Simple column references can be pushed down. They do no compute and do not increase or
  decrease the amount of data being processed.
  A projection that reduces the number of columns can eliminate unnecessary data early,
  but this method only considers one expression at a time, not a projection as a whole.
- [`ExpressionPlacement::MoveTowardsLeafNodes`](../operations/datafusion_expr_common.placement.ExpressionPlacement.md#op-3f7da67a9c55715af50a0a77): Cheap expressions can be pushed down to leaves to take advantage of
  early computation and potential optimizations at the data source level.
  For example `struct_col['field']` is cheap to compute (just an Arc clone of the nested array for `'field'`)
  and thus can reduce data early in the plan at very low compute cost.
  It may even be possible to eliminate the expression entirely if the data source can project only the needed field
  (as e.g. Parquet can).
