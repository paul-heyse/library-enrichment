# `sqlparser::ast::query::JoinOperator::AsOf`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JoinOperator.AsOf.json).

<a id="op-aadd0762272d0a9bc2d81667"></a>
## constraint

`struct_field` · `sqlparser::ast::query::JoinOperator::AsOf::constraint` · sqlparser 0.62.0

```rust
constraint: JoinConstraint
```

Source: `src/ast/query.rs:2843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional constraint applied to the `ASOF` join.

<a id="op-6fa0d51501a4ac7362ef2c7b"></a>
## match_condition

`struct_field` · `sqlparser::ast::query::JoinOperator::AsOf::match_condition` · sqlparser 0.62.0

```rust
match_condition: Expr
```

Source: `src/ast/query.rs:2841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Condition used to match records in the `ASOF` join.
