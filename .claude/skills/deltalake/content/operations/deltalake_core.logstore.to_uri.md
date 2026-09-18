# `deltalake_core::logstore::to_uri`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.to_uri.json).

<a id="op-3c015b9f0e82f02325f58211"></a>
## to_uri

`function` · `deltalake_core::logstore::to_uri` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_uri(root: &url::Url, location: &object_store::path::Path) -> String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L643).

Source: `crates/core/src/logstore/mod.rs:643`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Join the given `root` [Url] with the [Path] to produce a URI (String) of the two together.

This is largely a convenience function to help with the nuances of empty [Path] and file [Url]s

Unresolved upstream links (retained, not inferred): `Url`, `Path`.
