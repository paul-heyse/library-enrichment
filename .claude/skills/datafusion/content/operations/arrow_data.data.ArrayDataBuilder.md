# `arrow_data::data::ArrayDataBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.data.ArrayDataBuilder.json).

<a id="op-0a0f2a3a6b77674c3513d310"></a>
## ArrayDataBuilder

`struct` · `arrow_data::data::ArrayDataBuilder` · arrow-data 59.3.0

```rust
struct ArrayDataBuilder
```

Source: `src/data.rs:2081`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Builder for [`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) type

<a id="op-f25cfec5a58f73d2f69d6de9"></a>
## add_buffer

`function` · `arrow_data::data::ArrayDataBuilder::add_buffer` · arrow-data 59.3.0

```rust
fn add_buffer(self, b: Buffer) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2172`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Adds a single buffer to the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)'s buffers

<a id="op-00df8da4136c26b8b9ba4827"></a>
## add_buffers

`function` · `arrow_data::data::ArrayDataBuilder::add_buffers` · arrow-data 59.3.0

```rust
fn add_buffers<I: IntoIterator<Item = Buffer>>(self, bs: I) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2178`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Adds multiple buffers to the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)'s buffers

<a id="op-f20654f974c7797b02187a20"></a>
## add_child_data

`function` · `arrow_data::data::ArrayDataBuilder::add_child_data` · arrow-data 59.3.0

```rust
fn add_child_data(self, r: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2190`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Adds a single child data to the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)'s child data

<a id="op-279374e0536f44ca21884937"></a>
## align_buffers

`function` · `arrow_data::data::ArrayDataBuilder::align_buffers` · arrow-data 59.3.0

```rust
fn align_buffers(self, align_buffers: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2285`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Ensure that all buffers are aligned, copying data if necessary

Rust requires that arrays are aligned to their corresponding primitive,
see [`Layout::array`](std::alloc::Layout::array) and [`std::mem::align_of`].

[`ArrayData`](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78) therefore requires that all buffers have at least this alignment,
to allow for [slice](std::slice) based APIs. See [`BufferSpec::FixedWidth`](../operations/arrow_data.data.BufferSpec.md#op-1d156935023f6782fe3c438f).

As this alignment is architecture specific, and not guaranteed by all arrow implementations,
this flag is provided to automatically copy buffers to a new correctly aligned allocation
when necessary, making it useful when interacting with buffers produced by other systems,
e.g. IPC or FFI.

If this flag is not enabled, `[Self::build`] return an error on encountering
insufficiently aligned buffers.

Unresolved upstream links (retained, not inferred): ``std::mem::align_of``, `std::slice`, `std::alloc::Layout::array`.

<a id="op-7e0d6ed978ed8029b2d94932"></a>
## buffers

`function` · `arrow_data::data::ArrayDataBuilder::buffers` · arrow-data 59.3.0

```rust
fn buffers(self, v: Vec<Buffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2166`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the buffers of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-2c075cb7ad1597591c8c13e5"></a>
## build

`function` · `arrow_data::data::ArrayDataBuilder::build` · arrow-data 59.3.0

```rust
fn build(self) -> Result<ArrayData, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2222`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Creates an `ArrayData`, consuming `self`

# Safety

By default the underlying buffers are checked to ensure they are valid
Arrow data. However, if the [`Self::skip_validation`](../operations/arrow_data.data.ArrayDataBuilder.md#op-e6301d6c2e631c515af13cbb) flag has been set
to true (by the `unsafe` API) this validation is skipped. If the data is
not valid, undefined behavior will result.

<a id="op-2b069b23efccabe7bac0f999"></a>
## build_unchecked

`function` · `arrow_data::data::ArrayDataBuilder::build_unchecked` · arrow-data 59.3.0

```rust
unsafe fn build_unchecked(self) -> ArrayData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2210`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Creates an array data, without any validation

Note: This is shorthand for
```rust
# #[expect(unsafe_op_in_unsafe_fn)]
# let mut builder = arrow_data::ArrayDataBuilder::new(arrow_schema::DataType::Null);
# let _ = unsafe {
builder.skip_validation(true).build().unwrap()
# };
```

# Safety

The same caveats as [`ArrayData::new_unchecked`](../operations/arrow_data.data.ArrayData.md#op-58db8f9c56a83105ec11830c)
apply.

<a id="op-bdc0ddb8947792f61a8311c8"></a>
## child_data

`function` · `arrow_data::data::ArrayDataBuilder::child_data` · arrow-data 59.3.0

```rust
fn child_data(self, v: Vec<ArrayData>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2184`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the child data of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-6572ae19c1c621442e60a6d8"></a>
## data_type

`function` · `arrow_data::data::ArrayDataBuilder::data_type` · arrow-data 59.3.0

```rust
fn data_type(self, data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2125`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Creates a new array data builder from an existing one, changing the data type

<a id="op-aa64ca4714e69b6e9045ae41"></a>
## fmt

`function` · `arrow_data::data::ArrayDataBuilder::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2080, 10], "end": [2080, 15], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data.rs:2080`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb1b23a13f38ea2801b6c15e"></a>
## from

`function` · `arrow_data::data::ArrayDataBuilder::from` · arrow-data 59.3.0

```rust
fn from(d: ArrayData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2311, 1], "end": [2326, 2], "filename": "src/data.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/data.rs:2312`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e7430519bb8795f0d5c6c4"></a>
## len

`function` · `arrow_data::data::ArrayDataBuilder::len` · arrow-data 59.3.0

```rust
const fn len(self, n: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2132`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the length of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-e983a180c4d8b61e8a575c6a"></a>
## new

`function` · `arrow_data::data::ArrayDataBuilder::new` · arrow-data 59.3.0

```rust
const fn new(data_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2109`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Creates a new array data builder

<a id="op-1aba4023afce9ef4e3f0f12e"></a>
## null_bit_buffer

`function` · `arrow_data::data::ArrayDataBuilder::null_bit_buffer` · arrow-data 59.3.0

```rust
fn null_bit_buffer(self, buf: Option<Buffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2152`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the `null_bit_buffer` of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-77c112649520557a9e80b11d"></a>
## null_count

`function` · `arrow_data::data::ArrayDataBuilder::null_count` · arrow-data 59.3.0

```rust
fn null_count(self, null_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2146`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the null count of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-a6133f1b65734e74b7f56cc2"></a>
## nulls

`function` · `arrow_data::data::ArrayDataBuilder::nulls` · arrow-data 59.3.0

```rust
fn nulls(self, nulls: Option<NullBuffer>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2138`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the null buffer of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-2406f95c4166edac6b19fc84"></a>
## offset

`function` · `arrow_data::data::ArrayDataBuilder::offset` · arrow-data 59.3.0

```rust
const fn offset(self, n: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2160`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Sets the offset of the [ArrayData](../operations/arrow_data.data.ArrayData.md#op-5666976474a0e5276141ce78)

<a id="op-e6301d6c2e631c515af13cbb"></a>
## skip_validation

`function` · `arrow_data::data::ArrayDataBuilder::skip_validation` · arrow-data 59.3.0

```rust
unsafe fn skip_validation(self, skip_validation: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::ArrayDataBuilder", "path": "ArrayDataBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2106, 1], "end": [2309, 2], "filename": "src/data.rs"}, "trait": null, "trait_path": null}`

Source: `src/data.rs:2303`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Skips validation of the data.

If this flag is enabled, `[Self::build`] will skip validation of the
data

If this flag is not enabled, `[Self::build`] will validate that all
buffers are valid and will return an error if any data is invalid.
Validation can be expensive.

# Safety

If validation is skipped, the buffers must form a valid Arrow array,
otherwise undefined behavior will result
