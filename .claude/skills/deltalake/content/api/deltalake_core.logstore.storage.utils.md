# `deltalake_core::logstore::storage::utils`

Crate `deltalake-core` · 1 public items · structured records in [`model/deltalake_core.logstore.storage.utils.json`](../model/deltalake_core.logstore.storage.utils.json)

## commit_uri_from_version

`function` · `deltalake_core::logstore::storage::utils::commit_uri_from_version`

Also reachable as `deltalake::logstore::commit_uri_from_version`, `deltalake_core::logstore::commit_uri_from_version`

```rust
fn commit_uri_from_version(version: Option<kernel::Version>) -> object_store::path::Path
```

Return the uri of commit version.

```rust
# use deltalake_core::logstore::*;
use object_store::path::Path;
let uri = commit_uri_from_version(Some(1));
assert_eq!(uri, Path::from("_delta_log/00000000000000000001.json"));
```

---
