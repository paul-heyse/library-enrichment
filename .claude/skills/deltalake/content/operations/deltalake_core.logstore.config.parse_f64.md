# `deltalake_core::logstore::config::parse_f64`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.parse_f64.json).

<a id="op-ba5866e71a700d6f27c9e439"></a>
## parse_f64

`function` · `deltalake_core::logstore::config::parse_f64` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_f64(value: &str) -> DeltaResult<f64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L296).

Source: `crates/core/src/logstore/config.rs:296`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse a string into an `f64`, returning a descriptive error on failure.

```
use deltalake_core::logstore::config::parse_f64;
assert_eq!(parse_f64("3.14").unwrap(), 3.14);
assert!(parse_f64("not_a_number").is_err());
```
