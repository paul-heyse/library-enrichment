# `sqlparser::ast::ddl::AlterTableOperation::AddConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.AddConstraint.json).

<a id="op-2429fe2aa39770be5d993bad"></a>
## constraint

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddConstraint::constraint` · sqlparser 0.62.0

```rust
constraint: ast::table_constraints::TableConstraint
```

Source: `src/ast/ddl.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The table constraint to add.

<a id="op-5778e9c7cdd473d9572bffe6"></a>
## not_valid

`struct_field` · `sqlparser::ast::ddl::AlterTableOperation::AddConstraint::not_valid` · sqlparser 0.62.0

```rust
not_valid: bool
```

Source: `src/ast/ddl.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the constraint should be marked `NOT VALID`.
