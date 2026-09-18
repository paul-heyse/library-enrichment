# `deltalake_mount::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.register_handlers.json).

<a id="op-ba5b85f6294b3248aa279354"></a>
## register_handlers

`function` · `deltalake_mount::register_handlers` · deltalake-mount 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/lib.rs#L103).

Source: `crates/mount/src/lib.rs:103`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [ObjectStoreFactory](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md#op-ecc00d1a52536f5b5d865cff) for common Mount [Url] schemes

Unresolved upstream links (retained, not inferred): `Url`.
