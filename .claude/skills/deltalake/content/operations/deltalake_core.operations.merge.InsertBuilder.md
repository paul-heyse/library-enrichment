# `deltalake_core::operations::merge::InsertBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.InsertBuilder.json).

<a id="op-239fd22eb6560d671613f671"></a>
## InsertBuilder

`struct` · `deltalake_core::operations::merge::InsertBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct InsertBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L509).

Source: `crates/core/src/operations/merge/mod.rs:509`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for insert clauses

<a id="op-7e9b8ee66573a31670877ba3"></a>
## default

`function` · `deltalake_core::operations::merge::InsertBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> InsertBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L508).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::InsertBuilder", "path": "InsertBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [508, 10], "end": [508, 17], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/merge/mod.rs:508`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfca49b4eb26cc3212c55671"></a>
## predicate

`function` · `deltalake_core::operations::merge::InsertBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn predicate<E: Into<Expression>>(self, predicate: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L518).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::InsertBuilder", "path": "InsertBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [533, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:518`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Perform the insert operation when the predicate is satisfied

<a id="op-9d671a80c81427ffd85ab83b"></a>
## set

`function` · `deltalake_core::operations::merge::InsertBuilder::set` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn set<C: Into<DeltaColumn>, E: Into<Expression>>(self, column: C, expression: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L525).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::InsertBuilder", "path": "InsertBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 1], "end": [533, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:525`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Which values to insert into the target tables. If a target column is not
specified then null is inserted.

<a id="op-bce92859b7ef38463bcc08c3"></a>
## predicate

`struct_field` · `deltalake_core::operations::merge::InsertBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L511).

Source: `crates/core/src/operations/merge/mod.rs:511`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Only insert records that match the predicate

<a id="op-c2ebdb09a4811457d6c0ad43"></a>
## set

`struct_field` · `deltalake_core::operations::merge::InsertBuilder::set` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
set: std::collections::HashMap<datafusion::common::Column, delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L513).

Source: `crates/core/src/operations/merge/mod.rs:513`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

What value each column is inserted with
