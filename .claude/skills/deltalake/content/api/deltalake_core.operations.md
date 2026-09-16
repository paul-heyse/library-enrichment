# `deltalake_core::operations`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.operations.json`](../model/deltalake_core.operations.json)

## get_num_idx_cols_and_stats_columns

`function` · `deltalake_core::operations::get_num_idx_cols_and_stats_columns`

Also reachable as `deltalake::operations::get_num_idx_cols_and_stats_columns`

```rust
fn get_num_idx_cols_and_stats_columns(config: Option<&delta_kernel::table_properties::TableProperties>, configuration: std::collections::HashMap<String, Option<String>>) -> (delta_kernel::table_properties::DataSkippingNumIndexedCols, Option<Vec<String>>)
```

Get the num_idx_columns and stats_columns from the table configuration in the state
If table_config does not exist (only can occur in the first write action) it takes
the configuration that was passed to the writerBuilder.

---

## CustomExecuteHandler

`trait` · `deltalake_core::operations::CustomExecuteHandler`

Also reachable as `deltalake::operations::CustomExecuteHandler`

```rust
trait CustomExecuteHandler: Send + Sync
```

**Implementors** (1)

- `deltalake_lakefs::execute::LakeFSCustomExecuteHandler`

**Methods** (4)

```rust
async fn after_post_commit_hook(&self, log_store: &LogStoreRef, file_operation: bool, operation_id: Uuid) -> DeltaResult<()>
async fn before_post_commit_hook(&self, log_store: &LogStoreRef, file_operation: bool, operation_id: Uuid) -> DeltaResult<()>
async fn post_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
async fn pre_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

Hook for embedding custom behavior into the lifecycle of a Delta operation.

Implementors can run arbitrary async code around an operation's execution and its post-commit
hook, e.g. to integrate external transaction coordination, metrics, or cleanup. Each callback
receives the operation's [`LogStoreRef`] and a unique `operation_id`.

---
