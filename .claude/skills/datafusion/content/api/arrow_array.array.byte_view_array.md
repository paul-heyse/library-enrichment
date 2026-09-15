# `arrow_array::array::byte_view_array`

Crate `arrow-array` · 3 public items · structured records in [`model/arrow_array.array.byte_view_array.json`](../model/arrow_array.array.byte_view_array.json)

## GenericByteViewArray

`struct` · `arrow_array::array::byte_view_array::GenericByteViewArray`

```rust
struct GenericByteViewArray<T: ByteViewType + ?Sized>
```

**Implements**: `arrow_array::array::Array`, `core::convert::From`, `core::iter::traits::collect::FromIterator`

**Derives**: Clone, Debug, PartialEq

**Methods** (27)

```rust
fn bytes_iter(&self) -> impl Iterator<Item = &[u8]>
unsafe fn compare_unchecked(left: &GenericByteViewArray<T>, left_idx: usize, right: &GenericByteViewArray<T>, right_idx: usize) -> Ordering
fn data_buffers(&self) -> &[Buffer]
fn from_iter_values<Ptr, I>(iter: I) -> Self where Ptr: AsRef<T::Native>, I: IntoIterator<Item = Ptr>
fn gc(&self) -> Self
fn inline_key_fast(raw: u128) -> u128
unsafe fn inline_value(view: &u128, len: usize) -> &[u8]
fn into_parts(self) -> (ScalarBuffer<u128>, Arc<[Buffer]>, Option<NullBuffer>)
fn is_ascii(&self) -> bool
fn iter(&self) -> ArrayIter<&Self>
fn lengths(&self) -> impl ExactSizeIterator<Item = u32> + Clone
fn new<U>(views: ScalarBuffer<u128>, buffers: U, nulls: Option<NullBuffer>) -> Self where U: Into<Arc<[Buffer]>>
fn new_null(len: usize) -> Self
fn new_scalar(value: impl AsRef<T::Native>) -> Scalar<Self>
unsafe fn new_unchecked<U>(views: ScalarBuffer<u128>, buffers: U, nulls: Option<NullBuffer>) -> Self where U: Into<Arc<[Buffer]>>
fn prefix_bytes_iter(&self, prefix_len: usize) -> impl Iterator<Item = &[u8]>
fn slice(&self, offset: usize, length: usize) -> Self
fn suffix_bytes_iter(&self, suffix_len: usize) -> impl Iterator<Item = &[u8]>
fn to_binary_view(self) -> BinaryViewArray
fn to_string_view(self) -> Result<StringViewArray, ArrowError>
unsafe fn to_string_view_unchecked(self) -> StringViewArray
fn total_buffer_bytes_used(&self) -> usize
fn total_bytes_len(&self) -> usize
fn try_new<U>(views: ScalarBuffer<u128>, buffers: U, nulls: Option<NullBuffer>) -> Result<Self, ArrowError> where U: Into<Arc<[Buffer]>>
fn value(&self, i: usize) -> &T::Native
unsafe fn value_unchecked(&self, idx: usize) -> &T::Native
fn views(&self) -> &ScalarBuffer<u128>
```

**via `arrow_array::array::Array`**

```rust
fn as_any(&self) -> &dyn Any
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn data_type(&self) -> &DataType
fn get_array_memory_size(&self) -> usize
fn get_buffer_memory_size(&self) -> usize
fn into_data(self) -> ArrayData
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn logical_null_count(&self) -> usize
fn nulls(&self) -> Option<&NullBuffer>
fn offset(&self) -> usize
fn shrink_to_fit(&mut self)
fn slice(&self, offset: usize, length: usize) -> ArrayRef
fn to_data(&self) -> ArrayData
```

**via `core::convert::From`**

```rust
fn from(byte_array: &GenericByteArray<FROM>) -> Self
fn from(data: ArrayData) -> Self
```

**via `core::iter::traits::collect::FromIterator`**

```rust
fn from_iter<I: IntoIterator<Item = &'a Option<Ptr>>>(iter: I) -> Self
fn from_iter<I: IntoIterator<Item = Option<Ptr>>>(iter: I) -> Self
```

[Variable-size Binary View Layout]: An array of variable length bytes views.

This array type is used to store variable length byte data (e.g. Strings, Binary)
and has efficient operations such as `take`, `filter`, and comparison.

[Variable-size Binary View Layout]: https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout

This is different from [`GenericByteArray`], which also stores variable
length byte data, as it represents strings with an offset and length. `take`
and `filter` like operations are implemented by manipulating the "views"
(`u128`) without modifying the bytes. Each view also stores an inlined
prefix which speed up comparisons.

# See Also

* [`StringViewArray`] for storing utf8 encoded string data
* [`BinaryViewArray`] for storing bytes
* [`ByteView`] to interpret `u128`s layout of the views.

[`ByteView`]: arrow_data::ByteView

# Layout: "views" and buffers

A `GenericByteViewArray` stores variable length byte strings. An array of
`N` elements is stored as `N` fixed length "views" and a variable number
of variable length "buffers".

Each view is a `u128` value whose layout is different depending on the
length of the string stored at that location:

```text
                        ┌──────┬────────────────────────┐
                        │length│      string value      │
   Strings (len <= 12)  │      │    (padded with 0)     │
                        └──────┴────────────────────────┘
                         0    31                      127

                        ┌───────┬───────┬───────┬───────┐
                        │length │prefix │  buf  │offset │
   Strings (len > 12)   │       │       │ index │       │
                        └───────┴───────┴───────┴───────┘
                         0    31       63      95    127
```

* Strings with length <= 12 ([`MAX_INLINE_VIEW_LEN`]) are stored directly in
  the view. See [`Self::inline_value`] to access the inlined prefix from a
  short view.

* Strings with length > 12: The first four bytes are stored inline in the
  view and the entire string is stored in one of the buffers. See [`ByteView`]
  to access the fields of the these views.

As with other arrays, the optimized kernels in [`arrow_compute`] are likely
the easiest and fastest way to work with this data. However, it is possible
to access the views and buffers directly for more control.

For example

```rust
# use arrow_array::StringViewArray;
# use arrow_array::Array;
use arrow_data::ByteView;
let array = StringViewArray::from(vec![
  "hello",
  "this string is longer than 12 bytes",
  "this string is also longer than 12 bytes"
]);

// ** Examine the first view (short string) **
assert!(array.is_valid(0)); // Check for nulls
let short_view: u128 = array.views()[0]; // "hello"
// get length of the string
let len = short_view as u32;
assert_eq!(len, 5); // strings less than 12 bytes are stored in the view
// SAFETY: `view` is a valid view
let value = unsafe {
  StringViewArray::inline_value(&short_view, len as usize)
};
assert_eq!(value, b"hello");

// ** Examine the third view (long string) **
assert!(array.is_valid(12)); // Check for nulls
let long_view: u128 = array.views()[2]; // "this string is also longer than 12 bytes"
let len = long_view as u32;
assert_eq!(len, 40); // strings longer than 12 bytes are stored in the buffer
let view = ByteView::from(long_view); // use ByteView to access the fields
assert_eq!(view.length, 40);
assert_eq!(view.buffer_index, 0);
assert_eq!(view.offset, 35); // data starts after the first long string
// Views for long strings store a 4 byte prefix
let prefix = view.prefix.to_le_bytes();
assert_eq!(&prefix, b"this");
let value = array.value(2); // get the string value (see `value` implementation for how to access the bytes directly)
assert_eq!(value, "this string is also longer than 12 bytes");
```

[`MAX_INLINE_VIEW_LEN`]: arrow_data::MAX_INLINE_VIEW_LEN
[`arrow_compute`]: https://docs.rs/arrow/latest/arrow/compute/index.html

Unlike [`GenericByteArray`], there are no constraints on the offsets other
than they must point into a valid buffer. However, they can be out of order,
non continuous and overlapping.

For example, in the following diagram, the strings "FishWasInTownToday" and
"CrumpleFacedFish" are both longer than 12 bytes and thus are stored in a
separate buffer while the string "LavaMonster" is stored inlined in the
view. In this case, the same bytes for "Fish" are used to store both strings.

[`ByteView`]: arrow_data::ByteView

```text
                                                                           ┌───┐
                        ┌──────┬──────┬──────┬──────┐               offset │...│
"FishWasInTownTodayYay" │  21  │ Fish │  0   │ 115  │─ ─              103  │Mr.│
                        └──────┴──────┴──────┴──────┘   │      ┌ ─ ─ ─ ─ ▶ │Cru│
                        ┌──────┬──────┬──────┬──────┐                      │mpl│
"CrumpleFacedFish"      │  16  │ Crum │  0   │ 103  │─ ─│─ ─ ─ ┘           │eFa│
                        └──────┴──────┴──────┴──────┘                      │ced│
                        ┌──────┬────────────────────┐   └ ─ ─ ─ ─ ─ ─ ─ ─ ▶│Fis│
"LavaMonster"           │  11  │   LavaMonster      │                      │hWa│
                        └──────┴────────────────────┘               offset │sIn│
                                                                      115  │Tow│
                                                                           │nTo│
                                                                           │day│
                                 u128 "views"                              │Yay│
                                                                  buffer 0 │...│
                                                                           └───┘
```

---

## BinaryViewArray

`type_alias` · `arrow_array::array::byte_view_array::BinaryViewArray`

```rust
type BinaryViewArray = GenericByteViewArray<types::BinaryViewType>
```

A [`GenericByteViewArray`] of `[u8]`

See [`GenericByteViewArray`] for format and layout details.

# Example
```
use arrow_array::BinaryViewArray;
let array = BinaryViewArray::from_iter_values(vec![b"hello" as &[u8], b"world", b"lulu", b"large payload over 12 bytes"]);
assert_eq!(array.value(0), b"hello");
assert_eq!(array.value(3), b"large payload over 12 bytes");
```

---

## StringViewArray

`type_alias` · `arrow_array::array::byte_view_array::StringViewArray`

```rust
type StringViewArray = GenericByteViewArray<types::StringViewType>
```

A [`GenericByteViewArray`] that stores utf8 data

See [`GenericByteViewArray`] for format and layout details.

# Example
```
use arrow_array::StringViewArray;
let array = StringViewArray::from_iter_values(vec!["hello", "world", "lulu", "large payload over 12 bytes"]);
assert_eq!(array.value(0), "hello");
assert_eq!(array.value(3), "large payload over 12 bytes");
```

---
