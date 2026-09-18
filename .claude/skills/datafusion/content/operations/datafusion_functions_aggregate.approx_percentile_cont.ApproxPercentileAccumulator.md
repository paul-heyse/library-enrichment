# `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_percentile_cont.ApproxPercentileAccumulator.json).

<a id="op-79f57d80cb33db90b58d679e"></a>
## ApproxPercentileAccumulator

`struct` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct ApproxPercentileAccumulator
```

Source: `src/approx_percentile_cont.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b187115417fce72aac21b2db"></a>
## evaluate

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [458, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6a1e9c720fd8d8b38cde786"></a>
## fmt

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 10], "end": [331, 15], "filename": "src/approx_percentile_cont.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/approx_percentile_cont.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-650a8b15be46986b443fd846"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [458, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82f473ad78dc3e9fb8410eb4"></a>
## new

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new(percentile: f64, return_type: DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [398, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": null, "trait_path": null}`

Source: `src/approx_percentile_cont.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-419a61ae94998d746f00a3fe"></a>
## new_with_max_size

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::new_with_max_size` · datafusion-functions-aggregate 55.1.0

```rust
fn new_with_max_size(percentile: f64, return_type: DataType, max_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [338, 1], "end": [398, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": null, "trait_path": null}`

Source: `src/approx_percentile_cont.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5fd40fcda459fb5254ad26f"></a>
## size

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [458, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont.rs:453`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f77c09b773a4238dd07871fc"></a>
## state

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [458, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2322cd5afb2a5aec8b80c5ba"></a>
## update_batch

`function` · `datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont::ApproxPercentileAccumulator", "path": "ApproxPercentileAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 1], "end": [458, 2], "filename": "src/approx_percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
