# `sqlparser::ast::query::LimitClause::LimitOffset`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.LimitClause.LimitOffset.json).

<a id="op-e773bab566d87ef822f39441"></a>
## limit

`struct_field` · `sqlparser::ast::query::LimitClause::LimitOffset::limit` · sqlparser 0.62.0

```rust
limit: Option<Expr>
```

Source: `src/ast/query.rs:3061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LIMIT { <N> | ALL }` expression.

<a id="op-54d72e68cbab5b0106ee89be"></a>
## limit_by

`struct_field` · `sqlparser::ast::query::LimitClause::LimitOffset::limit_by` · sqlparser 0.62.0

```rust
limit_by: Vec<Expr>
```

Source: `src/ast/query.rs:3065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `BY { <expr>,... }` list used by some dialects (ClickHouse).

<a id="op-7d335bf334c398d80912f2d5"></a>
## offset

`struct_field` · `sqlparser::ast::query::LimitClause::LimitOffset::offset` · sqlparser 0.62.0

```rust
offset: Option<Offset>
```

Source: `src/ast/query.rs:3063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `OFFSET` expression with optional `ROW(S)` keyword.
