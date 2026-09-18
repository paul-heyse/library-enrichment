# `deltalake_core::operations::merge::DeleteBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.merge.DeleteBuilder.json).

<a id="op-30184efca22a56495255423c"></a>
## DeleteBuilder

`struct` · `deltalake_core::operations::merge::DeleteBuilder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L537).

Source: `crates/core/src/operations/merge/mod.rs:537`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Builder for delete clauses

<a id="op-5891ab4903d646fead12926f"></a>
## default

`function` · `deltalake_core::operations::merge::DeleteBuilder::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> DeleteBuilder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L536).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [536, 10], "end": [536, 17], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/merge/mod.rs:536`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e02e907503342d42c6815d9"></a>
## predicate

`function` · `deltalake_core::operations::merge::DeleteBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn predicate<E: Into<Expression>>(self, predicate: E) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L543).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::merge::DeleteBuilder", "path": "DeleteBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [547, 2], "filename": "crates/core/src/operations/merge/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/merge/mod.rs:543`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete a record when the predicate is satisfied

<a id="op-9957a5018835cdf687b14575"></a>
## predicate

`struct_field` · `deltalake_core::operations::merge::DeleteBuilder::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<delta_datafusion::Expression>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/merge/mod.rs#L538).

Source: `crates/core/src/operations/merge/mod.rs:538`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
