# `datafusion_physical_plan::execution_plan::apply_expression_roots`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.apply_expression_roots.json).

<a id="op-d8f6900037992304af2eeed5"></a>
## apply_expression_roots

`function` · `datafusion_physical_plan::execution_plan::apply_expression_roots` · datafusion-physical-plan 55.1.0

```rust
fn apply_expression_roots<I>(roots: I, f: &mut dyn FnMut(&std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<datafusion_common::tree_node::TreeNodeRecursion>) -> datafusion_common::Result<datafusion_common::tree_node::TreeNodeRecursion> where I: IntoIterator, I::Item: AsPhysicalExprRef
```

Source: `src/execution_plan.rs:1104`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Applies `f` to a shallow sequence of physical expression roots.

[`TreeNodeRecursion::Stop`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-fbbedd1f0409ac786df535a1) stops iteration and is returned immediately.
[`TreeNodeRecursion::Jump`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-5524dc7ddeb08b422f96ae27) is normalized to [`TreeNodeRecursion::Continue`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-128238c101f7ed2798f6d11c)
because this function does not visit expression children.
