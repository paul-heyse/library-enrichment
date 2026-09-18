# `arrow_data::data::ArrayData`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.data.ArrayData.json).

<a id="op-5666976474a0e5276141ce78"></a>
## ArrayData

`struct` · `arrow_data::data::ArrayData` · arrow-data 59.3.0

```rust
struct ArrayData
```

Source: `src/data.rs:208`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

A generic representation of Arrow array data which encapsulates common attributes
and operations for Arrow array.

Specific operations for different arrays types (e.g., primitive, list, struct)
are implemented in `Array`.

# Memory Layout

`ArrayData` has references to one or more underlying data buffers
and optional child ArrayData, depending on type as illustrated
below. Bitmaps are not shown for simplicity but they are stored
similarly to the buffers.

```text
                       offset
                      points to
┌───────────────────┐ start of  ┌───────┐       Different
│                   │   data    │       │     ArrayData may
│ArrayData {        │           │....   │     also refers to
│  data_type: ...   │   ─ ─ ─ ─▶│1234   │  ┌ ─  the same
│  offset: ... ─ ─ ─│─ ┘        │4372   │      underlying
│  len: ...    ─ ─ ─│─ ┐        │4888   │  │     buffer with different offset/len
│  buffers: [       │           │5882   │◀─
│    ...            │  │        │4323   │
│  ]                │   ─ ─ ─ ─▶│4859   │
│  child_data: [    │           │....   │
│    ...            │           │       │
│  ]                │           └───────┘
│}                  │
│                   │            Shared Buffer uses
│               │   │            bytes::Bytes to hold
└───────────────────┘            actual data values
          ┌ ─ ─ ┘

          ▼
┌───────────────────┐
│ArrayData {        │
│  ...              │
│}                  │
│                   │
└───────────────────┘

Child ArrayData may also have its own buffers and children
```

<a id="op-5ed5dad6435e96951ca1aa72"></a>
## align_buffers

`function` · `arrow_data::data::ArrayData::align_buffers` · arrow-data 59.3.0

```rust
fn align_buffers(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:847`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Verifies that the buffers meet the minimum alignment requirements for the data type

Buffers that are not adequately aligned will be copied to a new aligned allocation

This can be useful for when interacting with data sent over IPC or FFI, that may
not meet the minimum alignment requirements

This also aligns buffers of children data

<a id="op-0feaa310f1f7cdd45f101360"></a>
## buffer

`function` · `arrow_data::data::ArrayData::buffer` · arrow-data 59.3.0

```rust
fn buffer<T: ArrowNativeType>(&self, buffer: usize) -> &[T]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:663`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the `buffer` as a slice of type `T` starting at self.offset

# Panics
This function panics if:
* the buffer is not byte-aligned with type T, or
* the datatype is `Boolean` (it corresponds to a bit-packed buffer where the offset is not applicable)

<a id="op-bc163067fd7278cb5e95cae1"></a>
## buffers

`function` · `arrow_data::data::ArrayData::buffers` · arrow-data 59.3.0

```rust
fn buffers(&self) -> &[Buffer]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:429`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) storing data for this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-c22efa19480d0d3d69b41196"></a>
## builder

`function` · `arrow_data::data::ArrayData::builder` · arrow-data 59.3.0

```rust
const fn builder(data_type: DataType) -> ArrayDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:418`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a builder to construct a [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) instance of the same [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)

<a id="op-feeb89ed63791699897df841"></a>
## child_data

`function` · `arrow_data::data::ArrayData::child_data` · arrow-data 59.3.0

```rust
fn child_data(&self) -> &[ArrayData]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:435`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a slice of children [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78). This will be non
empty for type such as lists and structs.

<a id="op-a6b4a9f9fde8600b7c120a22"></a>
## claim

`function` · `arrow_data::data::ArrayData::claim` · arrow-data 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1767`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Claim memory used by this ArrayData in the provided memory pool.

This claims memory for:
- All buffers in self.buffers
- All child ArrayData recursively
- The null buffer if present

<a id="op-e2c3c7a6375b69038033dc17"></a>
## clone

`function` · `arrow_data::data::ArrayData::clone` · arrow-data 59.3.0

```rust
fn clone(&self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 17], "end": [207, 22], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/data.rs:207`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41f4143d44c0d9de3262a431"></a>
## data_type

`function` · `arrow_data::data::ArrayData::data_type` · arrow-data 59.3.0

```rust
const fn data_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:424`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a reference to the [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c) of this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-cada56aa894877e0ec802734"></a>
## eq

`function` · `arrow_data::data::ArrayData::eq` · arrow-data 59.3.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2014, 1], "end": [2018, 2], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data.rs:2015`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67f3b17a10155e816a062dfc"></a>
## fmt

`function` · `arrow_data::data::ArrayData::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 10], "end": [207, 15], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data.rs:207`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c876041a7600ad2f320a9484"></a>
## get_array_memory_size

`function` · `arrow_data::data::ArrayData::get_array_memory_size` · arrow-data 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:599`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the total number of bytes of memory occupied
physically by this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) and all its [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s and
children. (See also diagram on [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)).

Equivalent to:
 `size_of_val(self)` +
 [`Self::get_buffer_memory_size`](../operations/arrow_data.data.ArrayData.md#op-304dc8b6ef115a2de3a78588) +
 `size_of_val(child)` for all children

<a id="op-304dc8b6ef115a2de3a78588"></a>
## get_buffer_memory_size

`function` · `arrow_data::data::ArrayData::get_buffer_memory_size` · arrow-data 59.3.0

```rust
fn get_buffer_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:500`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the total number of bytes of memory occupied by the
buffers owned by this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) and all of its
children. (See also diagram on [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)).

Note that this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) may only refer to a subset of the
data in the underlying [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s (due to `offset` and
`length`), but the size returned includes the entire size of
the buffers.

If multiple [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)s refer to the same underlying
[`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s they will both report the same size.

<a id="op-158c61f3d160ef3e6f1bc42d"></a>
## get_slice_memory_size

`function` · `arrow_data::data::ArrayData::get_slice_memory_size` · arrow-data 59.3.0

```rust
fn get_slice_memory_size(&self) -> Result<usize, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:526`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the total number of the bytes of memory occupied by
the buffers by this slice of [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) (See also diagram on [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)).

This is approximately the number of bytes if a new
[`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) was formed by creating new [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s with
exactly the data needed.

For example, a [`DataType::Int64`](../operations/arrow_schema.datatype.DataType.md#op-b5e46464bd4cab7092efa1ed) with `100` elements,
[`Self::get_slice_memory_size`](../operations/arrow_data.data.ArrayData.md#op-158c61f3d160ef3e6f1bc42d) would return `100 * 8 = 800`. If
the [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) was then [`Self::slice`](../operations/arrow_data.data.ArrayData.md#op-dbef994babb47a15c02ddfa4)ed to refer to its
first `20` elements, then [`Self::get_slice_memory_size`](../operations/arrow_data.data.ArrayData.md#op-158c61f3d160ef3e6f1bc42d) on the
sliced [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) would return `20 * 8 = 160`.

<a id="op-1d76cb8a52dc671af6ed2274"></a>
## into_builder

`function` · `arrow_data::data::ArrayData::into_builder` · arrow-data 59.3.0

```rust
fn into_builder(self) -> ArrayDataBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1756`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Converts this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) into an [`ArrayDataBuilder`](../operations/arrow_data.data.ArrayDataBuilder.md#op-0a0f2a3a6b77674c3513d310)

<a id="op-e10e8ad2af4d3f2e6e50eca9"></a>
## into_parts

`function` · `arrow_data::data::ArrayData::into_parts` · arrow-data 59.3.0

```rust
fn into_parts(self) -> (DataType, usize, Option<NullBuffer>, usize, Vec<Buffer>, Vec<ArrayData>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:394`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Return the constituent parts of this ArrayData

This is the inverse of [`ArrayData::try_new`](../operations/arrow_data.data.ArrayData.md#op-99b200b063542608c0189bd6).

Returns `(data_type, len, nulls, offset, buffers, child_data)`

<a id="op-b203ca66184c24083f5e917f"></a>
## is_empty

`function` · `arrow_data::data::ArrayData::is_empty` · arrow-data 59.3.0

```rust
const fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:470`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns whether this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) is empty

<a id="op-3abcbde591515671ad340e08"></a>
## is_null

`function` · `arrow_data::data::ArrayData::is_null` · arrow-data 59.3.0

```rust
fn is_null(&self, i: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:441`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns whether the element at index `i` is null

<a id="op-ef0a4cacc4fb6a9f50aa0d25"></a>
## is_valid

`function` · `arrow_data::data::ArrayData::is_valid` · arrow-data 59.3.0

```rust
fn is_valid(&self, i: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:458`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns whether the element at index `i` is not null

<a id="op-3a302998aa67b6a07fba1472"></a>
## len

`function` · `arrow_data::data::ArrayData::len` · arrow-data 59.3.0

```rust
const fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:464`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the length (i.e., number of elements) of this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78).

<a id="op-5365c99fce9e18c354697cd7"></a>
## new_empty

`function` · `arrow_data::data::ArrayData::new_empty` · arrow-data 59.3.0

```rust
fn new_empty(data_type: &DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:835`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a new empty [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) valid for `data_type`.

<a id="op-f87d1dcb540a8be70b5f6406"></a>
## new_null

`function` · `arrow_data::data::ArrayData::new_null` · arrow-data 59.3.0

```rust
fn new_null(data_type: &DataType, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:672`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a new [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) valid for `data_type` containing `len` null values

# Panics
This function panics if:
* the datatype `data_type` has incorrect layout

<a id="op-58db8f9c56a83105ec11830c"></a>
## new_unchecked

`function` · `arrow_data::data::ArrayData::new_unchecked` · arrow-data 59.3.0

```rust
unsafe fn new_unchecked(data_type: DataType, len: usize, null_count: Option<usize>, null_bit_buffer: Option<Buffer>, offset: usize, buffers: Vec<Buffer>, child_data: Vec<ArrayData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:288`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Create a new ArrayData instance;

If `null_count` is not specified, the number of nulls in
null_bit_buffer is calculated.

If the number of nulls is 0 then the null_bit_buffer
is set to `None`.

# Safety

The input values *must* form a valid Arrow array for
`data_type`, or undefined behavior can result.

Note: This is a low level API and most users of the arrow
crate should create arrays using the methods in the `array`
module.

<a id="op-a2a1080fb3d5d433f7910365"></a>
## null_count

`function` · `arrow_data::data::ArrayData::null_count` · arrow-data 59.3.0

```rust
fn null_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:482`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the total number of nulls in this array

<a id="op-c68baf36217560d054504010"></a>
## nulls

`function` · `arrow_data::data::ArrayData::nulls` · arrow-data 59.3.0

```rust
fn nulls(&self) -> Option<&NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:452`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns a reference to the null buffer of this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) if any

Note: [`ArrayData::offset`](../operations/arrow_data.data.ArrayData.md#op-f8a3b841ade62c547b838c05) does NOT apply to the returned [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)

<a id="op-f8a3b841ade62c547b838c05"></a>
## offset

`function` · `arrow_data::data::ArrayData::offset` · arrow-data 59.3.0

```rust
const fn offset(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:476`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns the offset of this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-2c1809ccd51cb86a0d6bb5fc"></a>
## ptr_eq

`function` · `arrow_data::data::ArrayData::ptr_eq` · arrow-data 59.3.0

```rust
fn ptr_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1724`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Returns true if this `ArrayData` is equal to `other`, using pointer comparisons
to determine buffer equality. This is cheaper than `PartialEq::eq` but may
return false when the arrays are logically equal

<a id="op-dbef994babb47a15c02ddfa4"></a>
## slice

`function` · `arrow_data::data::ArrayData::slice` · arrow-data 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:624`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Creates a zero-copy slice of itself. This creates a new
[`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) pointing at the same underlying [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s with a
different offset and len

# Panics

Panics if `offset + length` overflows or is greater than `self.len()`.

<a id="op-99b200b063542608c0189bd6"></a>
## try_new

`function` · `arrow_data::data::ArrayData::try_new` · arrow-data 59.3.0

```rust
fn try_new(data_type: DataType, len: usize, null_bit_buffer: Option<Buffer>, offset: usize, buffers: Vec<Buffer>, child_data: Vec<ArrayData>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:324`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Create a new ArrayData, validating that the provided buffers form a valid
Arrow array of the specified data type.

If the number of nulls in `null_bit_buffer` is 0 then the null_bit_buffer
is set to `None`.

Internally this calls through to [`Self::validate_data`](../operations/arrow_data.data.ArrayData.md#op-d7715b55aa96799db7ab8fc9)

Note: This is a low level API and most users of the arrow crate should create
arrays using the builders found in [arrow_array](https://docs.rs/arrow-array)
or [`ArrayDataBuilder`](../operations/arrow_data.data.ArrayDataBuilder.md#op-0a0f2a3a6b77674c3513d310).

See also [`Self::into_parts`](../operations/arrow_data.data.ArrayData.md#op-e10e8ad2af4d3f2e6e50eca9) to recover the fields

<a id="op-519d22b78f14fab70c684476"></a>
## validate

`function` · `arrow_data::data::ArrayData::validate` · arrow-data 59.3.0

```rust
fn validate(&self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:872`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

"cheap" validation of an `ArrayData`. Ensures buffers are
sufficiently sized to store `len` + `offset` total elements of
`data_type` and performs other inexpensive consistency checks.

This check is "cheap" in the sense that it does not validate the
contents of the buffers (e.g. that all offsets for UTF8 arrays
are within the bounds of the values buffer).

See [ArrayData::validate_data](../operations/arrow_data.data.ArrayData.md#op-d7715b55aa96799db7ab8fc9) to validate fully the offset content
and the validity of utf8 data

<a id="op-d7715b55aa96799db7ab8fc9"></a>
## validate_data

`function` · `arrow_data::data::ArrayData::validate_data` · arrow-data 59.3.0

```rust
fn validate_data(&self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1367`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validate that the data contained within this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) is valid

1. Null count is correct
2. All offsets are valid
3. All String data is valid UTF-8
4. All dictionary offsets are valid

Internally this calls:

* [`Self::validate`](../operations/arrow_data.data.ArrayData.md#op-519d22b78f14fab70c684476)
* [`Self::validate_nulls`](../operations/arrow_data.data.ArrayData.md#op-5d7e546f125f4fd18c858eaf)
* [`Self::validate_values`](../operations/arrow_data.data.ArrayData.md#op-5969aa148375ad230a164a06)

Note: this does not recurse into children, for a recursive variant
see [`Self::validate_full`](../operations/arrow_data.data.ArrayData.md#op-63536abaf605137241d35aff)

<a id="op-63536abaf605137241d35aff"></a>
## validate_full

`function` · `arrow_data::data::ArrayData::validate_full` · arrow-data 59.3.0

```rust
fn validate_full(&self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1379`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Performs a full recursive validation of this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) and all its children

This is equivalent to calling [`Self::validate_data`](../operations/arrow_data.data.ArrayData.md#op-d7715b55aa96799db7ab8fc9) on this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)
and all its children recursively

<a id="op-5d7e546f125f4fd18c858eaf"></a>
## validate_nulls

`function` · `arrow_data::data::ArrayData::validate_nulls` · arrow-data 59.3.0

```rust
fn validate_nulls(&self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1405`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates the values stored within this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) are valid
without recursing into child [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

Does not (yet) check
1. Union type_ids are valid see [#85](https://github.com/apache/arrow-rs/issues/85)
2. the the null count is correct and that any
3. nullability requirements of its children are correct

[#85]: https://github.com/apache/arrow-rs/issues/85

<a id="op-5969aa148375ad230a164a06"></a>
## validate_values

`function` · `arrow_data::data::ArrayData::validate_values` · arrow-data 59.3.0

```rust
fn validate_values(&self) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [271, 1], "end": [1783, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:1486`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates the values stored within this [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) are valid
without recursing into child [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

Does not (yet) check
1. Union type_ids are valid see [#85](https://github.com/apache/arrow-rs/issues/85)
