# `deltalake_core::open_table_with_ds`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.open_table_with_ds.json).

<a id="op-c24c8e572a52cd9624dd730b"></a>
## open_table_with_ds

`function` · `deltalake_core::open_table_with_ds` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn open_table_with_ds(table_url: url::Url, ds: impl AsRef<str>) -> Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L173).

Source: `crates/core/src/lib.rs:173`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a DeltaTable from the given URL.

Loads metadata from the version appropriate based on the given ISO-8601/RFC-3339 timestamp.
Infers the storage backend to use from the scheme in the given table URL.

Will fail fast if specified `table_url` is a local path but doesn't exist.
