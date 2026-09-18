# `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.avg_distinct.numeric.Float64DistinctAvgAccumulator.json).

<a id="op-09081bb2ad1a93aae91db40b"></a>
## Float64DistinctAvgAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct Float64DistinctAvgAccumulator
```

Source: `src/aggregate/avg_distinct/numeric.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Specialized implementation of `AVG DISTINCT` for Float64 values, leveraging
the existing DistinctSumAccumulator implementation.

<a id="op-8d8d8fcc6e92d8a409428470"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [43, 2], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/avg_distinct/numeric.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcaa223b26f77055cedb9637"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [78, 2], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/avg_distinct/numeric.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44fd1ff881065c096ac74581"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/avg_distinct/numeric.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5bca9661bf8d7fd4f2eebc1"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [78, 2], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/avg_distinct/numeric.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-327c2580b5adf34066db98ca"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [78, 2], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/avg_distinct/numeric.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d54235cd5b095cf231caa78d"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [78, 2], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/avg_distinct/numeric.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7505fbf99d91cfca04c53ae0"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::avg_distinct::numeric::Float64DistinctAvgAccumulator", "path": "Float64DistinctAvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [78, 2], "filename": "src/aggregate/avg_distinct/numeric.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/avg_distinct/numeric.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
