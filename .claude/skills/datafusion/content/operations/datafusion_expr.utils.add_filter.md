# `datafusion_expr::utils::add_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.add_filter.json).

<a id="op-5e3ba29551fd4668cf9f4baa"></a>
## add_filter

`function` · `datafusion_expr::utils::add_filter` · datafusion-expr 55.1.0

```rust
fn add_filter(plan: LogicalPlan, predicates: &[&Expr]) -> datafusion_common::Result<LogicalPlan>
```

Source: `src/utils.rs:1339`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a new [LogicalPlan](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) that filters the output of  `plan` with a
[LogicalPlan::Filter](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-0db7c924f38d73ed1007a273) with all `predicates` ANDed.

# Example
Before:
```text
plan
```

After:
```text
Filter(predicate)
  plan
```
