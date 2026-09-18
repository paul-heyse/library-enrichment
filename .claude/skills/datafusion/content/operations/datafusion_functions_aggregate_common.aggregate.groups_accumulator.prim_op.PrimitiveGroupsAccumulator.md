# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.prim_op.PrimitiveGroupsAccumulator.json).

<a id="op-bd7b272e28c32581419350f3"></a>
## PrimitiveGroupsAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct PrimitiveGroupsAccumulator<T, F> where T: ArrowPrimitiveType + Send, F: Fn(&mut T::Native, T::Native) + Send + Sync + 'static
```

Source: `src/aggregate/groups_accumulator/prim_op.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

An accumulator that implements a single operation over
[`ArrowPrimitiveType`](../operations/arrow_array.types.ArrowPrimitiveType.md#op-ddd581d2aed24174207ba803) where the accumulated state is the same as
the input type (such as `Sum`)

F: The function to apply to two elements. The first argument is
the existing value and should be updated with the second value
(e.g. [`BitAndAssign`] style).

[`BitAndAssign`]: std::ops::BitAndAssign

Unresolved upstream links (retained, not inferred): `std::ops::BitAndAssign`.

<a id="op-fc5954e123b2b11d7e0e56b3"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::convert_to_state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [84, 1], "end": [195, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Converts an input batch directly to a state batch

The state is:
- self.prim_fn for all non null, non filtered values
- null otherwise

<a id="op-db980d3e1b656910d82a97de"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [84, 1], "end": [195, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6b73be64b6cb4c5e67bcb5b"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "F"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}]}, "is_negative": false, "span": {"begin": [40, 10], "end": [40, 15], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f8781d60711142d3d25dcc9"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [84, 1], "end": [195, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e70af5f1d564ac8aa1a18fd"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(data_type: &DataType, prim_fn: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [82, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ae1fad64931fcd97d247dfe"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [84, 1], "end": [195, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9906897dfefc74a0dfbd2f8e"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [84, 1], "end": [195, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9455373e90afb25fa389235"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [84, 1], "end": [195, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0761b9400c90924039d624d3"></a>
## with_starting_value

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator::with_starting_value` · datafusion-functions-aggregate-common 55.1.0

```rust
fn with_starting_value(self, starting_value: T::Native) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::prim_op::PrimitiveGroupsAccumulator", "path": "PrimitiveGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}], "generic_params": [], "type": {"generic": "T"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"borrowed_ref": {"is_mutable": true, "lifetime": null, "type": {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}}}, {"qualified_path": {"args": null, "name": "Native", "self_type": {"generic": "T"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": ""}}}], "output": null}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [62, 1], "end": [82, 2], "filename": "src/aggregate/groups_accumulator/prim_op.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/prim_op.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Set the starting values for new groups
