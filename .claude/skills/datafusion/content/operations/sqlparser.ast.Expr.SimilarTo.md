# `sqlparser::ast::Expr::SimilarTo`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.SimilarTo.json).

<a id="op-508b3dc30dadc1524fe732ec"></a>
## escape_char

`struct_field` · `sqlparser::ast::Expr::SimilarTo::escape_char` · sqlparser 0.62.0

```rust
escape_char: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:1024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional escape character.

<a id="op-7d6f800948b6b8460bc4a79e"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::SimilarTo::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to test.

<a id="op-e950c29840fa935bc1cf17c4"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::SimilarTo::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:1018`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `NOT` is present.

<a id="op-d0cee54577f4dca285e8e72e"></a>
## pattern

`struct_field` · `sqlparser::ast::Expr::SimilarTo::pattern` · sqlparser 0.62.0

```rust
pattern: Box<Expr>
```

Source: `src/ast/mod.rs:1022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pattern expression.
