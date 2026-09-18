# `deltalake_core::kernel::transaction::CommitProperties`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.CommitProperties.json).

<a id="op-0dd291fe008825d5408d61a7"></a>
## CommitProperties

`struct` · `deltalake_core::kernel::transaction::CommitProperties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L550).

Source: `crates/core/src/kernel/transaction/mod.rs:550`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

End user facing interface to be used by operations on the table.
Enable controlling commit behaviour and modifying metadata that is written during a commit.

<a id="op-b42add8650847f5f685e3a00"></a>
## clone

`function` · `deltalake_core::kernel::transaction::CommitProperties::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L547).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [547, 10], "end": [547, 15], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/transaction/mod.rs:547`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e90a9fc3676aebd0555f23c3"></a>
## default

`function` · `deltalake_core::kernel::transaction::CommitProperties::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L559).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [558, 1], "end": [568, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/transaction/mod.rs:559`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-535d54195308db8474eda659"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::CommitProperties::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L547).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [547, 17], "end": [547, 22], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/transaction/mod.rs:547`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98963111c785f5e16e3f10e0"></a>
## with_application_transaction

`function` · `deltalake_core::kernel::transaction::CommitProperties::with_application_transaction` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_application_transaction(self, txn: Transaction) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L593).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 1], "end": [609, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:593`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add an additional application transaction to the commit

<a id="op-5120171ce9751caba37fe1a8"></a>
## with_application_transactions

`function` · `deltalake_core::kernel::transaction::CommitProperties::with_application_transactions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_application_transactions(self, txn: Vec<Transaction>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L599).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 1], "end": [609, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:599`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Override application transactions for the commit

<a id="op-08711506f4484afa8edb3446"></a>
## with_cleanup_expired_logs

`function` · `deltalake_core::kernel::transaction::CommitProperties::with_cleanup_expired_logs` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_cleanup_expired_logs(self, cleanup_expired_logs: Option<bool>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L605).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 1], "end": [609, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:605`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify if it should clean up the logs when the logRetentionDuration interval is met

<a id="op-5bcfe89405a3eb7b51cff29d"></a>
## with_create_checkpoint

`function` · `deltalake_core::kernel::transaction::CommitProperties::with_create_checkpoint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_create_checkpoint(self, create_checkpoint: bool) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L587).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 1], "end": [609, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:587`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify if it should create a checkpoint when the commit interval condition is met

<a id="op-521f71362ddaf4ec207f580c"></a>
## with_max_retries

`function` · `deltalake_core::kernel::transaction::CommitProperties::with_max_retries` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_max_retries(self, max_retries: usize) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L581).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 1], "end": [609, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:581`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify maximum number of times to retry the transaction before failing to commit

<a id="op-55f8ddb14dbf4d7449f3f2b1"></a>
## with_metadata

`function` · `deltalake_core::kernel::transaction::CommitProperties::with_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_metadata(self, metadata: impl IntoIterator<Item = (String, serde_json::Value)>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L572).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitProperties", "path": "CommitProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [570, 1], "end": [609, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:572`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Specify metadata the be committed

<a id="op-a1ccaad4f5545f4a09d7a648"></a>
## app_metadata

`struct_field` · `deltalake_core::kernel::transaction::CommitProperties::app_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
app_metadata: std::collections::HashMap<String, serde_json::Value>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L551).

Source: `crates/core/src/kernel/transaction/mod.rs:551`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e019ed939b3ece5eb8e9509c"></a>
## app_transaction

`struct_field` · `deltalake_core::kernel::transaction::CommitProperties::app_transaction` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
app_transaction: Vec<kernel::Transaction>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L552).

Source: `crates/core/src/kernel/transaction/mod.rs:552`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28edb07988907d3763146634"></a>
## cleanup_expired_logs

`struct_field` · `deltalake_core::kernel::transaction::CommitProperties::cleanup_expired_logs` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
cleanup_expired_logs: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L555).

Source: `crates/core/src/kernel/transaction/mod.rs:555`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de8e70dafee001a598ee3403"></a>
## create_checkpoint

`struct_field` · `deltalake_core::kernel::transaction::CommitProperties::create_checkpoint` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
create_checkpoint: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L554).

Source: `crates/core/src/kernel/transaction/mod.rs:554`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ff92784fbc7a0c74fdb4baf"></a>
## max_retries

`struct_field` · `deltalake_core::kernel::transaction::CommitProperties::max_retries` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_retries: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L553).

Source: `crates/core/src/kernel/transaction/mod.rs:553`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
