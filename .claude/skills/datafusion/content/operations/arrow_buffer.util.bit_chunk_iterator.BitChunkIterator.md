# `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_chunk_iterator.BitChunkIterator.json).

<a id="op-e4bd8e2a3d13e38eb1671af7"></a>
## BitChunkIterator

`struct` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator` · arrow-buffer 59.3.0

```rust
struct BitChunkIterator<'a>
```

Source: `src/util/bit_chunk_iterator.rs:245`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Iterator over chunks of 64 bits represented as an u64

<a id="op-cb339a78334c8d085f741d33"></a>
## Item

`assoc_type` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator::Item` · arrow-buffer 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunkIterator", "path": "BitChunkIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [384, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_chunk_iterator.rs:342`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4939eba9f383ccf042dabc88"></a>
## fmt

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunkIterator", "path": "BitChunkIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 10], "end": [244, 15], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/bit_chunk_iterator.rs:244`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd332340af06a6368dee5fd5"></a>
## len

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator::len` · arrow-buffer 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunkIterator", "path": "BitChunkIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [386, 1], "end": [391, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::exact_size::ExactSizeIterator", "path": "ExactSizeIterator"}, "trait_path": "core::iter::traits::exact_size::ExactSizeIterator"}`

Source: `src/util/bit_chunk_iterator.rs:388`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bf81ff03936162f8514fdbd"></a>
## next

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator::next` · arrow-buffer 59.3.0

```rust
fn next(&mut self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunkIterator", "path": "BitChunkIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [384, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_chunk_iterator.rs:345`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9896b48531fadf7bcb46a524"></a>
## size_hint

`function` · `arrow_buffer::util::bit_chunk_iterator::BitChunkIterator::size_hint` · arrow-buffer 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_chunk_iterator::BitChunkIterator", "path": "BitChunkIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [341, 1], "end": [384, 2], "filename": "src/util/bit_chunk_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_chunk_iterator.rs:378`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
