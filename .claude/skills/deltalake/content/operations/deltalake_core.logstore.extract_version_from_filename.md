# `deltalake_core::logstore::extract_version_from_filename`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.extract_version_from_filename.json).

<a id="op-281ba6ef64c537e955f919e3"></a>
## extract_version_from_filename

`function` · `deltalake_core::logstore::extract_version_from_filename` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn extract_version_from_filename(name: &str) -> Option<kernel::Version>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L778).

Source: `crates/core/src/logstore/mod.rs:778`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Extract version from a file name in the delta log
