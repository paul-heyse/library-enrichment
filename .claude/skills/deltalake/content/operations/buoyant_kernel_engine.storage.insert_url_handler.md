# `buoyant_kernel_engine::storage::insert_url_handler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel_engine.storage.insert_url_handler.json).

<a id="op-08ea42e4a0353b4a51927e2f"></a>
## insert_url_handler

`function` · `buoyant_kernel_engine::storage::insert_url_handler` · buoyant_kernel_engine 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn insert_url_handler(scheme: impl AsRef<str>, handler_closure: std::sync::Arc<dyn Fn(&url::Url, std::collections::HashMap<String, String>) -> Result<(Box<dyn ObjectStore>, delta_kernel::object_store::path::Path), delta_kernel::object_store::Error> + Send + Sync>) -> Result<(), delta_kernel::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/storage.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/default-engine/src/storage.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Insert a new URL handler for [store_from_url_opts](../operations/buoyant_kernel_engine.storage.store_from_url_opts.md#op-75ce4f5f9e5b42e65bdcf25e) with the given `scheme`. This allows
users to provide their own custom URL handler to plug new
[delta_kernel::object_store::ObjectStore] instances into delta-kernel, which is used by
[store_from_url_opts](../operations/buoyant_kernel_engine.storage.store_from_url_opts.md#op-75ce4f5f9e5b42e65bdcf25e) to parse the URL.

Unresolved upstream links (retained, not inferred): `delta_kernel::object_store::ObjectStore`.
