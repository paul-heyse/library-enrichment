# `datafusion_expr::expr_schema`

Crate `datafusion-expr` · 2 public items · structured records in [`model/datafusion_expr.expr_schema.json`](../model/datafusion_expr.expr_schema.json)

## cast_subquery

`function` · `datafusion_expr::expr_schema::cast_subquery`

```rust
fn cast_subquery(subquery: Subquery, cast_to_type: &arrow::datatypes::DataType) -> datafusion_common::Result<Subquery>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_schema.cast_subquery.md).


Cast subquery in InSubquery/ScalarSubquery to a given type.

1. **Projection plan**: If the subquery is a projection (i.e. a SELECT statement with specific
   columns), it casts the first expression in the projection to the target type and creates a
   new projection with the casted expression.
2. **Non-projection plan**: If the subquery isn't a projection, it adds a projection to the plan
   with the casted first column.

---

## ExprSchemable

`trait` · `datafusion_expr::expr_schema::ExprSchemable`

Also reachable as `datafusion::logical_expr::ExprSchemable`, `datafusion_expr::ExprSchemable`

```rust
trait ExprSchemable
```

**Implementors** (1)

- `datafusion_expr::expr::Expr`

**Methods** (6)

```rust
fn cast_to(self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr>
fn data_type_and_nullable(&self, schema: &dyn ExprSchema) -> Result<(DataType, bool)>
fn get_type(&self, schema: &dyn ExprSchema) -> Result<DataType>
fn metadata(&self, schema: &dyn ExprSchema) -> Result<FieldMetadata>
fn nullable(&self, input_schema: &dyn ExprSchema) -> Result<bool>
fn to_field(&self, input_schema: &dyn ExprSchema) -> Result<(Option<TableReference>, Arc<Field>)>
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.expr_schema.ExprSchemable.md).


Trait to allow expr to typable with respect to a schema

---
