# `datafusion_physical_plan::filter_pushdown::ChildPushdownResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.ChildPushdownResult.json).

<a id="op-3d2ecd206686b6ef4bee5c4c"></a>
## ChildPushdownResult

`struct` · `datafusion_physical_plan::filter_pushdown::ChildPushdownResult` · datafusion-physical-plan 55.1.0

```rust
struct ChildPushdownResult
```

Source: `src/filter_pushdown.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The result of pushing down filters into a child node.

This is the result provided to nodes in [`ExecutionPlan::handle_child_pushdown_result`].
Nodes process this result and convert it into a [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286)
that is returned to their parent.

[`ExecutionPlan::handle_child_pushdown_result`]: crate::ExecutionPlan::handle_child_pushdown_result

<a id="op-28f8a608aeb63f932c332f9f"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::ChildPushdownResult::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ChildPushdownResult
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::ChildPushdownResult", "path": "ChildPushdownResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 17], "end": [203, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96d97207aa64b10ef4ed7d4b"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::ChildPushdownResult::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::filter_pushdown::ChildPushdownResult", "path": "ChildPushdownResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [203, 10], "end": [203, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8180cfa24538c5bbfd795142"></a>
## parent_filters

`struct_field` · `datafusion_physical_plan::filter_pushdown::ChildPushdownResult::parent_filters` · datafusion-physical-plan 55.1.0

```rust
parent_filters: Vec<ChildFilterPushdownResult>
```

Source: `src/filter_pushdown.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The parent filters that were pushed down as received by the current node when [`ExecutionPlan::gather_filters_for_pushdown`](crate::ExecutionPlan::handle_child_pushdown_result) was called.
Note that this may *not* be the same as the filters that were passed to the children as the current node may have modified them
(e.g. by reassigning column indices) when it returned them from [`ExecutionPlan::gather_filters_for_pushdown`](crate::ExecutionPlan::handle_child_pushdown_result) in a [`FilterDescription`](../operations/datafusion_physical_plan.filter_pushdown.FilterDescription.md#op-0116d642527aa63591adbd0f).
Attached to each filter is a [`PushedDown`](../operations/datafusion_physical_plan.filter_pushdown.PushedDown.md#op-4168cba55c6e8d62d2eccce0) *per child* that indicates whether the filter was supported or unsupported by each child.
To get combined results see [`ChildFilterPushdownResult::any`](../operations/datafusion_physical_plan.filter_pushdown.ChildFilterPushdownResult.md#op-47e85b0b009be02ca350d683) and [`ChildFilterPushdownResult::all`](../operations/datafusion_physical_plan.filter_pushdown.ChildFilterPushdownResult.md#op-7581eca6d4cfc872a42b3574).

<a id="op-6d5503887989d31609936fae"></a>
## self_filters

`struct_field` · `datafusion_physical_plan::filter_pushdown::ChildPushdownResult::self_filters` · datafusion-physical-plan 55.1.0

```rust
self_filters: Vec<Vec<PushedDownPredicate>>
```

Source: `src/filter_pushdown.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The result of pushing down each filter this node provided into each of it's children.
The outer vector corresponds to each child, and the inner vector corresponds to each filter.
Since this node may have generated a different filter for each child the inner vector may have different lengths or the expressions may not match at all.
It is up to each node to interpret this result based on the filters it provided for each child in [`ExecutionPlan::gather_filters_for_pushdown`](crate::ExecutionPlan::handle_child_pushdown_result).
