# `sqlparser::ast::query::ConnectByKind::ConnectBy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ConnectByKind.ConnectBy.json).

<a id="op-8500bd1a903bd64d6487dd1c"></a>
## connect_token

`struct_field` · `sqlparser::ast::query::ConnectByKind::ConnectBy::connect_token` · sqlparser 0.62.0

```rust
connect_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/query.rs:1238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the `CONNECT` token

<a id="op-bb1c27624af9932226b8a950"></a>
## nocycle

`struct_field` · `sqlparser::ast::query::ConnectByKind::ConnectBy::nocycle` · sqlparser 0.62.0

```rust
nocycle: bool
```

Source: `src/ast/query.rs:1243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[CONNECT BY] NOCYCLE

Optional on [Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Hierarchical-Queries.html#GUID-0118DF1D-B9A9-41EB-8556-C6E7D6A5A84E__GUID-5377971A-F518-47E4-8781-F06FEB3EF993)

<a id="op-495e2937c9bf14bf54c37371"></a>
## relationships

`struct_field` · `sqlparser::ast::query::ConnectByKind::ConnectBy::relationships` · sqlparser 0.62.0

```rust
relationships: Vec<Expr>
```

Source: `src/ast/query.rs:1246`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

join conditions denoting the hierarchical relationship
