# `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.native.FloatDistinctCountAccumulator.json).

<a id="op-6284054fdcdd782d8f3560a2"></a>
## FloatDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct FloatDistinctCountAccumulator<T: ArrowPrimitiveType>
```

Source: `src/aggregate/count_distinct/native.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63afb71d4a7e5e3b9b70246d"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [152, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/count_distinct/native.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef298edaee90d8d880e313f1"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [175, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4af00ab69d817b4cbf88af79"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 10], "end": [135, 15], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/native.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8bbaef2d4bbdd05728eb7f2"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [175, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c9138d71bd1eb1388fc1ae4"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [146, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/native.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c3f2d43790925a95dcef38b"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [175, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36935970eac04c0cf622160a"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [175, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a274c5b93b14f7527169231"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::FloatDistinctCountAccumulator", "path": "FloatDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [175, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
