# `deltalake_core::table::builder::parse_table_uri`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.builder.parse_table_uri.json).

<a id="op-36d11dc5406345e147a525ff"></a>
## parse_table_uri

`function` · `deltalake_core::table::builder::parse_table_uri` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_table_uri(table_uri: impl AsRef<str>) -> DeltaResult<url::Url>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/builder.rs#L388).

Source: `crates/core/src/table/builder.rs:388`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Attempt to create a Url from given table location.

The location could be:
 * A valid URL, which will be parsed and returned
 * A path to a directory, which will be created and then converted to a URL.

Extra slashes will be removed from the end path as well.

Parse a table URI to a URL without creating directories.
This is useful for opening existing tables where we don't want to create directories.
