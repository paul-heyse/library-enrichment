# `arrow_array::ffi::to_ffi`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi.to_ffi.json).

<a id="op-2d894bb1a42520ac50f3042b"></a>
## to_ffi

`function` · `arrow_array::ffi::to_ffi` · arrow-array 59.3.0

```rust
fn to_ffi(data: &arrow_data::ArrayData) -> std::result::Result<(FFI_ArrowArray, FFI_ArrowSchema), arrow_schema::ArrowError>
```

Source: `src/ffi.rs:271`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Export to the C Data Interface
