# `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_create_database.CreateDatabaseBuilder.json).

<a id="op-f1445dccbfe087fdf4696353"></a>
## CreateDatabaseBuilder

`struct` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder` · sqlparser 0.62.0

```rust
struct CreateDatabaseBuilder
```

Source: `src/ast/helpers/stmt_create_database.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Builder for create database statement variant ([1]).

This structure helps building and accessing a create database with more ease, without needing to:
- Match the enum itself a lot of times; or
- Moving a lot of variables around the code.

# Example
```rust
use sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder;
use sqlparser::ast::{ColumnDef, Ident, ObjectName};
let builder = CreateDatabaseBuilder::new(ObjectName::from(vec![Ident::new("database_name")]))
   .if_not_exists(true);
// You can access internal elements with ease
assert!(builder.if_not_exists);
// Convert to a statement
assert_eq!(
   builder.build().to_string(),
   "CREATE DATABASE IF NOT EXISTS database_name"
)
```

[1]: Statement::CreateDatabase

<a id="op-168d392594b18ef84abb2039"></a>
## Error

`assoc_type` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::Error` · sqlparser 0.62.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [365, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ast/helpers/stmt_create_database.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd776c00d69e197e59af05d2"></a>
## build

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::build` · sqlparser 0.62.0

```rust
fn build(self) -> Statement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Build the `CREATE DATABASE` statement.

<a id="op-555722d51c0912b9369c0fc3"></a>
## catalog

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog` · sqlparser 0.62.0

```rust
catalog: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog name.

<a id="op-8a9ad266c6b9f3b109b7e8c9"></a>
## catalog

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog` · sqlparser 0.62.0

```rust
fn catalog(self, catalog: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:199`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the catalog for the database.

<a id="op-49e3a4eabe87d665025fde44"></a>
## catalog_sync

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog_sync` · sqlparser 0.62.0

```rust
fn catalog_sync(self, catalog_sync: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the catalog sync for the database.

<a id="op-7b81fa5604bfc110b265c67c"></a>
## catalog_sync

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog_sync` · sqlparser 0.62.0

```rust
catalog_sync: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:97`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog sync configuration.

<a id="op-5456bef8b04c9aab939ddeb7"></a>
## catalog_sync_namespace_flatten_delimiter

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog_sync_namespace_flatten_delimiter` · sqlparser 0.62.0

```rust
fn catalog_sync_namespace_flatten_delimiter(self, catalog_sync_namespace_flatten_delimiter: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the catalog sync namespace flatten delimiter for the database.

<a id="op-deda47cc4e9665442cbf0e61"></a>
## catalog_sync_namespace_flatten_delimiter

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog_sync_namespace_flatten_delimiter` · sqlparser 0.62.0

```rust
catalog_sync_namespace_flatten_delimiter: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional namespace flatten delimiter for catalog sync.

<a id="op-94beff947e88b9e47ea44614"></a>
## catalog_sync_namespace_mode

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog_sync_namespace_mode` · sqlparser 0.62.0

```rust
fn catalog_sync_namespace_mode(self, catalog_sync_namespace_mode: Option<CatalogSyncNamespaceMode>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the catalog sync namespace mode for the database.

<a id="op-f65946e0d6f57bf9f71267c6"></a>
## catalog_sync_namespace_mode

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::catalog_sync_namespace_mode` · sqlparser 0.62.0

```rust
catalog_sync_namespace_mode: Option<ast::CatalogSyncNamespaceMode>
```

Source: `src/ast/helpers/stmt_create_database.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog sync namespace mode.

<a id="op-0e032c6668404407508c200a"></a>
## clone

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::clone` · sqlparser 0.62.0

```rust
clone: Option<ast::ObjectName>
```

Source: `src/ast/helpers/stmt_create_database.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `CLONE` source object name.

<a id="op-45ecd443d1b2f778bc2195de"></a>
## clone

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateDatabaseBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/stmt_create_database.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27244ba7b16d4c1f622c7979"></a>
## clone_clause

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::clone_clause` · sqlparser 0.62.0

```rust
fn clone_clause(self, clone: Option<ObjectName>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the clone clause for the database.

<a id="op-9b2e6772562455710e1ab083"></a>
## comment

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::comment` · sqlparser 0.62.0

```rust
fn comment(self, comment: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the comment for the database.

<a id="op-c24d59aa49cfab9785af6bcc"></a>
## comment

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::comment` · sqlparser 0.62.0

```rust
comment: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional comment attached to the database.

<a id="op-30f2f48e7180ac9fba8d0b11"></a>
## data_retention_time_in_days

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::data_retention_time_in_days` · sqlparser 0.62.0

```rust
fn data_retention_time_in_days(self, data_retention_time_in_days: Option<u64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the data retention time in days.

<a id="op-8320b1dffe7750fe094d7e4c"></a>
## data_retention_time_in_days

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::data_retention_time_in_days` · sqlparser 0.62.0

```rust
data_retention_time_in_days: Option<u64>
```

Source: `src/ast/helpers/stmt_create_database.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional data retention time in days.

<a id="op-40dca841c741dec9bc6339a2"></a>
## db_name

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::db_name` · sqlparser 0.62.0

```rust
db_name: ast::ObjectName
```

Source: `src/ast/helpers/stmt_create_database.rs:59`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The database name to create.

<a id="op-238b575e77f9b2a68f51f4dc"></a>
## default_charset

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::default_charset` · sqlparser 0.62.0

```rust
default_charset: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:91`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default character set (MySQL).

<https://dev.mysql.com/doc/refman/8.4/en/create-database.html>

<a id="op-b45f26c86f149d664f1526d8"></a>
## default_charset

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::default_charset` · sqlparser 0.62.0

```rust
fn default_charset(self, default_charset: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the default character set for the database.

<a id="op-17eca85ca1b7a558e6fb5970"></a>
## default_collation

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::default_collation` · sqlparser 0.62.0

```rust
default_collation: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default collation (MySQL).

<https://dev.mysql.com/doc/refman/8.4/en/create-database.html>

<a id="op-3f59239a61f1308608507dc8"></a>
## default_collation

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::default_collation` · sqlparser 0.62.0

```rust
fn default_collation(self, default_collation: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the default collation for the database.

<a id="op-aad9f4ee26de339e1728be25"></a>
## default_ddl_collation

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::default_ddl_collation` · sqlparser 0.62.0

```rust
default_ddl_collation: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:83`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default DDL collation.

<a id="op-e3d578b5de91b78cebfd9688"></a>
## default_ddl_collation

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::default_ddl_collation` · sqlparser 0.62.0

```rust
fn default_ddl_collation(self, default_ddl_collation: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the default DDL collation.

<a id="op-ba04f374110a691e286c2cb4"></a>
## deserialize

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 49], "end": [55, 60], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/stmt_create_database.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c7957e9745b697c9786ca5e"></a>
## eq

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateDatabaseBuilder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 24], "end": [54, 33], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/stmt_create_database.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9008a035a98684b2f7d8871d"></a>
## external_volume

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::external_volume` · sqlparser 0.62.0

```rust
external_volume: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:77`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional external volume identifier.

<a id="op-a64b240171bf22149845aaad"></a>
## external_volume

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::external_volume` · sqlparser 0.62.0

```rust
fn external_volume(self, external_volume: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the external volume for the database.

<a id="op-d4fe4a9e6e0d37264ac5fd3e"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/stmt_create_database.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de8f5ea390caf58728e43f5f"></a>
## hash

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 39], "end": [54, 43], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/stmt_create_database.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-243180999dc3eb07e289454c"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/helpers/stmt_create_database.rs:61`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-e095b5d779fed69296790a5e"></a>
## if_not_exists

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::if_not_exists` · sqlparser 0.62.0

```rust
fn if_not_exists(self, if_not_exists: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set whether to use `IF NOT EXISTS`.

<a id="op-6c04c1b9c20b9e25eda759d0"></a>
## location

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::location` · sqlparser 0.62.0

```rust
fn location(self, location: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the location for the database.

<a id="op-f38792c009779a526c5aeef9"></a>
## location

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::location` · sqlparser 0.62.0

```rust
location: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:63`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage location for the database.

<a id="op-612f6e9e2553ea133210ed25"></a>
## managed_location

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::managed_location` · sqlparser 0.62.0

```rust
managed_location: Option<String>
```

Source: `src/ast/helpers/stmt_create_database.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional managed storage location.

<a id="op-ed4afae1b5f834f9c48c7487"></a>
## managed_location

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::managed_location` · sqlparser 0.62.0

```rust
fn managed_location(self, managed_location: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the managed location for the database.

<a id="op-0da0089d1ff29cb8a83c9e64"></a>
## max_data_extension_time_in_days

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::max_data_extension_time_in_days` · sqlparser 0.62.0

```rust
fn max_data_extension_time_in_days(self, max_data_extension_time_in_days: Option<u64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the maximum data extension time in days.

<a id="op-a6f9b3648fef2d3f401d2eca"></a>
## max_data_extension_time_in_days

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::max_data_extension_time_in_days` · sqlparser 0.62.0

```rust
max_data_extension_time_in_days: Option<u64>
```

Source: `src/ast/helpers/stmt_create_database.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional max data extension time in days.

<a id="op-a5f22c79fed4ecc1f1bd1eab"></a>
## new

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::new` · sqlparser 0.62.0

```rust
fn new(name: ObjectName) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new `CreateDatabaseBuilder` with the given database name.

# Arguments

* `name` - The name of the database to be created.

<a id="op-746dd17a2ee3399a7dd23411"></a>
## or_replace

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/helpers/stmt_create_database.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `OR REPLACE` was specified.

<a id="op-77bb0ec6c680811e944cf0c0"></a>
## or_replace

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::or_replace` · sqlparser 0.62.0

```rust
fn or_replace(self, or_replace: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set whether this is an `OR REPLACE` operation.

<a id="op-1d5af8acf728579956f7413c"></a>
## replace_invalid_characters

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::replace_invalid_characters` · sqlparser 0.62.0

```rust
fn replace_invalid_characters(self, replace_invalid_characters: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set whether to replace invalid characters.

<a id="op-23289fca3f4c677dd83e7274"></a>
## replace_invalid_characters

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::replace_invalid_characters` · sqlparser 0.62.0

```rust
replace_invalid_characters: Option<bool>
```

Source: `src/ast/helpers/stmt_create_database.rs:81`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to replace invalid characters.

<a id="op-82ad93ddc774228e1cbf1ffd"></a>
## serialize

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 38], "end": [55, 47], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/stmt_create_database.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a074d525455a4e4d6335db3a"></a>
## storage_serialization_policy

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::storage_serialization_policy` · sqlparser 0.62.0

```rust
fn storage_serialization_policy(self, storage_serialization_policy: Option<StorageSerializationPolicy>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the storage serialization policy.

<a id="op-d1cf9e256312762718fb2ddd"></a>
## storage_serialization_policy

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::storage_serialization_policy` · sqlparser 0.62.0

```rust
storage_serialization_policy: Option<ast::StorageSerializationPolicy>
```

Source: `src/ast/helpers/stmt_create_database.rs:85`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage serialization policy.

<a id="op-2f53a12cb2651678e38596b8"></a>
## transient

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::transient` · sqlparser 0.62.0

```rust
transient: bool
```

Source: `src/ast/helpers/stmt_create_database.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the database is `TRANSIENT`.

<a id="op-dc06b0d6e590848c300adc34"></a>
## transient

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::transient` · sqlparser 0.62.0

```rust
fn transient(self, transient: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set whether this is a transient database.

<a id="op-0a8fcb0eaf5f78bfae55da06"></a>
## try_from

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::try_from` · sqlparser 0.62.0

```rust
fn try_from(stmt: Statement) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 1], "end": [365, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ast/helpers/stmt_create_database.rs:311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c00e4251964f71419f85c63d"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 40], "end": [56, 45], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/stmt_create_database.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed4981070d52e3a7fcc2fe7f"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 47], "end": [56, 55], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/stmt_create_database.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f37b268bca23115774f5271"></a>
## with_contacts

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::with_contacts` · sqlparser 0.62.0

```rust
fn with_contacts(self, with_contacts: Option<Vec<ContactEntry>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the contacts for the database.

<a id="op-2eff644acf1d396c1b73ac99"></a>
## with_contacts

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::with_contacts` · sqlparser 0.62.0

```rust
with_contacts: Option<Vec<ast::ContactEntry>>
```

Source: `src/ast/helpers/stmt_create_database.rs:105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional contact entries associated with the database.

<a id="op-399682dd8ca2d64ee5db9a25"></a>
## with_tags

`struct_field` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::with_tags` · sqlparser 0.62.0

```rust
with_tags: Option<Vec<ast::Tag>>
```

Source: `src/ast/helpers/stmt_create_database.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional tags attached to the database.

<a id="op-bfe1473b3ac378eda53a3538"></a>
## with_tags

`function` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder::with_tags` · sqlparser 0.62.0

```rust
fn with_tags(self, with_tags: Option<Vec<Tag>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder", "path": "CreateDatabaseBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [306, 2], "filename": "src/ast/helpers/stmt_create_database.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_database.rs:268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the tags for the database.
