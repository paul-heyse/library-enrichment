# `deltalake_core::logstore::logstore_for`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.logstore_for.json).

<a id="op-279ce794191628a2624ba37a"></a>
## logstore_for

`function` · `deltalake_core::logstore::logstore_for` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logstore_for(location: &url::Url, storage_config: StorageConfig) -> DeltaResult<LogStoreRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L180).

Source: `crates/core/src/logstore/mod.rs:180`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the [LogStoreRef](../operations/deltalake_core.logstore.LogStoreRef.md#op-1b56118828480c7f5e758964) for the provided [Url] location

This will use the built-in process global [crate::storage::ObjectStoreRegistry] by default

```rust
# use deltalake_core::logstore::*;
# use std::collections::HashMap;
# use url::Url;
let location = Url::parse("memory:///").expect("Failed to make location");
let storage_config = StorageConfig::default();
let logstore = logstore_for(&location, storage_config).expect("Failed to get a logstore");
```

Unresolved upstream links (retained, not inferred): `Url`.
