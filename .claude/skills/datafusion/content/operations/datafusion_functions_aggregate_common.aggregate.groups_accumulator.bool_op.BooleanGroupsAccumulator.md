# `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.aggregate.groups_accumulator.bool_op.BooleanGroupsAccumulator.json).

<a id="op-d1ddeed9a620a3512609616c"></a>
## BooleanGroupsAccumulator

`struct` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator` · datafusion-functions-aggregate-common 55.1.0

```rust
struct BooleanGroupsAccumulator<F> where F: Fn(bool, bool) -> bool + Send + Sync + 'static
```

Source: `src/aggregate/groups_accumulator/bool_op.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

An accumulator that implements a single operation over a
[`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) where the accumulated state is also boolean (such
as [`BitAndAssign`])

F: The function to apply to two elements. The first argument is
the existing value and should be updated with the second value
(e.g. [`BitAndAssign`] style).

[`BitAndAssign`]: std::ops::BitAndAssign

Unresolved upstream links (retained, not inferred): `std::ops::BitAndAssign`.

<a id="op-427b338801592037727371fe"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::convert_to_state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [70, 1], "end": [159, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b18a10e57c443c66e8163da1"></a>
## evaluate

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::evaluate` · datafusion-functions-aggregate-common 55.1.0

```rust
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [70, 1], "end": [159, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce4e592c3cde7796c4ea3ef4"></a>
## fmt

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::fmt` · datafusion-functions-aggregate-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6db0ca7d0726aad8f614b762"></a>
## merge_batch

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::merge_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [70, 1], "end": [159, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7c996f67a2a1f34120ac1fa"></a>
## new

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::new` · datafusion-functions-aggregate-common 55.1.0

```rust
fn new(bool_fn: F, identity: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [56, 1], "end": [68, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e2faf5801c6681fda2c66f5"></a>
## size

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::size` · datafusion-functions-aggregate-common 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [70, 1], "end": [159, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edd67bb34b0c9ad8743f9aa4"></a>
## state

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::state` · datafusion-functions-aggregate-common 55.1.0

```rust
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [70, 1], "end": [159, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68bdd597fe3a595d19c92b78"></a>
## update_batch

`function` · `datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator::update_batch` · datafusion-functions-aggregate-common 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_functions_aggregate_common::aggregate::groups_accumulator::bool_op::BooleanGroupsAccumulator", "path": "BooleanGroupsAccumulator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"primitive": "bool"}, {"primitive": "bool"}], "output": {"primitive": "bool"}}}, "id": "core::ops::function::Fn", "path": "Fn"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [70, 1], "end": [159, 2], "filename": "src/aggregate/groups_accumulator/bool_op.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/aggregate/groups_accumulator/bool_op.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
