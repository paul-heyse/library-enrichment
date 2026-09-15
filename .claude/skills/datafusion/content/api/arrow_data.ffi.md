# `arrow_data::ffi`

Crate `arrow-data` · 1 public items · structured records in [`model/arrow_data.ffi.json`](../model/arrow_data.ffi.json)

## FFI_ArrowArray

`struct` · `arrow_data::ffi::FFI_ArrowArray`

Also reachable as `arrow::ffi::FFI_ArrowArray`, `arrow_array::ffi::FFI_ArrowArray`

```rust
struct FFI_ArrowArray
```

**Fields**: `length`, `null_count`, `offset`, `n_buffers`, `n_children`, `buffers`, `children`, `dictionary`, `release`, `private_data`

**Implements**: `core::ops::drop::Drop`

**Derives**: Debug, Send, Sync

**Methods** (15)

```rust
fn buffer(&self, index: usize) -> *const u8
fn child(&self, index: usize) -> &FFI_ArrowArray
fn dictionary(&self) -> Option<&Self>
fn empty() -> Self
unsafe fn from_raw(array: *mut FFI_ArrowArray) -> Self
fn is_empty(&self) -> bool
fn is_released(&self) -> bool
fn len(&self) -> usize
fn new(data: &ArrayData) -> Self
fn null_count(&self) -> usize
fn null_count_opt(&self) -> Option<usize>
fn num_buffers(&self) -> usize
fn num_children(&self) -> usize
fn offset(&self) -> usize
unsafe fn set_null_count(&mut self, null_count: i64)
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

ABI-compatible struct for ArrowArray from C Data Interface
See <https://arrow.apache.org/docs/format/CDataInterface.html#the-arrowarray-structure>

```
# use arrow_data::ArrayData;
# use arrow_data::ffi::FFI_ArrowArray;
fn export_array(array: &ArrayData) -> FFI_ArrowArray {
    FFI_ArrowArray::new(array)
}
```

---
