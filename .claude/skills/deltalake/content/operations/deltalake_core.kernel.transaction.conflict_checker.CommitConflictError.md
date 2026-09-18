# `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.conflict_checker.CommitConflictError.json).

<a id="op-98fbf5ccfb94fd8b59a30164"></a>
## CommitConflictError

`enum` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CommitConflictError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L27).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Exceptions raised during commit conflict resolution

<a id="op-388393ffb1515b302f8de85b"></a>
## ConcurrentAppend

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::ConcurrentAppend` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ConcurrentAppend
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L34).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

This exception occurs when a concurrent operation adds files in the same partition
(or anywhere in an un-partitioned table) that your operation reads. The file additions
can be caused by INSERT, DELETE, UPDATE, or MERGE operations.

<a id="op-a914d8c3840a1b3e4e8c378a"></a>
## ConcurrentDeleteDelete

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::ConcurrentDeleteDelete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ConcurrentDeleteDelete
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L48).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:48`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

This exception occurs when a concurrent operation deleted a file that your operation also deletes.
This could be caused by two concurrent compaction operations rewriting the same files.

<a id="op-760ba0887277a3e55bd53a35"></a>
## ConcurrentDeleteRead

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::ConcurrentDeleteRead` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ConcurrentDeleteRead
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L41).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

This exception occurs when a concurrent operation deleted a file that your operation read.
Common causes are a DELETE, UPDATE, or MERGE operation that rewrites files.

<a id="op-9b5fd4e3b41f7c59d4b92bbb"></a>
## ConcurrentTransaction

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::ConcurrentTransaction` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ConcurrentTransaction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L59).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:59`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If a streaming query using the same checkpoint location is started multiple times concurrently
and tries to write to the Delta table at the same time. You should never have two streaming
queries use the same checkpoint location and run at the same time.

<a id="op-6c0726eb44e0ae573ea16b85"></a>
## CorruptedState

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::CorruptedState` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CorruptedState
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L79).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the snapshot has missing or corrupted data

<a id="op-9003d568dd5c32e59989a7c3"></a>
## MetadataChanged

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::MetadataChanged` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MetadataChanged
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L53).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

This exception occurs when a concurrent transaction updates the metadata of a Delta table.
Common causes are ALTER TABLE operations or writes to your Delta table that update the schema of the table.

<a id="op-c4d96db9c1701606a799b388"></a>
## NoMetadata

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::NoMetadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NoMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L93).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when no metadata was found in the DeltaTable.

<a id="op-14fc93a080964874b465bcb4"></a>
## Predicate

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::Predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Predicate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L86).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:86`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when evaluating predicate

<a id="op-ec29dc8c479df6a7846122e8"></a>
## ProtocolChanged

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::ProtocolChanged` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ProtocolChanged
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L67).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:67`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

This exception can occur in the following cases:
- When your Delta table is upgraded to a new version. For future operations to succeed
  you may need to upgrade your Delta Lake version.
- When multiple writers are creating or replacing a table at the same time.
- When multiple writers are writing to an empty path at the same time.

<a id="op-f12cc571314ab30e68bf90e5"></a>
## UnsupportedReaderVersion

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::UnsupportedReaderVersion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnsupportedReaderVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L75).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the table requires an unsupported writer version

<a id="op-eb3d5da3cd9ce5be46d79b4b"></a>
## UnsupportedWriterVersion

`variant` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::UnsupportedWriterVersion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnsupportedWriterVersion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L71).

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when the table requires an unsupported writer version

<a id="op-1da28f07e4bf3304aec0c9e8"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::conflict_checker::CommitConflictError", "path": "CommitConflictError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 28], "end": [26, 33], "filename": "crates/core/src/kernel/transaction/conflict_checker.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-247a8e23037dfb53ef5209c8"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::conflict_checker::CommitConflictError", "path": "CommitConflictError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 26], "filename": "crates/core/src/kernel/transaction/conflict_checker.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be2b8e28381273b82e2acc15"></a>
## source

`function` · `deltalake_core::kernel::transaction::conflict_checker::CommitConflictError::source` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/conflict_checker.rs#L26).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::conflict_checker::CommitConflictError", "path": "CommitConflictError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 26], "filename": "crates/core/src/kernel/transaction/conflict_checker.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/core/src/kernel/transaction/conflict_checker.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
