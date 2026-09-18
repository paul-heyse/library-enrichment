# `datafusion_physical_expr::intervals::test_utils::gen_conjunctive_numerical_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.test_utils.gen_conjunctive_numerical_expr.json).

<a id="op-a03091b7ea0e0e0e602d0946"></a>
## gen_conjunctive_numerical_expr

`function` · `datafusion_physical_expr::intervals::test_utils::gen_conjunctive_numerical_expr` · datafusion-physical-expr 55.1.0

```rust
fn gen_conjunctive_numerical_expr(left_col: std::sync::Arc<dyn PhysicalExpr>, right_col: std::sync::Arc<dyn PhysicalExpr>, op: (datafusion_expr::Operator, datafusion_expr::Operator, datafusion_expr::Operator, datafusion_expr::Operator), a: datafusion_common::ScalarValue, b: datafusion_common::ScalarValue, c: datafusion_common::ScalarValue, d: datafusion_common::ScalarValue, bounds: (datafusion_expr::Operator, datafusion_expr::Operator)) -> std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/intervals/test_utils.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This test function generates a conjunctive statement with two numeric
terms with the following form:
left_col (op_1) a  >/>= right_col (op_2) b AND left_col (op_3) c </<= right_col (op_4) d
