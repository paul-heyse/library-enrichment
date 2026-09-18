# `sqlparser::ast::Statement::Commit`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Commit.json).

<a id="op-94eab30917affa0f40018e0f"></a>
## chain

`struct_field` · `sqlparser::ast::Statement::Commit::chain` · sqlparser 0.62.0

```rust
chain: bool
```

Source: `src/ast/mod.rs:4317`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `AND [ NO ] CHAIN` was present.

<a id="op-bb7d4e278d57d8713ab99516"></a>
## end

`struct_field` · `sqlparser::ast::Statement::Commit::end` · sqlparser 0.62.0

```rust
end: bool
```

Source: `src/ast/mod.rs:4319`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when this `COMMIT` was parsed as an `END` block terminator.

<a id="op-5c8670caaf79c3bb5dd27fbe"></a>
## modifier

`struct_field` · `sqlparser::ast::Statement::Commit::modifier` · sqlparser 0.62.0

```rust
modifier: Option<TransactionModifier>
```

Source: `src/ast/mod.rs:4321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional transaction modifier for commit semantics.
