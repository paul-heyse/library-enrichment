# `datafusion_substrait::logical_plan::consumer::utils::from_substrait_sorts`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.logical_plan.consumer.utils.from_substrait_sorts.json).

<a id="op-23e0a15737026d50a4dea2ce"></a>
## from_substrait_sorts

`function` · `datafusion_substrait::logical_plan::consumer::utils::from_substrait_sorts` · datafusion-substrait 55.1.0

```rust
async fn from_substrait_sorts(consumer: &impl SubstraitConsumer, substrait_sorts: &Vec<substrait::proto::SortField>, input_schema: &datafusion::common::DFSchema) -> datafusion::common::Result<Vec<datafusion::logical_expr::expr::Sort>>
```

Source: `src/logical_plan/consumer/utils.rs:503`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

Convert Substrait Sorts to DataFusion Exprs
