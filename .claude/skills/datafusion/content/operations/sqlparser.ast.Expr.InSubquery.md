# `sqlparser::ast::Expr::InSubquery`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.InSubquery.json).

<a id="op-37993b4e9563e3518d76476e"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::InSubquery::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left-hand expression to test for membership.

<a id="op-7f94c5cc8e77f1f8cabe384f"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::InSubquery::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:956`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when the `NOT` modifier is present.

<a id="op-c480c39194c01b3dd095a020"></a>
## subquery

`struct_field` · `sqlparser::ast::Expr::InSubquery::subquery` · sqlparser 0.62.0

```rust
subquery: Box<Query>
```

Source: `src/ast/mod.rs:954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The subquery providing the candidate values.
