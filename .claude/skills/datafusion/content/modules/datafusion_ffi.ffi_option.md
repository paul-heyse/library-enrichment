# `datafusion_ffi::ffi_option`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.ffi_option.json).

<a id="op-6a83aeb3d7029d3440f56dd4"></a>
## ffi_option

`module` · `datafusion_ffi::ffi_option` · datafusion-ffi 55.1.0

```rust
mod ffi_option
```

Source: `src/ffi_option.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI-safe Option and Result types that do not require `IStable` bounds.

stabby's `Option<T>` and `Result<T, E>` require `T: IStable` for niche
optimization. Many of our FFI structs contain self-referential function
pointers and cannot implement `IStable`. These simple `#[repr(C)]` types
provide the same FFI-safe semantics without that constraint.
