# `datafusion_sql`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.json).

<a id="op-22f32c632cae0eb354be0e76"></a>
## datafusion_sql

`module` · `datafusion_sql` · datafusion-sql 55.1.0

```rust
mod datafusion_sql
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

This crate provides:

1. A SQL parser, [`DFParser`], that translates SQL query text into
   an abstract syntax tree (AST), [`Statement`].

2. A SQL query planner [`SqlToRel`] that creates [`LogicalPlan`]s
   from [`Statement`]s.

3. A SQL [`unparser`](../modules/datafusion_sql.unparser.md#op-c2473a48a5ecbc2eebc3101b) that converts [`Expr`]s and [`LogicalPlan`]s
   into SQL query text.

[`DFParser`]: parser::DFParser
[`Statement`]: parser::Statement
[`SqlToRel`]: planner::SqlToRel
[`LogicalPlan`]: datafusion_expr::logical_plan::LogicalPlan
[`Expr`]: datafusion_expr::expr::Expr
