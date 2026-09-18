# `deltalake_core::kernel::transaction::TransactionError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.TransactionError.json).

<a id="op-c23a31a79225899dcc5279f3"></a>
## TransactionError

`enum` · `deltalake_core::kernel::transaction::TransactionError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum TransactionError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L172).

Source: `crates/core/src/kernel/transaction/mod.rs:172`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error raised while commititng transaction

<a id="op-e62512c311157c4466ed38f3"></a>
## CommitConflict

`variant` · `deltalake_core::kernel::transaction::TransactionError::CommitConflict` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CommitConflict
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L194).

Source: `crates/core/src/kernel/transaction/mod.rs:194`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when a commit conflict occurred

<a id="op-3a0f13507416a66d905e1985"></a>
## DeltaTableAppendOnly

`variant` · `deltalake_core::kernel::transaction::TransactionError::DeltaTableAppendOnly` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DeltaTableAppendOnly
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L204).

Source: `crates/core/src/kernel/transaction/mod.rs:204`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The transaction includes Remove action with data change but Delta table is append-only

<a id="op-ecc3042ab514766512c5acd5"></a>
## LogStoreError

`variant` · `deltalake_core::kernel::transaction::TransactionError::LogStoreError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LogStoreError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L217).

Source: `crates/core/src/kernel/transaction/mod.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The transaction failed to commit due to an error in an implementation-specific layer.
Currently used by DynamoDb-backed S3 log store when database operations fail.

<a id="op-42330fb1809dfed188b7fddd"></a>
## MaxCommitAttempts

`variant` · `deltalake_core::kernel::transaction::TransactionError::MaxCommitAttempts` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MaxCommitAttempts
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L198).

Source: `crates/core/src/kernel/transaction/mod.rs:198`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when maximum number of commit trioals is exceeded

<a id="op-052d9defd561c82dbc4d3719"></a>
## ObjectStore

`variant` · `deltalake_core::kernel::transaction::TransactionError::ObjectStore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ObjectStore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L186).

Source: `crates/core/src/kernel/transaction/mod.rs:186`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when reading the delta log object failed.

<a id="op-9134fc1c8eac4235167100e8"></a>
## SerializeLogJson

`variant` · `deltalake_core::kernel::transaction::TransactionError::SerializeLogJson` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SerializeLogJson
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L179).

Source: `crates/core/src/kernel/transaction/mod.rs:179`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when reading the delta log object failed.

<a id="op-77927d235354843918a3ca1a"></a>
## TableFeaturesRequired

`variant` · `deltalake_core::kernel::transaction::TransactionError::TableFeaturesRequired` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TableFeaturesRequired
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L212).

Source: `crates/core/src/kernel/transaction/mod.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when table features are required but not specified

<a id="op-1757dd400a152b3a54ffee73"></a>
## UnsupportedTableFeatures

`variant` · `deltalake_core::kernel::transaction::TransactionError::UnsupportedTableFeatures` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnsupportedTableFeatures
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L208).

Source: `crates/core/src/kernel/transaction/mod.rs:208`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error returned when unsupported table features are required

<a id="op-ab277057e0fcfe546a0db97b"></a>
## VersionAlreadyExists

`variant` · `deltalake_core::kernel::transaction::TransactionError::VersionAlreadyExists` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VersionAlreadyExists
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L175).

Source: `crates/core/src/kernel/transaction/mod.rs:175`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Version already exists

<a id="op-9392bbc9c8d71a35626b3e20"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::TransactionError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::TransactionError", "path": "TransactionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 28], "end": [171, 33], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/transaction/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9124a26e1aecb7b14d49863"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::TransactionError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::TransactionError", "path": "TransactionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 10], "end": [171, 26], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/kernel/transaction/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db01806f29be7e10b5316188"></a>
## from

`function` · `deltalake_core::kernel::transaction::TransactionError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: ObjectStoreError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::TransactionError", "path": "TransactionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 9], "end": [188, 16], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::Error", "path": "Error"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/transaction/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2d87bcc63638c4b45886a20"></a>
## from

`function` · `deltalake_core::kernel::transaction::TransactionError::from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(source: CommitConflictError) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::TransactionError", "path": "TransactionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 20], "end": [194, 27], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::conflict_checker::CommitConflictError", "path": "CommitConflictError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `crates/core/src/kernel/transaction/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c35ca385b5c1f084d2c208"></a>
## source

`function` · `deltalake_core::kernel::transaction::TransactionError::source` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L171).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::TransactionError", "path": "TransactionError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [171, 10], "end": [171, 26], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/core/src/kernel/transaction/mod.rs:171`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
