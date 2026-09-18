# `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.native.BoolArray256DistinctCountAccumulator.json).

<a id="op-dc9b187b2c41d95f71b19117"></a>
## BoolArray256DistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct BoolArray256DistinctCountAccumulator
```

Source: `src/aggregate/count_distinct/native.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Optimized COUNT DISTINCT accumulator for u8 using a bool array.
Uses 256 bytes to track all possible u8 values.

<a id="op-39357e9c188b72d5aaf6ef69"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [195, 1], "end": [199, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/count_distinct/native.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2aec484b138d1e6728b5d1b"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [255, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee8420eaafe05267f920d1ef"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 10], "end": [179, 15], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/native.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-087a7b273b686d241918149e"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [255, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d71453eb655d0cc73a16d6c"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [184, 1], "end": [193, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/native.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e96a3a533c584612984d542c"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [255, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c66287376eb423bbf6a1d2f"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [255, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c83a9615263d1b2e97e0c3d"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::BoolArray256DistinctCountAccumulator", "path": "BoolArray256DistinctCountAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [255, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
