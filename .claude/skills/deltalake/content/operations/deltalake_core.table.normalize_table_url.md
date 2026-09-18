# `deltalake_core::table::normalize_table_url`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.normalize_table_url.json).

<a id="op-9173882a8726180c1e6b6559"></a>
## normalize_table_url

`function` · `deltalake_core::table::normalize_table_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn normalize_table_url(url: &url::Url) -> url::Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/mod.rs#L510).

Source: `crates/core/src/table/mod.rs:510`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Normalize a given [Url] to **always** contain a trailing slash. This is critically important
for assumptions about [Url] equivalency and more importantly for **joining** on a Url`.

This function will also remove redundant slashes in the ]Url] path which can cause other
equivalency failures

```ignore
 left.join("_delta_log"); // produces `s3://bucket/prefix/_delta_log`
 right.join("_delta_log"); // produces `s3://bucket/_delta_log`
```

Unresolved upstream links (retained, not inferred): `Url`.
