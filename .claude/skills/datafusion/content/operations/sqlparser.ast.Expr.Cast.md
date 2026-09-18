# `sqlparser::ast::Expr::Cast`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Cast.json).

<a id="op-283e02480e7b4ff639613a58"></a>
## array

`struct_field` · `sqlparser::ast::Expr::Cast::array` · sqlparser 0.62.0

```rust
array: bool
```

Source: `src/ast/mod.rs:1098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] allows CAST(... AS type ARRAY) in functional index definitions for InnoDB
multi-valued indices. It's not really a datatype, and is only allowed in `CAST` in key
specifications, so it's a flag here.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/cast-functions.html#function_cast

<a id="op-a7ebf2bedee8ddcfbb6a53cd"></a>
## data_type

`struct_field` · `sqlparser::ast::Expr::Cast::data_type` · sqlparser 0.62.0

```rust
data_type: DataType
```

Source: `src/ast/mod.rs:1092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target data type.

<a id="op-78984bde268a1c1ae4a4f197"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Cast::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1090`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression being cast.

<a id="op-9e6f822d14f5aa122fcecb16"></a>
## format

`struct_field` · `sqlparser::ast::Expr::Cast::format` · sqlparser 0.62.0

```rust
format: Option<CastFormat>
```

Source: `src/ast/mod.rs:1102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional CAST(string_expression AS type FORMAT format_string_expression) as used by [BigQuery]

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/format-elements#formatting_syntax

<a id="op-4e56403d33580f1d5e4a4b74"></a>
## kind

`struct_field` · `sqlparser::ast::Expr::Cast::kind` · sqlparser 0.62.0

```rust
kind: CastKind
```

Source: `src/ast/mod.rs:1088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The cast kind (e.g., `CAST`, `TRY_CAST`).
