# `sqlparser::ast::query::PipeOperator::Limit`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Limit.json).

<a id="op-d71f7a9ebf5c2705896900b5"></a>
## expr

`struct_field` · `sqlparser::ast::query::PipeOperator::Limit::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:3165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression specifying the number of rows to return.

<a id="op-a51216fedadadaec67f0af97"></a>
## offset

`struct_field` · `sqlparser::ast::query::PipeOperator::Limit::offset` · sqlparser 0.62.0

```rust
offset: Option<Expr>
```

Source: `src/ast/query.rs:3167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional offset expression provided inline with `LIMIT`.
