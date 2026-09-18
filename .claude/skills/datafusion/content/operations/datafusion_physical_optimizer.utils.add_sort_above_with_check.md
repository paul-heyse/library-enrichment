# `datafusion_physical_optimizer::utils::add_sort_above_with_check`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.utils.add_sort_above_with_check.json).

<a id="op-6f61de08ff2213ee4948bd3c"></a>
## add_sort_above_with_check

`function` · `datafusion_physical_optimizer::utils::add_sort_above_with_check` · datafusion-physical-optimizer 55.1.0

```rust
fn add_sort_above_with_check<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>) -> datafusion_common::Result<datafusion_physical_plan::tree_node::PlanContext<T>>
```

Source: `src/utils.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This utility function adds a `SortExec` above an operator according to the
given ordering requirements while preserving the original partitioning. If
requirement is already satisfied no `SortExec` is added.
