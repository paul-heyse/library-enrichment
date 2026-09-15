# `arrow_buffer::util::bit_chunk_iterator`

Crate `arrow-buffer` · 4 public items · structured records in [`model/arrow_buffer.util.bit_chunk_iterator.json`](../model/arrow_buffer.util.bit_chunk_iterator.json)

## BitChunkIterator

`struct` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator`

Also reachable as `arrow::util::bit_chunk_iterator::BitChunkIterator`

```rust
struct BitChunkIterator<'a>
```

**Implements**: `core::iter::traits::exact_size::ExactSizeIterator`, `core::iter::traits::iterator::Iterator`

**Derives**: Debug

**via `core::iter::traits::exact_size::ExactSizeIterator`**

```rust
fn len(&self) -> usize
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<u64>
fn size_hint(&self) -> (usize, Option<usize>)
```

Iterator over chunks of 64 bits represented as an u64

---

## BitChunks

`struct` · `arrow_buffer::util::bit_chunk_iterator::BitChunks`

Also reachable as `arrow::util::bit_chunk_iterator::BitChunks`

```rust
struct BitChunks<'a>
```

**Implements**: `core::iter::traits::collect::IntoIterator`

**Derives**: Debug

**Methods** (8)

```rust
const fn chunk_len(&self) -> usize
const fn iter(&self) -> BitChunkIterator<'a>
fn iter_padded(&self) -> impl Iterator<Item = u64> + 'a
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
fn num_bytes(&self) -> usize
fn num_u64s(&self) -> usize
fn remainder_bits(&self) -> u64
const fn remainder_len(&self) -> usize
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

Iterates over an arbitrarily aligned byte buffer 64 bits at a time

[`Self::iter`] yields iterator of `u64`, and a remainder. The first byte in the buffer
will be the least significant byte in output u64

---

## UnalignedBitChunk

`struct` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunk`

Also reachable as `arrow::util::bit_chunk_iterator::UnalignedBitChunk`

```rust
struct UnalignedBitChunk<'a>
```

**Derives**: Debug

**Methods** (8)

```rust
fn chunks(&self) -> &'a [u64]
fn count_ones(&self) -> usize
fn iter(&self) -> UnalignedBitChunkIterator<'a>
fn lead_padding(&self) -> usize
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
fn prefix(&self) -> Option<u64>
fn suffix(&self) -> Option<u64>
fn trailing_padding(&self) -> usize
```

Iterates over an arbitrarily aligned byte buffer

Yields an iterator of aligned u64, along with the leading and trailing
u64 necessary to align the buffer to a 8-byte boundary

This is unlike [`BitChunkIterator`] which only exposes a trailing u64,
and consequently has to perform more work for each read

---

## UnalignedBitChunkIterator

`type_alias` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunkIterator`

Also reachable as `arrow::util::bit_chunk_iterator::UnalignedBitChunkIterator`

```rust
type UnalignedBitChunkIterator<'a> = std::iter::Chain<std::iter::Chain<std::option::IntoIter<u64>, std::iter::Cloned<std::slice::Iter<'a, u64>>>, std::option::IntoIter<u64>>
```

Iterator over an [`UnalignedBitChunk`]

---
