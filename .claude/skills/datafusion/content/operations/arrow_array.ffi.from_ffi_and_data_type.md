# `arrow_array::ffi::from_ffi_and_data_type`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi.from_ffi_and_data_type.json).

<a id="op-6592e1ebde64094fc4e04700"></a>
## from_ffi_and_data_type

`function` · `arrow_array::ffi::from_ffi_and_data_type` · arrow-array 59.3.0

```rust
unsafe fn from_ffi_and_data_type(array: FFI_ArrowArray, data_type: arrow_schema::DataType) -> std::result::Result<arrow_data::ArrayData, arrow_schema::ArrowError>
```

Source: `src/ffi.rs:305`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Import [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) from the C Data Interface

# Safety

This struct assumes that the incoming data agrees with the C data interface.
