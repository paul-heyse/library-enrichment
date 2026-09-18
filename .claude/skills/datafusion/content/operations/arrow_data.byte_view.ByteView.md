# `arrow_data::byte_view::ByteView`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.byte_view.ByteView.json).

<a id="op-f50179c7bea821afbdfdb4ae"></a>
## ByteView

`struct` · `arrow_data::byte_view::ByteView` · arrow-data 59.3.0

```rust
struct ByteView
```

Source: `src/byte_view.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Helper to access views of [`GenericByteViewArray`] (`StringViewArray` and
`BinaryViewArray`) where the length is greater than 12 bytes.

See Also:
* [`GenericByteViewArray`] for more information on the layout of the views.
* [`validate_binary_view`](../operations/arrow_data.byte_view.validate_binary_view.md#op-80cb33b4c74239e085b8c230) and [`validate_string_view`](../operations/arrow_data.byte_view.validate_string_view.md#op-e3c4b9595ac60d42def55c44) to validate

# Example: Create a new u128 view

```rust
# use arrow_data::ByteView;;
// Create a view for a string of length 20
// first four bytes are "Rust"
// stored in buffer 3
// at offset 42
let prefix = "Rust";
let view = ByteView::new(20, prefix.as_bytes())
  .with_buffer_index(3)
  .with_offset(42);

// create the final u128
let v = view.as_u128();
assert_eq!(v, 0x2a000000037473755200000014);
```

# Example: decode a `u128` into its constituent fields
```rust
# use arrow_data::ByteView;
// Convert a u128 to a ByteView
// See validate_{string,binary}_view functions to validate
let v = ByteView::from(0x2a000000037473755200000014);

assert_eq!(v.length, 20);
assert_eq!(v.prefix, 0x74737552);
assert_eq!(v.buffer_index, 3);
assert_eq!(v.offset, 42);
```

[`GenericByteViewArray`]: https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html

<a id="op-cb4bd7009f7fc5d930b79611"></a>
## as_u128

`function` · `arrow_data::byte_view::ByteView::as_u128` · arrow-data 59.3.0

```rust
fn as_u128(self) -> u128
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [126, 2], "filename": "src/byte_view.rs"}, "trait": null, "trait_path": null}`

Source: `src/byte_view.rs:120`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Convert `ByteView` to `u128` by concatenating the fields

<a id="op-924e840bdc8286ddc9c9654d"></a>
## buffer_index

`struct_field` · `arrow_data::byte_view::ByteView::buffer_index` · arrow-data 59.3.0

```rust
buffer_index: u32
```

Source: `src/byte_view.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

The buffer index.

<a id="op-8f52454f71ad579ad1717b5f"></a>
## clone

`function` · `arrow_data::byte_view::ByteView::clone` · arrow-data 59.3.0

```rust
fn clone(&self) -> ByteView
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 23], "end": [68, 28], "filename": "src/byte_view.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/byte_view.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a5d7150cde5547a279c209f"></a>
## default

`function` · `arrow_data::byte_view::ByteView::default` · arrow-data 59.3.0

```rust
fn default() -> ByteView
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 30], "end": [68, 37], "filename": "src/byte_view.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/byte_view.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ecbf48f12f1c2e522fb8342"></a>
## fmt

`function` · `arrow_data::byte_view::ByteView::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/byte_view.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/byte_view.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c970e527a7ec97fb0868909"></a>
## from

`function` · `arrow_data::byte_view::ByteView::from` · arrow-data 59.3.0

```rust
fn from(value: u128) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [138, 2], "filename": "src/byte_view.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u128"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/byte_view.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7631e798fda6b852b552540"></a>
## length

`struct_field` · `arrow_data::byte_view::ByteView::length` · arrow-data 59.3.0

```rust
length: u32
```

Source: `src/byte_view.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

The length of the string/bytes.

<a id="op-5aa540b25077558202cd2aaa"></a>
## new

`function` · `arrow_data::byte_view::ByteView::new` · arrow-data 59.3.0

```rust
fn new(length: u32, prefix: &[u8]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [126, 2], "filename": "src/byte_view.rs"}, "trait": null, "trait_path": null}`

Source: `src/byte_view.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Construct a [`ByteView`](../operations/arrow_data.byte_view.ByteView.md#op-f50179c7bea821afbdfdb4ae) for data `length` of bytes with the specified prefix.

See example on [`ByteView`](../operations/arrow_data.byte_view.ByteView.md#op-f50179c7bea821afbdfdb4ae) docs

Notes:
* the length should always be greater than [`MAX_INLINE_VIEW_LEN`](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md#op-cc5752925eeccf8fdeefe7ed)
  (Data less than 12 bytes is stored as an inline view)
* buffer and offset are set to `0`

# Panics
If the prefix is not exactly 4 bytes

<a id="op-502c6d41306a07ddd0ea73d1"></a>
## offset

`struct_field` · `arrow_data::byte_view::ByteView::offset` · arrow-data 59.3.0

```rust
offset: u32
```

Source: `src/byte_view.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

The offset into the buffer.

<a id="op-5c7d054208ede2576a986e0c"></a>
## prefix

`struct_field` · `arrow_data::byte_view::ByteView::prefix` · arrow-data 59.3.0

```rust
prefix: u32
```

Source: `src/byte_view.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

First 4 bytes of string/bytes data.

<a id="op-08452dbda553a1a3e167832b"></a>
## with_buffer_index

`function` · `arrow_data::byte_view::ByteView::with_buffer_index` · arrow-data 59.3.0

```rust
fn with_buffer_index(self, buffer_index: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [126, 2], "filename": "src/byte_view.rs"}, "trait": null, "trait_path": null}`

Source: `src/byte_view.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Set the [`Self::buffer_index`](../operations/arrow_data.byte_view.ByteView.md#op-924e840bdc8286ddc9c9654d) field

<a id="op-4c27d5f33ce2350a049596b0"></a>
## with_offset

`function` · `arrow_data::byte_view::ByteView::with_offset` · arrow-data 59.3.0

```rust
fn with_offset(self, offset: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::byte_view::ByteView", "path": "ByteView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [126, 2], "filename": "src/byte_view.rs"}, "trait": null, "trait_path": null}`

Source: `src/byte_view.rs:113`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Set the [`Self::offset`](../operations/arrow_data.byte_view.ByteView.md#op-502c6d41306a07ddd0ea73d1) field
