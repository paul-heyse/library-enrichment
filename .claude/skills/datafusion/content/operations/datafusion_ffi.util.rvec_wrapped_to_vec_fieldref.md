# `datafusion_ffi::util::rvec_wrapped_to_vec_fieldref`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.util.rvec_wrapped_to_vec_fieldref.json).

<a id="op-813770db19c5edd0bc420716"></a>
## rvec_wrapped_to_vec_fieldref

`function` · `datafusion_ffi::util::rvec_wrapped_to_vec_fieldref` · datafusion-ffi 55.1.0

```rust
fn rvec_wrapped_to_vec_fieldref(fields: &stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> Result<Vec<arrow_schema::FieldRef>, arrow::error::ArrowError>
```

Source: `src/util.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is a utility function to convert an FFI friendly vector of [`WrappedSchema`](../operations/datafusion_ffi.arrow_wrappers.WrappedSchema.md#op-a113b49f8c890a0cf373e941)
to their equivalent [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf).
