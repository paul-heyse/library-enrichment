# `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.bytes.BytesDistinctCountAccumulator.json).

<a id="op-ee852029a35da6e7a6edfe71"></a>
## BytesDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct BytesDistinctCountAccumulator<O: OffsetSizeTrait>
```

Source: `src/aggregate/count_distinct/bytes.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Specialized implementation of
`COUNT DISTINCT` for [`StringArray`] [`LargeStringArray`],
[`BinaryArray`] and [`LargeBinaryArray`].

[`StringArray`]: arrow::array::StringArray
[`LargeStringArray`]: arrow::array::LargeStringArray
[`BinaryArray`]: arrow::array::BinaryArray
[`LargeBinaryArray`]: arrow::array::LargeBinaryArray

<a id="op-982413d4c896fc4083ea7936"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [92, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1783e4bb865163be1e15edf7"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/bytes.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b62377ffc212cc898ba6a9f6"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [92, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ebc5c03c7d026b40e02d9d1"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(output_type: OutputType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [45, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/bytes.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a271299d4292530e44e7f2f"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [92, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-439e68b314ccc75acad42dcc"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [92, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f99282accc754481a966cc7"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "O"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::bytes::BytesDistinctCountAccumulator", "path": "BytesDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::array::list_array::OffsetSizeTrait", "path": "OffsetSizeTrait"}}}], "default": null, "is_synthetic": false}}, "name": "O"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [92, 2], "filename": "src/aggregate/count_distinct/bytes.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/bytes.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
