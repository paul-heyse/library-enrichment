# `datafusion_expr::expr_rewriter::coerce_plan_expr_for_schema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_rewriter.coerce_plan_expr_for_schema.json).

<a id="op-e9eedfbf32a8e9d02c164ba9"></a>
## coerce_plan_expr_for_schema

`function` · `datafusion_expr::expr_rewriter::coerce_plan_expr_for_schema` · datafusion-expr 55.1.0

```rust
fn coerce_plan_expr_for_schema(plan: LogicalPlan, schema: &datafusion_common::DFSchema) -> datafusion_common::Result<LogicalPlan>
```

Source: `src/expr_rewriter/mod.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns plan with expressions coerced to types compatible with
schema types
