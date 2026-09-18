# `sqlparser::ast::Expr::Substring`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Substring.json).

<a id="op-24b3d5bc1a9632e0e816b1f4"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Substring::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Source expression.

<a id="op-456daf8a6386511650f2444d"></a>
## shorthand

`struct_field` · `sqlparser::ast::Expr::Substring::shorthand` · sqlparser 0.62.0

```rust
shorthand: bool
```

Source: `src/ast/mod.rs:1181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

true if the expression is represented using the `SUBSTR` shorthand
This flag is used for formatting.

<a id="op-52cab0517ab58927514459de"></a>
## special

`struct_field` · `sqlparser::ast::Expr::Substring::special` · sqlparser 0.62.0

```rust
special: bool
```

Source: `src/ast/mod.rs:1177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

false if the expression is represented using the `SUBSTRING(expr [FROM start] [FOR len])` syntax
true if the expression is represented using the `SUBSTRING(expr, start, len)` syntax
This flag is used for formatting.

<a id="op-7d71ca4d04ba73492dc85d99"></a>
## substring_for

`struct_field` · `sqlparser::ast::Expr::Substring::substring_for` · sqlparser 0.62.0

```rust
substring_for: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:1172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `FOR` expression.

<a id="op-23d3c6ae026ba3fb275c25e2"></a>
## substring_from

`struct_field` · `sqlparser::ast::Expr::Substring::substring_from` · sqlparser 0.62.0

```rust
substring_from: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:1170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `FROM` expression.
