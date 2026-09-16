# `buoyant_kernel_engine::storage`

Crate `buoyant_kernel_engine` · 3 public items · structured records in [`model/buoyant_kernel_engine.storage.json`](../model/buoyant_kernel_engine.storage.json)

## insert_url_handler

`function` · `buoyant_kernel_engine::storage::insert_url_handler`

Also reachable as `delta_kernel_default_engine::storage::insert_url_handler`

```rust
fn insert_url_handler(scheme: impl AsRef<str>, handler_closure: std::sync::Arc<dyn Fn(&url::Url, std::collections::HashMap<String, String>) -> Result<(Box<dyn ObjectStore>, delta_kernel::object_store::path::Path), delta_kernel::object_store::Error> + Send + Sync>) -> Result<(), delta_kernel::Error>
```

Insert a new URL handler for [store_from_url_opts] with the given `scheme`. This allows
users to provide their own custom URL handler to plug new
[delta_kernel::object_store::ObjectStore] instances into delta-kernel, which is used by
[store_from_url_opts] to parse the URL.

---

## store_from_url

`function` · `buoyant_kernel_engine::storage::store_from_url`

Also reachable as `delta_kernel_default_engine::storage::store_from_url`

```rust
fn store_from_url(url: &url::Url) -> delta_kernel::DeltaResult<std::sync::Arc<dyn ObjectStore>>
```

Create an [`ObjectStore`] from a URL.

Returns an `Arc<dyn ObjectStore>` ready to use with [`crate::DefaultEngine`].

This function checks for custom URL handlers registered via [`insert_url_handler`]
before falling back to [`object_store`]'s default behavior.

# Example

```rust
# use url::Url;
# use buoyant_kernel_engine as delta_kernel_default_engine;
# use delta_kernel_default_engine::storage::store_from_url;
# use delta_kernel::DeltaResult;
# fn example() -> DeltaResult<()> {
let url = Url::parse("file:///path/to/table")?;
let store = store_from_url(&url)?;
# Ok(())
# }
```

---

## store_from_url_opts

`function` · `buoyant_kernel_engine::storage::store_from_url_opts`

Also reachable as `delta_kernel_default_engine::storage::store_from_url_opts`

```rust
fn store_from_url_opts<I, K, V>(url: &url::Url, options: I) -> delta_kernel::DeltaResult<std::sync::Arc<dyn ObjectStore>> where I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V: Into<String>
```

Create an [`ObjectStore`] from a URL with custom options.

Returns an `Arc<dyn ObjectStore>` ready to use with [`crate::DefaultEngine`].

This function checks for custom URL handlers registered via [`insert_url_handler`]
before falling back to [`object_store`]'s default behavior.

# Example

```rust
# use url::Url;
# use std::collections::HashMap;
# use buoyant_kernel_engine as delta_kernel_default_engine;
# use delta_kernel_default_engine::storage::store_from_url_opts;
# use delta_kernel::DeltaResult;
# fn example() -> DeltaResult<()> {
let url = Url::parse("s3://my-bucket/path/to/table")?;
let options = HashMap::from([("region", "us-west-2")]);
let store = store_from_url_opts(&url, options)?;
# Ok(())
# }
```

---
