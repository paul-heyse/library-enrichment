# CustomExecuteHandler

`deltalake_core::operations::CustomExecuteHandler`

```rust
trait CustomExecuteHandler: Send + Sync
```

Also reachable as `deltalake::operations::CustomExecuteHandler`

Prose: [`api/deltalake_core.operations.md`](../api/deltalake_core.operations.md#customexecutehandler) · records: [`model/deltalake_core.operations.json`](../model/deltalake_core.operations.json)

## Required

Every implementation must supply these.

```rust
async fn after_post_commit_hook(&self, log_store: &LogStoreRef, file_operation: bool, operation_id: Uuid) -> DeltaResult<()>
async fn before_post_commit_hook(&self, log_store: &LogStoreRef, file_operation: bool, operation_id: Uuid) -> DeltaResult<()>
async fn post_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
async fn pre_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

## Implementors (1)

Read one before writing your own.

- `deltalake_lakefs::execute::LakeFSCustomExecuteHandler`

## Demonstrated by 1 upstream example(s)

- [`corpus/tests/it/command_update_table_metadata.rs`](../corpus/tests/it/command_update_table_metadata.rs)

## Documentation

Hook for embedding custom behavior into the lifecycle of a Delta operation.

Implementors can run arbitrary async code around an operation's execution and its post-commit
hook, e.g. to integrate external transaction coordination, metrics, or cleanup. Each callback
receives the operation's [`LogStoreRef`] and a unique `operation_id`.
