# `arrow_buffer::util::bit_iterator::BitSliceIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_iterator.BitSliceIterator.json).

<a id="op-c825a8467c523296ef7280a9"></a>
## BitSliceIterator

`struct` · `arrow_buffer::util::bit_iterator::BitSliceIterator` · arrow-buffer 59.3.0

```rust
struct BitSliceIterator<'a>
```

Source: `src/util/bit_iterator.rs:188`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Iterator of contiguous ranges of set bits within a provided packed bitmask

Returns `(usize, usize)` each representing an interval where the corresponding
bits in the provides mask are set

the first value is the start of the range (inclusive) and the second value is the end of the range (exclusive)


<a id="op-6f639c8e5a427ad86c43ea30"></a>
## Item

`assoc_type` · `arrow_buffer::util::bit_iterator::BitSliceIterator::Item` · arrow-buffer 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitSliceIterator", "path": "BitSliceIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [274, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:233`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95fc18ae3c128a3a534adf67"></a>
## fmt

`function` · `arrow_buffer::util::bit_iterator::BitSliceIterator::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitSliceIterator", "path": "BitSliceIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [187, 10], "end": [187, 15], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/bit_iterator.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44f4a7de5c1fc21a2f3241dc"></a>
## new

`function` · `arrow_buffer::util::bit_iterator::BitSliceIterator::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitSliceIterator", "path": "BitSliceIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [230, 2], "filename": "src/util/bit_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_iterator.rs:198`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BitSliceIterator`](../operations/arrow_buffer.util.bit_iterator.BitSliceIterator.md#op-c825a8467c523296ef7280a9) from the provided `buffer`,
and `offset` and `len` in bits

<a id="op-2f1b9c263520107a326565d1"></a>
## next

`function` · `arrow_buffer::util::bit_iterator::BitSliceIterator::next` · arrow-buffer 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitSliceIterator", "path": "BitSliceIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [274, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
