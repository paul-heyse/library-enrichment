# `deltalake_core::operations::merge::UpdateBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.UpdateBuilder.json).

<a id="op-859ffe4a52329196a18c5d03"></a>
## UpdateBuilder

`struct` · `deltalake_core::operations::merge::UpdateBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct UpdateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L480).

Source: `crates/core/src/operations/merge/mod.rs:480`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for update clauses

<a id="op-798f3b59e368c38338f0c43d"></a>
## default

`function` · `deltalake_core::operations::merge::UpdateBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> UpdateBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L478).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 10], "end": [478, 17], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/merge/mod.rs:478`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e6155d4aa6b345c4e3dcc77"></a>
## predicate

`function` · `deltalake_core::operations::merge::UpdateBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn predicate<E: Into<Expression>>(self, predicate: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L489).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 1], "end": [505, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:489`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Perform the update operation when the predicate is satisfied

<a id="op-4980bf48ba2e8c9800e19831"></a>
## update

`function` · `deltalake_core::operations::merge::UpdateBuilder::update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update<C: Into<DeltaColumn>, E: Into<Expression>>(self, column: C, expression: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L497).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::UpdateBuilder", "path": "UpdateBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 1], "end": [505, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:497`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How a column from the target table should be updated.
In the match case the expression may contain both source and target columns.
In the source not match case the expression may only contain target columns

<a id="op-533d284940e2a233294f844e"></a>
## predicate

`struct_field` · `deltalake_core::operations::merge::UpdateBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L482).

Source: `crates/core/src/operations/merge/mod.rs:482`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only update records that match the predicate

<a id="op-800912ac30ceac7d96252d1a"></a>
## updates

`struct_field` · `deltalake_core::operations::merge::UpdateBuilder::updates` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
updates: std::collections::HashMap<datafusion::common::Column, delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L484).

Source: `crates/core/src/operations/merge/mod.rs:484`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

How to update columns in the target table
