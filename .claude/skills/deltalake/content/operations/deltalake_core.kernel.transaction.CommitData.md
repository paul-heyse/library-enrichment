# `deltalake_core::kernel::transaction::CommitData`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.CommitData.json).

<a id="op-4ed2b07e63a73d9d377d13de"></a>
## CommitData

`struct` · `deltalake_core::kernel::transaction::CommitData` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct CommitData
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L303).

Source: `crates/core/src/kernel/transaction/mod.rs:303`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Data that was actually written to the log store.

<a id="op-aa9b98c1215343272a9da7f0"></a>
## actions

`struct_field` · `deltalake_core::kernel::transaction::CommitData::actions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
actions: Vec<kernel::Action>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L305).

Source: `crates/core/src/kernel/transaction/mod.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The actions

<a id="op-aec158d3d387ca1fa8f32b56"></a>
## app_metadata

`struct_field` · `deltalake_core::kernel::transaction::CommitData::app_metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
app_metadata: std::collections::HashMap<String, serde_json::Value>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L309).

Source: `crates/core/src/kernel/transaction/mod.rs:309`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The Metadata

<a id="op-5999b8170d44142882317d10"></a>
## app_transactions

`struct_field` · `deltalake_core::kernel::transaction::CommitData::app_transactions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
app_transactions: Vec<kernel::Transaction>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L311).

Source: `crates/core/src/kernel/transaction/mod.rs:311`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Application specific transaction

<a id="op-36bd2663dbe37d9ecd94a89a"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::CommitData::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L302).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitData", "path": "CommitData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 10], "end": [302, 15], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/transaction/mod.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cee775096da19d0fab00d6bb"></a>
## get_bytes

`function` · `deltalake_core::kernel::transaction::CommitData::get_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_bytes(&self) -> Result<bytes::Bytes, TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L492).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitData", "path": "CommitData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [537, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:492`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Obtain the byte representation of the commit.

<a id="op-a21fe10ccbf85163dc57a96b"></a>
## new

`function` · `deltalake_core::kernel::transaction::CommitData::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(actions: Vec<Action>, operation: DeltaOperation, app_metadata: HashMap<String, Value>, app_transactions: Vec<Transaction>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L452).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitData", "path": "CommitData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [537, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:452`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create new data to be committed

<a id="op-2890f5f7d6c4be548a0e2a98"></a>
## operation

`struct_field` · `deltalake_core::kernel::transaction::CommitData::operation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
operation: protocol::DeltaOperation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L307).

Source: `crates/core/src/kernel/transaction/mod.rs:307`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The Operation

<a id="op-9c31e7a9caa3efe37d0323ca"></a>
## update_retry_count_in_bytes

`function` · `deltalake_core::kernel::transaction::CommitData::update_retry_count_in_bytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn update_retry_count_in_bytes(bytes: &Bytes, num_retries: u64) -> Result<Bytes, TransactionError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L504).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::CommitData", "path": "CommitData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [450, 1], "end": [537, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/transaction/mod.rs:504`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update num_retries in operationMetrics within serialized bytes.
This allows updating the retry count without re-serializing all actions.
