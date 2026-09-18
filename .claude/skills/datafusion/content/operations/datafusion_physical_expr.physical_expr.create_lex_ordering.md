# `datafusion_physical_expr::physical_expr::create_lex_ordering`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.physical_expr.create_lex_ordering.json).

<a id="op-f58978ebe2142bd0e4b29a92"></a>
## create_lex_ordering

`function` · `datafusion_physical_expr::physical_expr::create_lex_ordering` · datafusion-physical-expr 55.1.0

```rust
fn create_lex_ordering(schema: &arrow::datatypes::SchemaRef, sort_order: &[Vec<datafusion_expr::SortExpr>], execution_props: &datafusion_expr::execution_props::ExecutionProps) -> datafusion_common::Result<Vec<LexOrdering>>
```

Source: `src/physical_expr.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Creates a vector of [LexOrdering](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1) from a vector of logical expression
