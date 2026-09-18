# `deltalake_core::kernel::transaction::protocol::ProtocolChecker`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.protocol.ProtocolChecker.json).

<a id="op-f72d05a66a8c5e29703384ba"></a>
## ProtocolChecker

`struct` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
struct ProtocolChecker
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L68).

Source: `crates/core/src/kernel/transaction/protocol.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44ead6f0afa26e598fc49940"></a>
## can_commit

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_commit` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn can_commit(&self, snapshot: &dyn TableReference, actions: &[Action], operation: &DeltaOperation) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L260).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:260`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2197b896c252c4a717bff25"></a>
## can_read_from

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_read_from` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn can_read_from(&self, snapshot: &dyn TableReference) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L199).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check if delta-rs can read form the given delta table.

<a id="op-9a65d13a01a3236a76050780"></a>
## can_read_from_protocol

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_read_from_protocol` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn can_read_from_protocol(&self, protocol: &Protocol) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L203).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:203`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f087de5991d3e750d8260bd"></a>
## can_write_to

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_write_to` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn can_write_to(&self, snapshot: &dyn TableReference) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L231).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:231`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check if delta-rs can write to the given delta table.

<a id="op-1a167f89e08e6518c9174b76"></a>
## check_append_only

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::check_append_only` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn check_append_only(&self, snapshot: &EagerSnapshot) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L94).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check append-only at the high level (operation level)

<a id="op-fbd9de7a5e38df362c428ac4"></a>
## check_can_write_timestamp_nanos

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::check_can_write_timestamp_nanos` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn check_can_write_timestamp_nanos(&self, snapshot: &EagerSnapshot, schema: &Schema) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check can write_timestamp_nanos.
Requires both timestampNanos and timestampNtz features.

<a id="op-711af206824b39c3de948ac3"></a>
## check_can_write_timestamp_ntz

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::check_can_write_timestamp_ntz` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn check_can_write_timestamp_ntz(&self, snapshot: &EagerSnapshot, schema: &Schema) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L124).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:124`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check can write_timestamp_ntz

<a id="op-4b6ea147f2ba02168f97a25d"></a>
## check_can_write_variant

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::check_can_write_variant` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn check_can_write_variant(&self, snapshot: &EagerSnapshot, schema: &Schema) -> Result<(), TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L162).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:162`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check can write variant

<a id="op-6fdaccaa58a561021525fe4e"></a>
## default_reader_version

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::default_reader_version` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn default_reader_version(&self) -> i32
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L85).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-666cfb37b1c1867c4121a26f"></a>
## default_writer_version

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::default_writer_version` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn default_writer_version(&self) -> i32
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d3a4da864a1b1f6318c617d"></a>
## new

`function` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::new` · deltalake-core 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn new(reader_features: HashSet<TableFeature>, writer_features: HashSet<TableFeature>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L75).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::protocol::ProtocolChecker", "path": "ProtocolChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [299, 2], "filename": "crates/core/src/kernel/transaction/protocol.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/protocol.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new protocol checker.

<a id="op-e9fe72f19b2d5acd243c8f20"></a>
## reader_features

`struct_field` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::reader_features` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
reader_features: std::collections::HashSet<delta_kernel::table_features::TableFeature>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L69).

Source: `crates/core/src/kernel/transaction/protocol.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-771ba5f8cfc080c98c4ed73d"></a>
## writer_features

`struct_field` · `deltalake_core::kernel::transaction::protocol::ProtocolChecker::writer_features` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
writer_features: std::collections::HashSet<delta_kernel::table_features::TableFeature>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/protocol.rs#L70).

Source: `crates/core/src/kernel/transaction/protocol.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
