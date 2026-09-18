# `datafusion_expr::expr_schema::cast_subquery`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_schema.cast_subquery.json).

<a id="op-76711ec36f5f31431eb55957"></a>
## cast_subquery

`function` · `datafusion_expr::expr_schema::cast_subquery` · datafusion-expr 55.1.0

```rust
fn cast_subquery(subquery: Subquery, cast_to_type: &arrow::datatypes::DataType) -> datafusion_common::Result<Subquery>
```

Source: `src/expr_schema.rs:795`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Cast subquery in InSubquery/ScalarSubquery to a given type.

1. **Projection plan**: If the subquery is a projection (i.e. a SELECT statement with specific
   columns), it casts the first expression in the projection to the target type and creates a
   new projection with the casted expression.
2. **Non-projection plan**: If the subquery isn't a projection, it adds a projection to the plan
   with the casted first column.
