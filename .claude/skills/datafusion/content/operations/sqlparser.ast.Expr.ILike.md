# `sqlparser::ast::Expr::ILike`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.ILike.json).

<a id="op-91c5e3c35d30b8382a1d1ff2"></a>
## any

`struct_field` · `sqlparser::ast::Expr::ILike::any` · sqlparser 0.62.0

```rust
any: bool
```

Source: `src/ast/mod.rs:1007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake supports the ANY keyword to match against a list of patterns
<https://docs.snowflake.com/en/sql-reference/functions/like_any>

<a id="op-90ead13dc73f8b70c2644706"></a>
## escape_char

`struct_field` · `sqlparser::ast::Expr::ILike::escape_char` · sqlparser 0.62.0

```rust
escape_char: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:1013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional escape character.

<a id="op-e0af618a5b6dffc9c331ed44"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::ILike::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1009`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to match.

<a id="op-feb5d7098f9a748062204a5e"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::ILike::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:1004`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `NOT` is present.

<a id="op-5e0d86218149924b96ed221e"></a>
## pattern

`struct_field` · `sqlparser::ast::Expr::ILike::pattern` · sqlparser 0.62.0

```rust
pattern: Box<Expr>
```

Source: `src/ast/mod.rs:1011`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pattern expression.
