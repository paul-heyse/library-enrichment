# `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.nested_loop_join.NestedLoopJoinExecBuilder.json).

<a id="op-5f026e1259f245c8a193c25a"></a>
## NestedLoopJoinExecBuilder

`struct` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder` · datafusion-physical-plan 55.1.0

```rust
struct NestedLoopJoinExecBuilder
```

Source: `src/joins/nested_loop_join.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Helps to build [`NestedLoopJoinExec`](../operations/datafusion_physical_plan.joins.nested_loop_join.NestedLoopJoinExec.md#op-879865d333b3afaee6f0226e).

<a id="op-1d21727cccc33cacb9e26b9c"></a>
## build

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(self) -> Result<NestedLoopJoinExec>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder", "path": "NestedLoopJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [310, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:274`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build resulting execution plan.

<a id="op-f15c4a5c0632db120e505e32"></a>
## from

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder::from` · datafusion-physical-plan 55.1.0

```rust
fn from(exec: &NestedLoopJoinExec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder", "path": "NestedLoopJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [312, 1], "end": [322, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec", "path": "NestedLoopJoinExec"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/joins/nested_loop_join.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2e9c40a019daf28882bdd91"></a>
## new

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, join_type: JoinType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder", "path": "NestedLoopJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [310, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Make a new [`NestedLoopJoinExecBuilder`](../operations/datafusion_physical_plan.joins.nested_loop_join.NestedLoopJoinExecBuilder.md#op-5f026e1259f245c8a193c25a).

<a id="op-d065675d059754ccba72d0eb"></a>
## with_filter

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder::with_filter` · datafusion-physical-plan 55.1.0

```rust
fn with_filter(self, filter: Option<JoinFilter>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder", "path": "NestedLoopJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [310, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set optional filter.

<a id="op-5d88d2da103f8ee74e5cbba8"></a>
## with_projection

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder", "path": "NestedLoopJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [310, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set projection from the vector.

<a id="op-f21c258132ecdee773363d49"></a>
## with_projection_ref

`function` · `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder::with_projection_ref` · datafusion-physical-plan 55.1.0

```rust
fn with_projection_ref(self, projection: Option<ProjectionRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExecBuilder", "path": "NestedLoopJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [310, 2], "filename": "src/joins/nested_loop_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/nested_loop_join.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set projection from the shared reference.
