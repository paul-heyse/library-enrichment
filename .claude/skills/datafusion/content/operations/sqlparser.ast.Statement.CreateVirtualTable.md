# `sqlparser::ast::Statement::CreateVirtualTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateVirtualTable.json).

<a id="op-e9fded424193973198b45294"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::CreateVirtualTable::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:3702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF NOT EXISTS` was specified.

<a id="op-ab0296bf944900f2855bcdf2"></a>
## module_args

`struct_field` · `sqlparser::ast::Statement::CreateVirtualTable::module_args` · sqlparser 0.62.0

```rust
module_args: Vec<Ident>
```

Source: `src/ast/mod.rs:3706`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arguments passed to the module.

<a id="op-429719504a569b8b3c4f2c83"></a>
## module_name

`struct_field` · `sqlparser::ast::Statement::CreateVirtualTable::module_name` · sqlparser 0.62.0

```rust
module_name: Ident
```

Source: `src/ast/mod.rs:3704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Module name used by the virtual table.

<a id="op-083b5e2879081f4e376f98d7"></a>
## name

`struct_field` · `sqlparser::ast::Statement::CreateVirtualTable::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:3700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the virtual table module instance.
