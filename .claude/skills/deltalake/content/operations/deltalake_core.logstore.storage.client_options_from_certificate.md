# `deltalake_core::logstore::storage::client_options_from_certificate`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.client_options_from_certificate.json).

<a id="op-272212d07887129d54714614"></a>
## client_options_from_certificate

`function` · `deltalake_core::logstore::storage::client_options_from_certificate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn client_options_from_certificate(path: &str) -> DeltaResult<object_store::ClientOptions>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/mod.rs#L112).

Source: `crates/core/src/logstore/storage/mod.rs:112`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Read a PEM certificate file and build [`object_store::ClientOptions`] with it.

Unresolved upstream links (retained, not inferred): ``object_store::ClientOptions``.
