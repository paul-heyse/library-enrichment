# `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.array_agg.DistinctArrayAggAccumulator.json).

<a id="op-0857a48ad0a6e572cd9d1af4"></a>
## DistinctArrayAggAccumulator

`struct` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct DistinctArrayAggAccumulator
```

Source: `src/array_agg.rs:840`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3222d411594e39479124332"></a>
## evaluate

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:1015`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93b780ef49cd37214fd86a3d"></a>
## fmt

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 10], "end": [839, 15], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array_agg.rs:839`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dbf060322ed857f230e5c8e"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:1000`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53e9618027e5b388fd9fed24"></a>
## retract_batch

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn retract_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:1066`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc8c1e7a65412adbae385969"></a>
## size

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:1169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec361507de463d6adff96192"></a>
## state

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:917`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c894dd8604380b00318e975e"></a>
## supports_retract_batch

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::supports_retract_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_retract_batch(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:1165`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b50cf6a58f946feaacef84c6"></a>
## try_new

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(datatype: &DataType, sort_options: Option<SortOptions>, ignore_nulls: bool) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [876, 1], "end": [914, 2], "filename": "src/array_agg.rs"}, "trait": null, "trait_path": null}`

Source: `src/array_agg.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd52a3b6274740cdd8dc885a"></a>
## update_batch

`function` · `datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::DistinctArrayAggAccumulator", "path": "DistinctArrayAggAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [916, 1], "end": [1191, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/array_agg.rs:921`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
