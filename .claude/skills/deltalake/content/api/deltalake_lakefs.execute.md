# `deltalake_lakefs::execute`

Crate `deltalake-lakefs` · 1 public items · structured records in [`model/deltalake_lakefs.execute.json`](../model/deltalake_lakefs.execute.json)

## LakeFSCustomExecuteHandler

`struct` · `deltalake_lakefs::execute::LakeFSCustomExecuteHandler`
[Full member contracts, output types and access classification](../operations/deltalake_lakefs.execute.LakeFSCustomExecuteHandler.md)

Also reachable as `deltalake::lakefs::LakeFSCustomExecuteHandler`, `deltalake_lakefs::LakeFSCustomExecuteHandler`

```rust
struct LakeFSCustomExecuteHandler
```

**Implements**: `deltalake_core::operations::CustomExecuteHandler`

**via `deltalake_core::operations::CustomExecuteHandler`**

```rust
async fn after_post_commit_hook(&self, log_store: &LogStoreRef, file_operations: bool, operation_id: Uuid) -> DeltaResult<()>
async fn before_post_commit_hook(&self, log_store: &LogStoreRef, file_operations: bool, operation_id: Uuid) -> DeltaResult<()>
async fn post_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
async fn pre_execute(&self, log_store: &LogStoreRef, operation_id: Uuid) -> DeltaResult<()>
```

---
