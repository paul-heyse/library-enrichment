# `datafusion_physical_plan::filter::collect_columns_from_predicate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.filter.collect_columns_from_predicate.json).

<a id="op-8f1ed4a120e4084b6ca8a354"></a>
## collect_columns_from_predicate

`function` · `datafusion_physical_plan::filter::collect_columns_from_predicate` · datafusion-physical-plan 55.1.0

```rust
fn collect_columns_from_predicate(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> EqualAndNonEqual<'_>
```

Source: `src/filter.rs:1360`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return the equals Column-Pairs and Non-equals Column-Pairs
