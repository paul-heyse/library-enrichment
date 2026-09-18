# `datafusion_ffi::util::vec_fieldref_to_rvec_wrapped`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.util.vec_fieldref_to_rvec_wrapped.json).

<a id="op-e9573a67333ef52a667efa88"></a>
## vec_fieldref_to_rvec_wrapped

`function` · `datafusion_ffi::util::vec_fieldref_to_rvec_wrapped` · datafusion-ffi 55.1.0

```rust
fn vec_fieldref_to_rvec_wrapped(fields: &[arrow_schema::FieldRef]) -> Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>, arrow::error::ArrowError>
```

Source: `src/util.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This is a utility function to convert a slice of [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) to its equivalent
FFI friendly counterpart, [`WrappedSchema`](../operations/datafusion_ffi.arrow_wrappers.WrappedSchema.md#op-a113b49f8c890a0cf373e941)
