# `sqlparser::ast::ddl::AlterFunctionAction::Security`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterFunctionAction.Security.json).

<a id="op-08733bc3b3981f59d8a3d4f1"></a>
## external

`struct_field` · `sqlparser::ast::ddl::AlterFunctionAction::Security::external` · sqlparser 0.62.0

```rust
external: bool
```

Source: `src/ast/ddl.rs:5450`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the optional `EXTERNAL` keyword was present.

<a id="op-8558c0db17dfcfddd98d83f7"></a>
## security

`struct_field` · `sqlparser::ast::ddl::AlterFunctionAction::Security::security` · sqlparser 0.62.0

```rust
security: ast::FunctionSecurity
```

Source: `src/ast/ddl.rs:5452`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Security mode.
