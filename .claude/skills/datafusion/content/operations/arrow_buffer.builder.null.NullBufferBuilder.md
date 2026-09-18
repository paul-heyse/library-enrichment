# `arrow_buffer::builder::null::NullBufferBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.builder.null.NullBufferBuilder.json).

<a id="op-f629e701548be729fb00ab9e"></a>
## NullBufferBuilder

`struct` · `arrow_buffer::builder::null::NullBufferBuilder` · arrow-buffer 59.3.0

```rust
struct NullBufferBuilder
```

Source: `src/builder/null.rs:53`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builder for creating [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48)s (bitmaps indicating validity/nulls).

# See also
* [`BooleanBufferBuilder`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-3b603b0bd38714441734f47c) for a lower-level bitmap builder.
* [`Self::allocated_size`](../operations/arrow_buffer.builder.null.NullBufferBuilder.md#op-1b9cedac41ad9759d35b01b3) for the current memory allocated by the builder.

# Performance

This builder only materializes the buffer when null values (`false`) are
appended. If you only append non-null, (`true`) to the builder, no buffer is
allocated and [`build`](#method.build) or [`finish`](#method.finish) return
`None`.

This optimization is **very** important for the performance as it avoids
allocating memory for the null buffer when there are no nulls.

# Example
```
# use arrow_buffer::NullBufferBuilder;
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(8);
// If no non null values are appended, the null buffer is not created
let buffer = builder.finish();
assert!(buffer.is_none());
// however, if a null value is appended, the null buffer is created
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(7);
builder.append_null();
let buffer = builder.finish().unwrap();
assert_eq!(buffer.len(), 8);
assert_eq!(buffer.iter().collect::<Vec<_>>(), vec![true, true, true, true, true, true, true, false]);
```

<a id="op-1b9cedac41ad9759d35b01b3"></a>
## allocated_size

`function` · `arrow_buffer::builder::null::NullBufferBuilder::allocated_size` · arrow-buffer 59.3.0

```rust
fn allocated_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Return the allocated size of this builder, in bytes, useful for memory accounting.

<a id="op-c3577c179ee68c34b4592798"></a>
## append

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append` · arrow-buffer 59.3.0

```rust
fn append(&mut self, not_null: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a boolean value into the builder.

<a id="op-3f7252c21cb797fb07e72224"></a>
## append_buffer

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append_buffer` · arrow-buffer 59.3.0

```rust
fn append_buffer(&mut self, buffer: &NullBuffer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:195`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Append [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) to this [`NullBufferBuilder`](../operations/arrow_buffer.builder.null.NullBufferBuilder.md#op-f629e701548be729fb00ab9e)

This is useful when you want to concatenate two null buffers.

<a id="op-da4a719229d1d1986bcf35c9"></a>
## append_n_non_nulls

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append_n_non_nulls` · arrow-buffer 59.3.0

```rust
fn append_n_non_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:105`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends `n` `true`s into the builder
to indicate that these `n` items are not nulls.

<a id="op-d5666b32067599d8ec411d8b"></a>
## append_n_nulls

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append_n_nulls` · arrow-buffer 59.3.0

```rust
fn append_n_nulls(&mut self, n: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends `n` `false`s into the builder
to indicate that these `n` items are nulls.

<a id="op-87e13cccaf1068f1ef561f70"></a>
## append_non_null

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append_non_null` · arrow-buffer 59.3.0

```rust
fn append_non_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:116`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a `true` into the builder
to indicate that this item is not null.

<a id="op-f6522503aa0fc1313795c2bd"></a>
## append_null

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append_null` · arrow-buffer 59.3.0

```rust
fn append_null(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a `false` into the builder
to indicate that this item is null.

<a id="op-a5f1ee744c569bf636fdee84"></a>
## append_slice

`function` · `arrow_buffer::builder::null::NullBufferBuilder::append_slice` · arrow-buffer 59.3.0

```rust
fn append_slice(&mut self, slice: &[bool])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:181`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a boolean slice into the builder
to indicate the validations of these items.

<a id="op-9282568b2275120081b03f06"></a>
## as_slice

`function` · `arrow_buffer::builder::null::NullBufferBuilder::as_slice` · arrow-buffer 59.3.0

```rust
fn as_slice(&self) -> Option<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:229`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the inner bitmap builder as slice

<a id="op-549e51c60c794c29fe4e1554"></a>
## as_slice_mut

`function` · `arrow_buffer::builder::null::NullBufferBuilder::as_slice_mut` · arrow-buffer 59.3.0

```rust
fn as_slice_mut(&mut self) -> Option<&mut [u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:249`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Return a mutable reference to the inner bitmap slice.

<a id="op-fa097313492e355cfccfe8f9"></a>
## build

`function` · `arrow_buffer::builder::null::NullBufferBuilder::build` · arrow-buffer 59.3.0

```rust
fn build(self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:218`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds the [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) without resetting the builder.

This consumes the builder. Use [`Self::finish`](../operations/arrow_buffer.builder.null.NullBufferBuilder.md#op-c552a9fcb1e7294309604c53) to reuse it.

<a id="op-c552a9fcb1e7294309604c53"></a>
## finish

`function` · `arrow_buffer::builder::null::NullBufferBuilder::finish` · arrow-buffer 59.3.0

```rust
fn finish(&mut self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:210`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds the [`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) and resets the builder.

Returns `None` if the builder only contains `true`s. Use [`Self::build`](../operations/arrow_buffer.builder.null.NullBufferBuilder.md#op-fa097313492e355cfccfe8f9)
when you don't need to reuse this builder.

<a id="op-39b847ef14320c3f8a9f0b49"></a>
## finish_cloned

`function` · `arrow_buffer::builder::null::NullBufferBuilder::finish_cloned` · arrow-buffer 59.3.0

```rust
fn finish_cloned(&self) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds the [NullBuffer](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) without resetting the builder.

<a id="op-01b46166bd61a7cd87b4749a"></a>
## fmt

`function` · `arrow_buffer::builder::null::NullBufferBuilder::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/builder/null.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/null.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-511c50be7710fd2c63c1d2c8"></a>
## is_empty

`function` · `arrow_buffer::builder::null::NullBufferBuilder::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Check if the builder is empty.

<a id="op-bae0292f9ee8613cdc3d65e4"></a>
## is_valid

`function` · `arrow_buffer::builder::null::NullBufferBuilder::is_valid` · arrow-buffer 59.3.0

```rust
fn is_valid(&self, index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Gets a bit in the buffer at `index`

<a id="op-158d413a952caab6e192bf1d"></a>
## len

`function` · `arrow_buffer::builder::null::NullBufferBuilder::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:262`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Return the number of bits in the buffer.

<a id="op-0bf1fc4a6aa0b0ee8baca7fe"></a>
## new

`function` · `arrow_buffer::builder::null::NullBufferBuilder::new` · arrow-buffer 59.3.0

```rust
fn new(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new empty builder.

Note that this method does not allocate any memory, regardless of the
`capacity` parameter. If an allocation is required, `capacity` is the
size in bits (not bytes) that will be allocated at minimum.

<a id="op-e473780c16854be5ab8d5c43"></a>
## new_from_buffer

`function` · `arrow_buffer::builder::null::NullBufferBuilder::new_from_buffer` · arrow-buffer 59.3.0

```rust
fn new_from_buffer(buffer: MutableBuffer, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new builder from a `MutableBuffer`.

<a id="op-7bdc04f7e09e24877f99f67d"></a>
## new_with_len

`function` · `arrow_buffer::builder::null::NullBufferBuilder::new_with_len` · arrow-buffer 59.3.0

```rust
fn new_with_len(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new builder with given length.

<a id="op-52ef42889f8cc0e15e5933d4"></a>
## set_bit

`function` · `arrow_buffer::builder::null::NullBufferBuilder::set_bit` · arrow-buffer 59.3.0

```rust
fn set_bit(&mut self, index: usize, v: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:152`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Sets a bit in the builder at `index`

<a id="op-f527dc879b72beb8dfb770b6"></a>
## truncate

`function` · `arrow_buffer::builder::null::NullBufferBuilder::truncate` · arrow-buffer 59.3.0

```rust
fn truncate(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::null::NullBufferBuilder", "path": "NullBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [270, 2], "filename": "src/builder/null.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/null.rs:171`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Truncates the builder to the given length

If `len` is greater than the buffer's current length, this has no effect
