# `arrow_buffer::util::bit_iterator::BitIndexU32Iterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_iterator.BitIndexU32Iterator.json).

<a id="op-1a32bb376f666a359f2b66b2"></a>
## BitIndexU32Iterator

`struct` · `arrow_buffer::util::bit_iterator::BitIndexU32Iterator` · arrow-buffer 59.3.0

```rust
struct BitIndexU32Iterator<'a>
```

Source: `src/util/bit_iterator.rs:326`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

An iterator of u32 whose index in a provided bitmask is true
Respects arbitrary offsets and slice lead/trail padding exactly like BitIndexIterator

<a id="op-9421b63bd6457a33d5fe7f19"></a>
## Item

`assoc_type` · `arrow_buffer::util::bit_iterator::BitIndexU32Iterator::Item` · arrow-buffer 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexU32Iterator", "path": "BitIndexU32Iterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [374, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:354`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8195ca9062310dcfc8d65dd5"></a>
## fmt

`function` · `arrow_buffer::util::bit_iterator::BitIndexU32Iterator::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexU32Iterator", "path": "BitIndexU32Iterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 10], "end": [325, 15], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util/bit_iterator.rs:325`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef8f992053a5c4792c8481a3"></a>
## new

`function` · `arrow_buffer::util::bit_iterator::BitIndexU32Iterator::new` · arrow-buffer 59.3.0

```rust
fn new(buffer: &'a [u8], offset: usize, len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexU32Iterator", "path": "BitIndexU32Iterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 1], "end": [351, 2], "filename": "src/util/bit_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/util/bit_iterator.rs:335`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [BitIndexU32Iterator](../operations/arrow_buffer.util.bit_iterator.BitIndexU32Iterator.md#op-1a32bb376f666a359f2b66b2) from the provided buffer,
offset and len in bits.

<a id="op-87c9400c4b8918a89072cf61"></a>
## next

`function` · `arrow_buffer::util::bit_iterator::BitIndexU32Iterator::next` · arrow-buffer 59.3.0

```rust
fn next(&mut self) -> Option<u32>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_buffer::util::bit_iterator::BitIndexU32Iterator", "path": "BitIndexU32Iterator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [353, 1], "end": [374, 2], "filename": "src/util/bit_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/util/bit_iterator.rs:357`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
