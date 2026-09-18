# `arrow_array::array::byte_view_array::GenericByteViewArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.byte_view_array.GenericByteViewArray.json).

<a id="op-af313e332b96011fb72a0226"></a>
## GenericByteViewArray

`struct` · `arrow_array::array::byte_view_array::GenericByteViewArray` · arrow-array 59.3.0

```rust
struct GenericByteViewArray<T: ByteViewType + ?Sized>
```

Source: `src/array/byte_view_array.rs:165`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

[Variable-size Binary View Layout]: An array of variable length bytes views.

This array type is used to store variable length byte data (e.g. Strings, Binary)
and has efficient operations such as `take`, `filter`, and comparison.

[Variable-size Binary View Layout]: https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout

This is different from [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443), which also stores variable
length byte data, as it represents strings with an offset and length. `take`
and `filter` like operations are implemented by manipulating the "views"
(`u128`) without modifying the bytes. Each view also stores an inlined
prefix which speed up comparisons.

# See Also

* [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) for storing utf8 encoded string data
* [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) for storing bytes
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
  the view. See [`Self::inline_value`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-60c2c954a5b6584568d93d65) to access the inlined prefix from a
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

Unlike [`GenericByteArray`](../operations/arrow_array.array.byte_array.GenericByteArray.md#op-e39e3ecbe5a4126397f47443), there are no constraints on the offsets other
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

<a id="op-fccc1b4cfd33df63690bc932"></a>
## as_any

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::as_any` · arrow-array 59.3.0

```rust
fn as_any(&self) -> &dyn Any
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:881`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8422b91d4a5ff0981ec91de2"></a>
## bytes_iter

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::bytes_iter` · arrow-array 59.3.0

```rust
fn bytes_iter(&self) -> impl Iterator<Item = &[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:370`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator over the bytes of this array, including null values

<a id="op-b1db9d242adbb829d791d258"></a>
## claim

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:958`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5860047f68d93f7303240f89"></a>
## clone

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [183, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/array/byte_view_array.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20ef1198d9a3796c64e12396"></a>
## compare_unchecked

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::compare_unchecked` · arrow-array 59.3.0

```rust
unsafe fn compare_unchecked(left: &GenericByteViewArray<T>, left_idx: usize, right: &GenericByteViewArray<T>, right_idx: usize) -> Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:782`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Compare two [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) at index `left_idx` and `right_idx`

Comparing two ByteView types are non-trivial.
It takes a bit of patience to understand why we don't just compare two &[u8] directly.

ByteView types give us the following two advantages, and we need to be careful not to lose them:
(1) For string/byte smaller than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes, the entire data is inlined in the view.
    Meaning that reading one array element requires only one memory access
    (two memory access required for StringArray, one for offset buffer, the other for value buffer).

(2) For string/byte larger than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes, we can still be faster than (for certain operations) StringArray/ByteArray,
    thanks to the inlined 4 bytes.
    Consider equality check:
    If the first four bytes of the two strings are different, we can return false immediately (with just one memory access).

If we directly compare two &[u8], we materialize the entire string (i.e., make multiple memory accesses), which might be unnecessary.
- Most of the time (eq, ord), we only need to look at the first 4 bytes to know the answer,
  e.g., if the inlined 4 bytes are different, we can directly return unequal without looking at the full string.

# Order check flow
(1) if both string are smaller than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes, we can directly compare the data inlined to the view.
(2) if any of the string is larger than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed) bytes, we need to compare the full string.
    (2.1) if the inlined 4 bytes are different, we can return the result immediately.
    (2.2) o.w., we need to compare the full string.

# Safety
The left/right_idx must within range of each array

Unresolved upstream links (retained, not inferred): `u8`.

<a id="op-9de9379ed3601882d7f8f8a6"></a>
## data_buffers

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::data_buffers` · arrow-array 59.3.0

```rust
fn data_buffers(&self) -> &[Buffer]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:305`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the buffers storing string data

<a id="op-bd631d6ab25f349efca21081"></a>
## data_type

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::data_type` · arrow-array 59.3.0

```rust
fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:893`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0ebb3077c7877d425eabf66"></a>
## eq

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [848, 1], "end": [852, 2], "filename": "src/array/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array/mod.rs:849`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2db99ae56033b32019eca76c"></a>
## fmt

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [869, 1], "end": [877, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array/byte_view_array.rs:870`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78b5d58777943a2f81026788"></a>
## from

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::from` · arrow-array 59.3.0

```rust
fn from(byte_array: &GenericByteArray<FROM>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "FROM"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": "ByteArrayType"}}}], "generic_params": [], "type": {"generic": "FROM"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "num_traits::cast::ToPrimitive", "path": "ToPrimitive"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Offset", "self_type": {"generic": "FROM"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "FROM"}, "trait": {"args": null, "id": "arrow_array::types::ByteArrayType", "path": ""}}}}}, "name": "Native"}]}}, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [1022, 1], "end": [1066, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "FROM"}}], "constraints": []}}, "id": "arrow_array::array::byte_array::GenericByteArray", "path": "GenericByteArray"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/byte_view_array.rs:1028`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d10a52e9fde154c0051f80b0"></a>
## from

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::from` · arrow-array 59.3.0

```rust
fn from(data: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [990, 1], "end": [1010, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/array/byte_view_array.rs:991`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a8217f1cd7faa28bf76afa9"></a>
## from_iter

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = &'a Option<Ptr>>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}, {"outlives": "'a"}], "generic_params": [], "type": {"generic": "Ptr"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [1084, 1], "end": [1094, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/byte_view_array.rs:1089`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db530f1790ae9b0d9dae46c0"></a>
## from_iter

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::from_iter` · arrow-array 59.3.0

```rust
fn from_iter<I: IntoIterator<Item = Option<Ptr>>>(iter: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Ptr"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": ""}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}}}], "generic_params": [], "type": {"generic": "Ptr"}}}]}, "is_negative": false, "span": {"begin": [1096, 1], "end": [1106, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Ptr"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `src/array/byte_view_array.rs:1100`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72374cce24f64fa99b2b8f24"></a>
## from_iter_values

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::from_iter_values` · arrow-array 59.3.0

```rust
fn from_iter_values<Ptr, I>(iter: I) -> Self where Ptr: AsRef<T::Native>, I: IntoIterator<Item = Ptr>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:279`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) based on an iterator of values without nulls

<a id="op-6990f77a609a6af715a9432e"></a>
## gc

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::gc` · arrow-array 59.3.0

```rust
fn gc(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:515`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a "compacted" version of this array

The original array will *not* be modified

# Garbage Collection

Before GC:
```text
                                       ┌──────┐
                                       │......│
                                       │......│
┌────────────────────┐       ┌ ─ ─ ─ ▶ │Data1 │   Large buffer
│       View 1       │─ ─ ─ ─          │......│  with data that
├────────────────────┤                 │......│ is not referred
│       View 2       │─ ─ ─ ─ ─ ─ ─ ─▶ │Data2 │ to by View 1 or
└────────────────────┘                 │......│      View 2
                                       │......│
   2 views, refer to                   │......│
  small portions of a                  └──────┘
     large buffer
```

After GC:

```text
┌────────────────────┐                 ┌─────┐    After gc, only
│       View 1       │─ ─ ─ ─ ─ ─ ─ ─▶ │Data1│     data that is
├────────────────────┤       ┌ ─ ─ ─ ▶ │Data2│    pointed to by
│       View 2       │─ ─ ─ ─          └─────┘     the views is
└────────────────────┘                                 left


        2 views
```
This method will compact the data buffers by recreating the view array and only include the data
that is pointed to by the views.

Note that it will copy the array regardless of whether the original array is compact.
Use with caution as this can be an expensive operation, only use it when you are sure that the view
array is significantly smaller than when it is originally created, e.g., after filtering or slicing.

Note: this function does not attempt to canonicalize / deduplicate values. For this
feature see  [`GenericByteViewBuilder::with_deduplicate_strings`](../operations/arrow_array.builder.generic_bytes_view_builder.GenericByteViewBuilder.md#op-1ffb842587c4b7fab00361d4).

<a id="op-e3411ff5d8c8bf5bc5456ef2"></a>
## get_array_memory_size

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:953`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f01571b695d1537484f8a15"></a>
## get_buffer_memory_size

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::get_buffer_memory_size` · arrow-array 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:944`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b2685b2e7e2e6b7da13d0e3"></a>
## inline_key_fast

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::inline_key_fast` · arrow-array 59.3.0

```rust
fn inline_key_fast(raw: u128) -> u128
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:864`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builds a 128-bit composite key for an inline value:

- High 96 bits: the inline data in big-endian byte order (for correct lexicographical sorting).
- Low  32 bits: the length in big-endian byte order, acting as a tiebreaker so shorter strings
  (or those with fewer meaningful bytes) always numerically sort before longer ones.

This function extracts the length and the 12-byte inline string data from the raw
little-endian `u128` representation, converts them to big-endian ordering, and packs them
into a single `u128` value suitable for fast, branchless comparisons.

# Why include length?

A pure 96-bit content comparison can’t distinguish between two values whose inline bytes
compare equal—either because one is a true prefix of the other or because zero-padding
hides extra bytes. By tucking the 32-bit length into the lower bits, a single `u128` compare
handles both content and length in one go.

Example: comparing "bar" (3 bytes) vs "bar\0" (4 bytes)

| String     | Bytes 0–4 (length LE) | Bytes 4–16 (data + padding)    |
|------------|-----------------------|---------------------------------|
| `"bar"`   | `03 00 00 00`         | `62 61 72` + 9 × `00`           |
| `"bar\0"`| `04 00 00 00`         | `62 61 72 00` + 8 × `00`        |

Both inline parts become `62 61 72 00…00`, so they tie on content. The length field
then differentiates:

```text
key("bar")   = 0x0000000000000000000062617200000003
key("bar\0") = 0x0000000000000000000062617200000004
⇒ key("bar") < key("bar\0")
```
- `raw` is treated as a 128-bit integer with its bits laid out as follows:
  - bits 0–31: length (little-endian)
  - bits 32–127: data (little-endian)

# Inlining and Endianness

This function uses platform-independent bitwise operations to construct a 128-bit key:
- `raw.swap_bytes() << 32` effectively clears the length bits and shifts the 12-byte inline data
  into the high 96 bits in Big-Endian order. This ensures the first byte of the string
  is the most significant byte of the resulting `u128`.
- `raw as u32` extracts the length as a numeric integer, which is then placed in the low 32 bits.

<a id="op-60c2c954a5b6584568d93d65"></a>
## inline_value

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::inline_value` · arrow-array 59.3.0

```rust
unsafe fn inline_value(view: &u128, len: usize) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the first `len` bytes the inline value of the view.

# Safety
- The `view` must be a valid element from `Self::views()` that adheres to the view layout.
- The `len` must be the length of the inlined value. It should never be larger than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed).

<a id="op-bcc3ac362cc419f1d042ea45"></a>
## into_data

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::into_data` · arrow-array 59.3.0

```rust
fn into_data(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:889`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e97fd48e3c008933557134ea"></a>
## into_parts

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (ScalarBuffer<u128>, Arc<[Buffer]>, Option<NullBuffer>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:293`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Deconstruct this array into its constituent parts

<a id="op-ba20947fee2374c0b49aad90"></a>
## is_ascii

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::is_ascii` · arrow-array 59.3.0

```rust
fn is_ascii(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::StringViewType", "path": "StringViewType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1162, 1], "end": [1181, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:1169`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns true if all data within this array is ASCII

<a id="op-c1ee6e434f3dcb1838c72489"></a>
## is_empty

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::is_empty` · arrow-array 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:905`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66359bd5b1994aca82ee3a00"></a>
## iter

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::iter` · arrow-array 59.3.0

```rust
fn iter(&self) -> ArrayIter<&Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Constructs a new iterator for iterating over the values of this array

<a id="op-cca4c342cec5d894b29cf247"></a>
## len

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::len` · arrow-array 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:901`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59c1e0fc4dd7b450bf248611"></a>
## lengths

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::lengths` · arrow-array 59.3.0

```rust
fn lengths(&self) -> impl ExactSizeIterator<Item = u32> + Clone
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:457`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return an iterator over the length of each array element, including null values.

Null values length would equal to the underlying bytes length and NOT 0

Example of getting 0 for null values
```rust
# use arrow_array::StringViewArray;
# use arrow_array::Array;
use arrow_data::ByteView;

fn lengths_with_zero_for_nulls(view: &StringViewArray) -> impl Iterator<Item = u32> {
    view.lengths()
        .enumerate()
        .map(|(index, length)| if view.is_null(index) { 0 } else { length })
}
```

<a id="op-fa4cda94099d455dc8c4b40b"></a>
## logical_null_count

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::logical_null_count` · arrow-array 59.3.0

```rust
fn logical_null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:939`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f0107aecf168c05ca91fea3"></a>
## new

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::new` · arrow-array 59.3.0

```rust
fn new<U>(views: ScalarBuffer<u128>, buffers: U, nulls: Option<NullBuffer>) -> Self where U: Into<Arc<[Buffer]>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:191`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) from the provided parts, panicking on failure

# Panics

Panics if [`GenericByteViewArray::try_new`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-8edff37e46c40db8d46f771a) returns an error

<a id="op-f48b3042ed0d9bc4e9b3712d"></a>
## new_null

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::new_null` · arrow-array 59.3.0

```rust
fn new_null(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) of length `len` where all values are null

<a id="op-467d95f40c865241f1e4ce5a"></a>
## new_scalar

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::new_scalar` · arrow-array 59.3.0

```rust
fn new_scalar(value: impl AsRef<T::Native>) -> Scalar<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:274`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`Scalar`](../operations/arrow_array.scalar.Scalar.md#op-0ca10f05b262b3afc7077257) from `value`

<a id="op-483ba2e41f80971bb1dbaef2"></a>
## new_unchecked

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked<U>(views: ScalarBuffer<u128>, buffers: U, nulls: Option<NullBuffer>) -> Self where U: Into<Arc<[Buffer]>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:241`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) from the provided parts, without validation

# Safety

Safe if [`Self::try_new`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-8edff37e46c40db8d46f771a) would not error

<a id="op-f1eb1cbcce0221bd506a2b76"></a>
## nulls

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::nulls` · arrow-array 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:935`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec99a53bca2eac518d1bba7d"></a>
## offset

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::offset` · arrow-array 59.3.0

```rust
fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:931`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f479956e5c544aefad0ef1c2"></a>
## prefix_bytes_iter

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::prefix_bytes_iter` · arrow-array 59.3.0

```rust
fn prefix_bytes_iter(&self, prefix_len: usize) -> impl Iterator<Item = &[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:389`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator over the first `prefix_len` bytes of each array
element, including null values.

If `prefix_len` is larger than the element's length, the iterator will
return an empty slice (`&[]`).

<a id="op-308793a90430e8acecd20ab9"></a>
## shrink_to_fit

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::shrink_to_fit` · arrow-array 59.3.0

```rust
fn shrink_to_fit(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:909`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39c985d792942db0c3276567"></a>
## slice

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:897`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8a1761e9a109c6f6d4c35b5"></a>
## slice

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:462`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a zero-copy slice of this array with the indicated offset and length.

<a id="op-a94c866b2fa3e85e4eab10e4"></a>
## suffix_bytes_iter

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::suffix_bytes_iter` · arrow-array 59.3.0

```rust
fn suffix_bytes_iter(&self, suffix_len: usize) -> impl Iterator<Item = &[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:419`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns an iterator over the last `suffix_len` bytes of each array
element, including null values.

Note that for [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) the last bytes may start in the middle
of a UTF-8 codepoint, and thus may not be a valid `&str`.

If `suffix_len` is larger than the element's length, the iterator will
return an empty slice (`&[]`).

<a id="op-e4ac4eaa3caa8eee76738fd5"></a>
## to_binary_view

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::to_binary_view` · arrow-array 59.3.0

```rust
fn to_binary_view(self) -> BinaryViewArray
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::StringViewType", "path": "StringViewType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1162, 1], "end": [1181, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:1164`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Convert the [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0) to [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c)

<a id="op-fc88049afafd1708e4d52bed"></a>
## to_data

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::to_data` · arrow-array 59.3.0

```rust
fn to_data(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [880, 1], "end": [967, 2], "filename": "src/array/byte_view_array.rs"}, "trait": {"args": null, "id": "arrow_array::array::Array", "path": "Array"}, "trait_path": "arrow_array::array::Array"}`

Source: `src/array/byte_view_array.rs:885`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23d623899b7d60ca920e9c49"></a>
## to_string_view

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::to_string_view` · arrow-array 59.3.0

```rust
fn to_string_view(self) -> Result<StringViewArray, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::BinaryViewType", "path": "BinaryViewType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1121, 1], "end": [1135, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:1124`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Convert the [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) to [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0)
If items not utf8 data, validate will fail and error returned.

<a id="op-8838b7841941c00201f99cfc"></a>
## to_string_view_unchecked

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::to_string_view_unchecked` · arrow-array 59.3.0

```rust
unsafe fn to_string_view_unchecked(self) -> StringViewArray
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::types::BinaryViewType", "path": "BinaryViewType"}}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1121, 1], "end": [1135, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:1132`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Convert the [`BinaryViewArray`](../operations/arrow_array.array.byte_view_array.BinaryViewArray.md#op-3ee392743cc781de5833990c) to [`StringViewArray`](../operations/arrow_array.array.byte_view_array.StringViewArray.md#op-f468d0a8f2aaecdb58e1f8c0)
# Safety
Caller is responsible for ensuring that items in array are utf8 data.

<a id="op-5c54664aab09caa82e17565e"></a>
## total_buffer_bytes_used

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::total_buffer_bytes_used` · arrow-array 59.3.0

```rust
fn total_buffer_bytes_used(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:741`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of bytes used by all non inlined views in all
buffers.

Note this does not account for views that point at the same underlying
data in buffers

For example, if the array has three strings views:
* View with length = 9 (inlined)
* View with length = 32 (non inlined)
* View with length = 16 (non inlined)

Then this method would report 48

<a id="op-e579a376349e48fb5dbbbc9a"></a>
## total_bytes_len

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::total_bytes_len` · arrow-array 59.3.0

```rust
fn total_bytes_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:717`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of bytes of all non-null values in this array.

Unlike [`Self::total_buffer_bytes_used`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-5c54664aab09caa82e17565e), this method includes inlined strings
(those with length ≤ [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed)), making it suitable as a
capacity hint when pre-allocating output buffers.

Null values are excluded from the sum.

# Example

```rust
# use arrow_array::StringViewArray;
let array = StringViewArray::from_iter(vec![
    Some("hello"),   // 5 bytes, inlined
    None,            // excluded
    Some("large payload over 12 bytes"),  // 27 bytes, non-inlined
]);
assert_eq!(array.total_bytes_len(), 5 + 27);
```

<a id="op-8edff37e46c40db8d46f771a"></a>
## try_new

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::try_new` · arrow-array 59.3.0

```rust
fn try_new<U>(views: ScalarBuffer<u128>, buffers: U, nulls: Option<NullBuffer>) -> Result<Self, ArrowError> where U: Into<Arc<[Buffer]>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:204`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [`GenericByteViewArray`](../operations/arrow_array.array.byte_view_array.GenericByteViewArray.md#op-af313e332b96011fb72a0226) from the provided parts, returning an error on failure

# Errors

* `views.len() != nulls.len()`
* [ByteViewType::validate](../operations/arrow_array.types.ByteViewType.md#op-9cd62882abef2950e75e1ec5) fails

<a id="op-0d70a5e3522caa693b630552"></a>
## value

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::value` · arrow-array 59.3.0

```rust
fn value(&self, i: usize) -> &T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:316`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i`

Note: This method does not check for nulls and the value is arbitrary
(but still well-defined) if [`is_null`](Self::is_null) returns true for the index.

# Panics
Panics if index `i` is out of bounds.

<a id="op-e4cc5d309b7800e1e1304789"></a>
## value_unchecked

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::value_unchecked` · arrow-array 59.3.0

```rust
unsafe fn value_unchecked(&self, idx: usize) -> &T::Native
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:337`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the element at index `i` without bounds checking

Note: This method does not check for nulls and the value is arbitrary
if [`is_null`](Self::is_null) returns true for the index.

# Safety

Caller is responsible for ensuring that the index is within the bounds
of the array

<a id="op-663e09df22b74f0f33a9444b"></a>
## views

`function` · `arrow_array::array::byte_view_array::GenericByteViewArray::views` · arrow-array 59.3.0

```rust
fn views(&self) -> &ScalarBuffer<u128>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::array::byte_view_array::GenericByteViewArray", "path": "GenericByteViewArray"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ByteViewType", "path": "ByteViewType"}}}, {"trait_bound": {"generic_params": [], "modifier": "maybe", "trait": {"args": null, "id": "core::marker::Sized", "path": "Sized"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [185, 1], "end": [867, 2], "filename": "src/array/byte_view_array.rs"}, "trait": null, "trait_path": null}`

Source: `src/array/byte_view_array.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the views buffer
