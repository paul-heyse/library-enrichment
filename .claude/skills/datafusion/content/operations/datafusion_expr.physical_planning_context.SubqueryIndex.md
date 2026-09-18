# `datafusion_expr::physical_planning_context::SubqueryIndex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.physical_planning_context.SubqueryIndex.json).

<a id="op-e515702b50ba885a0bc5b233"></a>
## SubqueryIndex

`struct` · `datafusion_expr::physical_planning_context::SubqueryIndex` · datafusion-expr 55.1.0

```rust
struct SubqueryIndex
```

Source: `src/physical_planning_context.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Index of a scalar subquery within a [`ScalarSubqueryResults`](../operations/datafusion_expr.physical_planning_context.ScalarSubqueryResults.md#op-6b0683e0d87a9cc813cb37e5) container.

<a id="op-39c32fa45d8d4c0a45bad2b3"></a>
## as_usize

`function` · `datafusion_expr::physical_planning_context::SubqueryIndex::as_usize` · datafusion-expr 55.1.0

```rust
const fn as_usize(self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::SubqueryIndex", "path": "SubqueryIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [124, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the underlying slot index.

<a id="op-e16a2fb57095d28e48fdc0d3"></a>
## clone

`function` · `datafusion_expr::physical_planning_context::SubqueryIndex::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> SubqueryIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::SubqueryIndex", "path": "SubqueryIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 10], "end": [111, 15], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/physical_planning_context.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c12bfe6583e14bb74e0493f"></a>
## eq

`function` · `datafusion_expr::physical_planning_context::SubqueryIndex::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SubqueryIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::SubqueryIndex", "path": "SubqueryIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 30], "end": [111, 39], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/physical_planning_context.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04a990c2f08271f42512df01"></a>
## fmt

`function` · `datafusion_expr::physical_planning_context::SubqueryIndex::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::SubqueryIndex", "path": "SubqueryIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 23], "end": [111, 28], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_planning_context.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd2b44d92f7944cf1716b45e"></a>
## hash

`function` · `datafusion_expr::physical_planning_context::SubqueryIndex::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::SubqueryIndex", "path": "SubqueryIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 45], "end": [111, 49], "filename": "src/physical_planning_context.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/physical_planning_context.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48ddfa5bd525261eeb8049b5"></a>
## new

`function` · `datafusion_expr::physical_planning_context::SubqueryIndex::new` · datafusion-expr 55.1.0

```rust
const fn new(index: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::physical_planning_context::SubqueryIndex", "path": "SubqueryIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [124, 2], "filename": "src/physical_planning_context.rs"}, "trait": null, "trait_path": null}`

Source: `src/physical_planning_context.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new subquery index.
