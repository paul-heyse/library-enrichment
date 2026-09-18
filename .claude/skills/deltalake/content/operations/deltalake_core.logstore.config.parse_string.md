# `deltalake_core::logstore::config::parse_string`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.parse_string.json).

<a id="op-f413d9e4f5ff8a9c0aba5e2e"></a>
## parse_string

`function` · `deltalake_core::logstore::config::parse_string` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_string(value: &str) -> DeltaResult<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L322).

Source: `crates/core/src/logstore/config.rs:322`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse a configuration value as a plain string (an infallible identity conversion).
