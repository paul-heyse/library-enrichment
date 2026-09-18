# `deltalake_core::kernel::transaction`

Crate `deltalake-core` · 14 public items · structured records in [`model/deltalake_core.kernel.transaction.json`](../model/deltalake_core.kernel.transaction.json)

## CommitBuilderError

`enum` · `deltalake_core::kernel::transaction::CommitBuilderError`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.CommitBuilderError.md)

Also reachable as `deltalake::kernel::transaction::CommitBuilderError`

```rust
enum CommitBuilderError
```

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error raised while commititng transaction

---

## TransactionError

`enum` · `deltalake_core::kernel::transaction::TransactionError`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.TransactionError.md)

Also reachable as `deltalake::kernel::transaction::TransactionError`

```rust
enum TransactionError
```

**Variants**: `VersionAlreadyExists`, `SerializeLogJson`, `ObjectStore`, `CommitConflict`, `MaxCommitAttempts`, `DeltaTableAppendOnly`, `UnsupportedTableFeatures`, `TableFeaturesRequired`, `LogStoreError`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: ObjectStoreError) -> Self
fn from(source: CommitConflictError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Error raised while commititng transaction

---

## CommitBuilder

`struct` · `deltalake_core::kernel::transaction::CommitBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.CommitBuilder.md)

Also reachable as `deltalake::kernel::transaction::CommitBuilder`

```rust
struct CommitBuilder
```

**Implements**: `core::convert::From`

**Derives**: Default

**Methods** (7)

```rust
fn build(self, table_data: Option<&'a dyn TableReference>, log_store: LogStoreRef, operation: DeltaOperation) -> PreCommit<'a>
fn with_actions(self, actions: Vec<Action>) -> Self
fn with_app_metadata(self, app_metadata: HashMap<String, Value>) -> Self
fn with_max_retries(self, max_retries: usize) -> Self
fn with_operation_id(self, operation_id: Uuid) -> Self
fn with_post_commit_hook(self, post_commit_hook: PostCommitHookProperties) -> Self
fn with_post_commit_hook_handler(self, handler: Option<Arc<dyn CustomExecuteHandler>>) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: CommitProperties) -> Self
```

Prepare data to be committed to the Delta log and control how the commit is performed

---

## CommitData

`struct` · `deltalake_core::kernel::transaction::CommitData`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.CommitData.md)

Also reachable as `deltalake::kernel::transaction::CommitData`

```rust
struct CommitData
```

**Fields**: `actions`, `operation`, `app_metadata`, `app_transactions`

**Derives**: Debug

**Methods** (3)

```rust
fn get_bytes(&self) -> Result<bytes::Bytes, TransactionError>
fn new(actions: Vec<Action>, operation: DeltaOperation, app_metadata: HashMap<String, Value>, app_transactions: Vec<Transaction>) -> Self
fn update_retry_count_in_bytes(bytes: &Bytes, num_retries: u64) -> Result<Bytes, TransactionError>
```

Data that was actually written to the log store.

---

## CommitMetrics

`struct` · `deltalake_core::kernel::transaction::CommitMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.CommitMetrics.md)

Also reachable as `deltalake::kernel::transaction::CommitMetrics`

```rust
struct CommitMetrics
```

**Fields**: `num_retries`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics describing the work performed to land a single commit.

---

## CommitProperties

`struct` · `deltalake_core::kernel::transaction::CommitProperties`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.CommitProperties.md)

Also reachable as `deltalake::kernel::transaction::CommitProperties`

```rust
struct CommitProperties
```

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn with_application_transaction(self, txn: Transaction) -> Self
fn with_application_transactions(self, txn: Vec<Transaction>) -> Self
fn with_cleanup_expired_logs(self, cleanup_expired_logs: Option<bool>) -> Self
fn with_create_checkpoint(self, create_checkpoint: bool) -> Self
fn with_max_retries(self, max_retries: usize) -> Self
fn with_metadata(self, metadata: impl IntoIterator<Item = (String, serde_json::Value)>) -> Self
```

End user facing interface to be used by operations on the table.
Enable controlling commit behaviour and modifying metadata that is written during a commit.

---

## FinalizedCommit

`struct` · `deltalake_core::kernel::transaction::FinalizedCommit`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.FinalizedCommit.md)

Also reachable as `deltalake::kernel::transaction::FinalizedCommit`

```rust
struct FinalizedCommit
```

**Fields**: `snapshot`, `version`, `metrics`

**Derives**: Debug

**Methods** (2)

```rust
fn snapshot(&self) -> DeltaTableState
fn version(&self) -> Version
```

A commit that successfully completed

---

## Metrics

`struct` · `deltalake_core::kernel::transaction::Metrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.Metrics.md)

Also reachable as `deltalake::kernel::transaction::Metrics`

```rust
struct Metrics
```

**Fields**: `num_retries`, `new_checkpoint_created`, `num_log_files_cleaned_up`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Aggregate metrics for a commit, combining commit-time and post-commit measurements.

---

## PostCommit

`struct` · `deltalake_core::kernel::transaction::PostCommit`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.PostCommit.md)

Also reachable as `deltalake::kernel::transaction::PostCommit`

```rust
struct PostCommit
```

**Fields**: `version`, `data`

**Implements**: `core::future::into_future::IntoFuture`

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Represents items for the post commit hook

---

## PostCommitHookProperties

`struct` · `deltalake_core::kernel::transaction::PostCommitHookProperties`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.PostCommitHookProperties.md)

Also reachable as `deltalake::kernel::transaction::PostCommitHookProperties`

```rust
struct PostCommitHookProperties
```

**Derives**: Clone, Copy, Debug

Properties for post commit hook.

---

## PostCommitMetrics

`struct` · `deltalake_core::kernel::transaction::PostCommitMetrics`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.PostCommitMetrics.md)

Also reachable as `deltalake::kernel::transaction::PostCommitMetrics`

```rust
struct PostCommitMetrics
```

**Fields**: `new_checkpoint_created`, `num_log_files_cleaned_up`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics describing work performed by post-commit hooks (checkpointing, log cleanup).

---

## PreCommit

`struct` · `deltalake_core::kernel::transaction::PreCommit`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.PreCommit.md)

Also reachable as `deltalake::kernel::transaction::PreCommit`

```rust
struct PreCommit<'a>
```

**Implements**: `core::future::into_future::IntoFuture`

**Methods** (1)

```rust
fn into_prepared_commit_future(self) -> BoxFuture<'a, DeltaResult<PreparedCommit<'a>>>
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Represents a commit that has not yet started but all details are finalized

---

## PreparedCommit

`struct` · `deltalake_core::kernel::transaction::PreparedCommit`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.PreparedCommit.md)

Also reachable as `deltalake::kernel::transaction::PreparedCommit`

```rust
struct PreparedCommit<'a>
```

**Implements**: `core::future::into_future::IntoFuture`

**Methods** (1)

```rust
fn commit_or_bytes(&self) -> &CommitOrBytes
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Represents a inflight commit

---

## TableReference

`trait` · `deltalake_core::kernel::transaction::TableReference`
[Full member contracts, output types and access classification](../operations/deltalake_core.kernel.transaction.TableReference.md)

Also reachable as `deltalake::kernel::transaction::TableReference`

```rust
trait TableReference: Send + Sync
```

**Implementors** (2)

- `deltalake_core::kernel::snapshot::EagerSnapshot`
- `deltalake_core::table::state::DeltaTableState`

**Methods** (4)

```rust
fn config(&self) -> &TableProperties
fn eager_snapshot(&self) -> &EagerSnapshot
fn metadata(&self) -> &Metadata
fn protocol(&self) -> &Protocol
```

Reference to some structure that contains mandatory attributes for performing a commit.

---
