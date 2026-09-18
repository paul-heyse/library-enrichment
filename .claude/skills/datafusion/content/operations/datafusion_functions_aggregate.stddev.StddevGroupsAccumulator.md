# `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.stddev.StddevGroupsAccumulator.json).

<a id="op-df3d57a4b846d16b6104bc6f"></a>
## StddevGroupsAccumulator

`struct` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct StddevGroupsAccumulator
```

Source: `src/stddev.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ca64cec087f06704f00e660"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::convert_to_state` · datafusion-functions-aggregate 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [358, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/stddev.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c684b1d4d8d2161f7e59b539"></a>
## evaluate

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self, emit_to: datafusion_expr::EmitTo) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [358, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/stddev.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0f8f56d51217eefa1685ad5"></a>
## fmt

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [303, 10], "end": [303, 15], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stddev.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a3b550328663bf5e1bc51e"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [358, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/stddev.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79417242efc6ce29badfd793"></a>
## new

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new(s_type: StatsType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [314, 2], "filename": "src/stddev.rs"}, "trait": null, "trait_path": null}`

Source: `src/stddev.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1684d9ab20d797eb8dcd7379"></a>
## size

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [358, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/stddev.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c22b27981e659d89227a149"></a>
## state

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self, emit_to: datafusion_expr::EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [358, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/stddev.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-621713417e3935933ab09709"></a>
## update_batch

`function` · `datafusion_functions_aggregate::stddev::StddevGroupsAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevGroupsAccumulator", "path": "StddevGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [316, 1], "end": [358, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/stddev.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
