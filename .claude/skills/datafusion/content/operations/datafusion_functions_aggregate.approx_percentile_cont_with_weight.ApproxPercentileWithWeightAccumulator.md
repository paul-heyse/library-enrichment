# `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_percentile_cont_with_weight.ApproxPercentileWithWeightAccumulator.json).

<a id="op-47a7bd70d082891b0c062ddf"></a>
## ApproxPercentileWithWeightAccumulator

`struct` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct ApproxPercentileWithWeightAccumulator
```

Source: `src/approx_percentile_cont_with_weight.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3b9a990b8f2f5b38903a1dd"></a>
## evaluate

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [363, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont_with_weight.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9b23af95c9a6216a4a4b422"></a>
## fmt

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 10], "end": [286, 15], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/approx_percentile_cont_with_weight.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1f3e51560090829917d211b"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [363, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont_with_weight.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eefef9b90575bddedfee5bed"></a>
## new

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new(approx_percentile_cont_accumulator: ApproxPercentileAccumulator) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [291, 1], "end": [297, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": null, "trait_path": null}`

Source: `src/approx_percentile_cont_with_weight.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49bc138a3519fe244a6b2e7c"></a>
## size

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [363, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont_with_weight.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29db5d9783fedfc0b822a70a"></a>
## state

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [363, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont_with_weight.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a6c7ac5b3b604c058ea69de"></a>
## update_batch

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileWithWeightAccumulator", "path": "ApproxPercentileWithWeightAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [299, 1], "end": [363, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/approx_percentile_cont_with_weight.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
