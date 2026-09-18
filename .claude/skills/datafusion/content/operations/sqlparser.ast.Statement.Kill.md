# `sqlparser::ast::Statement::Kill`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Kill.json).

<a id="op-701e3112b47f9eac268757ff"></a>
## id

`struct_field` · `sqlparser::ast::Statement::Kill::id` · sqlparser 0.62.0

```rust
id: u64
```

Source: `src/ast/mod.rs:4579`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The id of the process to kill.

<a id="op-bc2f71b559d00cb6c46f8aac"></a>
## modifier

`struct_field` · `sqlparser::ast::Statement::Kill::modifier` · sqlparser 0.62.0

```rust
modifier: Option<KillType>
```

Source: `src/ast/mod.rs:4576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional kill modifier (CONNECTION, QUERY, MUTATION).
