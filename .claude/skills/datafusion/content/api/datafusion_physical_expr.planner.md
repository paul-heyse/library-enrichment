# `datafusion_physical_expr::planner`

Crate `datafusion-physical-expr` · 3 public items · structured records in [`model/datafusion_physical_expr.planner.json`](../model/datafusion_physical_expr.planner.json)

## create_physical_expr

`function` · `datafusion_physical_expr::planner::create_physical_expr`

Also reachable as `datafusion::physical_expr::create_physical_expr`, `datafusion_physical_expr::create_physical_expr`

```rust
fn create_physical_expr(e: &datafusion_expr::Expr, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>>
```

[PhysicalExpr] evaluate DataFusion expressions such as `A + 1`, or `CAST(c1
AS int)`.

[PhysicalExpr] are the physical counterpart to [Expr] used in logical
planning, and can be evaluated directly on a [RecordBatch]. They are
normally created from [Expr] by a [PhysicalPlanner] and can be created
directly using [create_physical_expr].

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

Create a physical expression from a logical expression ([Expr]).

# Arguments

* `e` - The logical expression
* `input_dfschema` - The DataFusion schema for the input, used to resolve `Column` references
  to qualified or unqualified fields by name.
* `execution_props` - Per-execution properties such as the query start time.
* `planning_ctx` - The [`PhysicalPlanningContext`] used to resolve
  `Expr::ScalarSubquery` and `Expr::LambdaVariable` nodes. The physical
  planner threads the subquery index map and shared results container from
  its `ScalarSubqueryExec` construction into calls to
  `create_physical_expr`; the lambda variable qualifiers are added by this
  function itself as it descends into lambda bodies. Callers creating
  physical expressions outside of physical planning should pass
  `&PhysicalPlanningContext::default()`; converting a scalar subquery then returns a
  planning error.

---

## create_physical_exprs

`function` · `datafusion_physical_expr::planner::create_physical_exprs`

Also reachable as `datafusion::physical_expr::create_physical_exprs`, `datafusion_physical_expr::create_physical_exprs`

```rust
fn create_physical_exprs<'a, I>(exprs: I, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps, planning_ctx: &datafusion_expr::physical_planning_context::PhysicalPlanningContext) -> datafusion_common::Result<Vec<std::sync::Arc<dyn PhysicalExpr>>> where I: IntoIterator<Item = &'a datafusion_expr::Expr>
```

Create vector of Physical Expression from a vector of logical expression

See [`create_physical_expr`] for details on the `planning_ctx` argument.

---

## logical2physical

`function` · `datafusion_physical_expr::planner::logical2physical`

```rust
fn logical2physical(expr: &datafusion_expr::Expr, schema: &arrow::datatypes::Schema) -> std::sync::Arc<dyn PhysicalExpr>
```

Convert a logical expression to a physical expression (without any simplification, etc)

---
