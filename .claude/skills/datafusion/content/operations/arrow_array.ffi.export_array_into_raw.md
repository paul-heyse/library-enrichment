# `arrow_array::ffi::export_array_into_raw`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi.export_array_into_raw.json).

<a id="op-0dfd74ec3e05febc556fe282"></a>
## export_array_into_raw

`function` · `arrow_array::ffi::export_array_into_raw` · arrow-array 59.3.0

```rust
unsafe fn export_array_into_raw(src: array::ArrayRef, out_array: *mut FFI_ArrowArray, out_schema: *mut FFI_ArrowSchema) -> std::result::Result<(), arrow_schema::ArrowError>
```

Source: `src/ffi.rs:128`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Exports an array to raw pointers of the C Data Interface provided by the consumer.
# Safety
Assumes that these pointers represent valid C Data Interfaces, both in memory
representation and lifetime via the `release` mechanism.

This function copies the content of two FFI structs [arrow_data::ffi::FFI_ArrowArray](../operations/arrow_data.ffi.FFI_ArrowArray.md#op-c531e3ad0f070327205a60e2) and
[arrow_schema::ffi::FFI_ArrowSchema](../operations/arrow_schema.ffi.FFI_ArrowSchema.md#op-702e6726e4207bb5becbbc93) in the array to the location pointed by the raw pointers.
Usually the raw pointers are provided by the array data consumer.
