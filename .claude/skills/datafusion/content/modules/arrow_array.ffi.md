# `arrow_array::ffi`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.ffi.json).

<a id="op-b725a1de54eced6ce05f9387"></a>
## ffi

`module` · `arrow_array::ffi` · arrow-array 59.3.0

```rust
mod ffi
```

Source: `src/ffi.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Contains declarations to bind to the [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html).

Generally, this module is divided in two main interfaces:
One interface maps C ABI to native Rust types, i.e. convert c-pointers, c_char, to native rust.
This is handled by [FFI_ArrowSchema](../operations/arrow_schema.ffi.FFI_ArrowSchema.md#op-702e6726e4207bb5becbbc93) and [FFI_ArrowArray](../operations/arrow_data.ffi.FFI_ArrowArray.md#op-c531e3ad0f070327205a60e2).

The second interface maps native Rust types to the Rust-specific implementation of Arrow such as `format` to `Datatype`,
`Buffer`, etc. This is handled by `from_ffi` and `to_ffi`.


Export to FFI

```rust
# use std::sync::Arc;
# use arrow_array::{Int32Array, Array, make_array};
# use arrow_data::ArrayData;
# use arrow_array::ffi::{to_ffi, from_ffi};
# use arrow_schema::ArrowError;
# fn main() -> Result<(), ArrowError> {
// create an array natively

let array = Int32Array::from(vec![Some(1), None, Some(3)]);
let data = array.into_data();

// Export it
let (out_array, out_schema) = to_ffi(&data)?;

// import it
let data = unsafe { from_ffi(out_array, &out_schema) }?;
let array = Int32Array::from(data);

// verify
assert_eq!(array, Int32Array::from(vec![Some(1), None, Some(3)]));
#
# Ok(())
# }
```

Import from FFI

```
# use std::ptr::addr_of_mut;
# use arrow_array::ffi::{from_ffi, FFI_ArrowArray};
# use arrow_array::{ArrayRef, make_array};
# use arrow_schema::{ArrowError, ffi::FFI_ArrowSchema};
#
/// A foreign data container that can export to C Data interface
struct ForeignArray {};

impl ForeignArray {
    /// Export from foreign array representation to C Data interface
    /// e.g. <https://github.com/apache/arrow/blob/fc1f9ebbc4c3ae77d5cfc2f9322f4373d3d19b8a/python/pyarrow/array.pxi#L1552>
    fn export_to_c(&self, array: *mut FFI_ArrowArray, schema: *mut FFI_ArrowSchema) {
        // ...
    }
}

/// Import an [`ArrayRef`] from a [`ForeignArray`]
fn import_array(foreign: &ForeignArray) -> Result<ArrayRef, ArrowError> {
    let mut schema = FFI_ArrowSchema::empty();
    let mut array = FFI_ArrowArray::empty();
    foreign.export_to_c(addr_of_mut!(array), addr_of_mut!(schema));
    Ok(make_array(unsafe { from_ffi(array, &schema) }?))
}
```
