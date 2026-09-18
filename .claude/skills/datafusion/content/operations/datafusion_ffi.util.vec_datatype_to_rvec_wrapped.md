# `datafusion_ffi::util::vec_datatype_to_rvec_wrapped`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.util.vec_datatype_to_rvec_wrapped.json).

<a id="op-489785b6d23124a2ff24ca8c"></a>
## vec_datatype_to_rvec_wrapped

`function` · `datafusion_ffi::util::vec_datatype_to_rvec_wrapped` · datafusion-ffi 55.1.0

```rust
fn vec_datatype_to_rvec_wrapped(data_types: &[arrow::datatypes::DataType]) -> Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>, arrow::error::ArrowError>
```

Source: `src/util.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is a utility function to convert a slice of [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) to its equivalent
FFI friendly counterpart, [`WrappedSchema`](../operations/datafusion_ffi.arrow_wrappers.WrappedSchema.md#op-a113b49f8c890a0cf373e941)
