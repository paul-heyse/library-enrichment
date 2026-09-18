# `arrow_buffer::util::bit_chunk_iterator::BitChunks`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_chunk_iterator.BitChunks.json).

<a id="op-b8340de6eea8c71484dde9af"></a>
## BitChunks

`struct` · `arrow_buffer::util::bit_chunk_iterator::BitChunks` · arrow-buffer 59.3.0

```rust
struct BitChunks<'a>
```

Source: `src/util/bit_chunk_iterator.rs:210`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Iterates over an arbitrarily aligned byte buffer 64 bits at a time

[`Self::iter`](../operations/arrow_buffer.util.bit_chunk_iterator.BitChunks.md#op-cf7ae73df41677a2924c77e8) yields iterator of `u64`, and a remainder. The first byte in the buffer
will be the least significant byte in output u64

<a id="op-8d086c78e3f416ccba0f3fe3"></a>
## IntoIter

`assoc_type` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::IntoIter` · arrow-buffer 59.3.0

```rust
IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 1], "end": [339, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/util/bit_chunk_iterator.rs:334`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90deec8c0d45363cf7997b40"></a>
## Item

`assoc_type` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::Item` · arrow-buffer 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 1], "end": [339, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/util/bit_chunk_iterator.rs:333`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55ace3408bff5a2c193ee564"></a>
## chunk_len

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::chunk_len` · arrow-buffer 59.3.0

```rust
const fn chunk_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:261`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of `u64` chunks

<a id="op-284b6744af4df29efc646104"></a>
## fmt

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [209, 10], "end": [209, 15], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/bit_chunk_iterator.rs:209`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8842d4b6212e4e8b8ab152d8"></a>
## into_iter

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::into_iter` · arrow-buffer 59.3.0

```rust
fn into_iter(self) -> Self::IntoIter
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 1], "end": [339, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `src/util/bit_chunk_iterator.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf7ae73df41677a2924c77e8"></a>
## iter

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::iter` · arrow-buffer 59.3.0

```rust
const fn iter(&self) -> BitChunkIterator<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:316`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator over chunks of 64 bits represented as an `u64`

<a id="op-9750cd5be2fe8bd13777d1a3"></a>
## iter_padded

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::iter_padded` · arrow-buffer 59.3.0

```rust
fn iter_padded(&self) -> impl Iterator<Item = u64> + 'a
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:327`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator over chunks of 64 bits, with the remaining bits zero padded to 64-bits

<a id="op-8601fa883a4904c59bdc306f"></a>
## new

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [220, 1], "end": [241, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:222`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BitChunks`](../operations/arrow_buffer.util.bit_chunk_iterator.BitChunks.md#op-b8340de6eea8c71484dde9af) from a byte array, and an offset and length in bits

<a id="op-3b41b5f2d6e56696fc03cf32"></a>
## num_bytes

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::num_bytes` · arrow-buffer 59.3.0

```rust
fn num_bytes(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:310`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Return the number of *bytes* that are needed to represent all bits
(including remainder).

<a id="op-d8dc59f4d5d201afb99ab880"></a>
## num_u64s

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::num_u64s` · arrow-buffer 59.3.0

```rust
fn num_u64s(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Return the number of `u64` that are needed to represent all bits
(including remainder).

This is equal to `chunk_len + 1` if there is a remainder,
otherwise it is equal to `chunk_len`.

<a id="op-a6519e932c7153ca6f58376e"></a>
## remainder_bits

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::remainder_bits` · arrow-buffer 59.3.0

```rust
fn remainder_bits(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the bitmask of remaining bits

<a id="op-3d900aac350736d08d12e4e1"></a>
## remainder_len

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunks::remainder_len` · arrow-buffer 59.3.0

```rust
const fn remainder_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunks", "path": "BitChunks"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 1], "end": [330, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:255`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of remaining bits, guaranteed to be between 0 and 63 (inclusive)
