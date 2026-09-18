# `sqlparser::ast::query::PipeOperator::Aggregate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Aggregate.json).

<a id="op-50f8292e55e09211efe7f658"></a>
## full_table_exprs

`struct_field` · `sqlparser::ast::query::PipeOperator::Aggregate::full_table_exprs` · sqlparser 0.62.0

```rust
full_table_exprs: Vec<ExprWithAliasAndOrderBy>
```

Source: `src/ast/query.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expressions computed for each row prior to grouping.

<a id="op-bf8131f1f522b9437adb6879"></a>
## group_by_expr

`struct_field` · `sqlparser::ast::query::PipeOperator::Aggregate::group_by_expr` · sqlparser 0.62.0

```rust
group_by_expr: Vec<ExprWithAliasAndOrderBy>
```

Source: `src/ast/query.rs:3243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grouping expressions for aggregation.
