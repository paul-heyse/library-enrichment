# `arrow_buffer::util::bit_iterator::BitIndexIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_iterator.BitIndexIterator.json).

<a id="op-c2ca62d89850047c97347f85"></a>
## BitIndexIterator

`struct` · `arrow_buffer::util::bit_iterator::BitIndexIterator` · arrow-buffer 59.3.0

```rust
struct BitIndexIterator<'a>
```

Source: `src/util/bit_iterator.rs:281`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

An iterator of `usize` whose index in a provided bitmask is true

This provides the best performance on most masks, apart from those which contain
large runs and therefore favour [`BitSliceIterator`](../operations/arrow_buffer.util.bit_iterator.BitSliceIterator.md#op-c825a8467c523296ef7280a9)

<a id="op-77f384a62e52734762beacf7"></a>
## Item

`assoc_type` · `arrow_buffer::util::bit_iterator::BitIndexIterator::Item` · arrow-buffer 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexIterator", "path": "BitIndexIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 1], "end": [321, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:306`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f1a921b79bd315a3008d7c2"></a>
## fmt

`function` · `arrow_buffer::util::bit_iterator::BitIndexIterator::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexIterator", "path": "BitIndexIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 10], "end": [280, 15], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/bit_iterator.rs:280`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60b189a1f807f631354331a6"></a>
## new

`function` · `arrow_buffer::util::bit_iterator::BitIndexIterator::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexIterator", "path": "BitIndexIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [303, 2], "filename": "src/util/bit_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_iterator.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BitIndexIterator`](../operations/arrow_buffer.util.bit_iterator.BitIndexIterator.md#op-c2ca62d89850047c97347f85) from the provide `buffer`,
and `offset` and `len` in bits

<a id="op-68f049c9f217a1cc1be1e766"></a>
## next

`function` · `arrow_buffer::util::bit_iterator::BitIndexIterator::next` · arrow-buffer 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexIterator", "path": "BitIndexIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [305, 1], "end": [321, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:309`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
