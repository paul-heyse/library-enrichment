# `deltalake_core::operations::CustomExecuteHandler`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.CustomExecuteHandler.json).

<a id="op-400721e2bb8ee5c31a372fbd"></a>
## CustomExecuteHandler

`trait` · `deltalake_core::operations::CustomExecuteHandler` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait CustomExecuteHandler: Send + Sync
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L269).

Source: `crates/core/src/operations/mod.rs:269`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Hook for embedding custom behavior into the lifecycle of a Delta operation.

Implementors can run arbitrary async code around an operation's execution and its post-commit
hook, e.g. to integrate external transaction coordination, metrics, or cleanup. Each callback
receives the operation's [`LogStoreRef`](../operations/deltalake_core.logstore.LogStoreRef.md#op-1b56118828480c7f5e758964) and a unique `operation_id`.

<a id="op-09ce07a0d349e1a2f9d033e2"></a>
## after_post_commit_hook

`function` · `deltalake_core::operations::CustomExecuteHandler::after_post_commit_hook` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn after_post_commit_hook(&self, log_store: &LogStoreRef, file_operation: bool, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L285).

Source: `crates/core/src/operations/mod.rs:285`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execute arbitrary code at the end of the post commit hook.

<a id="op-e9ac2af947da07f25d252987"></a>
## before_post_commit_hook

`function` · `deltalake_core::operations::CustomExecuteHandler::before_post_commit_hook` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn before_post_commit_hook(&self, log_store: &LogStoreRef, file_operation: bool, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L277).

Source: `crates/core/src/operations/mod.rs:277`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execute arbitrary code at the start of the post commit hook.

<a id="op-14a7ec62f055d743aa18d00a"></a>
## post_execute

`function` · `deltalake_core::operations::CustomExecuteHandler::post_execute` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn post_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L274).

Source: `crates/core/src/operations/mod.rs:274`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execute arbitrary code at the end of a delta operation.

<a id="op-b2a98d663d71e28d4be1445a"></a>
## pre_execute

`function` · `deltalake_core::operations::CustomExecuteHandler::pre_execute` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn pre_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/mod.rs#L271).

Source: `crates/core/src/operations/mod.rs:271`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Execute arbitrary code at the start of a delta operation.
