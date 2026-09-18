# `sqlparser::ast::CreateFunctionBody::AsBeforeOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateFunctionBody.AsBeforeOptions.json).

<a id="op-a765242c40218dcfd7364b67"></a>
## body

`struct_field` · `sqlparser::ast::CreateFunctionBody::AsBeforeOptions::body` · sqlparser 0.62.0

```rust
body: Expr
```

Source: `src/ast/mod.rs:10132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The primary expression.

<a id="op-809c830df4eb78ffc6cd79d6"></a>
## link_symbol

`struct_field` · `sqlparser::ast::CreateFunctionBody::AsBeforeOptions::link_symbol` · sqlparser 0.62.0

```rust
link_symbol: Option<Expr>
```

Source: `src/ast/mod.rs:10141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Link symbol if the primary expression contains the name of shared library file.

Example:
```sql
CREATE FUNCTION cas_in(input cstring) RETURNS cas
AS 'MODULE_PATHNAME', 'cas_in_wrapper'
```
[PostgreSQL]: https://www.postgresql.org/docs/current/sql-createfunction.html
