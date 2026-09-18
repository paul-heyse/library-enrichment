# `deltalake_core::delta_datafusion::table_provider`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.delta_datafusion.table_provider.json`](../model/deltalake_core.delta_datafusion.table_provider.json)

## DeltaScanConfig

`struct` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.table_provider.DeltaScanConfig.md)

Also reachable as `deltalake::delta_datafusion::DeltaScanConfig`, `deltalake_core::delta_datafusion::DeltaScanConfig`

```rust
struct DeltaScanConfig
```

**Fields**: `file_column_name`, `wrap_partition_values`, `enable_parquet_pushdown`, `schema_force_view_types`, `schema`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default

**Methods** (6)

```rust
fn new() -> Self
fn new_from_session(session: &dyn Session) -> Self
fn with_file_column_name<S: ToString>(self, name: S) -> Self
fn with_parquet_pushdown(self, pushdown: bool) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
fn with_wrap_partition_values(self, wrap: bool) -> Self
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Include additional metadata columns during a [`crate::delta_datafusion::DeltaScanNext`]

---

## DeltaScanConfigBuilder

`struct` · `deltalake_core::delta_datafusion::table_provider::DeltaScanConfigBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.table_provider.DeltaScanConfigBuilder.md)

Also reachable as `deltalake::delta_datafusion::DeltaScanConfigBuilder`, `deltalake_core::delta_datafusion::DeltaScanConfigBuilder`

```rust
struct DeltaScanConfigBuilder
```

**Derives**: Clone, Debug, Default

**Methods** (7)

```rust
fn build(&self, snapshot: &EagerSnapshot) -> DeltaResult<DeltaScanConfig>
fn new() -> Self
fn with_file_column(self, include: bool) -> Self
fn with_file_column_name<S: ToString>(self, name: &S) -> Self
fn with_parquet_pushdown(self, pushdown: bool) -> Self
fn with_schema(self, schema: SchemaRef) -> Self
fn wrap_partition_values(self, wrap: bool) -> Self
```

Used to specify if additional metadata columns are exposed to the user

---

## TableProviderBuilder

`struct` · `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.delta_datafusion.table_provider.TableProviderBuilder.md)

Also reachable as `deltalake::delta_datafusion::TableProviderBuilder`, `deltalake_core::delta_datafusion::TableProviderBuilder`

```rust
struct TableProviderBuilder
```

**Implements**: `core::future::into_future::IntoFuture`

**Derives**: Debug, Default

**Methods** (10)

```rust
async fn build(self) -> Result<next::DeltaScan>
fn with_adds(self, adds: impl IntoIterator<Item = Add>) -> Self
fn with_eager_snapshot(self, snapshot: impl Into<Arc<EagerSnapshot>>) -> Self
fn with_file_column(self, file_column: impl ToString) -> Self
fn with_file_paths(self, paths: impl IntoIterator<Item = impl Into<String>>) -> Self
fn with_file_selection(self, selection: next::FileSelection) -> Self
fn with_log_store(self, log_store: impl Into<Arc<dyn LogStore>>) -> Self
fn with_session<S>(self, session: Arc<S>) -> Self where S: Session + 'static
fn with_snapshot(self, snapshot: impl Into<Arc<Snapshot>>) -> Self
fn with_table_version(self, version: impl Into<Option<Version>>) -> Self
```

**via `core::future::into_future::IntoFuture`**

```rust
fn into_future(self) -> Self::IntoFuture
```

Builder for a datafusion [TableProvider] for a Delta table

A table provider can be built by providing either a log store, a Snapshot,
or an eager snapshot. If some Snapshot is provided, that will be used directly,
and no IO will be performed when building the provider.

---
