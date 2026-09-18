# `datafusion_physical_plan::aggregates::evaluate_many`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.evaluate_many.json).

<a id="op-f2e9680c24498f8c0fd41264"></a>
## evaluate_many

`function` · `datafusion_physical_plan::aggregates::evaluate_many` · datafusion-physical-plan 55.1.0

```rust
fn evaluate_many(expr: &[Vec<std::sync::Arc<dyn PhysicalExpr>>], batch: &arrow::record_batch::RecordBatch) -> datafusion_common::Result<Vec<Vec<arrow::array::ArrayRef>>>
```

Source: `src/aggregates/mod.rs:2967`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Evaluates groups of expressions against a record batch.
