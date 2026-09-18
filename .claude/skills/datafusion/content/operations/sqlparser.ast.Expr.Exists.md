# `sqlparser::ast::Expr::Exists`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Exists.json).

<a id="op-83c4e751d509481b1319ab17"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::Exists::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:1261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `EXISTS` is negated (`NOT EXISTS`).

<a id="op-01caafa8b01325c17712f223"></a>
## subquery

`struct_field` · `sqlparser::ast::Expr::Exists::subquery` · sqlparser 0.62.0

```rust
subquery: Box<Query>
```

Source: `src/ast/mod.rs:1259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The subquery checked by `EXISTS`.
