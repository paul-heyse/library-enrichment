# `deltalake_lakefs::execute::LakeFSCustomExecuteHandler`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.execute.LakeFSCustomExecuteHandler.json).

<a id="op-a0ca5ff422178e1f4718b8e1"></a>
## LakeFSCustomExecuteHandler

`struct` · `deltalake_lakefs::execute::LakeFSCustomExecuteHandler` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct LakeFSCustomExecuteHandler
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/execute.rs#L12).

Source: `crates/lakefs/src/execute.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9983ad377fa7b3c532f8a81e"></a>
## after_post_commit_hook

`function` · `deltalake_lakefs::execute::LakeFSCustomExecuteHandler::after_post_commit_hook` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn after_post_commit_hook(&self, log_store: &LogStoreRef, file_operations: bool, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/execute.rs#L70).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::execute::LakeFSCustomExecuteHandler", "path": "LakeFSCustomExecuteHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 1], "end": [89, 2], "filename": "crates/lakefs/src/execute.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::CustomExecuteHandler", "path": "CustomExecuteHandler"}, "trait_path": "deltalake_core::operations::CustomExecuteHandler"}`

Source: `crates/lakefs/src/execute.rs:70`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba41531adb1b235c50e498a7"></a>
## before_post_commit_hook

`function` · `deltalake_lakefs::execute::LakeFSCustomExecuteHandler::before_post_commit_hook` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn before_post_commit_hook(&self, log_store: &LogStoreRef, file_operations: bool, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/execute.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::execute::LakeFSCustomExecuteHandler", "path": "LakeFSCustomExecuteHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 1], "end": [89, 2], "filename": "crates/lakefs/src/execute.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::CustomExecuteHandler", "path": "CustomExecuteHandler"}, "trait_path": "deltalake_core::operations::CustomExecuteHandler"}`

Source: `crates/lakefs/src/execute.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61533a6d5d887079e145d19b"></a>
## post_execute

`function` · `deltalake_lakefs::execute::LakeFSCustomExecuteHandler::post_execute` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn post_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/execute.rs#L28).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::execute::LakeFSCustomExecuteHandler", "path": "LakeFSCustomExecuteHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 1], "end": [89, 2], "filename": "crates/lakefs/src/execute.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::CustomExecuteHandler", "path": "CustomExecuteHandler"}, "trait_path": "deltalake_core::operations::CustomExecuteHandler"}`

Source: `crates/lakefs/src/execute.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e23b8a59ccd1da6e2e720929"></a>
## pre_execute

`function` · `deltalake_lakefs::execute::LakeFSCustomExecuteHandler::pre_execute` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn pre_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/execute.rs#L17).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::execute::LakeFSCustomExecuteHandler", "path": "LakeFSCustomExecuteHandler"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [15, 1], "end": [89, 2], "filename": "crates/lakefs/src/execute.rs"}, "trait": {"args": null, "id": "deltalake_core::operations::CustomExecuteHandler", "path": "CustomExecuteHandler"}, "trait_path": "deltalake_core::operations::CustomExecuteHandler"}`

Source: `crates/lakefs/src/execute.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
