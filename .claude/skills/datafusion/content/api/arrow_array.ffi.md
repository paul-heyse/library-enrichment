# `arrow_array::ffi`

Crate `arrow-array` · 4 public items · structured records in [`model/arrow_array.ffi.json`](../model/arrow_array.ffi.json)

## export_array_into_raw

`function` · `arrow_array::ffi::export_array_into_raw`

> **Deprecated** — since 52.0.0: Use FFI_ArrowArray::new and FFI_ArrowSchema::try_from

Also reachable as `arrow::array::export_array_into_raw`, `arrow::ffi::export_array_into_raw`

```rust
unsafe fn export_array_into_raw(src: array::ArrayRef, out_array: *mut FFI_ArrowArray, out_schema: *mut FFI_ArrowSchema) -> std::result::Result<(), arrow_schema::ArrowError>
```

Exports an array to raw pointers of the C Data Interface provided by the consumer.
# Safety
Assumes that these pointers represent valid C Data Interfaces, both in memory
representation and lifetime via the `release` mechanism.

This function copies the content of two FFI structs [arrow_data::ffi::FFI_ArrowArray] and
[arrow_schema::ffi::FFI_ArrowSchema] in the array to the location pointed by the raw pointers.
Usually the raw pointers are provided by the array data consumer.

---

## from_ffi

`function` · `arrow_array::ffi::from_ffi`

Also reachable as `arrow::ffi::from_ffi`

```rust
unsafe fn from_ffi(array: FFI_ArrowArray, schema: &FFI_ArrowSchema) -> std::result::Result<arrow_data::ArrayData, arrow_schema::ArrowError>
```

Import [ArrayData] from the C Data Interface

# Safety

This struct assumes that the incoming data agrees with the C data interface.

---

## from_ffi_and_data_type

`function` · `arrow_array::ffi::from_ffi_and_data_type`

Also reachable as `arrow::ffi::from_ffi_and_data_type`

```rust
unsafe fn from_ffi_and_data_type(array: FFI_ArrowArray, data_type: arrow_schema::DataType) -> std::result::Result<arrow_data::ArrayData, arrow_schema::ArrowError>
```

Import [ArrayData] from the C Data Interface

# Safety

This struct assumes that the incoming data agrees with the C data interface.

---

## to_ffi

`function` · `arrow_array::ffi::to_ffi`

Also reachable as `arrow::ffi::to_ffi`

```rust
fn to_ffi(data: &arrow_data::ArrayData) -> std::result::Result<(FFI_ArrowArray, FFI_ArrowSchema), arrow_schema::ArrowError>
```

Export to the C Data Interface

---
