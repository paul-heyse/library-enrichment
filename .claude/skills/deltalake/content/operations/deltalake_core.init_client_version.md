# `deltalake_core::init_client_version`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.init_client_version.json).

<a id="op-08fbf09822895264d96af90f"></a>
## init_client_version

`function` · `deltalake_core::init_client_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn init_client_version(version: &str)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/lib.rs#L191).

Source: `crates/core/src/lib.rs:191`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Record the client version reported by `crate_version`.

Bindings (such as the Python package) call this once at startup so that user agents and
telemetry identify the embedding client rather than the bare core crate version. Subsequent
calls after the first are ignored.
