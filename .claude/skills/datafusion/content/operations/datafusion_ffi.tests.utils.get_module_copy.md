# `datafusion_ffi::tests::utils::get_module_copy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.tests.utils.get_module_copy.json).

<a id="op-d624c69315cdf5ef43a5dc73"></a>
## get_module_copy

`function` · `datafusion_ffi::tests::utils::get_module_copy` · datafusion-ffi 55.1.0

```rust
fn get_module_copy(name: &str) -> datafusion_common::Result<tests::ForeignLibraryModule>
```

Source: `src/tests/utils.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

Load an independent copy of the integration-test cdylib.

Copying to a unique path makes the dynamic loader create a separate image
with its own library marker and Rust object graph.
