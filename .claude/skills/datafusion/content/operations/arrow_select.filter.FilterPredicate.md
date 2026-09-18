# `arrow_select::filter::FilterPredicate`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.filter.FilterPredicate.json).

<a id="op-13d555f716d9c3db0d29bdbd"></a>
## FilterPredicate

`struct` · `arrow_select::filter::FilterPredicate` · arrow-select 59.3.0

```rust
struct FilterPredicate
```

Source: `src/filter.rs:429`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

A filtering predicate that can be applied to an [`Array`](../operations/arrow_array.array.Array.md#op-cabb9fba5e3e968cee823d21)

<a id="op-07abafdf9a22403d3bfd85df"></a>
## count

`function` · `arrow_select::filter::FilterPredicate::count` · arrow-select 59.3.0

```rust
fn count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterPredicate", "path": "FilterPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [517, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:467`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Number of rows being selected based on this [`FilterPredicate`](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd)

<a id="op-5fe81cc083d413b3f0be1467"></a>
## filter

`function` · `arrow_select::filter::FilterPredicate::filter` · arrow-select 59.3.0

```rust
fn filter(&self, values: &dyn Array) -> Result<ArrayRef, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterPredicate", "path": "FilterPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [517, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:437`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Selects rows from `values` based on this [`FilterPredicate`](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd)

<a id="op-39b83cbfce66d6e5a50e70d9"></a>
## filter_nulls

`function` · `arrow_select::filter::FilterPredicate::filter_nulls` · arrow-select 59.3.0

```rust
fn filter_nulls(&self, nulls: Option<&NullBuffer>) -> Option<NullBuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterPredicate", "path": "FilterPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [517, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:496`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Filters the given `nulls` buffer using this predicate.

Returns `None` when there is nothing to track in the output, either
because the input `nulls` was `None`, the input had no nulls, or the
filtered result has no nulls. Otherwise returns the filtered
[`NullBuffer`](../operations/arrow_buffer.buffer.null.NullBuffer.md#op-d8209d291b86c5deb0050e48) with its precomputed null count.

<a id="op-71633f5acf7bee57fadf5cb7"></a>
## filter_record_batch

`function` · `arrow_select::filter::FilterPredicate::filter_record_batch` · arrow-select 59.3.0

```rust
fn filter_record_batch(&self, record_batch: &RecordBatch) -> Result<RecordBatch, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterPredicate", "path": "FilterPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [517, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:445`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Returns a filtered [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) containing only the rows that are selected by this
[`FilterPredicate`](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd).

This is the equivalent of calling [filter](../operations/arrow_select.filter.filter.md#op-81498df4434a7e4fa4ec4f9e) on each column of the [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

<a id="op-075416e1cf79150ecd777bf4"></a>
## fmt

`function` · `arrow_select::filter::FilterPredicate::fmt` · arrow-select 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterPredicate", "path": "FilterPredicate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [428, 10], "end": [428, 15], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter.rs:428`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
