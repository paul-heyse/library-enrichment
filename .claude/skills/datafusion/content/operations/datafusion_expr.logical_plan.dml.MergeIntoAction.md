# `datafusion_expr::logical_plan::dml::MergeIntoAction`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.MergeIntoAction.json).

<a id="op-decb14bc0312044834d43abf"></a>
## MergeIntoAction

`enum` · `datafusion_expr::logical_plan::dml::MergeIntoAction` · datafusion-expr 55.1.0

```rust
enum MergeIntoAction
```

Source: `src/logical_plan/dml.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The action for a single WHEN clause.

<a id="op-58c609d4b8ecfae9bbc0e224"></a>
## Delete

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoAction::Delete` · datafusion-expr 55.1.0

```rust
Delete
```

Source: `src/logical_plan/dml.rs:488`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80122b0bb157a5b2c0b475ef"></a>
## Insert

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoAction::Insert` · datafusion-expr 55.1.0

```rust
Insert
```

Source: `src/logical_plan/dml.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`INSERT (col1, col2, ...) VALUES (expr1, expr2, ...)`. `columns` may
be empty, meaning all columns.

<a id="op-e1225094717b494fe2c468df"></a>
## Update

`variant` · `datafusion_expr::logical_plan::dml::MergeIntoAction::Update` · datafusion-expr 55.1.0

```rust
Update
```

Source: `src/logical_plan/dml.rs:481`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`UPDATE SET col1 = expr1, col2 = expr2, ...`, stored as
`(column_name, value_expr)` pairs.

<a id="op-06a0504fb504dc242b89cbdc"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::MergeIntoAction::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> MergeIntoAction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoAction", "path": "MergeIntoAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 17], "end": [477, 22], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d81179b93fd390ca5ffbb97f"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::MergeIntoAction::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &MergeIntoAction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoAction", "path": "MergeIntoAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 24], "end": [477, 33], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-976e89d7c4042cf617c234c0"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::MergeIntoAction::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoAction", "path": "MergeIntoAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 10], "end": [477, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3886e52fc48973e8af41508c"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::MergeIntoAction::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoAction", "path": "MergeIntoAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 51], "end": [477, 55], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db9e5efec8baef9e2d5b715e"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::MergeIntoAction::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &MergeIntoAction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoAction", "path": "MergeIntoAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [477, 39], "end": [477, 49], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
