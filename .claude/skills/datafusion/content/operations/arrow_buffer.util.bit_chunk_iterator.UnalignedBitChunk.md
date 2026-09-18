# `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_chunk_iterator.UnalignedBitChunk.json).

<a id="op-78cb4a10dbbfb26e2d423148"></a>
## UnalignedBitChunk

`struct` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk` · arrow-buffer 59.3.0

```rust
struct UnalignedBitChunk<'a>
```

Source: `src/util/bit_chunk_iterator.rs:31`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Iterates over an arbitrarily aligned byte buffer

Yields an iterator of aligned u64, along with the leading and trailing
u64 necessary to align the buffer to a 8-byte boundary

This is unlike [`BitChunkIterator`](../operations/arrow_buffer.util.bit_chunk_iterator.BitChunkIterator.md#op-e4bd8e2a3d13e38eb1671af7) which only exposes a trailing u64,
and consequently has to perform more work for each read

<a id="op-3c09bfdd04355c90e46a6e01"></a>
## chunks

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::chunks` · arrow-buffer 59.3.0

```rust
fn chunks(&self) -> &'a [u64]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:155`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns reference to the chunks

<a id="op-2b40805940ef22d41ff614fd"></a>
## count_ones

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::count_ones` · arrow-buffer 59.3.0

```rust
fn count_ones(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Counts the number of ones

<a id="op-869bf0c124bced4104bc2b6b"></a>
## fmt

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/bit_chunk_iterator.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f8e5900fe47a302df499d09"></a>
## iter

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::iter` · arrow-buffer 59.3.0

```rust
fn iter(&self) -> UnalignedBitChunkIterator<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:160`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns an iterator over the chunks

<a id="op-691b8c5df60c62685cf3afa1"></a>
## lead_padding

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::lead_padding` · arrow-buffer 59.3.0

```rust
fn lead_padding(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of leading padding bits

<a id="op-b0edaabf9611b7cecaa9ddc6"></a>
## new

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a from a byte array, and and an offset and length in bits

<a id="op-7e37d4a7e9e5e8b61a51616c"></a>
## prefix

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::prefix` · arrow-buffer 59.3.0

```rust
fn prefix(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the prefix, if any

<a id="op-b9720ae800df230cfdf4c72d"></a>
## suffix

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::suffix` · arrow-buffer 59.3.0

```rust
fn suffix(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:150`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the suffix, if any

<a id="op-4eb563ef5d815f8c0757eb8d"></a>
## trailing_padding

`function` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk::trailing_padding` · arrow-buffer 59.3.0

```rust
fn trailing_padding(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk", "path": "UnalignedBitChunk"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [171, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_chunk_iterator.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the number of trailing padding bits
