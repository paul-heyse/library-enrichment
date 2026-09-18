# `deltalake_core::logstore::config`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.config.json).

<a id="op-01d0de7312134a509d1455f4"></a>
## config

`module` · `deltalake_core::logstore::config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod config
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/config.rs#L1).

Source: `crates/core/src/logstore/config.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration for the Delta Log Store.

This module manages the various pieces of configuration for the Delta Log Store.
It provides methods for parsing and updating configuration settings. All configuration
is parsed from String -> String mappings.

Specific pieces of configuration must implement the `TryUpdateKey` trait which
defines how to update internal fields based on key-value pairs.
