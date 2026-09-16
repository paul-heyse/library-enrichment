# `deltalake_core`

Crate `deltalake-core` · 6 public items · structured records in [`model/deltalake_core.json`](../model/deltalake_core.json)

## crate_version

`function` · `deltalake_core::crate_version`

Also reachable as `deltalake::crate_version`

```rust
fn crate_version() -> &'static str
```

Returns Rust core version or custom set client_version such as the py-binding

```
// Always returns a non-empty version string.
assert!(!deltalake_core::crate_version().is_empty());
```

---

## init_client_version

`function` · `deltalake_core::init_client_version`

Also reachable as `deltalake::init_client_version`

```rust
fn init_client_version(version: &str)
```

Record the client version reported by `crate_version`.

Bindings (such as the Python package) call this once at startup so that user agents and
telemetry identify the embedding client rather than the bare core crate version. Subsequent
calls after the first are ignored.

---

## open_table

`function` · `deltalake_core::open_table`

Also reachable as `deltalake::open_table`

```rust
async fn open_table(table_url: url::Url) -> Result<DeltaTable, DeltaTableError>
```

Creates and loads a DeltaTable from the given URL with current metadata.
Infers the storage backend to use from the scheme in the given table URL.

Will fail fast if specified `table_url` is a local path but doesn't exist.

---

## open_table_with_ds

`function` · `deltalake_core::open_table_with_ds`

Also reachable as `deltalake::open_table_with_ds`

```rust
async fn open_table_with_ds(table_url: url::Url, ds: impl AsRef<str>) -> Result<DeltaTable, DeltaTableError>
```

Creates a DeltaTable from the given URL.

Loads metadata from the version appropriate based on the given ISO-8601/RFC-3339 timestamp.
Infers the storage backend to use from the scheme in the given table URL.

Will fail fast if specified `table_url` is a local path but doesn't exist.

---

## open_table_with_storage_options

`function` · `deltalake_core::open_table_with_storage_options`

Also reachable as `deltalake::open_table_with_storage_options`

```rust
async fn open_table_with_storage_options(table_url: url::Url, storage_options: std::collections::HashMap<String, String>) -> Result<DeltaTable, DeltaTableError>
```

Same as `open_table`, but also accepts storage options to aid in building the table for a deduced
`StorageService`.

Will fail fast if specified `table_url` is a local path but doesn't exist.

---

## open_table_with_version

`function` · `deltalake_core::open_table_with_version`

Also reachable as `deltalake::open_table_with_version`

```rust
async fn open_table_with_version(table_url: url::Url, version: kernel::Version) -> Result<DeltaTable, DeltaTableError>
```

Creates a DeltaTable from the given URL and loads it with the metadata from the given version.
Infers the storage backend to use from the scheme in the given table URL.

Will fail fast if specified `table_url` is a local path but doesn't exist.

---
