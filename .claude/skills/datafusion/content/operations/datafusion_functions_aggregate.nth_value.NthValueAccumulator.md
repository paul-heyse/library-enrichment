# `datafusion_functions_aggregate::nth_value::NthValueAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.nth_value.NthValueAccumulator.json).

<a id="op-1a5644ecb6b115ab07c1c6bc"></a>
## NthValueAccumulator

`struct` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct NthValueAccumulator
```

Source: `src/nth_value.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db43d38bb06a7e75e6d3b915"></a>
## evaluate

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [540, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ba980ef185bfad207b09f9d"></a>
## fmt

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 10], "end": [312, 15], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/nth_value.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6010b4c05a937aa04e98f7d4"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [540, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82c1fbc43935abd6ff0671b4"></a>
## size

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [540, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b5561e5d69a22c6354ad0f1"></a>
## state

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [540, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d552b9e712f7d952ea8f9a6"></a>
## try_new

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(n: i64, datatype: &DataType, ordering_dtypes: &[DataType], ordering_req: LexOrdering) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 1], "end": [412, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new order-sensitive NTH_VALUE accumulator based on the given
item data type.

<a id="op-cfea43fa1757f7ba16e119fd"></a>
## update_batch

`function` · `datafusion_functions_aggregate::nth_value::NthValueAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAccumulator", "path": "NthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [414, 1], "end": [540, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Updates its state with the `values`. Assumes data in the `values` satisfies the required
ordering for the accumulator (across consecutive batches, not just batch-wise).
