# `deltalake_core::kernel::transaction::PostCommit`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.PostCommit.json).

<a id="op-0be7f31d7c260246c256d378"></a>
## PostCommit

`struct` · `deltalake_core::kernel::transaction::PostCommit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PostCommit
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1039).

Source: `crates/core/src/kernel/transaction/mod.rs:1039`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents items for the post commit hook

<a id="op-8e1f6aab60ebd7a6796e91da"></a>
## IntoFuture

`assoc_type` · `deltalake_core::kernel::transaction::PostCommit::IntoFuture` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoFuture = Pin<Box<dyn Future<Output = <PostCommit as IntoFuture>::Output> + Send>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1203).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::PostCommit", "path": "PostCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1201, 1], "end": [1223, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/kernel/transaction/mod.rs:1203`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac053af56f7b4e9c6fe51d14"></a>
## Output

`assoc_type` · `deltalake_core::kernel::transaction::PostCommit::Output` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<FinalizedCommit, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1202).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::PostCommit", "path": "PostCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1201, 1], "end": [1223, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/kernel/transaction/mod.rs:1202`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9e2061d6e2509c5ecd91db8"></a>
## data

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::data` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data: CommitData
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1043).

Source: `crates/core/src/kernel/transaction/mod.rs:1043`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The data that was committed to the log store

<a id="op-cd9876b44c105940682f114c"></a>
## into_future

`function` · `deltalake_core::kernel::transaction::PostCommit::into_future` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_future(self) -> Self::IntoFuture
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1205).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::PostCommit", "path": "PostCommit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1201, 1], "end": [1223, 2], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::future::into_future::IntoFuture", "path": "IntoFuture"}, "trait_path": "core::future::into_future::IntoFuture"}`

Source: `crates/core/src/kernel/transaction/mod.rs:1205`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43d9461498688f322c5c5528"></a>
## version

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
version: kernel::Version
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1041).

Source: `crates/core/src/kernel/transaction/mod.rs:1041`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The winning version number of the commit

<a id="op-1dae438591f9ef933e113482"></a>
## cleanup_expired_logs

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::cleanup_expired_logs` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
cleanup_expired_logs: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1045).

Source: `crates/core/src/kernel/transaction/mod.rs:1045`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4e37cc08993be58a91876e3"></a>
## create_checkpoint

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::create_checkpoint` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
create_checkpoint: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1044).

Source: `crates/core/src/kernel/transaction/mod.rs:1044`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bd2e87a10f2481655e3a456"></a>
## custom_execute_handler

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::custom_execute_handler` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
custom_execute_handler: Option<std::sync::Arc<dyn CustomExecuteHandler>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1048).

Source: `crates/core/src/kernel/transaction/mod.rs:1048`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5ed0e0e1a6b6950994c2ab5"></a>
## log_store

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1046).

Source: `crates/core/src/kernel/transaction/mod.rs:1046`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f0340040c60be4efde848a"></a>
## metrics

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::metrics` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metrics: CommitMetrics
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1049).

Source: `crates/core/src/kernel/transaction/mod.rs:1049`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44b70af9b5e4ee605d8e0f9f"></a>
## table_data

`struct_field` · `deltalake_core::kernel::transaction::PostCommit::table_data` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
table_data: Option<Box<dyn TableReference>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L1047).

Source: `crates/core/src/kernel/transaction/mod.rs:1047`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
