# `deltalake_core::open_table_with_storage_options`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.open_table_with_storage_options.json).

<a id="op-14e3af0e23e1477e14a027e0"></a>
## open_table_with_storage_options

`function` · `deltalake_core::open_table_with_storage_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn open_table_with_storage_options(table_url: url::Url, storage_options: std::collections::HashMap<String, String>) -> Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L141).

Source: `crates/core/src/lib.rs:141`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Same as `open_table`, but also accepts storage options to aid in building the table for a deduced
`StorageService`.

Will fail fast if specified `table_url` is a local path but doesn't exist.
