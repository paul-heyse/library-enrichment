# `deltalake_core::logstore::storage::utils::commit_uri_from_version`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.utils.commit_uri_from_version.json).

<a id="op-48c35063f654d16fc41602ef"></a>
## commit_uri_from_version

`function` · `deltalake_core::logstore::storage::utils::commit_uri_from_version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn commit_uri_from_version(version: Option<kernel::Version>) -> object_store::path::Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/utils.rs#L19).

Source: `crates/core/src/logstore/storage/utils.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Return the uri of commit version.

```rust
# use deltalake_core::logstore::*;
use object_store::path::Path;
let uri = commit_uri_from_version(Some(1));
assert_eq!(uri, Path::from("_delta_log/00000000000000000001.json"));
```
