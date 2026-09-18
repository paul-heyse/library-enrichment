# `deltalake_lakefs::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.register_handlers.json).

<a id="op-3f2d14915eae140bd28c06cf"></a>
## register_handlers

`function` · `deltalake_lakefs::register_handlers` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L42).

Source: `crates/lakefs/src/lib.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [ObjectStoreFactory] for common LakeFS [Url] schemes

Unresolved upstream links (retained, not inferred): `Url`.
