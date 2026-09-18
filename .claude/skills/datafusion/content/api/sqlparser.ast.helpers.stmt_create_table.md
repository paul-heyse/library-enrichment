# `sqlparser::ast::helpers::stmt_create_table`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.ast.helpers.stmt_create_table.json`](../model/sqlparser.ast.helpers.stmt_create_table.json)

## CreateTableBuilder

`struct` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder`

```rust
struct CreateTableBuilder
```

**Fields**: `or_replace`, `temporary`, `external`, `global`, `if_not_exists`, `transient`, `volatile`, `iceberg`, `snapshot`, `dynamic`, `name`, `columns`, `constraints`, `hive_distribution`, `hive_formats`, `file_format`, `location`, `query`, `without_rowid`, `like`, `clone`, `version`, `comment`, `on_commit`, `on_cluster`, `primary_key`, `order_by`, `partition_by`, `cluster_by`, `clustered_by`, `inherits`, `partition_of`, `for_values`, `strict`, `copy_grants`, `enable_schema_evolution`, `change_tracking`, `data_retention_time_in_days`, `max_data_extension_time_in_days`, `default_ddl_collation`, `with_aggregation_policy`, `with_row_access_policy`, `with_storage_lifecycle_policy`, `with_tags`, `base_location`, `external_volume`, `catalog`, `catalog_sync`, `storage_serialization_policy`, `table_options`, `target_lag`, `warehouse`, `refresh_mode`, `initialize`, `require_user`, `diststyle`, `distkey`, `sortkey`, `backup`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::ast::visitor::Visit`, `sqlparser::ast::visitor::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (60)

```rust
fn backup(self, backup: Option<bool>) -> Self
fn base_location(self, base_location: Option<String>) -> Self
fn build(self) -> CreateTable
fn catalog(self, catalog: Option<String>) -> Self
fn catalog_sync(self, catalog_sync: Option<String>) -> Self
fn change_tracking(self, change_tracking: Option<bool>) -> Self
fn clone_clause(self, clone: Option<ObjectName>) -> Self
fn cluster_by(self, cluster_by: Option<WrappedCollection<Vec<Expr>>>) -> Self
fn clustered_by(self, clustered_by: Option<ClusteredBy>) -> Self
fn columns(self, columns: Vec<ColumnDef>) -> Self
fn comment_after_column_def(self, comment: Option<CommentDef>) -> Self
fn constraints(self, constraints: Vec<TableConstraint>) -> Self
fn copy_grants(self, copy_grants: bool) -> Self
fn data_retention_time_in_days(self, data_retention_time_in_days: Option<u64>) -> Self
fn default_ddl_collation(self, default_ddl_collation: Option<String>) -> Self
fn distkey(self, distkey: Option<Expr>) -> Self
fn diststyle(self, diststyle: Option<DistStyle>) -> Self
fn dynamic(self, dynamic: bool) -> Self
fn enable_schema_evolution(self, enable_schema_evolution: Option<bool>) -> Self
fn external(self, external: bool) -> Self
fn external_volume(self, external_volume: Option<String>) -> Self
fn file_format(self, file_format: Option<FileFormat>) -> Self
fn for_values(self, for_values: Option<ForValues>) -> Self
fn global(self, global: Option<bool>) -> Self
fn hive_distribution(self, hive_distribution: HiveDistributionStyle) -> Self
fn hive_formats(self, hive_formats: Option<HiveFormat>) -> Self
fn iceberg(self, iceberg: bool) -> Self
fn if_not_exists(self, if_not_exists: bool) -> Self
fn inherits(self, inherits: Option<Vec<ObjectName>>) -> Self
fn initialize(self, initialize: Option<InitializeKind>) -> Self
fn like(self, like: Option<CreateTableLikeKind>) -> Self
fn location(self, location: Option<String>) -> Self
fn max_data_extension_time_in_days(self, max_data_extension_time_in_days: Option<u64>) -> Self
fn new(name: ObjectName) -> Self
fn on_cluster(self, on_cluster: Option<Ident>) -> Self
fn on_commit(self, on_commit: Option<OnCommit>) -> Self
fn or_replace(self, or_replace: bool) -> Self
fn order_by(self, order_by: Option<OneOrManyWithParens<Expr>>) -> Self
fn partition_by(self, partition_by: Option<Box<Expr>>) -> Self
fn partition_of(self, partition_of: Option<ObjectName>) -> Self
fn primary_key(self, primary_key: Option<Box<Expr>>) -> Self
fn query(self, query: Option<Box<Query>>) -> Self
fn refresh_mode(self, refresh_mode: Option<RefreshModeKind>) -> Self
fn require_user(self, require_user: bool) -> Self
fn snapshot(self, snapshot: bool) -> Self
fn sortkey(self, sortkey: Option<Vec<Expr>>) -> Self
fn storage_serialization_policy(self, storage_serialization_policy: Option<StorageSerializationPolicy>) -> Self
fn strict(self, strict: bool) -> Self
fn table_options(self, table_options: CreateTableOptions) -> Self
fn target_lag(self, target_lag: Option<String>) -> Self
fn temporary(self, temporary: bool) -> Self
fn transient(self, transient: bool) -> Self
fn version(self, version: Option<TableVersion>) -> Self
fn volatile(self, volatile: bool) -> Self
fn warehouse(self, warehouse: Option<Ident>) -> Self
fn with_aggregation_policy(self, with_aggregation_policy: Option<ObjectName>) -> Self
fn with_row_access_policy(self, with_row_access_policy: Option<RowAccessPolicy>) -> Self
fn with_storage_lifecycle_policy(self, with_storage_lifecycle_policy: Option<StorageLifecyclePolicy>) -> Self
fn with_tags(self, with_tags: Option<Vec<Tag>>) -> Self
fn without_rowid(self, without_rowid: bool) -> Self
```

**via `core::convert::From`**

```rust
fn from(table: CreateTable) -> Self
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

[Full member, field, variant and typed contracts](../operations/sqlparser.ast.helpers.stmt_create_table.CreateTableBuilder.md).


Builder for create table statement variant ([1]).

This structure helps building and accessing a create table with more ease, without needing to:
- Match the enum itself a lot of times; or
- Moving a lot of variables around the code.

# Example
```rust
use sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder;
use sqlparser::ast::{ColumnDef, DataType, Ident, ObjectName};
let builder = CreateTableBuilder::new(ObjectName::from(vec![Ident::new("table_name")]))
   .if_not_exists(true)
   .columns(vec![ColumnDef {
       name: Ident::new("c1"),
       data_type: DataType::Int(None),
       options: vec![],
}]);
// You can access internal elements with ease
assert!(builder.if_not_exists);
// Convert to a statement
assert_eq!(
   builder.build().to_string(),
   "CREATE TABLE IF NOT EXISTS table_name (c1 INT)"
)
```

[1]: crate::ast::Statement::CreateTable

---
