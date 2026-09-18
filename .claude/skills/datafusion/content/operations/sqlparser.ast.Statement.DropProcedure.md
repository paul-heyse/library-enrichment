# `sqlparser::ast::Statement::DropProcedure`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.DropProcedure.json).

<a id="op-80fcda3e240838c9db42cad7"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::Statement::DropProcedure::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/mod.rs:3959`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional drop behavior (`CASCADE` or `RESTRICT`).

<a id="op-f7b1249d075da6d2e3c464f2"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::DropProcedure::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:3955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF EXISTS` was present.

<a id="op-3c45dcb782cf77424b5866d2"></a>
## proc_desc

`struct_field` · `sqlparser::ast::Statement::DropProcedure::proc_desc` · sqlparser 0.62.0

```rust
proc_desc: Vec<FunctionDesc>
```

Source: `src/ast/mod.rs:3957`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more functions/procedures to drop.
