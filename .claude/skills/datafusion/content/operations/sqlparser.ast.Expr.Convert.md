# `sqlparser::ast::Expr::Convert`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Convert.json).

<a id="op-bfdb36884d0654a2d8e48aa3"></a>
## charset

`struct_field` · `sqlparser::ast::Expr::Convert::charset` · sqlparser 0.62.0

```rust
charset: Option<ObjectName>
```

Source: `src/ast/mod.rs:1077`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional target character encoding (e.g., `utf8mb4`).

<a id="op-8b891cf966ef6f9cf1e4143e"></a>
## data_type

`struct_field` · `sqlparser::ast::Expr::Convert::data_type` · sqlparser 0.62.0

```rust
data_type: Option<DataType>
```

Source: `src/ast/mod.rs:1075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The target data type, if provided.

<a id="op-d3aac3361f06cb2b9ceee406"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Convert::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression to convert.

<a id="op-b17d02c27e3d9ec95cb84767"></a>
## is_try

`struct_field` · `sqlparser::ast::Expr::Convert::is_try` · sqlparser 0.62.0

```rust
is_try: bool
```

Source: `src/ast/mod.rs:1071`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CONVERT (false) or TRY_CONVERT (true)
<https://learn.microsoft.com/en-us/sql/t-sql/functions/try-convert-transact-sql?view=sql-server-ver16>

<a id="op-8016018ce01deb0e2e00e631"></a>
## styles

`struct_field` · `sqlparser::ast::Expr::Convert::styles` · sqlparser 0.62.0

```rust
styles: Vec<Expr>
```

Source: `src/ast/mod.rs:1083`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

How to translate the expression.

[MSSQL]: https://learn.microsoft.com/en-us/sql/t-sql/functions/cast-and-convert-transact-sql?view=sql-server-ver16#style

<a id="op-85a1321430e1fb4afb50582c"></a>
## target_before_value

`struct_field` · `sqlparser::ast::Expr::Convert::target_before_value` · sqlparser 0.62.0

```rust
target_before_value: bool
```

Source: `src/ast/mod.rs:1079`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when target precedes the value (MSSQL syntax).
