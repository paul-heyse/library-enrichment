# `sqlparser::ast::Statement::CreateSchema`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateSchema.json).

<a id="op-a1d99ee66dbc89dfa7a55b77"></a>
## clone

`struct_field` · `sqlparser::ast::Statement::CreateSchema::clone` · sqlparser 0.62.0

```rust
clone: Option<ObjectName>
```

Source: `src/ast/mod.rs:4371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Clones a schema

```sql
CREATE SCHEMA myschema CLONE otherschema
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-clone#databases-schemas)

<a id="op-e84c848fe84aaced8c9fff25"></a>
## default_collate_spec

`struct_field` · `sqlparser::ast::Statement::CreateSchema::default_collate_spec` · sqlparser 0.62.0

```rust
default_collate_spec: Option<Expr>
```

Source: `src/ast/mod.rs:4363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default collation specification for the schema.

```sql
CREATE SCHEMA myschema DEFAULT COLLATE 'und:ci';
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_schema_statement)

<a id="op-e0489f769e917f23026cefd7"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::CreateSchema::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:4339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `IF NOT EXISTS` was present.

<a id="op-c391e076cdfbadcd1cf87b9b"></a>
## options

`struct_field` · `sqlparser::ast::Statement::CreateSchema::options` · sqlparser 0.62.0

```rust
options: Option<Vec<SqlOption>>
```

Source: `src/ast/mod.rs:4355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Schema options.

```sql
CREATE SCHEMA myschema OPTIONS(key1='value1');
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_schema_statement)

<a id="op-0b5e668f8d1a0e6ff55c4503"></a>
## schema_name

`struct_field` · `sqlparser::ast::Statement::CreateSchema::schema_name` · sqlparser 0.62.0

```rust
schema_name: SchemaName
```

Source: `src/ast/mod.rs:4337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<schema name> | AUTHORIZATION <schema authorization identifier>  | <schema name>  AUTHORIZATION <schema authorization identifier>`

<a id="op-e8820f0fc49492a861106ede"></a>
## with

`struct_field` · `sqlparser::ast::Statement::CreateSchema::with` · sqlparser 0.62.0

```rust
with: Option<Vec<SqlOption>>
```

Source: `src/ast/mod.rs:4347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Schema properties.

```sql
CREATE SCHEMA myschema WITH (key1='value1');
```

[Trino](https://trino.io/docs/current/sql/create-schema.html)
