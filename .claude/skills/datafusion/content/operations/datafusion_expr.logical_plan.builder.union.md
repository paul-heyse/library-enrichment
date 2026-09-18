# `datafusion_expr::logical_plan::builder::union`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.union.json).

<a id="op-54e9bb17514175cbe14178f8"></a>
## union

`function` · `datafusion_expr::logical_plan::builder::union` · datafusion-expr 55.1.0

```rust
fn union(left_plan: logical_plan::LogicalPlan, right_plan: logical_plan::LogicalPlan) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Source: `src/logical_plan/builder.rs:1919`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Union two [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s.

Constructs the UNION plan, but does not perform type-coercion. Therefore the
subtree expressions will not be properly typed until the optimizer pass.

If a properly typed UNION plan is needed, refer to [`TypeCoercionRewriter::coerce_union`]
or alternatively, merge the union input schema using [`coerce_union_schema`] and
apply the expression rewrite with [`coerce_plan_expr_for_schema`](../operations/datafusion_expr.expr_rewriter.coerce_plan_expr_for_schema.md#op-e9eedfbf32a8e9d02c164ba9).

[`TypeCoercionRewriter::coerce_union`]: https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_union
[`coerce_union_schema`]: https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/fn.coerce_union_schema.html
