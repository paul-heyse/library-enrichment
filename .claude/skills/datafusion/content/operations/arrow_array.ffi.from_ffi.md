# `arrow_array::ffi::from_ffi`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi.from_ffi.json).

<a id="op-6efe3e738f0231a98a588e1d"></a>
## from_ffi

`function` · `arrow_array::ffi::from_ffi` · arrow-array 59.3.0

```rust
unsafe fn from_ffi(array: FFI_ArrowArray, schema: &FFI_ArrowSchema) -> std::result::Result<arrow_data::ArrayData, arrow_schema::ArrowError>
```

Source: `src/ffi.rs:282`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Import [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) from the C Data Interface

# Safety

This struct assumes that the incoming data agrees with the C data interface.
