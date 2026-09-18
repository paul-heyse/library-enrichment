# `datafusion_physical_plan::joins::utils::reorder_output_after_swap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.reorder_output_after_swap.json).

<a id="op-15dafac2d4fd608851765259"></a>
## reorder_output_after_swap

`function` · `datafusion_physical_plan::joins::utils::reorder_output_after_swap` · datafusion-physical-plan 55.1.0

```rust
fn reorder_output_after_swap(plan: std::sync::Arc<dyn ExecutionPlan>, left_schema: &arrow::datatypes::Schema, right_schema: &arrow::datatypes::Schema) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Source: `src/joins/utils.rs:2035`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

When the order of the join inputs are changed, the output order of columns
must remain the same.

Joins output columns from their left input followed by their right input.
Thus if the inputs are reordered, the output columns must be reordered to
match the original order.
