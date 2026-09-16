# `deltalake_core::delta_datafusion::table_provider::next`

Crate `deltalake-core` · 5 public items · structured records in [`model/deltalake_core.delta_datafusion.table_provider.next.json`](../model/deltalake_core.delta_datafusion.table_provider.next.json)

## MissingSelectedFilePolicy

`enum` · `deltalake_core::delta_datafusion::table_provider::next::MissingSelectedFilePolicy`

Also reachable as `deltalake::delta_datafusion::MissingSelectedFilePolicy`, `deltalake_core::delta_datafusion::MissingSelectedFilePolicy`

```rust
enum MissingSelectedFilePolicy
```

**Variants**: `Error`, `Ignore`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Policy for selected files that are not active in a scan snapshot.

---

## SnapshotWrapper

`enum` · `deltalake_core::delta_datafusion::table_provider::next::SnapshotWrapper`

```rust
enum SnapshotWrapper
```

**Variants**: `Snapshot`, `EagerSnapshot`

**Implements**: `core::convert::From`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(esnap: EagerSnapshot) -> Self
fn from(snap: Arc<Snapshot>) -> Self
fn from(esnap: Arc<EagerSnapshot>) -> Self
fn from(snap: Snapshot) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## DeletionVectorSelection

`struct` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection`

Also reachable as `deltalake::delta_datafusion::DeletionVectorSelection`, `deltalake_core::delta_datafusion::DeletionVectorSelection`

```rust
struct DeletionVectorSelection
```

**Fields**: `filepath`, `keep_mask`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

Deletion vector selection for one data file.

---

## DeltaScan

`struct` · `deltalake_core::delta_datafusion::table_provider::next::DeltaScan`

Also reachable as `deltalake::delta_datafusion::DeltaScanNext`, `deltalake_core::delta_datafusion::DeltaScanNext`

```rust
struct DeltaScan
```

**Implements**: `datafusion_session::table::TableProvider`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn builder() -> TableProviderBuilder
async fn deletion_vectors(&self, session: &dyn Session) -> Result<Vec<DeletionVectorSelection>>
fn new(snapshot: impl Into<SnapshotWrapper>, config: DeltaScanConfig) -> Result<Self>
fn with_adds(self, adds: impl IntoIterator<Item = Add>) -> Self
fn with_file_paths(self, paths: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_file_selection(self, selection: FileSelection) -> Self
```

**via `datafusion_session::table::TableProvider`**

```rust
fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>>
fn get_table_definition(&self) -> Option<&str>
async fn insert_into(&self, state: &dyn Session, input: Arc<dyn ExecutionPlan>, insert_op: InsertOp) -> Result<Arc<dyn ExecutionPlan>>
async fn scan(&self, session: &dyn Session, projection: Option<&Vec<usize>>, filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>>
fn schema(&self) -> SchemaRef
fn supports_filters_pushdown(&self, filter: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>>
fn table_type(&self) -> TableType
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

An executable, serializable Delta table scan.

`DeltaScan` captures everything needed to read a consistent set of data files from a
table snapshot — the resolved schemas, optional file-skipping predicates, deletion-vector
aware file selection and the originating log store. It is the unit produced by
[`TableProviderBuilder`] and consumed by DataFusion's execution layer.

---

## FileSelection

`struct` · `deltalake_core::delta_datafusion::table_provider::next::FileSelection`

Also reachable as `deltalake::delta_datafusion::FileSelection`, `deltalake_core::delta_datafusion::FileSelection`

```rust
struct FileSelection
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn from_adds(adds: impl IntoIterator<Item = Add>) -> Self
fn from_file_paths(paths: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_missing_file_policy(self, policy: MissingSelectedFilePolicy) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

File selection for a [`DeltaScan`] snapshot.

A selection limits a scan to explicit files, for example paths returned by
[`crate::DeltaTable::get_files_by_partitions`] or Add actions produced by maintenance tasks.

The input identifies files. Metadata comes from the scan snapshot, including
deletion vectors, partition values, statistics, column mapping, and tags.

Empty selections produce empty scans. Duplicate inputs are deduplicated after the scan snapshot
is known. Add inputs contribute only their path. The default policy returns an error for files
that are not active in the scan snapshot. [`MissingSelectedFilePolicy::Ignore`] skips those
files. Query pruning does not mark a selected file as missing.

Absolute URLs must be under the table root. Paths outside the table root are rejected during
scan planning with redacted error output.
Username, password, query, and fragment are stripped from URLs before storage.

Selections with paths are resolved against snapshot metadata before the data scan. Each scan
with a file selection requires that metadata pass.

---
