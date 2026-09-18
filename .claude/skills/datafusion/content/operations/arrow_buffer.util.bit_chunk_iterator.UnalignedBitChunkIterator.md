# `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunkIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_chunk_iterator.UnalignedBitChunkIterator.json).

<a id="op-bd8ff3ce64287f11554f2337"></a>
## UnalignedBitChunkIterator

`type_alias` · `arrow_buffer::util::bit_chunk_iterator::UnalignedBitChunkIterator` · arrow-buffer 59.3.0

```rust
type UnalignedBitChunkIterator<'a> = std::iter::Chain<std::iter::Chain<std::option::IntoIter<u64>, std::iter::Cloned<std::slice::Iter<'a, u64>>>, std::option::IntoIter<u64>>
```

Source: `src/util/bit_chunk_iterator.rs:174`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Iterator over an [`UnalignedBitChunk`](../operations/arrow_buffer.util.bit_chunk_iterator.UnalignedBitChunk.md#op-78cb4a10dbbfb26e2d423148)
