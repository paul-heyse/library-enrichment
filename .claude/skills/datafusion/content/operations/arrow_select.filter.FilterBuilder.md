# `arrow_select::filter::FilterBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_select.filter.FilterBuilder.json).

<a id="op-cffeb561fc559f04206cfdc3"></a>
## FilterBuilder

`struct` · `arrow_select::filter::FilterBuilder` · arrow-select 59.3.0

```rust
struct FilterBuilder
```

Source: `src/filter.rs:248`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

A builder to construct [`FilterPredicate`](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd)

<a id="op-13b20c586080d2ff8d87c970"></a>
## build

`function` · `arrow_select::filter::FilterBuilder::build` · arrow-select 59.3.0

```rust
fn build(self) -> FilterPredicate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterBuilder", "path": "FilterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [324, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:317`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Construct the final `FilterPredicate`

<a id="op-1f85928de5192fea1292eb11"></a>
## fmt

`function` · `arrow_select::filter::FilterBuilder::fmt` · arrow-select 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterBuilder", "path": "FilterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [247, 10], "end": [247, 15], "filename": "src/filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter.rs:247`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e605d3ffc3e4ca14bee9ed54"></a>
## is_optimize_beneficial

`function` · `arrow_select::filter::FilterBuilder::is_optimize_beneficial` · arrow-select 59.3.0

```rust
fn is_optimize_beneficial(data_type: &DataType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterBuilder", "path": "FilterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [324, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Determines if calling [FilterBuilder::optimize](../operations/arrow_select.filter.FilterBuilder.md#op-58a30c42976fa26d4bc93e1e) is beneficial for the
given type even when filtering just a single array.

See [`FilterBuilder::optimize`](../operations/arrow_select.filter.FilterBuilder.md#op-58a30c42976fa26d4bc93e1e) for more details.

<a id="op-f24cf9cf3d6d9c71b6fc80f4"></a>
## new

`function` · `arrow_select::filter::FilterBuilder::new` · arrow-select 59.3.0

```rust
fn new(filter: &BooleanArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterBuilder", "path": "FilterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [324, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:256`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Create a new [`FilterBuilder`](../operations/arrow_select.filter.FilterBuilder.md#op-cffeb561fc559f04206cfdc3) that can be used to construct a [`FilterPredicate`](../operations/arrow_select.filter.FilterPredicate.md#op-13d555f716d9c3db0d29bdbd)

<a id="op-58a30c42976fa26d4bc93e1e"></a>
## optimize

`function` · `arrow_select::filter::FilterBuilder::optimize` · arrow-select 59.3.0

```rust
fn optimize(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_select::filter::FilterBuilder", "path": "FilterBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 1], "end": [324, 2], "filename": "src/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter.rs:285`. [Exact documentation build](https://docs.rs/crate/arrow-select/59.3.0/json).

Compute an optimized representation of the provided `filter` mask that can be
applied to an array more quickly.

When filtering multiple arrays (e.g. a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) or a
[`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) with multiple fields), optimizing the filter can provide
significant performance benefits.

However, optimization takes time and can have a larger memory footprint
than the original mask, so it is often faster to filter a single array,
without filter optimization.
