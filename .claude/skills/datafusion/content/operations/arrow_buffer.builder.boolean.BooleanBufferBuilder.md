# `arrow_buffer::builder::boolean::BooleanBufferBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.builder.boolean.BooleanBufferBuilder.json).

<a id="op-3b603b0bd38714441734f47c"></a>
## BooleanBufferBuilder

`struct` · `arrow_buffer::builder::boolean::BooleanBufferBuilder` · arrow-buffer 59.3.0

```rust
struct BooleanBufferBuilder
```

Source: `src/builder/boolean.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

 Builder for [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)

 Builds a packed buffer of bits representing boolean values. Each bit in the
 buffer corresponds to a boolean value,

 # See Also

 * [`NullBufferBuilder`] for building [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5)s for representing nulls
 * [`BufferBuilder`] for building [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b)s

 # Example
 ```
 # use arrow_buffer::builder::BooleanBufferBuilder;
 let mut builder = BooleanBufferBuilder::new(10);
 builder.append(true);
 builder.append(false);
 builder.append_n(3, true); // append 3 trues
 let buffer = builder.build();
 assert_eq!(buffer.len(), 5); // 5 bits appended
 assert_eq!(buffer.values(), &[0b00011101_u8]); // packed bits
```

 [`BufferBuilder`]: crate::builder::BufferBuilder
 [`NullBufferBuilder`]: crate::builder::NullBufferBuilder

<a id="op-5c608814c66c2e421eb0fd09"></a>
## advance

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::advance` · arrow-buffer 59.3.0

```rust
fn advance(&mut self, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Advances the buffer by `additional` bits

<a id="op-63bcc4e65530dd5c6ab82e0f"></a>
## append

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::append` · arrow-buffer 59.3.0

```rust
fn append(&mut self, v: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:182`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a boolean `v` into the buffer

<a id="op-acbaae68b4729208f87046e3"></a>
## append_buffer

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::append_buffer` · arrow-buffer 59.3.0

```rust
fn append_buffer(&mut self, buffer: &BooleanBuffer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:292`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Append [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) to this [`BooleanBufferBuilder`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-3b603b0bd38714441734f47c)

<a id="op-8c851f4a5adbd0ef5e0c7f59"></a>
## append_n

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::append_n` · arrow-buffer 59.3.0

```rust
fn append_n(&mut self, additional: usize, v: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends n `additional` bits of value `v` into the buffer

<a id="op-9e9d8859bbf627f6cbb034ac"></a>
## append_packed_range

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::append_packed_range` · arrow-buffer 59.3.0

```rust
fn append_packed_range(&mut self, range: Range<usize>, to_set: &[u8])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:275`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Append `range` bits from `to_set`

`to_set` is a slice of bits packed LSB-first into `[u8]`

# Panics

Panics if `to_set` does not contain `ceil(range.end / 8)` bytes

<a id="op-cdbc5f0cef1dd7538e9795d5"></a>
## append_slice

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::append_slice` · arrow-buffer 59.3.0

```rust
fn append_slice(&mut self, slice: &[bool])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends a slice of booleans into the buffer

<a id="op-708150c0ca8f94a2df093071"></a>
## append_word

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::append_word` · arrow-buffer 59.3.0

```rust
fn append_word(&mut self, word: u64, count: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:197`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Appends the low `count` bits from `word` into the buffer.

`word` is treated as a packed LSB-first bitmap. Only the lowest
`count` bits are appended; higher bits are ignored.

This is significantly faster than calling [`Self::append`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-63bcc4e65530dd5c6ab82e0f) in a
loop when the caller already has bits packed into a `u64`.

<a id="op-cd04175075294f09c5a09d8a"></a>
## as_slice

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::as_slice` · arrow-buffer 59.3.0

```rust
fn as_slice(&self) -> &[u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:298`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the packed bits

<a id="op-3b0300c74b9cb78d8dbc1924"></a>
## as_slice_mut

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::as_slice_mut` · arrow-buffer 59.3.0

```rust
fn as_slice_mut(&mut self) -> &mut [u8]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:303`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the packed bits

<a id="op-db50ca2198f8cbf3c552d9cb"></a>
## build

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::build` · arrow-buffer 59.3.0

```rust
fn build(self) -> BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:321`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds a [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) without resetting the builder.

This consumes the builder. Use [`Self::finish`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-81d5ed6eb6ae6420d00a6eeb) to reuse it.

<a id="op-f57898d213728d1289080cd2"></a>
## capacity

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::capacity` · arrow-buffer 59.3.0

```rust
fn capacity(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:123`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the capacity of the buffer, in bits (not bytes)

Note this

# Example
```
# use arrow_buffer::builder::BooleanBufferBuilder;
// empty requires 0 bytes
let b = BooleanBufferBuilder::new(0);
assert_eq!(0, b.capacity());
// Creating space for 1 bit results in 64 bytes (space for 512 bits)
// (64 is the minimum allocation size for 64 bit architectures)
let mut b = BooleanBufferBuilder::new(1);
assert_eq!(512, b.capacity());
// 1000 bits requires 128 bytes (space for 1024 bits)
b.append_n(1000, true);
assert_eq!(1024, b.capacity());
```

<a id="op-0a74b6c7b272372ce446aedc"></a>
## extend_trusted_len

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::extend_trusted_len` · arrow-buffer 59.3.0

```rust
unsafe fn extend_trusted_len<I>(&mut self, iterator: I) where I: Iterator<Item = bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:335`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Extends the builder from a trusted length iterator of booleans.
# Safety
Callers must ensure that `iter` reports an exact size via `size_hint`.


<a id="op-81d5ed6eb6ae6420d00a6eeb"></a>
## finish

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::finish` · arrow-buffer 59.3.0

```rust
fn finish(&mut self) -> BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:311`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Resets this builder and returns a [`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5).

Use [`Self::build`](../operations/arrow_buffer.builder.boolean.BooleanBufferBuilder.md#op-db50ca2198f8cbf3c552d9cb) when you don't need to reuse this builder.

<a id="op-50f3ade81c114a4fc5994d22"></a>
## finish_cloned

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::finish_cloned` · arrow-buffer 59.3.0

```rust
fn finish_cloned(&self) -> BooleanBuffer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:326`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Builds the [BooleanBuffer](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) without resetting the builder.

<a id="op-29f84e9da7d68beebd008e0d"></a>
## fmt

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/builder/boolean.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/builder/boolean.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f593a6d2996ce5c5a67c3b36"></a>
## get_bit

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::get_bit` · arrow-buffer 59.3.0

```rust
fn get_bit(&self, index: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:94`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Gets a bit in the buffer at `index`

<a id="op-4204b860a63aa970f485b586"></a>
## is_empty

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::is_empty` · arrow-buffer 59.3.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:100`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns true if empty

<a id="op-2e7a1c455293dc3e26910ad9"></a>
## len

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the length of the buffer

<a id="op-304386108dc5a022f82ada30"></a>
## new

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::new` · arrow-buffer 59.3.0

```rust
fn new(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:59`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new `BooleanBufferBuilder` with sufficient space for
`capacity` bits (not bytes).

The capacity is rounded up to the nearest multiple of 8 for the
allocation.

<a id="op-e1c53ddf7bc50e7251279275"></a>
## new_from_buffer

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::new_from_buffer` · arrow-buffer 59.3.0

```rust
fn new_from_buffer(buffer: MutableBuffer, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Creates a new `BooleanBufferBuilder` from [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673) of `len`

<a id="op-999f863e12e56ad46ddb4597"></a>
## reserve

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::reserve` · arrow-buffer 59.3.0

```rust
fn reserve(&mut self, additional: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:161`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Reserve space to at least `additional` new bits.
Capacity will be `>= self.len() + additional`.

<a id="op-d6384e37937eb899f01befaa"></a>
## resize

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::resize` · arrow-buffer 59.3.0

```rust
fn resize(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:173`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Resizes the buffer, either truncating its contents (with no change in capacity), or
growing it (potentially reallocating it) and writing `false` in the newly available bits.

<a id="op-d54b156b3e0ae9ab68aceca2"></a>
## set_bit

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::set_bit` · arrow-buffer 59.3.0

```rust
fn set_bit(&mut self, index: usize, v: bool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Sets a bit in the buffer at `index`

<a id="op-d6d27a50e924a81668930fd0"></a>
## truncate

`function` · `arrow_buffer::builder::boolean::BooleanBufferBuilder::truncate` · arrow-buffer 59.3.0

```rust
fn truncate(&mut self, len: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::builder::boolean::BooleanBufferBuilder", "path": "BooleanBufferBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [343, 2], "filename": "src/builder/boolean.rs"}, "trait": null, "trait_path": null}`

Source: `src/builder/boolean.rs:142`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Truncates the builder to the given length

If `len` is greater than the buffer's current length, this has no effect
