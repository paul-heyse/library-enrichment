# `sqlparser::ast::query::PipeOperator::Except`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.Except.json).

<a id="op-bff2c042d3098194a5fee2e5"></a>
## queries

`struct_field` · `sqlparser::ast::query::PipeOperator::Except::queries` · sqlparser 0.62.0

```rust
queries: Vec<Query>
```

Source: `src/ast/query.rs:3292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The queries to exclude from the input set.

<a id="op-d1784f449d7e7a6c532d13cf"></a>
## set_quantifier

`struct_field` · `sqlparser::ast::query::PipeOperator::Except::set_quantifier` · sqlparser 0.62.0

```rust
set_quantifier: SetQuantifier
```

Source: `src/ast/query.rs:3290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set quantifier for the `EXCEPT` operator.
