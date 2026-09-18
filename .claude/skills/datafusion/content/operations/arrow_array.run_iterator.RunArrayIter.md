# `arrow_array::run_iterator::RunArrayIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.run_iterator.RunArrayIter.json).

<a id="op-3e1e068d4ed0f52dee3767b4"></a>
## RunArrayIter

`struct` · `arrow_array::run_iterator::RunArrayIter` · arrow-array 59.3.0

```rust
struct RunArrayIter<'a, R, V> where R: RunEndIndexType, V: Sync + Send, &'a V: ArrayAccessor, <&'a V as ArrayAccessor>::Item: Default
```

Source: `src/run_iterator.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The [`RunArrayIter`](../operations/arrow_array.run_iterator.RunArrayIter.md#op-3e1e068d4ed0f52dee3767b4) provides an idiomatic way to iterate over the run array.
It returns Some(T) if there is a value or None if the value is null.

The iterator comes with a cost as it has to iterate over three arrays to determine
the value to be returned. The run_ends array is used to determine the index of the value.
The nulls array is used to determine if the value is null and the values array is used to
get the value.

Unlike other iterators in this crate, [`RunArrayIter`](../operations/arrow_array.run_iterator.RunArrayIter.md#op-3e1e068d4ed0f52dee3767b4) does not use [`ArrayAccessor`](../operations/arrow_array.array.ArrayAccessor.md#op-0f7f2e64730382bf2e8d3393)
because the run array accessor does binary search to access each value which is too slow.
The run array iterator can determine the next value in constant time.


<a id="op-f8a319eeb617bcdb8e5f82ce"></a>
## Item

`assoc_type` · `arrow_array::run_iterator::RunArrayIter::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::run_iterator::RunArrayIter", "path": "RunArrayIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [71, 1], "end": [120, 2], "filename": "src/run_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/run_iterator.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4dc177332fd2404e42d16d3"></a>
## fmt

`function` · `arrow_array::run_iterator::RunArrayIter::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::run_iterator::RunArrayIter", "path": "RunArrayIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/run_iterator.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/run_iterator.rs:35`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3da36f8a1ccfda1726ee9972"></a>
## new

`function` · `arrow_array::run_iterator::RunArrayIter::new` · arrow-array 59.3.0

```rust
fn new(array: TypedRunArray<'a, R, V>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::run_iterator::RunArrayIter", "path": "RunArrayIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [50, 1], "end": [69, 2], "filename": "src/run_iterator.rs"}, "trait": null, "trait_path": null}`

Source: `src/run_iterator.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

create a new iterator

<a id="op-3e19d2e3afb5468b6cfbe1d6"></a>
## next

`function` · `arrow_array::run_iterator::RunArrayIter::next` · arrow-array 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::run_iterator::RunArrayIter", "path": "RunArrayIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [71, 1], "end": [120, 2], "filename": "src/run_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/run_iterator.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfbf3291eed59e2de9964e2d"></a>
## next_back

`function` · `arrow_array::run_iterator::RunArrayIter::next_back` · arrow-array 59.3.0

```rust
fn next_back(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::run_iterator::RunArrayIter", "path": "RunArrayIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [122, 1], "end": [161, 2], "filename": "src/run_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::double_ended::DoubleEndedIterator", "path": "DoubleEndedIterator"}, "trait_path": "core::iter::traits::double_ended::DoubleEndedIterator"}`

Source: `src/run_iterator.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26179aa57d2f8c4109e67634"></a>
## size_hint

`function` · `arrow_array::run_iterator::RunArrayIter::size_hint` · arrow-array 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}, {"type": {"generic": "R"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "arrow_array::run_iterator::RunArrayIter", "path": "RunArrayIter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "R"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::RunEndIndexType", "path": "RunEndIndexType"}}}], "generic_params": [], "type": {"generic": "R"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "V"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}], "generic_params": [], "type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::default::Default", "path": "Default"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Item", "self_type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"generic": "V"}}}, "trait": {"args": null, "id": "arrow_array::array::ArrayAccessor", "path": "ArrayAccessor"}}}}}]}, "is_negative": false, "span": {"begin": [71, 1], "end": [120, 2], "filename": "src/run_iterator.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/run_iterator.rs:114`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
