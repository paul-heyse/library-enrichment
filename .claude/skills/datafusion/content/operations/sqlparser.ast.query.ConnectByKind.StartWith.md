# `sqlparser::ast::query::ConnectByKind::StartWith`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ConnectByKind.StartWith.json).

<a id="op-3f23d38658d76f89df2a8635"></a>
## condition

`struct_field` · `sqlparser::ast::query::ConnectByKind::StartWith::condition` · sqlparser 0.62.0

```rust
condition: Box<Expr>
```

Source: `src/ast/query.rs:1258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

condition selecting the root rows of the hierarchy

<a id="op-de811e10973aec685bcaeab4"></a>
## start_token

`struct_field` · `sqlparser::ast::query::ConnectByKind::StartWith::start_token` · sqlparser 0.62.0

```rust
start_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/query.rs:1255`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

the `START` token
