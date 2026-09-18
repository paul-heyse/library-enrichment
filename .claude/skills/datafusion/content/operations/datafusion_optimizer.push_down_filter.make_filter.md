# `datafusion_optimizer::push_down_filter::make_filter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.push_down_filter.make_filter.json).

<a id="op-ff4e6c5531d57233cd6edec7"></a>
## make_filter

`function` · `datafusion_optimizer::push_down_filter::make_filter` · datafusion-optimizer 55.1.0

```rust
fn make_filter(predicate: datafusion_expr::Expr, input: std::sync::Arc<datafusion_expr::logical_plan::LogicalPlan>) -> datafusion_common::Result<datafusion_expr::logical_plan::LogicalPlan>
```

Source: `src/push_down_filter.rs:1347`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Creates a new LogicalPlan::Filter node.

Deprecated: use [`Filter::try_new`] directly.

Unresolved upstream links (retained, not inferred): ``Filter::try_new``.
