# `sqlparser::ast::query::PipeOperator::Call`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Call.json).

<a id="op-697276abbf8e9e2865f6c397"></a>
## alias

`struct_field` · `sqlparser::ast::query::PipeOperator::Call::alias` · sqlparser 0.62.0

```rust
alias: Option<Ident>
```

Source: `src/ast/query.rs:3303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the result table.

<a id="op-a2c8c119bfe15f8abae90622"></a>
## function

`struct_field` · `sqlparser::ast::query::PipeOperator::Call::function` · sqlparser 0.62.0

```rust
function: Function
```

Source: `src/ast/query.rs:3301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function or procedure to call which returns a table.
