# `sqlparser::ast::Statement::AlterConnector`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.AlterConnector.json).

<a id="op-1e07e58a3a512f8c94a553d9"></a>
## name

`struct_field` · `sqlparser::ast::Statement::AlterConnector::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:3852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the connector to alter.

<a id="op-c62b0d87f39b0b53ead5dd7a"></a>
## owner

`struct_field` · `sqlparser::ast::Statement::AlterConnector::owner` · sqlparser 0.62.0

```rust
owner: Option<ddl::AlterConnectorOwner>
```

Source: `src/ast/mod.rs:3858`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional new owner specification.

<a id="op-b400bc97cb80b11de6b51c62"></a>
## properties

`struct_field` · `sqlparser::ast::Statement::AlterConnector::properties` · sqlparser 0.62.0

```rust
properties: Option<Vec<SqlOption>>
```

Source: `src/ast/mod.rs:3854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional connector properties to set.

<a id="op-8bbef9ef9f30157cdede5a8d"></a>
## url

`struct_field` · `sqlparser::ast::Statement::AlterConnector::url` · sqlparser 0.62.0

```rust
url: Option<String>
```

Source: `src/ast/mod.rs:3856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional new URL for the connector.
