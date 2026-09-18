# `datafusion_ffi::get_library_marker_id`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.get_library_marker_id.json).

<a id="op-66c1f07f1e28422eccc970cc"></a>
## get_library_marker_id

`function` · `datafusion_ffi::get_library_marker_id` · datafusion-ffi 55.1.0

```rust
extern "C" fn get_library_marker_id() -> usize
```

Source: `src/lib.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

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
