# `datafusion_ffi::tests::utils`

Crate `datafusion-ffi` · 2 public items · structured records in [`model/datafusion_ffi.tests.utils.json`](../model/datafusion_ffi.tests.utils.json)

## get_module

`function` · `datafusion_ffi::tests::utils::get_module`

```rust
fn get_module() -> datafusion_common::Result<tests::ForeignLibraryModule>
```

---

## get_module_copy

`function` · `datafusion_ffi::tests::utils::get_module_copy`

```rust
fn get_module_copy(name: &str) -> datafusion_common::Result<tests::ForeignLibraryModule>
```

Load an independent copy of the integration-test cdylib.

Copying to a unique path makes the dynamic loader create a separate image
with its own library marker and Rust object graph.

---
