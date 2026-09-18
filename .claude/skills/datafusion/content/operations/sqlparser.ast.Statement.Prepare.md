# `sqlparser::ast::Statement::Prepare`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Prepare.json).

<a id="op-5bbabe85eb028e16abd33bc2"></a>
## data_types

`struct_field` · `sqlparser::ast::Statement::Prepare::data_types` · sqlparser 0.62.0

```rust
data_types: Vec<DataType>
```

Source: `src/ast/mod.rs:4564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional data types for parameters.

<a id="op-8c16d74d8ea9a66dfe6982dd"></a>
## name

`struct_field` · `sqlparser::ast::Statement::Prepare::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:4562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the prepared statement.

<a id="op-c9aeb481f4f4ffccae7fb085"></a>
## statement

`struct_field` · `sqlparser::ast::Statement::Prepare::statement` · sqlparser 0.62.0

```rust
statement: Box<Statement>
```

Source: `src/ast/mod.rs:4566`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Statement being prepared.
