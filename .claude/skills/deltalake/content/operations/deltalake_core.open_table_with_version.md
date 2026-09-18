# `deltalake_core::open_table_with_version`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.open_table_with_version.json).

<a id="op-a060995db0082c8b330c2848"></a>
## open_table_with_version

`function` · `deltalake_core::open_table_with_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn open_table_with_version(table_url: url::Url, version: kernel::Version) -> Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L156).

Source: `crates/core/src/lib.rs:156`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates a DeltaTable from the given URL and loads it with the metadata from the given version.
Infers the storage backend to use from the scheme in the given table URL.

Will fail fast if specified `table_url` is a local path but doesn't exist.
