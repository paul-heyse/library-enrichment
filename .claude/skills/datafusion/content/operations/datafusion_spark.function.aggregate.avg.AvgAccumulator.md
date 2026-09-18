# `datafusion_spark::function::aggregate::avg::AvgAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.aggregate.avg.AvgAccumulator.json).

<a id="op-fb0d0a2783d3a4a9d4c72032"></a>
## AvgAccumulator

`struct` · `datafusion_spark::function::aggregate::avg::AvgAccumulator` · datafusion-spark 55.1.0

```rust
struct AvgAccumulator
```

Source: `src/function/aggregate/avg.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

An accumulator to compute the average

<a id="op-27cf8850da04c7b0f02c3f70"></a>
## default

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::default` · datafusion-spark 55.1.0

```rust
fn default() -> AvgAccumulator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 17], "end": [154, 24], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/aggregate/avg.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1a9f7e2f52ef027f48434c1"></a>
## evaluate

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::evaluate` · datafusion-spark 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [205, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/function/aggregate/avg.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-089a4a0f8bbd43a7059f344d"></a>
## fmt

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 10], "end": [154, 15], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/aggregate/avg.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-262fea80b35e41b7eae3f30b"></a>
## merge_batch

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::merge_batch` · datafusion-spark 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [205, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/function/aggregate/avg.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8b8f745d569d52fa775d55b"></a>
## size

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::size` · datafusion-spark 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [205, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/function/aggregate/avg.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19a97e8417a65c19707031f"></a>
## state

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::state` · datafusion-spark 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [205, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/function/aggregate/avg.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-038ddbaf3a47c26f38e4722c"></a>
## update_batch

`function` · `datafusion_spark::function::aggregate::avg::AvgAccumulator::update_batch` · datafusion-spark 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::aggregate::avg::AvgAccumulator", "path": "AvgAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [205, 2], "filename": "src/function/aggregate/avg.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/function/aggregate/avg.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
