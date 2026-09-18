# `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.json).

<a id="op-df5a142ff6ba00afb8cb9286"></a>
## FilterPushdownPropagation

`struct` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation` · datafusion-physical-plan 55.1.0

```rust
struct FilterPushdownPropagation<T>
```

Source: `src/filter_pushdown.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The result of pushing down filters into a node.

Returned from [`ExecutionPlan::handle_child_pushdown_result`] to communicate
to the optimizer:

1. What to do with any parent filters that could not be pushed down into the children.
2. If the node needs to be replaced in the execution plan with a new node or not.

[`ExecutionPlan::handle_child_pushdown_result`]: crate::ExecutionPlan::handle_child_pushdown_result

<a id="op-2781ac8bfe7eb52256fce6bf"></a>
## all_unsupported

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::all_unsupported` · datafusion-physical-plan 55.1.0

```rust
fn all_unsupported(child_pushdown_result: ChildPushdownResult) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [293, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286) that tells the parent node that no filters were pushed down regardless of the child results.

<a id="op-52608230e4d208f0a7b4e162"></a>
## clone

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> FilterPushdownPropagation<T>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::clone::Clone", "path": "$crate::clone::Clone"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 17], "end": [227, 22], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/filter_pushdown.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4258bd5b83d9310317ed828c"></a>
## filters

`struct_field` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::filters` · datafusion-physical-plan 55.1.0

```rust
filters: Vec<PushedDown>
```

Source: `src/filter_pushdown.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Which parent filters were pushed down into this node's children.

<a id="op-7783abf36d7715e41c8d9452"></a>
## fmt

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 10], "end": [227, 15], "filename": "src/filter_pushdown.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_pushdown.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d13c63c6e3905c916eeef408"></a>
## if_all

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::if_all` · datafusion-physical-plan 55.1.0

```rust
fn if_all(child_pushdown_result: ChildPushdownResult) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [293, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286) that tells the parent node that each parent filter
is supported if it was supported by *all* children.

<a id="op-39bdb57d228b4f831aeb4e1c"></a>
## if_any

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::if_any` · datafusion-physical-plan 55.1.0

```rust
fn if_any(child_pushdown_result: ChildPushdownResult) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [293, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286) that tells the parent node that each parent filter
is supported if it was supported by *any* child.

<a id="op-5143e7e241c3c5a457a7c899"></a>
## updated_node

`struct_field` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::updated_node` · datafusion-physical-plan 55.1.0

```rust
updated_node: Option<T>
```

Source: `src/filter_pushdown.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The updated node, if it was updated during pushdown

<a id="op-2d7cf1531ddd78bc20b6ecea"></a>
## with_parent_pushdown_result

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::with_parent_pushdown_result` · datafusion-physical-plan 55.1.0

```rust
fn with_parent_pushdown_result(filters: Vec<PushedDown>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [293, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286) with the specified filter support.
This transmits up to our parent node what the result of pushing down the filters into our node and possibly our subtree was.

<a id="op-9d37fce664ffb94f2f7f9fac"></a>
## with_updated_node

`function` · `datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation::with_updated_node` · datafusion-physical-plan 55.1.0

```rust
fn with_updated_node(self, updated_node: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::filter_pushdown::FilterPushdownPropagation", "path": "FilterPushdownPropagation"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 1], "end": [293, 2], "filename": "src/filter_pushdown.rs"}, "trait": null, "trait_path": null}`

Source: `src/filter_pushdown.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Bind an updated node to the [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286).
Use this when the current node wants to update itself in the tree or replace itself with a new node (e.g. one of it's children).
You do not need to call this if one of the children of the current node may have updated itself, that is handled by the optimizer.
