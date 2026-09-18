# `sqlparser::ast::Statement::RaisError`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.RaisError.json).

<a id="op-d07197c58f11951bdac2d7e1"></a>
## arguments

`struct_field` · `sqlparser::ast::Statement::RaisError::arguments` · sqlparser 0.62.0

```rust
arguments: Vec<Expr>
```

Source: `src/ast/mod.rs:4878`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Substitution arguments for the message.

<a id="op-53fa8c3f124f2e7fa9b30b70"></a>
## message

`struct_field` · `sqlparser::ast::Statement::RaisError::message` · sqlparser 0.62.0

```rust
message: Box<Expr>
```

Source: `src/ast/mod.rs:4872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Error message expression or identifier.

<a id="op-491ca856015f2d1095eb4482"></a>
## options

`struct_field` · `sqlparser::ast::Statement::RaisError::options` · sqlparser 0.62.0

```rust
options: Vec<RaisErrorOption>
```

Source: `src/ast/mod.rs:4880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional `WITH` options for RAISERROR.

<a id="op-67eaf9d3638bdd6166dda7ba"></a>
## severity

`struct_field` · `sqlparser::ast::Statement::RaisError::severity` · sqlparser 0.62.0

```rust
severity: Box<Expr>
```

Source: `src/ast/mod.rs:4874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Severity expression.

<a id="op-e81dbf9ae152965fc88ada3b"></a>
## state

`struct_field` · `sqlparser::ast::Statement::RaisError::state` · sqlparser 0.62.0

```rust
state: Box<Expr>
```

Source: `src/ast/mod.rs:4876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

State expression.
