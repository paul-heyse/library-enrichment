# `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.DistinctVarianceAccumulator.json).

<a id="op-cf0ff218d113a423846d9d58"></a>
## DistinctVarianceAccumulator

`struct` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct DistinctVarianceAccumulator
```

Source: `src/variance.rs:627`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dc3925b1434717d1854bdd5"></a>
## evaluate

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [689, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:646`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d66b91312f72c0e738ade5f"></a>
## fmt

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [626, 10], "end": [626, 15], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variance.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30d41b84419b02044dfb35f4"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [689, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:686`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a11bf3ae78d0e32c93074443"></a>
## new

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new(stat_type: StatsType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [632, 1], "end": [639, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:633`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-271f98b4a8ef072056dab567"></a>
## size

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [689, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:678`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b7dcb6753f3f4c51d2a9871"></a>
## state

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self) -> Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [689, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:682`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ecdf34b4d4896fd17f1c59b"></a>
## update_batch

`function` · `datafusion_functions_aggregate::variance::DistinctVarianceAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::DistinctVarianceAccumulator", "path": "DistinctVarianceAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [641, 1], "end": [689, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/variance.rs:642`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
