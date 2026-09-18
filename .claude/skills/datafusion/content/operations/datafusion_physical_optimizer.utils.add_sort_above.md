# `datafusion_physical_optimizer::utils::add_sort_above`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.add_sort_above.json).

<a id="op-d9180ab19a40fb3ad11aef88"></a>
## add_sort_above

`function` · `datafusion_physical_optimizer::utils::add_sort_above` · datafusion-physical-optimizer 55.1.0

```rust
fn add_sort_above<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>) -> datafusion_physical_plan::tree_node::PlanContext<T>
```

Source: `src/utils.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This utility function adds a `SortExec` above an operator according to the
given ordering requirements while preserving the original partitioning.

Note that this updates the plan in both the `PlanContext.children` and
the `PlanContext.plan`'s children. Therefore its not required to sync
the child plans with [`PlanContext::update_plan_from_children`].

Unresolved upstream links (retained, not inferred): ``PlanContext::update_plan_from_children``.
