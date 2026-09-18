# `datafusion_physical_expr::intervals::test_utils::gen_conjunctive_temporal_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.test_utils.gen_conjunctive_temporal_expr.json).

<a id="op-4764f63c48bb9fa160ea4bc5"></a>
## gen_conjunctive_temporal_expr

`function` · `datafusion_physical_expr::intervals::test_utils::gen_conjunctive_temporal_expr` · datafusion-physical-expr 55.1.0

```rust
fn gen_conjunctive_temporal_expr(left_col: std::sync::Arc<dyn PhysicalExpr>, right_col: std::sync::Arc<dyn PhysicalExpr>, op_1: datafusion_expr::Operator, op_2: datafusion_expr::Operator, op_3: datafusion_expr::Operator, op_4: datafusion_expr::Operator, a: datafusion_common::ScalarValue, b: datafusion_common::ScalarValue, c: datafusion_common::ScalarValue, d: datafusion_common::ScalarValue, schema: &arrow::datatypes::Schema) -> Result<std::sync::Arc<dyn PhysicalExpr>, datafusion_common::DataFusionError>
```

Source: `src/intervals/test_utils.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This test function generates a conjunctive statement with
two scalar values with the following form:
left_col (op_1) a  > right_col (op_2) b AND left_col (op_3) c < right_col (op_4) d
