# `datafusion_expr::utils::exprlist_to_fields`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.exprlist_to_fields.json).

<a id="op-3c3dc6d54191f75636f0ab8c"></a>
## exprlist_to_fields

`function` · `datafusion_expr::utils::exprlist_to_fields` · datafusion-expr 55.1.0

```rust
fn exprlist_to_fields<'a>(exprs: impl IntoIterator<Item = &'a Expr>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<(Option<datafusion_common::TableReference>, std::sync::Arc<arrow::datatypes::Field>)>>
```

Source: `src/utils.rs:855`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create schema fields from an expression list, for use in result set schema construction

This function converts a list of expressions into a list of complete schema fields,
making comprehensive determinations about each field's properties including:
- **Data type**: Resolved based on expression type and input schema context
- **Nullability**: Determined by expression-specific nullability rules
- **Metadata**: Computed based on expression type (preserving, merging, or generating new metadata)
- **Table reference scoping**: Establishing proper qualified field references

Each expression is converted to a field by calling [`Expr::to_field`](../operations/datafusion_expr.expr.Expr.md#op-77306878ad632fdf80f4fb7d), which performs
the complete field resolution process for all field properties.

# Returns

A `Result` containing a vector of `(Option<TableReference>, Arc<Field>)` tuples,
where each Field contains complete schema information (type, nullability, metadata)
and proper table reference scoping for the corresponding expression.
