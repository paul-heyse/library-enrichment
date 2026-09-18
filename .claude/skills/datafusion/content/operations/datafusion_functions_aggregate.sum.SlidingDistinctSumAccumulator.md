# `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.sum.SlidingDistinctSumAccumulator.json).

<a id="op-86a8821a3de9d36a72bef2fb"></a>
## SlidingDistinctSumAccumulator

`struct` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct SlidingDistinctSumAccumulator
```

Source: `src/sum.rs:593`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

A sliding‐window accumulator for `SUM(DISTINCT)` over Int64 columns.
Maintains a running sum so that `evaluate()` is O(1).

<a id="op-f89093389ea5a783b7edd26f"></a>
## evaluate

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cd369404b52b870bb4afc1a"></a>
## fmt

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 10], "end": [592, 15], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sum.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bba963b0904ee916526f7fd5"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:693`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29c7ec1e7f13f4f9d66a4cbe"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:708`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c84821e0f30a5d056d35bcd"></a>
## size

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:673`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43a6cedca838bb3d97df38e7"></a>
## state

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d192705298fdb63b07f64c33"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:714`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5314a2f53a7049e6571a6856"></a>
## try_new

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(data_type: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [602, 1], "end": [657, 2], "filename": "src/sum.rs"}, "trait": null, "trait_path": null}`

Source: `src/sum.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new accumulator; only `DataType::Int64` is supported.

<a id="op-4754bcd96f72f2e3c51f902a"></a>
## update_batch

`function` · `datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::SlidingDistinctSumAccumulator", "path": "SlidingDistinctSumAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [717, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/sum.rs:660`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
