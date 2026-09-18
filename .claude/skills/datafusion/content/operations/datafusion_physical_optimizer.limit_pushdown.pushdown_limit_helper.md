# `datafusion_physical_optimizer::limit_pushdown::pushdown_limit_helper`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limit_pushdown.pushdown_limit_helper.json).

<a id="op-0ff6da63266220d0ea2e772e"></a>
## pushdown_limit_helper

`function` · `datafusion_physical_optimizer::limit_pushdown::pushdown_limit_helper` · datafusion-physical-optimizer 55.1.0

```rust
fn pushdown_limit_helper(pushdown_plan: std::sync::Arc<dyn ExecutionPlan>, global_state: GlobalRequirements) -> datafusion_common::error::Result<(datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>, GlobalRequirements)>
```

Source: `src/limit_pushdown.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

This function is the main helper function of the `LimitPushDown` rule.
The helper takes an `ExecutionPlan` and a global (algorithm) state which is
an instance of `GlobalRequirements` and modifies these parameters while
checking if the limits can be pushed down or not.

If a limit is encountered, a [`TreeNodeRecursion::Stop`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-fbbedd1f0409ac786df535a1) is returned. Otherwise,
return a [`TreeNodeRecursion::Continue`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-128238c101f7ed2798f6d11c).
