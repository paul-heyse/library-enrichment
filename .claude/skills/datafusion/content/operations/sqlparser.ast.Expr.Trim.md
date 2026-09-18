# `sqlparser::ast::Expr::Trim`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Trim.json).

<a id="op-a3e76f29ecce2dec01db484a"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Trim::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1194`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression to trim from.

<a id="op-18bfff8aabe40e71a5405e97"></a>
## trim_characters

`struct_field` · `sqlparser::ast::Expr::Trim::trim_characters` · sqlparser 0.62.0

```rust
trim_characters: Option<Vec<Expr>>
```

Source: `src/ast/mod.rs:1196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional list of characters to trim (dialect-specific).

<a id="op-827bf7cc9f2380140747b41c"></a>
## trim_what

`struct_field` · `sqlparser::ast::Expr::Trim::trim_what` · sqlparser 0.62.0

```rust
trim_what: Option<Box<Expr>>
```

Source: `src/ast/mod.rs:1192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional expression specifying what to trim from the value `expr`.

<a id="op-efcc6949dde62fc6502b7307"></a>
## trim_where

`struct_field` · `sqlparser::ast::Expr::Trim::trim_where` · sqlparser 0.62.0

```rust
trim_where: Option<TrimWhereField>
```

Source: `src/ast/mod.rs:1190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Which side to trim: `BOTH`, `LEADING`, or `TRAILING`.
