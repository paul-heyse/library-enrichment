# `sqlparser::ast::helpers::stmt_create_database`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.ast.helpers.stmt_create_database.json`](../model/sqlparser.ast.helpers.stmt_create_database.json)

## CreateDatabaseBuilder

`struct` · `sqlparser::ast::helpers::stmt_create_database::CreateDatabaseBuilder`

```rust
struct CreateDatabaseBuilder
```

**Fields**: `db_name`, `if_not_exists`, `location`, `managed_location`, `or_replace`, `transient`, `clone`, `data_retention_time_in_days`, `max_data_extension_time_in_days`, `external_volume`, `catalog`, `replace_invalid_characters`, `default_ddl_collation`, `storage_serialization_policy`, `comment`, `default_charset`, `default_collation`, `catalog_sync`, `catalog_sync_namespace_mode`, `catalog_sync_namespace_flatten_delimiter`, `with_tags`, `with_contacts`

**Implements**: `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (23)

```rust
fn build(self) -> Statement
fn catalog(self, catalog: Option<String>) -> Self
fn catalog_sync(self, catalog_sync: Option<String>) -> Self
fn catalog_sync_namespace_flatten_delimiter(self, catalog_sync_namespace_flatten_delimiter: Option<String>) -> Self
fn catalog_sync_namespace_mode(self, catalog_sync_namespace_mode: Option<CatalogSyncNamespaceMode>) -> Self
fn clone_clause(self, clone: Option<ObjectName>) -> Self
fn comment(self, comment: Option<String>) -> Self
fn data_retention_time_in_days(self, data_retention_time_in_days: Option<u64>) -> Self
fn default_charset(self, default_charset: Option<String>) -> Self
fn default_collation(self, default_collation: Option<String>) -> Self
fn default_ddl_collation(self, default_ddl_collation: Option<String>) -> Self
fn external_volume(self, external_volume: Option<String>) -> Self
fn if_not_exists(self, if_not_exists: bool) -> Self
fn location(self, location: Option<String>) -> Self
fn managed_location(self, managed_location: Option<String>) -> Self
fn max_data_extension_time_in_days(self, max_data_extension_time_in_days: Option<u64>) -> Self
fn new(name: ObjectName) -> Self
fn or_replace(self, or_replace: bool) -> Self
fn replace_invalid_characters(self, replace_invalid_characters: Option<bool>) -> Self
fn storage_serialization_policy(self, storage_serialization_policy: Option<StorageSerializationPolicy>) -> Self
fn transient(self, transient: bool) -> Self
fn with_contacts(self, with_contacts: Option<Vec<ContactEntry>>) -> Self
fn with_tags(self, with_tags: Option<Vec<Tag>>) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(stmt: Statement) -> Result<Self, Self::Error>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::ast::visitor::Visit`**

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

**via `sqlparser::ast::visitor::VisitMut`**

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

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

---
