# `datafusion_sql::unparser::dialect::ScalarFnToSqlHandler`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.ScalarFnToSqlHandler.json).

<a id="op-fc0b32781614475f8630e346"></a>
## ScalarFnToSqlHandler

`type_alias` · `datafusion_sql::unparser::dialect::ScalarFnToSqlHandler` · datafusion-sql 55.1.0

```rust
type ScalarFnToSqlHandler = Box<dyn Fn(&super::Unparser<'_>, &[datafusion_expr::Expr]) -> datafusion_common::Result<Option<ast::Expr>> + Send + Sync>
```

Source: `src/unparser/dialect.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
