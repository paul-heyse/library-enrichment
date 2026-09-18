# `sqlparser::ast::Statement::UNCache`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.UNCache.json).

<a id="op-833a0033476cb33ef3a4ce43"></a>
## if_exists

`struct_field` · `sqlparser::ast::Statement::UNCache::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:4676`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF EXISTS` was present.

<a id="op-7632594e09db2e83a0faf706"></a>
## table_name

`struct_field` · `sqlparser::ast::Statement::UNCache::table_name` · sqlparser 0.62.0

```rust
table_name: ObjectName
```

Source: `src/ast/mod.rs:4674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name
