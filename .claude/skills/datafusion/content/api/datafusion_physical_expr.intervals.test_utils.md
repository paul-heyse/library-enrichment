# `datafusion_physical_expr::intervals::test_utils`

Crate `datafusion-physical-expr` · 2 public items · structured records in [`model/datafusion_physical_expr.intervals.test_utils.json`](../model/datafusion_physical_expr.intervals.test_utils.json)

## gen_conjunctive_numerical_expr

`function` · `datafusion_physical_expr::intervals::test_utils::gen_conjunctive_numerical_expr`

```rust
fn gen_conjunctive_numerical_expr(left_col: std::sync::Arc<dyn PhysicalExpr>, right_col: std::sync::Arc<dyn PhysicalExpr>, op: (datafusion_expr::Operator, datafusion_expr::Operator, datafusion_expr::Operator, datafusion_expr::Operator), a: datafusion_common::ScalarValue, b: datafusion_common::ScalarValue, c: datafusion_common::ScalarValue, d: datafusion_common::ScalarValue, bounds: (datafusion_expr::Operator, datafusion_expr::Operator)) -> std::sync::Arc<dyn PhysicalExpr>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.intervals.test_utils.gen_conjunctive_numerical_expr.md).


This test function generates a conjunctive statement with two numeric
terms with the following form:
left_col (op_1) a  >/>= right_col (op_2) b AND left_col (op_3) c </<= right_col (op_4) d

---

## gen_conjunctive_temporal_expr

`function` · `datafusion_physical_expr::intervals::test_utils::gen_conjunctive_temporal_expr`

```rust
fn gen_conjunctive_temporal_expr(left_col: std::sync::Arc<dyn PhysicalExpr>, right_col: std::sync::Arc<dyn PhysicalExpr>, op_1: datafusion_expr::Operator, op_2: datafusion_expr::Operator, op_3: datafusion_expr::Operator, op_4: datafusion_expr::Operator, a: datafusion_common::ScalarValue, b: datafusion_common::ScalarValue, c: datafusion_common::ScalarValue, d: datafusion_common::ScalarValue, schema: &arrow::datatypes::Schema) -> Result<std::sync::Arc<dyn PhysicalExpr>, datafusion_common::DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_expr.intervals.test_utils.gen_conjunctive_temporal_expr.md).


This test function generates a conjunctive statement with
two scalar values with the following form:
left_col (op_1) a  > right_col (op_2) b AND left_col (op_3) c < right_col (op_4) d

---
