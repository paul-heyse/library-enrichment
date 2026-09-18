# `datafusion_physical_expr_common::physical_expr::fmt_sql`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.fmt_sql.json).

<a id="op-5fc2d3b3a4f26d7a3eff8172"></a>
## fmt_sql

`function` · `datafusion_physical_expr_common::physical_expr::fmt_sql` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt_sql(expr: &dyn PhysicalExpr) -> impl Display + '_
```

Source: `src/physical_expr.rs:915`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Prints a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) in a SQL-like format

# Example
```
# // The boilerplate needed to create a `PhysicalExpr` for the example
use std::collections::HashMap;
# use std::fmt::Formatter;
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use arrow::datatypes::{DataType, Field, FieldRef, Schema};
# use datafusion_common::Result;
# use datafusion_expr_common::columnar_value::ColumnarValue;
# use datafusion_physical_expr_common::physical_expr::{fmt_sql, DynEq, PhysicalExpr};
# #[derive(Debug, PartialEq, Eq, Hash)]
# struct MyExpr {}
# impl PhysicalExpr for MyExpr {
# fn data_type(&self, input_schema: &Schema) -> Result<DataType> { unimplemented!() }
# fn nullable(&self, input_schema: &Schema) -> Result<bool> { unimplemented!() }
# fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> { unimplemented!() }
# fn return_field(&self, input_schema: &Schema) -> Result<FieldRef> { unimplemented!() }
# fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>{ unimplemented!() }
# fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>> { unimplemented!() }
# fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "CASE a > b THEN 1 ELSE 0 END") }
# }
# impl std::fmt::Display for MyExpr {fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { unimplemented!() } }
# fn make_physical_expr() -> Arc<dyn PhysicalExpr> { Arc::new(MyExpr{}) }
let expr: Arc<dyn PhysicalExpr> = make_physical_expr();
// wrap the expression in `sql_fmt` which can be used with
// `format!`, `to_string()`, etc
let expr_as_sql = fmt_sql(expr.as_ref());
assert_eq!(
  "The SQL: CASE a > b THEN 1 ELSE 0 END",
  format!("The SQL: {expr_as_sql}")
);
```
