# `datafusion_expr::logical_plan::tree_node`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.tree_node.json).

<a id="op-ee486bf162b6251d81b7775d"></a>
## tree_node

`module` · `datafusion_expr::logical_plan::tree_node` · datafusion-expr 55.1.0

```rust
mod tree_node
```

Source: `src/logical_plan/tree_node.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

 [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) based visiting and rewriting for [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s

Visiting (read only) APIs
* [`LogicalPlan::visit`](../operations/datafusion_common.tree_node.TreeNode.md#op-b7ab737d845b0a801118a434): recursively visit the node and all of its inputs
* [`LogicalPlan::visit_with_subqueries`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-d2f725875c279049fc96f907): recursively visit the node and all of its inputs, including subqueries
* [`LogicalPlan::apply_children`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-5da8c13d1e12c2c94ad45cc8): recursively visit all inputs of this node
* [`LogicalPlan::apply_expressions`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-cadaa16cd6ade9680d6cb6a9): (non recursively) visit all expressions of this node
* [`LogicalPlan::apply_subqueries`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-98ae917dee0a2ab135e1c90d): (non recursively) visit all subqueries of this node
* [`LogicalPlan::apply_with_subqueries`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-3645c45d939696c6a1ec91ee): recursively visit all inputs and embedded subqueries.

Rewriting (update) APIs:
* [`LogicalPlan::exists`](../operations/datafusion_common.tree_node.TreeNode.md#op-1dd3d36dcdbf23fccd60ad7a): search for an expression in a plan
* [`LogicalPlan::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17): recursively rewrite the node and all of its inputs
* [`LogicalPlan::map_children`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-dd2607b8f62b1af6eaee90dd): recursively rewrite all inputs of this node
* [`LogicalPlan::map_expressions`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-5aa96e67c3559c62a7922a8a): (non recursively) visit all expressions of this node
* [`LogicalPlan::map_subqueries`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-a12fe5f5d618b539a3e48e66): (non recursively) rewrite all subqueries of this node
* [`LogicalPlan::rewrite_with_subqueries`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-20798c575f68193a4ddc1f04): recursively rewrite the node and all of its inputs, including subqueries

(Re)creation APIs (these require substantial cloning and thus are slow):
* [`LogicalPlan::with_new_exprs`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-48b69499563a92bef71505fb): Create a new plan with different expressions
* [`LogicalPlan::expressions`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-586c0dd5f6fcdb0f90eeae74): Return a copy of the plan's expressions
