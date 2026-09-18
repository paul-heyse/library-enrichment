# `sqlparser::ast::Expr::InList`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.InList.json).

<a id="op-fc859a3b22123d57662e131b"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::InList::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:943`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left-hand expression to test for membership.

<a id="op-c85ac8ada2d570423c146d8a"></a>
## list

`struct_field` · `sqlparser::ast::Expr::InList::list` · sqlparser 0.62.0

```rust
list: Vec<Expr>
```

Source: `src/ast/mod.rs:945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Literal list of expressions to check against.

<a id="op-5ba20a802d1f65de12deaf64"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::InList::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:947`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when the `NOT` modifier is present.
