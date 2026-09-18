# `deltalake_core::logstore::config::parse_bool`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.parse_bool.json).

<a id="op-c027d55fc044f3959894f016"></a>
## parse_bool

`function` · `deltalake_core::logstore::config::parse_bool` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_bool(value: &str) -> DeltaResult<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L317).

Source: `crates/core/src/logstore/config.rs:317`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse a string into a `bool`, accepting common truthy spellings (e.g. "1", "true").

```
use deltalake_core::logstore::config::parse_bool;
assert!(parse_bool("true").unwrap());
assert!(parse_bool("1").unwrap());
assert!(!parse_bool("false").unwrap());
assert!(!parse_bool("0").unwrap());
```
