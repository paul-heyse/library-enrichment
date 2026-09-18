# `arrow_array::iterator::ArrayIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.iterator.ArrayIter.json).

<a id="op-4dca6024fde35b5be30c6ce8"></a>
## ArrayIter

`struct` · `arrow_array::iterator::ArrayIter` · arrow-array 59.3.0

```rust
struct ArrayIter<T: ArrayAccessor>
```

Source: `src/iterator.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

An iterator that returns Some(T) or None, that can be used on any [`ArrayAccessor`](../operations/arrow_array.array.ArrayAccessor.md#op-0f7f2e64730382bf2e8d3393)

# Performance

[`ArrayIter`](../operations/arrow_array.iterator.ArrayIter.md#op-4dca6024fde35b5be30c6ce8) provides an idiomatic way to iterate over an array, however, this
comes at the cost of performance. In particular the interleaved handling of
the null mask is often sub-optimal.

If performing an infallible operation, it is typically faster to perform the operation
on every index of the array, and handle the null mask separately. For [`PrimitiveArray`]
this functionality is provided by [`compute::unary`]

If performing a fallible operation, it isn't possible to perform the operation independently
of the null mask, as this might result in a spurious failure on a null index. However,
there are more efficient ways to iterate over just the non-null indices, this functionality
is provided by [`compute::try_unary`]

[`PrimitiveArray`]: crate::PrimitiveArray
[`compute::unary`]: https://docs.rs/arrow/latest/arrow/compute/fn.unary.html
[`compute::try_unary`]: https://docs.rs/arrow/latest/arrow/compute/fn.try_unary.html

<a id="op-270768d6398f7a0cb371afb5"></a>
## Item

`assoc_type` · `arrow_array::iterator::ArrayIter::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [137, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/iterator.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ddc3b4843ae3dd19ccc74f9"></a>
## clone

`function` · `arrow_array::iterator::ArrayIter::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> ArrayIter<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 22], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/iterator.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b96608ee94b96ae468002e72"></a>
## count

`function` · `arrow_array::iterator::ArrayIter::count` · arrow-array 59.3.0

```rust
fn count(self) -> usize where Self: Sized
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [137, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/iterator.rs:131`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-408b416ae2b365f57df4a532"></a>
## fmt

`function` · `arrow_array::iterator::ArrayIter::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/iterator.rs:47`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02d6bbec2170bd81ddaf9cd7"></a>
## last

`function` · `arrow_array::iterator::ArrayIter::last` · arrow-array 59.3.0

```rust
fn last(self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [137, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/iterator.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d255b815dc9d7118ca4bcd9"></a>
## new

`function` · `arrow_array::iterator::ArrayIter::new` · arrow-array 59.3.0

```rust
fn new(array: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [75, 2], "filename": "src/iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/iterator.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

create a new iterator

<a id="op-619cd184a8981650a2090599"></a>
## next

`function` · `arrow_array::iterator::ArrayIter::next` · arrow-array 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [137, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/iterator.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ecd10f247d9b7737b098ce4"></a>
## next_back

`function` · `arrow_array::iterator::ArrayIter::next_back` · arrow-array 59.3.0

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [176, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/iterator.rs:140`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b89d4f5f38ad748ca9370acf"></a>
## nth

`function` · `arrow_array::iterator::ArrayIter::nth` · arrow-array 59.3.0

```rust
fn nth(&mut self, n: usize) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [137, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/iterator.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a9c0ece95c48f25cb9d1992"></a>
## nth_back

`function` · `arrow_array::iterator::ArrayIter::nth_back` · arrow-array 59.3.0

```rust
fn nth_back(&mut self, n: usize) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [176, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/iterator.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74b7e0a8a307b6c34a1f4911"></a>
## size_hint

`function` · `arrow_array::iterator::ArrayIter::size_hint` · arrow-array 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "arrow_array::iterator::ArrayIter", "path": "ArrayIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [137, 2], "filename": "src/iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/iterator.rs:99`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
