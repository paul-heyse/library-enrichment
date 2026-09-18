# `deltalake_hdfs::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_hdfs.register_handlers.json).

<a id="op-097db961c1bd020c46cec917"></a>
## register_handlers

`function` · `deltalake_hdfs::register_handlers` · deltalake-hdfs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/hdfs/src/lib.rs#L53).

Source: `crates/hdfs/src/lib.rs:53`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [ObjectStoreFactory](../operations/deltalake_core.logstore.factories.ObjectStoreFactory.md#op-ecc00d1a52536f5b5d865cff) for common HDFS [Url] schemes

Unresolved upstream links (retained, not inferred): `Url`.
