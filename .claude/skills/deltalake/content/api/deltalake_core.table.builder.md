# `deltalake_core::table::builder`

Crate `deltalake-core` · 6 public items · structured records in [`model/deltalake_core.table.builder.json`](../model/deltalake_core.table.builder.json)

## DeltaTableConfigKey

`enum` · `deltalake_core::table::builder::DeltaTableConfigKey`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.builder.DeltaTableConfigKey.md)

Also reachable as `deltalake::table::builder::DeltaTableConfigKey`

```rust
enum DeltaTableConfigKey
```

**Variants**: `RequireFiles`, `LogBufferSize`, `LogBatchSize`, `SkipStats`, `IoRuntime`

Enumeration of recognized configuration keys, generated from the struct fields.

---

## DeltaVersion

`enum` · `deltalake_core::table::builder::DeltaVersion`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.builder.DeltaVersion.md)

Also reachable as `deltalake::DeltaVersion`, `deltalake::table::builder::DeltaVersion`, `deltalake_core::DeltaVersion`

```rust
enum DeltaVersion
```

**Variants**: `Newest`, `Version`, `Timestamp`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

possible version specifications for loading a delta table

---

## ensure_table_uri

`function` · `deltalake_core::table::builder::ensure_table_uri`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.builder.ensure_table_uri.md)

Also reachable as `deltalake::ensure_table_uri`, `deltalake::table::builder::ensure_table_uri`, `deltalake_core::ensure_table_uri`

```rust
fn ensure_table_uri(table_uri: impl AsRef<str>) -> DeltaResult<url::Url>
```

Will return an error if the location is not valid. For example,
Creates directories for local paths if they don't exist.

---

## parse_table_uri

`function` · `deltalake_core::table::builder::parse_table_uri`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.builder.parse_table_uri.md)

Also reachable as `deltalake::table::builder::parse_table_uri`

```rust
fn parse_table_uri(table_uri: impl AsRef<str>) -> DeltaResult<url::Url>
```

Attempt to create a Url from given table location.

The location could be:
 * A valid URL, which will be parsed and returned
 * A path to a directory, which will be created and then converted to a URL.

Extra slashes will be removed from the end path as well.

Parse a table URI to a URL without creating directories.
This is useful for opening existing tables where we don't want to create directories.

---

## DeltaTableBuilder

`struct` · `deltalake_core::table::builder::DeltaTableBuilder`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.builder.DeltaTableBuilder.md)

Also reachable as `deltalake::DeltaTableBuilder`, `deltalake::table::builder::DeltaTableBuilder`, `deltalake_core::DeltaTableBuilder`

```rust
struct DeltaTableBuilder
```

**Derives**: Debug

**Methods** (15)

```rust
fn build(self) -> DeltaResult<DeltaTable>
fn build_storage(&self) -> DeltaResult<LogStoreRef>
fn from_url(table_url: Url) -> DeltaResult<Self>
async fn load(self) -> DeltaResult<DeltaTable>
fn storage_options(&self) -> HashMap<String, String>
fn with_allow_http(self, allow_http: bool) -> Self
fn with_datestring(self, date_string: impl AsRef<str>) -> DeltaResult<Self>
fn with_io_runtime(self, io_runtime: IORuntime) -> Self
fn with_log_buffer_size(self, log_buffer_size: usize) -> DeltaResult<Self>
fn with_skip_stats(self, skip_stats: bool) -> Self
fn with_storage_backend(self, root_storage: Arc<DynObjectStore>, location: Url) -> Self
fn with_storage_options(self, storage_options: HashMap<String, String>) -> Self
fn with_timestamp(self, timestamp: DateTime<Utc>) -> Self
fn with_version(self, version: Version) -> Self
fn without_files(self) -> Self
```

builder for configuring a delta table load.

---

## DeltaTableConfig

`struct` · `deltalake_core::table::builder::DeltaTableConfig`
[Full member contracts, output types and access classification](../operations/deltalake_core.table.builder.DeltaTableConfig.md)

Also reachable as `deltalake::DeltaTableConfig`, `deltalake::table::builder::DeltaTableConfig`, `deltalake_core::DeltaTableConfig`

```rust
struct DeltaTableConfig
```

**Fields**: `require_files`, `log_buffer_size`, `log_batch_size`, `skip_stats`, `io_runtime`

**Implements**: `core::iter::traits::collect::FromIterator`, `deltalake_core::logstore::config::TryUpdateKey`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self
```

**via `deltalake_core::logstore::config::TryUpdateKey`**

```rust
fn load_from_environment(&mut self) -> DeltaResult<()>
fn try_update_key(&mut self, key: &str, v: &str) -> DeltaResult<Option<()>>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Configuration options for delta table

---
