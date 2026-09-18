# `datafusion_optimizer::optimize_projections::OptimizeProjections`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimize_projections.OptimizeProjections.json).

<a id="op-9c35e531780246e722ca5275"></a>
## OptimizeProjections

`struct` · `datafusion_optimizer::optimize_projections::OptimizeProjections` · datafusion-optimizer 55.1.0

```rust
struct OptimizeProjections
```

Source: `src/optimize_projections/mod.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Optimizer rule to prune unnecessary columns from intermediate schemas
inside the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da). This rule:
- Removes unnecessary columns that do not appear at the output and/or are
  not used during any computation step.
- Adds projections to decrease table column size before operators that
  benefit from a smaller memory footprint at its input.
- Removes unnecessary [`LogicalPlan::Projection`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-355e9362720824ea20f96f5c)s from the [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da).

`OptimizeProjections` is an optimizer rule that identifies and eliminates
columns from a logical plan that are not used by downstream operations.
This can improve query performance and reduce unnecessary data processing.

The rule analyzes the input logical plan, determines the necessary column
indices, and then removes any unnecessary columns. It also removes any
unnecessary projections from the plan tree.

## Schema, Field Properties, and Metadata Handling

The `OptimizeProjections` rule preserves schema and field metadata in most optimization scenarios:

**Schema-level metadata preservation by plan type**:
- **Window and Aggregate plans**: Schema metadata is preserved
- **Projection plans**: Schema metadata is preserved per [`projection_schema`](datafusion_expr::logical_plan::projection_schema).
- **Other logical plans**: Schema metadata is preserved unless [`LogicalPlan::recompute_schema`]
  is called on plan types that drop metadata

**Field-level properties and metadata**: Individual field properties are preserved when fields
are retained in the optimized plan, determined by [`exprlist_to_fields`](datafusion_expr::utils::exprlist_to_fields)
and [`ExprSchemable::to_field`](datafusion_expr::expr_schema::ExprSchemable::to_field).

**Field precedence**: When the same field appears multiple times, the optimizer
maintains one occurrence and removes duplicates (refer to `RequiredIndices::compact()`),
preserving the properties and metadata of that occurrence.

Unresolved upstream links (retained, not inferred): ``LogicalPlan::recompute_schema``.

<a id="op-845e4c121d844e4946413e9f"></a>
## apply_order

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [105, 2], "filename": "src/optimize_projections/mod.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_projections/mod.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-330204393bc6819359f7847c"></a>
## default

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> OptimizeProjections
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 17], "filename": "src/optimize_projections/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/optimize_projections/mod.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-703aee721c371102672cfdf8"></a>
## fmt

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 19], "end": [73, 24], "filename": "src/optimize_projections/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/optimize_projections/mod.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d18085595e7862dc7b25eba6"></a>
## name

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [105, 2], "filename": "src/optimize_projections/mod.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_projections/mod.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9c685958a2c2c2fd8f983e0"></a>
## new

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [81, 2], "filename": "src/optimize_projections/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimize_projections/mod.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7872a0443514bdbf080cf35f"></a>
## rewrite

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [105, 2], "filename": "src/optimize_projections/mod.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_projections/mod.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c551b1d47cc9fd77bf59689f"></a>
## supports_rewrite

`function` · `datafusion_optimizer::optimize_projections::OptimizeProjections::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimize_projections::OptimizeProjections", "path": "OptimizeProjections"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [105, 2], "filename": "src/optimize_projections/mod.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/optimize_projections/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
