# `sqlparser::ast::Statement::Assert`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Assert.json).

<a id="op-db127ac7a57ecb3f630a7129"></a>
## condition

`struct_field` · `sqlparser::ast::Statement::Assert::condition` · sqlparser 0.62.0

```rust
condition: Expr
```

Source: `src/ast/mod.rs:4500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Assertion condition expression.

<a id="op-b76a4bc3b3a0553bd12fc2c5"></a>
## message

`struct_field` · `sqlparser::ast::Statement::Assert::message` · sqlparser 0.62.0

```rust
message: Option<Expr>
```

Source: `src/ast/mod.rs:4502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional message expression.
