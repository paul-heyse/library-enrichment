# `buoyant_kernel_engine::storage::store_from_url_opts`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.storage.store_from_url_opts.json).

<a id="op-75ce4f5f9e5b42e65bdcf25e"></a>
## store_from_url_opts

`function` · `buoyant_kernel_engine::storage::store_from_url_opts` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn store_from_url_opts<I, K, V>(url: &url::Url, options: I) -> delta_kernel::DeltaResult<std::sync::Arc<dyn ObjectStore>> where I: IntoIterator<Item = (K, V)>, K: AsRef<str>, V: Into<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/storage.rs#L85).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/storage.rs:85`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Create an [`ObjectStore`] from a URL with custom options.

Returns an `Arc<dyn ObjectStore>` ready to use with [`crate::DefaultEngine`](../operations/buoyant_kernel_engine.DefaultEngine.md#op-6311b39f77a5eea5de61a8f5).

This function checks for custom URL handlers registered via [`insert_url_handler`](../operations/buoyant_kernel_engine.storage.insert_url_handler.md#op-08ea42e4a0353b4a51927e2f)
before falling back to [`object_store`](../modules/buoyant_kernel.arrow_compat.arrow_compat_shims.object_store.md#op-e0d64d61a46f44c82985eae6)'s default behavior.

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

Unresolved upstream links (retained, not inferred): ``ObjectStore``.
