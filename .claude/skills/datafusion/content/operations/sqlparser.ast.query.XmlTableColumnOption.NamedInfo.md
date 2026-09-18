# `sqlparser::ast::query::XmlTableColumnOption::NamedInfo`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.XmlTableColumnOption.NamedInfo.json).

<a id="op-95fa608dbfa13c60b7fad1b0"></a>
## default

`struct_field` · `sqlparser::ast::query::XmlTableColumnOption::NamedInfo::default` · sqlparser 0.62.0

```rust
default: Option<Expr>
```

Source: `src/ast/query.rs:4206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default value if path does not match

<a id="op-5fa0534c2924a4cc96ec6ac6"></a>
## nullable

`struct_field` · `sqlparser::ast::query::XmlTableColumnOption::NamedInfo::nullable` · sqlparser 0.62.0

```rust
nullable: bool
```

Source: `src/ast/query.rs:4208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the column is nullable (NULL=true, NOT NULL=false)

<a id="op-85508f47e1f12f375201c7bd"></a>
## path

`struct_field` · `sqlparser::ast::query::XmlTableColumnOption::NamedInfo::path` · sqlparser 0.62.0

```rust
path: Option<Expr>
```

Source: `src/ast/query.rs:4204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path to the column to be extracted. If None, defaults to the column name.

<a id="op-df8becbcf1a5a6eae248d7a3"></a>
## type

`struct_field` · `sqlparser::ast::query::XmlTableColumnOption::NamedInfo::type` · sqlparser 0.62.0

```rust
type: DataType
```

Source: `src/ast/query.rs:4202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of the column to be extracted.
