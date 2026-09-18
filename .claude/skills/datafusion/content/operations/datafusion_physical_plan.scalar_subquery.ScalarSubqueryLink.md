# `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.scalar_subquery.ScalarSubqueryLink.json).

<a id="op-b27d7012cec7c3943ce0e2b9"></a>
## ScalarSubqueryLink

`struct` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink` · datafusion-physical-plan 55.1.0

```rust
struct ScalarSubqueryLink
```

Source: `src/scalar_subquery.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Links a scalar subquery's execution plan to its index in the shared results
container. The [`ScalarSubqueryExec`](../operations/datafusion_physical_plan.scalar_subquery.ScalarSubqueryExec.md#op-a478f6274c9e53b0018ce559) that owns these links populates
`results[index]` at execution time, and [`ScalarSubqueryExpr`] instances
with the same index read from it.

[`ScalarSubqueryExpr`]: datafusion_physical_expr::scalar_subquery::ScalarSubqueryExpr

<a id="op-602246ece380d5299fc1f6c4"></a>
## clone

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ScalarSubqueryLink
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink", "path": "ScalarSubqueryLink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/scalar_subquery.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00495fb8ebc5ae367b3095b4"></a>
## fmt

`function` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink", "path": "ScalarSubqueryLink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/scalar_subquery.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar_subquery.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06b6a5b9b8242837b31ac785"></a>
## index

`struct_field` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink::index` · datafusion-physical-plan 55.1.0

```rust
index: datafusion_expr::physical_planning_context::SubqueryIndex
```

Source: `src/scalar_subquery.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Index into the shared results container.

<a id="op-6001bbe2a8cf56f95fd5aaa5"></a>
## plan

`struct_field` · `datafusion_physical_plan::scalar_subquery::ScalarSubqueryLink::plan` · datafusion-physical-plan 55.1.0

```rust
plan: std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/scalar_subquery.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The physical plan for the subquery.
