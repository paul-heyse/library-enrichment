# `datafusion_ffi::util::rvec_wrapped_to_vec_datatype`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.util.rvec_wrapped_to_vec_datatype.json).

<a id="op-fe598f698affbb798825c7e4"></a>
## rvec_wrapped_to_vec_datatype

`function` · `datafusion_ffi::util::rvec_wrapped_to_vec_datatype` · datafusion-ffi 55.1.0

```rust
fn rvec_wrapped_to_vec_datatype(data_types: &stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> Result<Vec<arrow::datatypes::DataType>, arrow::error::ArrowError>
```

Source: `src/util.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is a utility function to convert an FFI friendly vector of [`WrappedSchema`](../operations/datafusion_ffi.arrow_wrappers.WrappedSchema.md#op-a113b49f8c890a0cf373e941)
to their equivalent [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).
