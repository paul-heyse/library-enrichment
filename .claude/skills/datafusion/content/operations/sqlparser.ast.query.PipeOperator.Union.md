# `sqlparser::ast::query::PipeOperator::Union`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Union.json).

<a id="op-41476178ed717e34266b5d5d"></a>
## queries

`struct_field` · `sqlparser::ast::query::PipeOperator::Union::queries` · sqlparser 0.62.0

```rust
queries: Vec<Query>
```

Source: `src/ast/query.rs:3270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The queries to combine with `UNION`.

<a id="op-1d7544500e2bd4e45d687d58"></a>
## set_quantifier

`struct_field` · `sqlparser::ast::query::PipeOperator::Union::set_quantifier` · sqlparser 0.62.0

```rust
set_quantifier: SetQuantifier
```

Source: `src/ast/query.rs:3268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set quantifier (`ALL` or `DISTINCT`).
