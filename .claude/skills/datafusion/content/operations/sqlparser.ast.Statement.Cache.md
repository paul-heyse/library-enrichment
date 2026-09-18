# `sqlparser::ast::Statement::Cache`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.Cache.json).

<a id="op-94ba61166f3ae5d9eaa54a55"></a>
## has_as

`struct_field` · `sqlparser::ast::Statement::Cache::has_as` · sqlparser 0.62.0

```rust
has_as: bool
```

Source: `src/ast/mod.rs:4662`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` if `AS` keyword was present before the query.

<a id="op-99912678f6339e022b0b3b6d"></a>
## options

`struct_field` · `sqlparser::ast::Statement::Cache::options` · sqlparser 0.62.0

```rust
options: Vec<SqlOption>
```

Source: `src/ast/mod.rs:4664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table confs

<a id="op-bd196a991598deb833b24905"></a>
## query

`struct_field` · `sqlparser::ast::Statement::Cache::query` · sqlparser 0.62.0

```rust
query: Option<Box<Query>>
```

Source: `src/ast/mod.rs:4666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Cache table as a Query

<a id="op-ce6074d36826f4e70a2a90f9"></a>
## table_flag

`struct_field` · `sqlparser::ast::Statement::Cache::table_flag` · sqlparser 0.62.0

```rust
table_flag: Option<ObjectName>
```

Source: `src/ast/mod.rs:4657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table flag

<a id="op-6159ab41041614b3e724cbeb"></a>
## table_name

`struct_field` · `sqlparser::ast::Statement::Cache::table_name` · sqlparser 0.62.0

```rust
table_name: ObjectName
```

Source: `src/ast/mod.rs:4660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name
