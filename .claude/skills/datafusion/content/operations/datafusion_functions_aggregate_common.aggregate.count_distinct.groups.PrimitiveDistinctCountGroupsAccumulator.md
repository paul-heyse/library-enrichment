# `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.count_distinct.groups.PrimitiveDistinctCountGroupsAccumulator.json).

<a id="op-65ec429235e4265c5b9ef303"></a>
## PrimitiveDistinctCountGroupsAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct PrimitiveDistinctCountGroupsAccumulator<T: ArrowPrimitiveType> where T::Native: Eq + Hash
```

Source: `src/aggregate/count_distinct/groups.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff48289279e9527a3afdc03f"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::convert_to_state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> datafusion_common::Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [215, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/count_distinct/groups.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a37bff58962aed497cee85d"></a>
## default

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::default` · datafusion-functions-aggregate-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [53, 1], "end": [60, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aggregate/count_distinct/groups.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17143733a8ac0f8938489312"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self, emit_to: EmitTo) -> datafusion_common::Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [215, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/count_distinct/groups.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8647912a2e138dd6ec1371f5"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [215, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/count_distinct/groups.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa1109629f07f3655cb7d2b7"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [41, 1], "end": [51, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/count_distinct/groups.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b17a3a1be255e78675a6a771"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [215, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/count_distinct/groups.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-056c159151837b0c2396364d"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self, emit_to: EmitTo) -> datafusion_common::Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [215, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/count_distinct/groups.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d2df16f76cd5a1700d60bcd"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> datafusion_common::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::count_distinct::groups::PrimitiveDistinctCountGroupsAccumulator", "path": "PrimitiveDistinctCountGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "std::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::Eq", "path": "Eq"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [215, 2], "filename": "src/aggregate/count_distinct/groups.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/count_distinct/groups.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
