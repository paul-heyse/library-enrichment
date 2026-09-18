# `datafusion_physical_plan::aggregates::get_finer_aggregate_exprs_requirement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.get_finer_aggregate_exprs_requirement.json).

<a id="op-c11cd00874f69cbfdcfd836e"></a>
## get_finer_aggregate_exprs_requirement

`function` · `datafusion_physical_plan::aggregates::get_finer_aggregate_exprs_requirement` · datafusion-physical-plan 55.1.0

```rust
fn get_finer_aggregate_exprs_requirement(aggr_exprs: &mut [std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>], group_by: &PhysicalGroupBy, eq_properties: &datafusion_physical_expr::EquivalenceProperties, agg_mode: &AggregateMode) -> datafusion_common::Result<Vec<datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement>>
```

Source: `src/aggregates/mod.rs:2786`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Gets the common requirement that satisfies all the aggregate expressions.
When possible, chooses the requirement that is already satisfied by the
equivalence properties.

# Parameters

- `aggr_exprs`: A slice of `AggregateFunctionExpr` containing all the
  aggregate expressions.
- `group_by`: A reference to a `PhysicalGroupBy` instance representing the
  physical GROUP BY expression.
- `eq_properties`: A reference to an `EquivalenceProperties` instance
  representing equivalence properties for ordering.
- `agg_mode`: A reference to an `AggregateMode` instance representing the
  mode of aggregation.

# Returns

A `Result<Vec<PhysicalSortRequirement>>` instance, which is the requirement
that satisfies all the aggregate requirements. Returns an error in case of
conflicting requirements.
