# `deltalake_core::logstore::config::parse_usize`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.parse_usize.json).

<a id="op-e2acb35ecb5f13265fbbd179"></a>
## parse_usize

`function` · `deltalake_core::logstore::config::parse_usize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_usize(value: &str) -> DeltaResult<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L283).

Source: `crates/core/src/logstore/config.rs:283`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parse a string into a `usize`, returning a descriptive error on failure.

```
use deltalake_core::logstore::config::parse_usize;
assert_eq!(parse_usize("42").unwrap(), 42);
assert!(parse_usize("not_a_number").is_err());
```
