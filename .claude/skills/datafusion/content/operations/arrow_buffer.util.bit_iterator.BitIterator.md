# `arrow_buffer::util::bit_iterator::BitIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_iterator.BitIterator.json).

<a id="op-419367f35ab166fcd03401c9"></a>
## BitIterator

`struct` · `arrow_buffer::util::bit_iterator::BitIterator` · arrow-buffer 59.3.0

```rust
struct BitIterator<'a>
```

Source: `src/util/bit_iterator.rs:27`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Iterator over the bits within a packed bitmask

To efficiently iterate over just the set bits see [`BitIndexIterator`](../operations/arrow_buffer.util.bit_iterator.BitIndexIterator.md#op-c2ca62d89850047c97347f85) and [`BitSliceIterator`](../operations/arrow_buffer.util.bit_iterator.BitSliceIterator.md#op-c825a8467c523296ef7280a9)

<a id="op-74c0d90a5fea86405702a07b"></a>
## Item

`assoc_type` · `arrow_buffer::util::bit_iterator::BitIterator::Item` · arrow-buffer 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff5d4d93b21445f3791e640e"></a>
## clone

`function` · `arrow_buffer::util::bit_iterator::BitIterator::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> BitIterator<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 10], "end": [26, 15], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/util/bit_iterator.rs:26`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d6cd14e84cac47f42248a8d"></a>
## count

`function` · `arrow_buffer::util::bit_iterator::BitIterator::count` · arrow-buffer 59.3.0

```rust
fn count(self) -> usize where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca963f496e14a993fa3bc0c0"></a>
## last

`function` · `arrow_buffer::util::bit_iterator::BitIterator::last` · arrow-buffer 59.3.0

```rust
fn last(self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:106`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b471f9f82c80fdec5adf578a"></a>
## max

`function` · `arrow_buffer::util::bit_iterator::BitIterator::max` · arrow-buffer 59.3.0

```rust
fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:119`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6120562e5b0baedcbe94a803"></a>
## new

`function` · `arrow_buffer::util::bit_iterator::BitIterator::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [55, 2], "filename": "src/util/bit_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_iterator.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`BitIterator`](../operations/arrow_buffer.util.bit_iterator.BitIterator.md#op-419367f35ab166fcd03401c9) from the provided `buffer`,
and `offset` and `len` in bits

# Panic

Panics if `buffer` is too short for the provided offset and length

<a id="op-346dc78304138d8e9d7942ad"></a>
## next

`function` · `arrow_buffer::util::bit_iterator::BitIterator::next` · arrow-buffer 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a08416a36bcedaf42aadc4e9"></a>
## next_back

`function` · `arrow_buffer::util::bit_iterator::BitIterator::next_back` · arrow-buffer 59.3.0

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [178, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/util/bit_iterator.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c8149abdf2ae05ce88b035d"></a>
## nth

`function` · `arrow_buffer::util::bit_iterator::BitIterator::nth` · arrow-buffer 59.3.0

```rust
fn nth(&mut self, n: usize) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:85`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0dc26b41636fe94e524226a"></a>
## nth_back

`function` · `arrow_buffer::util::bit_iterator::BitIterator::nth_back` · arrow-buffer 59.3.0

```rust
fn nth_back(&mut self, n: usize) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [178, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/util/bit_iterator.rs:158`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c88f553450adffd33b003e0"></a>
## size_hint

`function` · `arrow_buffer::util::bit_iterator::BitIterator::size_hint` · arrow-buffer 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIterator", "path": "BitIterator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [142, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
