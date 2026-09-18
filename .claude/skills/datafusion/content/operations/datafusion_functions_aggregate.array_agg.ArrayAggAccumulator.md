# `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.array_agg.ArrayAggAccumulator.json).

<a id="op-5661ff88c274bafba334eeb3"></a>
## ArrayAggAccumulator

`struct` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct ArrayAggAccumulator
```

Source: `src/array_agg.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21b4294dad3dd3e2e852c199"></a>
## evaluate

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8076b91b340fcb76b564f876"></a>
## fmt

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 10], "end": [263, 15], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array_agg.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19e33e18b76409e5e37d4cb1"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f81997c8de3208ec7c615bec"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1a2c3db4c55f673194af5ed"></a>
## size

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04947966026addca367d4a8f"></a>
## state

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:402`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d740e82fdd1be40a5a6b2f2"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7f8d1833cf06bbccae43a1a"></a>
## try_new

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(datatype: &DataType, ignore_nulls: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [273, 1], "end": [344, 2], "filename": "src/array_agg.rs"}, "trait": null, "trait_path": null}`

Source: `src/array_agg.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

new array_agg accumulator based on given item data type

<a id="op-8bff5f3bcaab94cae102ef58"></a>
## update_batch

`function` · `datafusion_functions_aggregate::array_agg::ArrayAggAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAggAccumulator", "path": "ArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 1], "end": [494, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
