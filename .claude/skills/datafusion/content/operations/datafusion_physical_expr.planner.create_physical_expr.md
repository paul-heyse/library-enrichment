# `datafusion_physical_expr::planner::create_physical_expr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.planner.create_physical_expr.json).

<a id="op-b01a3449753ecee1419f6104"></a>
## create_physical_expr

`function` · `datafusion_physical_expr::planner::create_physical_expr` · datafusion-physical-expr 55.1.0

```rust
fn create_physical_expr(e: &datafusion_expr::Expr, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/planner.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

[PhysicalExpr](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) evaluate DataFusion expressions such as `A + 1`, or `CAST(c1
AS int)`.

[PhysicalExpr](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) are the physical counterpart to [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) used in logical
planning, and can be evaluated directly on a [RecordBatch]. They are
normally created from [Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) by a [PhysicalPlanner] and can be created
directly using [create_physical_expr](../operations/datafusion_physical_expr.planner.create_physical_expr.md#op-b01a3449753ecee1419f6104).

A Physical expression knows its type, nullability and how to evaluate itself.

[PhysicalPlanner]: https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html
[RecordBatch]: https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html

# Example: Create `PhysicalExpr` from `Expr`
```
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_common::DFSchema;
# use datafusion_expr::{Expr, col, lit};
# use datafusion_physical_expr::create_physical_expr;
# use datafusion_expr::execution_props::ExecutionProps;
# use datafusion_expr::physical_planning_context::PhysicalPlanningContext;
// For a logical expression `a = 1`, we can create a physical expression
let expr = col("a").eq(lit(1));
// To create a PhysicalExpr we need 1. a schema
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
// 2. ExecutionProps
let props = ExecutionProps::new();
// We can now create a PhysicalExpr. Expressions with no scalar
// subqueries use an empty `PhysicalPlanningContext`:
let physical_expr =
    create_physical_expr(&expr, &df_schema, &props, &PhysicalPlanningContext::default())
        .unwrap();
```

# Example: Executing a PhysicalExpr to obtain [ColumnarValue]
```
# use std::sync::Arc;
# use arrow::array::{cast::AsArray, BooleanArray, Int32Array, RecordBatch};
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_common::{assert_batches_eq, DFSchema};
# use datafusion_expr::{Expr, col, lit, ColumnarValue};
# use datafusion_physical_expr::create_physical_expr;
# use datafusion_expr::execution_props::ExecutionProps;
# use datafusion_expr::physical_planning_context::PhysicalPlanningContext;
# let expr = col("a").eq(lit(1));
# let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
# let df_schema = DFSchema::try_from(schema.clone()).unwrap();
# let props = ExecutionProps::new();
// Given a PhysicalExpr, for `a = 1` we can evaluate it against a RecordBatch like this:
let physical_expr =
    create_physical_expr(&expr, &df_schema, &props, &PhysicalPlanningContext::default())
        .unwrap();
// Input of [1,2,3]
let input_batch = RecordBatch::try_from_iter(vec![
  ("a", Arc::new(Int32Array::from(vec![1, 2, 3])) as _)
]).unwrap();
// The result is a ColumnarValue (either an Array or a Scalar)
let result = physical_expr.evaluate(&input_batch).unwrap();
// In this case, a BooleanArray with the result of the comparison
let ColumnarValue::Array(arr) = result else {
 panic!("Expected an array")
};
assert_eq!(arr.as_boolean(), &BooleanArray::from(vec![true, false, false]));
```

[ColumnarValue]: datafusion_expr::ColumnarValue

Create a physical expression from a logical expression ([Expr](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)).

# Arguments

* `e` - The logical expression
* `input_dfschema` - The DataFusion schema for the input, used to resolve `Column` references
  to qualified or unqualified fields by name.
* `execution_props` - Per-execution properties such as the query start time.
* `planning_ctx` - The [`PhysicalPlanningContext`](../operations/datafusion_expr.physical_planning_context.PhysicalPlanningContext.md#op-6c42198b3422be2c46b9cf8b) used to resolve
  `Expr::ScalarSubquery` and `Expr::LambdaVariable` nodes. The physical
  planner threads the subquery index map and shared results container from
  its `ScalarSubqueryExec` construction into calls to
  `create_physical_expr`; the lambda variable qualifiers are added by this
  function itself as it descends into lambda bodies. Callers creating
  physical expressions outside of physical planning should pass
  `&PhysicalPlanningContext::default()`; converting a scalar subquery then returns a
  planning error.
