# `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.VarianceGroupsAccumulator.json).

<a id="op-643d868cd4d39ad52fac0da7"></a>
## VarianceGroupsAccumulator

`struct` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator` · datafusion-functions-aggregate 55.1.0

```rust
struct VarianceGroupsAccumulator
```

Source: `src/variance.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-108bc297d52b21e9737723e1"></a>
## convert_to_state

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::convert_to_state` · datafusion-functions-aggregate 55.1.0

```rust
fn convert_to_state(&self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [624, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/variance.rs:586`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24372efc16102615d4ef13ce"></a>
## evaluate

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::evaluate` · datafusion-functions-aggregate 55.1.0

```rust
fn evaluate(&mut self, emit_to: datafusion_expr::EmitTo) -> Result<ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [624, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/variance.rs:569`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35e0a3477ee45ba18e17d295"></a>
## fmt

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [431, 10], "end": [431, 15], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variance.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3bffef62b3373f07072632a"></a>
## merge_batch

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::merge_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn merge_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [624, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/variance.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45626d340488ccb2473cb171"></a>
## new

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new(s_type: StatsType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [439, 1], "end": [502, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:440`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44aa81e8023216fc77c4a0cd"></a>
## size

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::size` · datafusion-functions-aggregate 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [624, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/variance.rs:619`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49d82a8180766e33d7e947ef"></a>
## state

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::state` · datafusion-functions-aggregate 55.1.0

```rust
fn state(&mut self, emit_to: datafusion_expr::EmitTo) -> Result<Vec<ArrayRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [624, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/variance.rs:574`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c29890df941432928461c446"></a>
## update_batch

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::update_batch` · datafusion-functions-aggregate 55.1.0

```rust
fn update_batch(&mut self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [504, 1], "end": [624, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr_common::groups_accumulator::GroupsAccumulator", "path": "GroupsAccumulator"}, "trait_path": "datafusion_expr_common::groups_accumulator::GroupsAccumulator"}`

Source: `src/variance.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47cf83e0db988f5a26395f5d"></a>
## variance

`function` · `datafusion_functions_aggregate::variance::VarianceGroupsAccumulator::variance` · datafusion-functions-aggregate 55.1.0

```rust
fn variance(&mut self, emit_to: datafusion_expr::EmitTo) -> (Vec<f64>, NullBuffer)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceGroupsAccumulator", "path": "VarianceGroupsAccumulator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [439, 1], "end": [502, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:479`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
