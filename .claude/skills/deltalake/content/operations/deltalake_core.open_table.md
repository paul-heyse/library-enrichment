# `deltalake_core::open_table`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.open_table.json).

<a id="op-a2e245c139792bf9e67b0a26"></a>
## open_table

`function` · `deltalake_core::open_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn open_table(table_url: url::Url) -> Result<DeltaTable, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L132).

Source: `crates/core/src/lib.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Creates and loads a DeltaTable from the given URL with current metadata.
Infers the storage backend to use from the scheme in the given table URL.

Will fail fast if specified `table_url` is a local path but doesn't exist.
