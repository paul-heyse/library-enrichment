# `deltalake_core::table`

Crate `deltalake-core` · 2 public items · structured records in [`model/deltalake_core.table.json`](../model/deltalake_core.table.json)

## normalize_table_url

`function` · `deltalake_core::table::normalize_table_url`

Also reachable as `deltalake::table::normalize_table_url`

```rust
fn normalize_table_url(url: &url::Url) -> url::Url
```

Normalize a given [Url] to **always** contain a trailing slash. This is critically important
for assumptions about [Url] equivalency and more importantly for **joining** on a Url`.

This function will also remove redundant slashes in the ]Url] path which can cause other
equivalency failures

```ignore
 left.join("_delta_log"); // produces `s3://bucket/prefix/_delta_log`
 right.join("_delta_log"); // produces `s3://bucket/_delta_log`
```

---

## DeltaTable

`struct` · `deltalake_core::table::DeltaTable`

Also reachable as `deltalake::DeltaTable`, `deltalake::table::DeltaTable`, `deltalake_core::DeltaTable`

```rust
struct DeltaTable
```

**Fields**: `state`, `config`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug

**Methods** (44)

```rust
fn add_columns(self) -> AddColumnBuilder
fn add_constraint(self) -> ConstraintBuilder
fn add_feature(self) -> AddTableFeatureBuilder
fn create(&self) -> CreateBuilder
fn delete(self) -> DeleteBuilder
fn drop_column_not_null(self) -> DropColumnNotNullBuilder
fn drop_constraints(self) -> DropConstraintBuilder
fn filesystem_check(self) -> FileSystemCheckBuilder
fn generate(self) -> GenerateBuilder
fn get_active_add_actions_by_partitions(&self, filters: &[FilterLiteral<'_>]) -> BoxStream<'_, DeltaResult<LogicalFileView>>
fn get_active_add_actions_by_predicate(&self, predicate: Option<PredicateRef>) -> BoxStream<'_, DeltaResult<LogicalFileView>>
fn get_file_uris(&self) -> DeltaResult<impl Iterator<Item = String> + '_>
async fn get_file_uris_by_partitions(&self, filters: &[FilterLiteral<'_>]) -> Result<Vec<String>, DeltaTableError>
async fn get_files_by_partitions(&self, filters: &[FilterLiteral<'_>]) -> Result<Vec<Path>, DeltaTableError>
async fn get_latest_version(&self) -> Result<Version, DeltaTableError>
async fn history(&self, limit: Option<usize>) -> Result<impl Iterator<Item = CommitInfo> + use<>, DeltaTableError>
async fn load(&mut self) -> Result<(), DeltaTableError>
async fn load_version(&mut self, version: Version) -> Result<(), DeltaTableError>
async fn load_with_datetime(&mut self, datetime: DateTime<Utc>) -> Result<(), DeltaTableError>
fn log_store(&self) -> LogStoreRef
fn merge<E: Into<Expression>>(self, source: datafusion::prelude::DataFrame, predicate: E) -> MergeBuilder
fn new(log_store: LogStoreRef, config: DeltaTableConfig) -> Self
fn new_in_memory() -> Self
fn object_store(&self) -> ObjectStoreRef
fn optimize<'a>(self) -> OptimizeBuilder<'a>
fn restore(self) -> RestoreBuilder
fn scan_cdf(self) -> CdfLoadBuilder
fn scan_table(&self) -> LoadBuilder
fn set_tbl_properties(self) -> SetTablePropertiesBuilder
fn snapshot(&self) -> DeltaResult<&DeltaTableState>
fn table_provider(&self) -> TableProviderBuilder
fn table_url(&self) -> &Url
async fn try_from_url(uri: Url) -> DeltaResult<Self>
async fn try_from_url_with_storage_options(uri: Url, storage_options: HashMap<String, String>) -> DeltaResult<Self>
fn update(self) -> UpdateBuilder
fn update_datafusion_session(&self, session: &dyn Session) -> DeltaResult<()>
fn update_field_metadata(self) -> UpdateFieldMetadataBuilder
async fn update_incremental(&mut self, max_version: Option<Version>) -> Result<(), DeltaTableError>
async fn update_state(&mut self) -> Result<(), DeltaTableError>
fn update_table_metadata(self) -> UpdateTableMetadataBuilder
fn vacuum(self) -> VacuumBuilder
async fn verify_deltatable_existence(&self) -> DeltaResult<bool>
fn version(&self) -> Option<Version>
fn write(self, batches: impl IntoIterator<Item = RecordBatch>) -> WriteBuilder
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
```

In memory representation of a Delta Table

A DeltaTable is a purely logical concept that represents a dataset that can evolve over time.
To attain concrete information about a table a snapshot need to be loaded.
Most commonly this is the latest state of the table, but may also loaded for a specific
version or point in time.

---
