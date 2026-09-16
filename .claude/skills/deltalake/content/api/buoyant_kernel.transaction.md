# `buoyant_kernel::transaction`

Crate `buoyant_kernel` · 10 public items · structured records in [`model/buoyant_kernel.transaction.json`](../model/buoyant_kernel.transaction.json)

## CommitResult

`enum` · `buoyant_kernel::transaction::CommitResult`

Also reachable as `delta_kernel::transaction::CommitResult`

```rust
enum CommitResult<S = ExistingTable>
```

**Variants**: `CommittedTransaction`, `ConflictedTransaction`, `RetryableTransaction`

**Derives**: Debug

**Methods** (1)

```rust
fn is_committed(&self) -> bool
```

The result of attempting to commit this transaction. If the commit was
successful/conflicted/retryable, the result is Ok(CommitResult), otherwise, if a nonrecoverable
error occurred, the result is Err(Error).

The commit result can be one of the following:
- [CommittedTransaction]: the transaction was successfully committed. [PostCommitStats] and in
  the future a post-commit snapshot can be obtained from the committed transaction.
- [ConflictedTransaction]: the transaction conflicted with an existing version. This transcation
  must be rebased before retrying. (currently no rebase APIs exist, caller must create new txn)
- [RetryableTransaction]: an IO (retryable) error occurred during the commit. This transaction
  can be retried without rebasing.

---

## AlterTable

`struct` · `buoyant_kernel::transaction::AlterTable`

Also reachable as `delta_kernel::transaction::AlterTable`

```rust
struct AlterTable
```

**Derives**: Debug

Marker type for alter-table (schema evolution) transactions.

Transactions in this state perform metadata-only commits. Data file operations are not
available at compile time because `AlterTable` does not implement [`SupportsDataFiles`].

---

## CommittedTransaction

`struct` · `buoyant_kernel::transaction::CommittedTransaction`

Also reachable as `delta_kernel::transaction::CommittedTransaction`

```rust
struct CommittedTransaction
```

**Derives**: Debug

**Methods** (3)

```rust
fn commit_version(&self) -> Version
fn post_commit_snapshot(&self) -> Option<&SnapshotRef>
fn post_commit_stats(&self) -> &PostCommitStats
```

This is the result of a successfully committed [Transaction]. One can retrieve the
[post_commit_stats], [commit version], and optionally the [post-commit snapshot] from this
struct.

[post_commit_stats]: Self::post_commit_stats
[commit version]: Self::commit_version
[post-commit snapshot]: Self::post_commit_snapshot

---

## ConflictedTransaction

`struct` · `buoyant_kernel::transaction::ConflictedTransaction`

Also reachable as `delta_kernel::transaction::ConflictedTransaction`

```rust
struct ConflictedTransaction<S = ExistingTable>
```

**Derives**: Debug

**Methods** (1)

```rust
fn conflict_version(&self) -> Version
```

This is the result of a conflicted [Transaction]. One can retrieve the [conflict version] from
this struct. In the future a rebase API will be provided (issue #1389).

[conflict version]: Self::conflict_version

---

## CreateTable

`struct` · `buoyant_kernel::transaction::CreateTable`

Also reachable as `delta_kernel::transaction::CreateTable`

```rust
struct CreateTable
```

**Implements**: `buoyant_kernel::transaction::SupportsDataFiles`

**Derives**: Debug

Marker type for create-table transactions.

Transactions in this state have a restricted API surface — operations that are semantically
invalid for table creation (e.g. file removal, domain metadata removal) are not available.

---

## ExistingTable

`struct` · `buoyant_kernel::transaction::ExistingTable`

Also reachable as `delta_kernel::transaction::ExistingTable`

```rust
struct ExistingTable
```

**Implements**: `buoyant_kernel::transaction::SupportsDataFiles`

**Derives**: Debug

Marker type for transactions on existing tables.

This is the default state for [`Transaction`] and provides the full set of operations
including file removal, deletion vector updates, and blind append semantics.

---

## PostCommitStats

`struct` · `buoyant_kernel::transaction::PostCommitStats`

Also reachable as `delta_kernel::transaction::PostCommitStats`

```rust
struct PostCommitStats
```

**Fields**: `commits_since_checkpoint`, `commits_since_log_compaction`

**Derives**: Debug

Kernel exposes information about the state of the table that engines might want to use to
trigger actions like checkpointing or log compaction. This struct holds that information.

---

## RetryableTransaction

`struct` · `buoyant_kernel::transaction::RetryableTransaction`

Also reachable as `delta_kernel::transaction::RetryableTransaction`

```rust
struct RetryableTransaction<S = ExistingTable>
```

**Fields**: `transaction`, `error`

**Derives**: Debug

A transaction that failed to commit due to a retryable error (e.g. IO error). The transaction
can be recovered with `RetryableTransaction::transaction` and retried without rebasing. The
associated error can be inspected via `RetryableTransaction::error`.

---

## Transaction

`struct` · `buoyant_kernel::transaction::Transaction`

Also reachable as `delta_kernel::transaction::Transaction`

```rust
struct Transaction<S = ExistingTable>
```

**Derives**: Debug

**Methods** (21)

```rust
fn add_files(&mut self, add_metadata: Box<dyn EngineData>)
fn add_files_schema(&self) -> &'static SchemaRef
fn commit(self, engine: &dyn Engine) -> DeltaResult<CommitResult<S>>
fn logical_partition_columns(&self) -> &[String]
fn partitioned_write_context(&self, partition_values: HashMap<String, Scalar>) -> DeltaResult<WriteContext>
fn remove_files(&mut self, remove_metadata: FilteredEngineData)
fn scan_metadata_to_engine_data(scan_metadata: impl Iterator<Item = DeltaResult<scan::ScanMetadata>>) -> impl Iterator<Item = DeltaResult<FilteredEngineData>>
fn set_data_change(&mut self, data_change: bool)
fn stats_columns(&self) -> Vec<ColumnName>
fn stats_schema(&self) -> DeltaResult<SchemaRef>
fn unpartitioned_write_context(&self) -> DeltaResult<WriteContext>
fn update_deletion_vectors(&mut self, new_dv_descriptors: HashMap<String, DeletionVectorDescriptor>, existing_data_files: impl Iterator<Item = DeltaResult<FilteredEngineData>>) -> DeltaResult<()>
fn with_blind_append(self) -> Self
fn with_commit_info(self, engine_commit_info: Box<dyn EngineData>, commit_info_schema: SchemaRef) -> Self
fn with_correlation_id(self, correlation_id: impl Into<Arc<str>>) -> Self
fn with_data_change(self, data_change: bool) -> Self
fn with_domain_metadata(self, domain: String, configuration: String) -> Self
fn with_domain_metadata_removed(self, domain: String) -> Self
fn with_engine_info(self, engine_info: impl Into<String>) -> Self
fn with_operation(self, operation: String) -> Self
fn with_transaction_id(self, app_id: String, version: i64) -> Self
```

A transaction represents an in-progress write to a table. After creating a transaction, changes
to the table may be staged via the transaction methods before calling `commit` to commit the
changes to the table.

The type parameter `S` controls which operations are available:
- [`ExistingTable`] (default): Full API for modifying existing tables.
- [`CreateTable`]: Restricted API for table creation (see
  [`CreateTableTransaction`](create_table::CreateTableTransaction)).

# Examples

```rust,ignore
// create a transaction
let mut txn = table.new_transaction(&engine)?;
// stage table changes (right now only commit info)
txn.commit_info(Box::new(ArrowEngineData::new(engine_commit_info)));
// commit! (consume the transaction)
txn.commit(&engine)?;
```

---

## SupportsDataFiles

`trait` · `buoyant_kernel::transaction::SupportsDataFiles`

Also reachable as `delta_kernel::transaction::SupportsDataFiles`

```rust
trait SupportsDataFiles
```

**Implementors** (2)

- `buoyant_kernel::transaction::CreateTable`
- `buoyant_kernel::transaction::ExistingTable`

Marker trait for transaction states that support data file operations.

Only transaction types that implement this trait can access methods for adding, removing, or
updating data files. This prevents compile-time misuse by states like `AlterTable` that
only perform metadata-only commits.

---
