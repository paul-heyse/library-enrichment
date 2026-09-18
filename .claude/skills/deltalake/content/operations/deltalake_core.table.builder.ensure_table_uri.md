# `deltalake_core::table::builder::ensure_table_uri`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.builder.ensure_table_uri.json).

<a id="op-eb88e00dd04f61994eecfd16"></a>
## ensure_table_uri

`function` · `deltalake_core::table::builder::ensure_table_uri` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ensure_table_uri(table_uri: impl AsRef<str>) -> DeltaResult<url::Url>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L420).

Source: `crates/core/src/table/builder.rs:420`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Will return an error if the location is not valid. For example,
Creates directories for local paths if they don't exist.
