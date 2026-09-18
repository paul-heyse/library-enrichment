# `deltalake_core::logstore::get_actions`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.get_actions.json).

<a id="op-697573bb185e8f60a5a7ce03"></a>
## get_actions

`function` · `deltalake_core::logstore::get_actions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_actions(version: kernel::Version, commit_log_bytes: &bytes::Bytes) -> Result<Vec<kernel::Action>, DeltaTableError>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L703).

Source: `crates/core/src/logstore/mod.rs:703`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Reads a commit and gets list of actions
