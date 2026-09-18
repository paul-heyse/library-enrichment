# `deltalake_core::logstore::config::str_is_truthy`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.str_is_truthy.json).

<a id="op-f675408eb73c353c94174ec1"></a>
## str_is_truthy

`function` · `deltalake_core::logstore::config::str_is_truthy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn str_is_truthy(val: &str) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L339).

Source: `crates/core/src/logstore/config.rs:339`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return true for all the stringly values typically associated with true

aka YAML booleans

```rust
# use deltalake_core::logstore::config::*;
for value in ["1", "true", "on", "YES", "Y"] {
    assert!(str_is_truthy(value));
}
for value in ["0", "FALSE", "off", "NO", "n", "bork"] {
    assert!(!str_is_truthy(value));
}
```
