# `sqlparser::ast::Expr::Between`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Between.json).

<a id="op-766da67132c2260db9df70b7"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Between::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression being compared.

<a id="op-6289516664910f7f90077070"></a>
## high

`struct_field` · `sqlparser::ast::Expr::Between::high` · sqlparser 0.62.0

```rust
high: Box<Expr>
```

Source: `src/ast/mod.rs:976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Upper bound.

<a id="op-d410781e54c430a7a6873bf3"></a>
## low

`struct_field` · `sqlparser::ast::Expr::Between::low` · sqlparser 0.62.0

```rust
low: Box<Expr>
```

Source: `src/ast/mod.rs:974`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Lower bound.

<a id="op-757eed0d62b7348c6476e45a"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::Between::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:972`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when the `NOT` modifier is present.
