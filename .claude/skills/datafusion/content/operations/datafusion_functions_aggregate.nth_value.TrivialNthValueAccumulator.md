# `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.nth_value.TrivialNthValueAccumulator.json).

<a id="op-1a8633723d7474e46d7b68a9"></a>
## TrivialNthValueAccumulator

`struct` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct TrivialNthValueAccumulator
```

Source: `src/nth_value.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab466f2545c1db756da5a69d"></a>
## evaluate

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [310, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0be9e0f2c870f831ff12e859"></a>
## fmt

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 10], "end": [191, 15], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/nth_value.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ced8bce3aeb13a5207ff839a"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [310, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-932bfa80abd82a508d6859d9"></a>
## size

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [310, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5421c071c7f0ae95419c036"></a>
## state

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [310, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:278`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8e80c28d53d788e2c70a752"></a>
## try_new

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::try_new` · datafusion-functions-aggregate 55.1.0

```rust
fn try_new(n: i64, datatype: &DataType) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [237, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new order-insensitive NTH_VALUE accumulator based on the given
item data type.

<a id="op-5e8672ce0bb5a892aeaea543"></a>
## update_batch

`function` · `datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::TrivialNthValueAccumulator", "path": "TrivialNthValueAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [310, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/nth_value.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Updates its state with the `values`. Assumes data in the `values` satisfies the required
ordering for the accumulator (across consecutive batches, not just batch-wise).
