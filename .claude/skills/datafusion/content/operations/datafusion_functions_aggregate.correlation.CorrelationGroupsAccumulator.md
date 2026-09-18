# `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.correlation.CorrelationGroupsAccumulator.json).

<a id="op-fd6c73d61d647d6c7a44ffc3"></a>
## CorrelationGroupsAccumulator

`struct` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct CorrelationGroupsAccumulator
```

Source: `src/correlation.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71e2a62f6878dbc56729bc0b"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::convert_to_state` · datafusion-functions-aggregate 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [595, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/correlation.rs:492`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c965c3a5cff2b095fd76438"></a>
## default

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> CorrelationGroupsAccumulator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [290, 10], "end": [290, 17], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/correlation.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3358b3bbf2e4d6c64ba796a"></a>
## evaluate

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self, emit_to: EmitTo) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [595, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/correlation.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f49b8b97e5269d2d0faeb0e"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [595, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/correlation.rs:542`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b72a33906b2ffa49ead6f54"></a>
## new

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [312, 2], "filename": "src/correlation.rs"}, "trait": null, "trait_path": null}`

Source: `src/correlation.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83062ae884c3759fda415ee0"></a>
## size

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [595, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/correlation.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d16c3302e25f9c3743ccea33"></a>
## state

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [595, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/correlation.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c5e05d522bb4f55d007c14"></a>
## update_batch

`function` · `datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::CorrelationGroupsAccumulator", "path": "CorrelationGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [374, 1], "end": [595, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/correlation.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
