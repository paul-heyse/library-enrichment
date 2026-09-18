# `sqlparser::ast::Statement::CreateProcedure`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateProcedure.json).

<a id="op-03d1a93621b6fd0a5e487615"></a>
## body

`struct_field` · `sqlparser::ast::Statement::CreateProcedure::body` · sqlparser 0.62.0

```rust
body: ConditionalStatements
```

Source: `src/ast/mod.rs:4451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Procedure body statements.

<a id="op-f9e39d5cb1fb5bc6621fb34c"></a>
## language

`struct_field` · `sqlparser::ast::Statement::CreateProcedure::language` · sqlparser 0.62.0

```rust
language: Option<Ident>
```

Source: `src/ast/mod.rs:4449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional language identifier.

<a id="op-ccf51e78bc9522153407a59c"></a>
## name

`struct_field` · `sqlparser::ast::Statement::CreateProcedure::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:4445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Procedure name.

<a id="op-1293d677e9b0dbdfcec42dc4"></a>
## or_alter

`struct_field` · `sqlparser::ast::Statement::CreateProcedure::or_alter` · sqlparser 0.62.0

```rust
or_alter: bool
```

Source: `src/ast/mod.rs:4443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OR ALTER` flag.

<a id="op-95d6876c8f3ba3c4ab5623f4"></a>
## params

`struct_field` · `sqlparser::ast::Statement::CreateProcedure::params` · sqlparser 0.62.0

```rust
params: Option<Vec<ProcedureParam>>
```

Source: `src/ast/mod.rs:4447`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional procedure parameters.
