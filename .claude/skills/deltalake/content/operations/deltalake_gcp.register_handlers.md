# `deltalake_gcp::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_gcp.register_handlers.json).

<a id="op-26254cbad8b6bfbf79bded73"></a>
## register_handlers

`function` · `deltalake_gcp::register_handlers` · deltalake-gcp 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/lib.rs#L96).

Source: `crates/gcp/src/lib.rs:96`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [ObjectStoreFactory](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md#op-ecc00d1a52536f5b5d865cff) for common Google Cloud [Url] schemes

Unresolved upstream links (retained, not inferred): `Url`.
