# `arrow_data::byte_view`

Crate `arrow-data` · 4 public items · structured records in [`model/arrow_data.byte_view.json`](../model/arrow_data.byte_view.json)

## MAX_INLINE_VIEW_LEN

`constant` · `arrow_data::byte_view::MAX_INLINE_VIEW_LEN`

Also reachable as `arrow::array::MAX_INLINE_VIEW_LEN`

```rust
const MAX_INLINE_VIEW_LEN: u32 = 12
```

[Full member, field, variant and typed contracts](../operations/arrow_data.byte_view.MAX_INLINE_VIEW_LEN.md).


The maximum number of bytes that can be stored inline in a byte view.

See [`ByteView`] and [`GenericByteViewArray`] for more information on the
layout of the views.

[`GenericByteViewArray`]: https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html

---

## validate_binary_view

`function` · `arrow_data::byte_view::validate_binary_view`

```rust
fn validate_binary_view(views: &[u128], buffers: &[arrow_buffer::Buffer]) -> Result<(), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.byte_view.validate_binary_view.md).


Validates the combination of `views` and `buffers` is a valid BinaryView

---

## validate_string_view

`function` · `arrow_data::byte_view::validate_string_view`

```rust
fn validate_string_view(views: &[u128], buffers: &[arrow_buffer::Buffer]) -> Result<(), arrow_schema::ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_data.byte_view.validate_string_view.md).


Validates the combination of `views` and `buffers` is a valid StringView

---

## ByteView

`struct` · `arrow_data::byte_view::ByteView`

Also reachable as `arrow::array::ByteView`

```rust
struct ByteView
```

**Fields**: `length`, `prefix`, `buffer_index`, `offset`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default

**Methods** (4)

```rust
fn as_u128(self) -> u128
fn new(length: u32, prefix: &[u8]) -> Self
fn with_buffer_index(self, buffer_index: u32) -> Self
fn with_offset(self, offset: u32) -> Self
```

**via `core::convert::From`**

```rust
fn from(value: u128) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_data.byte_view.ByteView.md).


Helper to access views of [`GenericByteViewArray`] (`StringViewArray` and
`BinaryViewArray`) where the length is greater than 12 bytes.

See Also:
* [`GenericByteViewArray`] for more information on the layout of the views.
* [`validate_binary_view`] and [`validate_string_view`] to validate

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

---
