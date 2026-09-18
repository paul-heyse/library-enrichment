# `deltalake_azure::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_azure.register_handlers.json).

<a id="op-85561ec0f6e032ca2cd5b0d2"></a>
## register_handlers

`function` · `deltalake_azure::register_handlers` · deltalake-azure 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/lib.rs#L94).

Source: `crates/azure/src/lib.rs:94`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [ObjectStoreFactory](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md#op-ecc00d1a52536f5b5d865cff) for common Azure [Url] schemes

Unresolved upstream links (retained, not inferred): `Url`.
