# `datafusion_expr::logical_plan::builder::subquery_alias`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.builder.subquery_alias.json).

<a id="op-53ab92cf55bbe3161af1bb62"></a>
## subquery_alias

`function` · `datafusion_expr::logical_plan::builder::subquery_alias` · datafusion-expr 55.1.0

```rust
fn subquery_alias(plan: logical_plan::LogicalPlan, alias: impl Into<datafusion_common::TableReference>) -> datafusion_common::Result<logical_plan::LogicalPlan>
```

Source: `src/logical_plan/builder.rs:2065`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a SubqueryAlias to wrap a LogicalPlan.
