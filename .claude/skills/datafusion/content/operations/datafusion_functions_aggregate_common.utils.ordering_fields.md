# `datafusion_functions_aggregate_common::utils::ordering_fields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate_common.utils.ordering_fields.json).

<a id="op-267ec74609fb04ca4e413b07"></a>
## ordering_fields

`function` · `datafusion_functions_aggregate_common::utils::ordering_fields` · datafusion-functions-aggregate-common 55.1.0

```rust
fn ordering_fields(order_bys: &[datafusion_physical_expr_common::sort_expr::PhysicalSortExpr], data_types: &[arrow::datatypes::DataType]) -> Vec<arrow::datatypes::FieldRef>
```

Source: `src/utils.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate-common/55.1.0/json).

Construct corresponding fields for the expressions in an ORDER BY clause.
