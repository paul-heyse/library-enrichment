# `datafusion_ffi::util`

Crate `datafusion-ffi` · 4 public items · structured records in [`model/datafusion_ffi.util.json`](../model/datafusion_ffi.util.json)

## rvec_wrapped_to_vec_datatype

`function` · `datafusion_ffi::util::rvec_wrapped_to_vec_datatype`

```rust
fn rvec_wrapped_to_vec_datatype(data_types: &stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> Result<Vec<arrow::datatypes::DataType>, arrow::error::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.util.rvec_wrapped_to_vec_datatype.md).


This is a utility function to convert an FFI friendly vector of [`WrappedSchema`]
to their equivalent [`DataType`].

---

## rvec_wrapped_to_vec_fieldref

`function` · `datafusion_ffi::util::rvec_wrapped_to_vec_fieldref`

```rust
fn rvec_wrapped_to_vec_fieldref(fields: &stabby::vec::Vec<arrow_wrappers::WrappedSchema>) -> Result<Vec<arrow_schema::FieldRef>, arrow::error::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.util.rvec_wrapped_to_vec_fieldref.md).


This is a utility function to convert an FFI friendly vector of [`WrappedSchema`]
to their equivalent [`Field`].

---

## vec_datatype_to_rvec_wrapped

`function` · `datafusion_ffi::util::vec_datatype_to_rvec_wrapped`

```rust
fn vec_datatype_to_rvec_wrapped(data_types: &[arrow::datatypes::DataType]) -> Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>, arrow::error::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.util.vec_datatype_to_rvec_wrapped.md).


This is a utility function to convert a slice of [`DataType`] to its equivalent
FFI friendly counterpart, [`WrappedSchema`]

---

## vec_fieldref_to_rvec_wrapped

`function` · `datafusion_ffi::util::vec_fieldref_to_rvec_wrapped`

```rust
fn vec_fieldref_to_rvec_wrapped(fields: &[arrow_schema::FieldRef]) -> Result<stabby::vec::Vec<arrow_wrappers::WrappedSchema>, arrow::error::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_ffi.util.vec_fieldref_to_rvec_wrapped.md).


This is a utility function to convert a slice of [`Field`] to its equivalent
FFI friendly counterpart, [`WrappedSchema`]

---
