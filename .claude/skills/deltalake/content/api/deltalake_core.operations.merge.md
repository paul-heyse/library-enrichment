# `deltalake_core::operations::merge`

Crate `deltalake-core` · 5 public items · structured records in [`model/deltalake_core.operations.merge.json`](../model/deltalake_core.operations.merge.json)

## DeleteBuilder

`struct` · `deltalake_core::operations::merge::DeleteBuilder`

Also reachable as `deltalake::operations::merge::DeleteBuilder`

```rust
struct DeleteBuilder
```

**Derives**: Default

**Methods** (1)

```rust
fn predicate<E: Into<Expression>>(self, predicate: E) -> Self
```

Builder for delete clauses

---

## InsertBuilder

`struct` · `deltalake_core::operations::merge::InsertBuilder`

Also reachable as `deltalake::operations::merge::InsertBuilder`

```rust
struct InsertBuilder
```

**Derives**: Default

**Methods** (2)

```rust
fn predicate<E: Into<Expression>>(self, predicate: E) -> Self
fn set<C: Into<DeltaColumn>, E: Into<Expression>>(self, column: C, expression: E) -> Self
```

Builder for insert clauses

---

## MergeBuilder

`struct` · `deltalake_core::operations::merge::MergeBuilder`

Also reachable as `deltalake::operations::merge::MergeBuilder`

```rust
struct MergeBuilder
```

**Implements**: `core::future::into_future::IntoFuture`, `deltalake_core::operations::Operation`

**Methods** (16)

```rust
fn new<E: Into<Expression>>(log_store: LogStoreRef, snapshot: Option<EagerSnapshot>, predicate: E, source: DataFrame) -> Self
fn when_matched_delete<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(DeleteBuilder) -> DeleteBuilder
fn when_matched_update<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(UpdateBuilder) -> UpdateBuilder
fn when_not_matched_by_source_delete<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(DeleteBuilder) -> DeleteBuilder
fn when_not_matched_by_source_update<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(UpdateBuilder) -> UpdateBuilder
fn when_not_matched_insert<F>(self, builder: F) -> DeltaResult<MergeBuilder> where F: FnOnce(InsertBuilder) -> InsertBuilder
fn with_commit_properties(self, commit_properties: CommitProperties) -> Self
fn with_custom_execute_handler(self, handler: Arc<dyn CustomExecuteHandler>) -> Self
fn with_merge_schema(self, merge_schema: bool) -> Self
fn with_safe_cast(self, safe_cast: bool) -> Self
fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self
fn with_session_state(self, state: Arc<dyn Session>) -> Self
fn with_source_alias<S: ToString>(self, alias: S) -> Self
fn with_streaming(self, streaming: bool) -> Self
fn with_target_alias<S: ToString>(self, alias: S) -> Self
fn with_writer_properties(self, writer_properties: WriterProperties) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

**via `deltalake_core::operations::Operation`**

```rust
fn get_custom_execute_handler(&self) -> Option<Arc<dyn CustomExecuteHandler>>
fn log_store(&self) -> &LogStoreRef
```

Merge records into a Delta Table.

---

## MergeMetrics

`struct` · `deltalake_core::operations::merge::MergeMetrics`

Also reachable as `deltalake::operations::merge::MergeMetrics`

```rust
struct MergeMetrics
```

**Fields**: `num_source_rows`, `num_target_rows_inserted`, `num_target_rows_updated`, `num_target_rows_deleted`, `num_target_rows_copied`, `num_output_rows`, `num_target_files_scanned`, `num_target_files_skipped_during_scan`, `num_target_files_added`, `num_target_files_removed`, `execution_time_ms`, `scan_time_ms`, `rewrite_time_ms`

**Implements**: `serde_core::ser::Serialize`

**Derives**: Debug, Default

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Metrics for the Merge Operation

---

## UpdateBuilder

`struct` · `deltalake_core::operations::merge::UpdateBuilder`

Also reachable as `deltalake::operations::merge::UpdateBuilder`

```rust
struct UpdateBuilder
```

**Derives**: Default

**Methods** (2)

```rust
fn predicate<E: Into<Expression>>(self, predicate: E) -> Self
fn update<C: Into<DeltaColumn>, E: Into<Expression>>(self, column: C, expression: E) -> Self
```

Builder for update clauses

---
