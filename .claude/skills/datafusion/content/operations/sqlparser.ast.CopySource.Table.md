# `sqlparser::ast::CopySource::Table`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopySource.Table.json).

<a id="op-0cd77bdeba7ac2595478a936"></a>
## columns

`struct_field` · `sqlparser::ast::CopySource::Table::columns` · sqlparser 0.62.0

```rust
columns: Vec<Ident>
```

Source: `src/ast/mod.rs:9261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A list of column names to copy. Empty list means that all columns
are copied.

<a id="op-47fb28b0ce1121cd8021db99"></a>
## table_name

`struct_field` · `sqlparser::ast::CopySource::Table::table_name` · sqlparser 0.62.0

```rust
table_name: ObjectName
```

Source: `src/ast/mod.rs:9258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the table to copy from.
