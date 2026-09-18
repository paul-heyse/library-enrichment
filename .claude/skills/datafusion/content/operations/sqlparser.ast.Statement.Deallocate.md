# `sqlparser::ast::Statement::Deallocate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Deallocate.json).

<a id="op-5cd915229a57171ea39fff9c"></a>
## name

`struct_field` · `sqlparser::ast::Statement::Deallocate::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:4523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name to deallocate (or `ALL`).

<a id="op-d4a3d84a20c36e0b425aba2a"></a>
## prepare

`struct_field` · `sqlparser::ast::Statement::Deallocate::prepare` · sqlparser 0.62.0

```rust
prepare: bool
```

Source: `src/ast/mod.rs:4525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `PREPARE` keyword was present.
