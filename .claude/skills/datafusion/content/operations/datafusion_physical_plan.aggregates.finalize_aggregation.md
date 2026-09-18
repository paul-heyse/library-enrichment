# `datafusion_physical_plan::aggregates::finalize_aggregation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.finalize_aggregation.json).

<a id="op-c49b7adf2b5834dadb28a5c2"></a>
## finalize_aggregation

`function` · `datafusion_physical_plan::aggregates::finalize_aggregation` · datafusion-physical-plan 55.1.0

```rust
fn finalize_aggregation(accumulators: &mut [AccumulatorItem], mode: &AggregateMode) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>>
```

Source: `src/aggregates/mod.rs:2937`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

returns a vector of ArrayRefs, where each entry corresponds to either the
final value (mode = Final, FinalPartitioned and Single) or states (mode = Partial)
