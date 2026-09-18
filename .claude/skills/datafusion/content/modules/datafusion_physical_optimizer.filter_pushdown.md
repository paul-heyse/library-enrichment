# `datafusion_physical_optimizer::filter_pushdown`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.filter_pushdown.json).

<a id="op-a6db4f7f5d82a1b9f3e07d1d"></a>
## filter_pushdown

`module` · `datafusion_physical_optimizer::filter_pushdown` · datafusion-physical-optimizer 55.1.0

```rust
mod filter_pushdown
```

Source: `src/filter_pushdown.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Filter Pushdown Optimization Process

The filter pushdown mechanism involves four key steps:
1. **Optimizer Asks Parent for a Filter Pushdown Plan**: The optimizer calls [`ExecutionPlan::gather_filters_for_pushdown`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-4d78429029b27c298789f1b5)
   on the parent node, passing in parent predicates and phase. The parent node creates a [`FilterDescription`]
   by inspecting its logic and children's schemas, determining which filters can be pushed to each child.
2. **Optimizer Executes Pushdown**: The optimizer recursively calls `push_down_filters` in this module on each child,
   passing the appropriate filters (`Vec<Arc<dyn PhysicalExpr>>`) for that child.
3. **Optimizer Gathers Results**: The optimizer collects [`FilterPushdownPropagation`](../operations/datafusion_physical_plan.filter_pushdown.FilterPushdownPropagation.md#op-df5a142ff6ba00afb8cb9286) results from children,
   containing information about which filters were successfully pushed down vs. unsupported.
4. **Parent Responds**: The optimizer calls [`ExecutionPlan::handle_child_pushdown_result`](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-be5acc8664105339204cc270) on the parent,
   passing a [`ChildPushdownResult`](../operations/datafusion_physical_plan.filter_pushdown.ChildPushdownResult.md#op-3d2ecd206686b6ef4bee5c4c) containing the aggregated pushdown outcomes. The parent decides
   how to handle filters that couldn't be pushed down (e.g., keep them as FilterExec nodes).

[`FilterDescription`]: datafusion_physical_plan::filter_pushdown::FilterDescription
