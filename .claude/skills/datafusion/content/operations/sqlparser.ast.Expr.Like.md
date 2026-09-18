# `sqlparser::ast::Expr::Like`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Like.json).

<a id="op-b152cfa08296030bef5d1cee"></a>
## any

`struct_field` · `sqlparser::ast::Expr::Like::any` · sqlparser 0.62.0

```rust
any: bool
```

Source: `src/ast/mod.rs:993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake supports the ANY keyword to match against a list of patterns
<https://docs.snowflake.com/en/sql-reference/functions/like_any>

<a id="op-ae569868a2bd88a4305fad55"></a>
## escape_char

`struct_field` · `sqlparser::ast::Expr::Like::escape_char` · sqlparser 0.62.0

```rust
escape_char: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional escape character.

<a id="op-cc667338b7119a1c122d781a"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Like::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to match.

<a id="op-7afe9889b2ce9b936e8e0e50"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::Like::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:990`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `NOT` is present.

<a id="op-118d710735f9ce26f8d03c05"></a>
## pattern

`struct_field` · `sqlparser::ast::Expr::Like::pattern` · sqlparser 0.62.0

```rust
pattern: Box<Expr>
```

Source: `src/ast/mod.rs:997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pattern expression.
