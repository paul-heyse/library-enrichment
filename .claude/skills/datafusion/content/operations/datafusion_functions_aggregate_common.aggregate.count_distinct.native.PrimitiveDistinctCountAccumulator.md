# `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.native.PrimitiveDistinctCountAccumulator.json).

<a id="op-52cf9862e727897879506941"></a>
## PrimitiveDistinctCountAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct PrimitiveDistinctCountAccumulator<T> where T: ArrowPrimitiveType + Send, T::Native: Eq + Hash
```

Source: `src/aggregate/count_distinct/native.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa37112f9a83b9d7c7ab0e98"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self) -> datafusion_common::Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [68, 1], "end": [133, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-261e511211486e6e82c46d4a"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/count_distinct/native.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4094817734a388e0c0e705b5"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, states: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [68, 1], "end": [133, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec78212ec4c2059c802937f4"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(data_type: &DataType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [55, 1], "end": [66, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/native.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b9611b291d91d1b5dcebb86"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [68, 1], "end": [133, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0af50040ed5cedf08cc1242"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self) -> datafusion_common::Result<Vec<ScalarValue>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [68, 1], "end": [133, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-186708bc4a7a2f66e4ad0a78"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef]) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::native::PrimitiveDistinctCountAccumulator", "path": "PrimitiveDistinctCountAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [68, 1], "end": [133, 2], "filename": "src/aggregate/count_distinct/native.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::accumulator::Accumulator", "path": "Accumulator"}, "trait_path": "datafusion_expr_common::accumulator::Accumulator"}`

Source: `src/aggregate/count_distinct/native.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
