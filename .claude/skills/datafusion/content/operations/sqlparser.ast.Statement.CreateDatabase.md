# `sqlparser::ast::Statement::CreateDatabase`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.CreateDatabase.json).

<a id="op-567cfc74dccb7c995ccd06d7"></a>
## catalog

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::catalog` · sqlparser 0.62.0

```rust
catalog: Option<String>
```

Source: `src/ast/mod.rs:4400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog name.

<a id="op-cc38d23b1927a171930c03d6"></a>
## catalog_sync

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::catalog_sync` · sqlparser 0.62.0

```rust
catalog_sync: Option<String>
```

Source: `src/ast/mod.rs:4414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog sync identifier.

<a id="op-668ace4d5619fd5f1294877f"></a>
## catalog_sync_namespace_flatten_delimiter

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::catalog_sync_namespace_flatten_delimiter` · sqlparser 0.62.0

```rust
catalog_sync_namespace_flatten_delimiter: Option<String>
```

Source: `src/ast/mod.rs:4418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional flatten delimiter for namespace sync.

<a id="op-e7055742f814e77d27c623d6"></a>
## catalog_sync_namespace_mode

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::catalog_sync_namespace_mode` · sqlparser 0.62.0

```rust
catalog_sync_namespace_mode: Option<CatalogSyncNamespaceMode>
```

Source: `src/ast/mod.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Catalog sync namespace mode.

<a id="op-15829bde99d11dba373e7ba7"></a>
## clone

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::clone` · sqlparser 0.62.0

```rust
clone: Option<ObjectName>
```

Source: `src/ast/mod.rs:4392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional clone source.

<a id="op-3e22698c449d75640f26b05e"></a>
## comment

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::comment` · sqlparser 0.62.0

```rust
comment: Option<String>
```

Source: `src/ast/mod.rs:4408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional comment.

<a id="op-28714959ab2fe541e227c1f0"></a>
## data_retention_time_in_days

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::data_retention_time_in_days` · sqlparser 0.62.0

```rust
data_retention_time_in_days: Option<u64>
```

Source: `src/ast/mod.rs:4394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional data retention time in days.

<a id="op-97a63744dbf0375efdcf2576"></a>
## db_name

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::db_name` · sqlparser 0.62.0

```rust
db_name: ObjectName
```

Source: `src/ast/mod.rs:4380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Database name.

<a id="op-19cacf6ff9fdf64aa0e9619a"></a>
## default_charset

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::default_charset` · sqlparser 0.62.0

```rust
default_charset: Option<String>
```

Source: `src/ast/mod.rs:4410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default character set (MySQL).

<a id="op-7018803f4fae19246bddf825"></a>
## default_collation

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::default_collation` · sqlparser 0.62.0

```rust
default_collation: Option<String>
```

Source: `src/ast/mod.rs:4412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default collation (MySQL).

<a id="op-50a14b25fe06dc012fc7f9fa"></a>
## default_ddl_collation

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::default_ddl_collation` · sqlparser 0.62.0

```rust
default_ddl_collation: Option<String>
```

Source: `src/ast/mod.rs:4404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default DDL collation string.

<a id="op-d5779fcf3c3812148b5d5c38"></a>
## external_volume

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::external_volume` · sqlparser 0.62.0

```rust
external_volume: Option<String>
```

Source: `src/ast/mod.rs:4398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional external volume identifier.

<a id="op-72d161c754473907483e89c2"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:4382`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF NOT EXISTS` flag.

<a id="op-226b73be64bcf59e9ba23ace"></a>
## location

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::location` · sqlparser 0.62.0

```rust
location: Option<String>
```

Source: `src/ast/mod.rs:4384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional location URI.

<a id="op-84e81614385356a3aea45271"></a>
## managed_location

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::managed_location` · sqlparser 0.62.0

```rust
managed_location: Option<String>
```

Source: `src/ast/mod.rs:4386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional managed location.

<a id="op-dab3fb2a02860d1010e8c752"></a>
## max_data_extension_time_in_days

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::max_data_extension_time_in_days` · sqlparser 0.62.0

```rust
max_data_extension_time_in_days: Option<u64>
```

Source: `src/ast/mod.rs:4396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional maximum data extension time in days.

<a id="op-28f31f1907c388d5f5b3c534"></a>
## or_replace

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/mod.rs:4388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OR REPLACE` flag.

<a id="op-db81255df2044a76f8b6077e"></a>
## replace_invalid_characters

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::replace_invalid_characters` · sqlparser 0.62.0

```rust
replace_invalid_characters: Option<bool>
```

Source: `src/ast/mod.rs:4402`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to replace invalid characters.

<a id="op-69179ed4e2d18af68fafb1a8"></a>
## storage_serialization_policy

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::storage_serialization_policy` · sqlparser 0.62.0

```rust
storage_serialization_policy: Option<StorageSerializationPolicy>
```

Source: `src/ast/mod.rs:4406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Storage serialization policy.

<a id="op-cf9e49552716d4c2c53140e2"></a>
## transient

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::transient` · sqlparser 0.62.0

```rust
transient: bool
```

Source: `src/ast/mod.rs:4390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TRANSIENT` flag.

<a id="op-b9c315a9a54ba91c1f91a4d4"></a>
## with_contacts

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::with_contacts` · sqlparser 0.62.0

```rust
with_contacts: Option<Vec<ContactEntry>>
```

Source: `src/ast/mod.rs:4422`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional contact entries for the database.

<a id="op-8b25ec435478390a7973bed9"></a>
## with_tags

`struct_field` · `sqlparser::ast::Statement::CreateDatabase::with_tags` · sqlparser 0.62.0

```rust
with_tags: Option<Vec<Tag>>
```

Source: `src/ast/mod.rs:4420`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional tags for the database.
