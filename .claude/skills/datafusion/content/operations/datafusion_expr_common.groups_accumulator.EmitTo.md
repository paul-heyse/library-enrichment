# `datafusion_expr_common::groups_accumulator::EmitTo`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.groups_accumulator.EmitTo.json).

<a id="op-5489f3d28ee46dc1a28c34e5"></a>
## EmitTo

`enum` · `datafusion_expr_common::groups_accumulator::EmitTo` · datafusion-expr-common 55.1.0

```rust
enum EmitTo
```

Source: `src/groups_accumulator.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Describes how many rows should be emitted during grouping.

<a id="op-3675659b93702fc0d63f9c0b"></a>
## All

`variant` · `datafusion_expr_common::groups_accumulator::EmitTo::All` · datafusion-expr-common 55.1.0

```rust
All
```

Source: `src/groups_accumulator.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Emit all groups

<a id="op-935d538c0b51c7f0267f586c"></a>
## First

`variant` · `datafusion_expr_common::groups_accumulator::EmitTo::First` · datafusion-expr-common 55.1.0

```rust
First
```

Source: `src/groups_accumulator.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Emit only the first `n` groups and shift all existing group
indexes down by `n`.

For example, if `n=10`, group_index `0, 1, ... 9` are emitted
and group indexes `10, 11, 12, ...` become `0, 1, 2, ...`.

<a id="op-76c08e2540e73eca950dddc2"></a>
## clone

`function` · `datafusion_expr_common::groups_accumulator::EmitTo::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> EmitTo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::groups_accumulator::EmitTo", "path": "EmitTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 17], "end": [24, 22], "filename": "src/groups_accumulator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/groups_accumulator.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18cbe3509c7ba20b51ad7fc6"></a>
## eq

`function` · `datafusion_expr_common::groups_accumulator::EmitTo::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &EmitTo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::groups_accumulator::EmitTo", "path": "EmitTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 30], "end": [24, 39], "filename": "src/groups_accumulator.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/groups_accumulator.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-868a5c4f5aa655c494f2c4f7"></a>
## fmt

`function` · `datafusion_expr_common::groups_accumulator::EmitTo::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::groups_accumulator::EmitTo", "path": "EmitTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/groups_accumulator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/groups_accumulator.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-775efdcabfaa94475aa515fc"></a>
## take_needed

`function` · `datafusion_expr_common::groups_accumulator::EmitTo::take_needed` · datafusion-expr-common 55.1.0

```rust
fn take_needed<T>(&self, v: &mut Vec<T>) -> Vec<T>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::groups_accumulator::EmitTo", "path": "EmitTo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [51, 2], "filename": "src/groups_accumulator.rs"}, "trait": null, "trait_path": null}`

Source: `src/groups_accumulator.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Removes the number of rows from `v` required to emit the right
number of rows, returning a `Vec` with elements taken, and the
remaining values in `v`.

This avoids copying if Self::All
