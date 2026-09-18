# `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.bytes.BytesViewDistinctCountAccumulator.json).

<a id="op-53fb8ae7f77e58b0dd18c0b8"></a>
## BytesViewDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct BytesViewDistinctCountAccumulator
```

Source: `src/aggregate/count_distinct/bytes.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Specialized implementation of
`COUNT DISTINCT` for [`StringViewArray`] and [`BinaryViewArray`].

[`StringViewArray`]: arrow::array::StringViewArray
[`BinaryViewArray`]: arrow::array::BinaryViewArray

<a id="op-5cec3085cefbc8d00b4afa1b"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [153, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-404155990eac0307973f818e"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 10], "end": [99, 15], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/bytes.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d68adcc19ef7c73f58eb017f"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [153, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2332dcafc34ae19fac0b4599"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(output_type: OutputType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [106, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/bytes.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f72253b8ffdb811c2ad3a04f"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [153, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d07e49d9afb65c51375c8515"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [153, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a296244b856f9a0510f069a"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesViewDistinctCountAccumulator", "path": "BytesViewDistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [153, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
