# `datafusion_ffi`

Crate `datafusion-ffi` · 5 public items · structured records in [`model/datafusion_ffi.json`](../model/datafusion_ffi.json)

## get_library_marker_id

`function` · `datafusion_ffi::get_library_marker_id`

```rust
extern "C" fn get_library_marker_id() -> usize
```

This utility is used to determine if two FFI structs are within
the same library. It is possible that the interplay between
foreign and local functions calls create one FFI struct that
references another. It is helpful to determine if a foreign
struct in the same library or called from a different one.
If we are in the same library, then we can access the underlying
types directly.

This function works by checking the address of the library
marker. Each library that implements the FFI code will have
a different address for the marker. By checking the marker
address we can determine if a struct is truly foreign or is
actually within the same originating library.

See the crate's `README.md` for additional information.

---

## version

`function` · `datafusion_ffi::version`

```rust
extern "C" fn version() -> u64
```

Returns the major version of the FFI implementation. If the API evolves,
we use the major version to identify compatibility over the unsafe
boundary. This call is intended to be used by implementers to validate
they have compatible libraries.

---

## df_result

`macro` · `datafusion_ffi::df_result`

```rust
macro_rules! df_result
```

This macro is a helpful conversion utility to convert from an FFI_Result to a
DataFusion result.

---

## sresult

`macro` · `datafusion_ffi::sresult`

```rust
macro_rules! sresult
```

This macro is a helpful conversion utility to convert from a DataFusion Result to an FFI_Result.

---

## sresult_return

`macro` · `datafusion_ffi::sresult_return`

```rust
macro_rules! sresult_return
```

This macro is a helpful conversion utility to convert from a DataFusion Result to an FFI_Result
and to also call return when it is an error. Since you cannot use `?` on an FFI_Result, this is designed
to mimic the pattern.

---
