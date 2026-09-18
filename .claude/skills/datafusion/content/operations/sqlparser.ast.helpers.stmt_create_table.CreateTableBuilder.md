# `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_create_table.CreateTableBuilder.json).

<a id="op-5f2cd391b23b3891d2d5f19b"></a>
## CreateTableBuilder

`struct` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder` · sqlparser 0.62.0

```rust
struct CreateTableBuilder
```

Source: `src/ast/helpers/stmt_create_table.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

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

<a id="op-751a14d3c0187cfcbeed3d82"></a>
## Error

`assoc_type` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::Error` · sqlparser 0.62.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [625, 1], "end": [638, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ast/helpers/stmt_create_table.rs:626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8741be6f008da337cebe01c5"></a>
## backup

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::backup` · sqlparser 0.62.0

```rust
fn backup(self, backup: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the Redshift `BACKUP` option.

<a id="op-a5b27891b0acbef087a8f700"></a>
## backup

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::backup` · sqlparser 0.62.0

```rust
backup: Option<bool>
```

Source: `src/ast/helpers/stmt_create_table.rs:185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `BACKUP` option.

<a id="op-9a44264a2db7a13a0803824f"></a>
## base_location

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::base_location` · sqlparser 0.62.0

```rust
base_location: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional base location for staged data.

<a id="op-e38ac0c629d73c2ab7d176d2"></a>
## base_location

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::base_location` · sqlparser 0.62.0

```rust
fn base_location(self, base_location: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set a base storage location for staged data.

<a id="op-65c9da8cd9d497a875450f8d"></a>
## build

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::build` · sqlparser 0.62.0

```rust
fn build(self) -> CreateTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Consume the builder and produce a `CreateTable`.

<a id="op-3eefdfdc949c47c61967b410"></a>
## catalog

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::catalog` · sqlparser 0.62.0

```rust
fn catalog(self, catalog: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the catalog name for the table.

<a id="op-df41dd89fd5dd148920649c1"></a>
## catalog

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::catalog` · sqlparser 0.62.0

```rust
catalog: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog name.

<a id="op-6ef2e4bd3dc97d4fa62ad15c"></a>
## catalog_sync

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::catalog_sync` · sqlparser 0.62.0

```rust
fn catalog_sync(self, catalog_sync: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set catalog synchronization option.

<a id="op-a37bddac864a26bad6ce956e"></a>
## catalog_sync

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::catalog_sync` · sqlparser 0.62.0

```rust
catalog_sync: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional catalog synchronization option.

<a id="op-2eb1e47cf71ca8fbab01493d"></a>
## change_tracking

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::change_tracking` · sqlparser 0.62.0

```rust
fn change_tracking(self, change_tracking: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enable or disable change tracking.

<a id="op-dcafe423354941c3206c7dc7"></a>
## change_tracking

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::change_tracking` · sqlparser 0.62.0

```rust
change_tracking: Option<bool>
```

Source: `src/ast/helpers/stmt_create_table.rs:141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional change tracking flag.

<a id="op-a6ca9b8f50f96e9352b50e69"></a>
## clone

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTableBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 17], "end": [64, 22], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/stmt_create_table.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da50819e6f5d159950ecb7ed"></a>
## clone

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::clone` · sqlparser 0.62.0

```rust
clone: Option<ast::ObjectName>
```

Source: `src/ast/helpers/stmt_create_table.rs:109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `CLONE` source object name.

<a id="op-59b0fbbc4416a545c7a357f9"></a>
## clone_clause

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::clone_clause` · sqlparser 0.62.0

```rust
fn clone_clause(self, clone: Option<ObjectName>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:350`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `CLONE` source object name.

<a id="op-4ec2f12cd0b30867a9a97fa4"></a>
## cluster_by

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::cluster_by` · sqlparser 0.62.0

```rust
fn cluster_by(self, cluster_by: Option<WrappedCollection<Vec<Expr>>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `CLUSTER BY` expression(s).

<a id="op-c19e7acee70129ab7329fc02"></a>
## cluster_by

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::cluster_by` · sqlparser 0.62.0

```rust
cluster_by: Option<ast::WrappedCollection<Vec<ast::Expr>>>
```

Source: `src/ast/helpers/stmt_create_table.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `CLUSTER BY` expressions.

<a id="op-018d5d28caf3dd7f07c89581"></a>
## clustered_by

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::clustered_by` · sqlparser 0.62.0

```rust
fn clustered_by(self, clustered_by: Option<ClusteredBy>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `CLUSTERED BY` clause.

<a id="op-76a1918a0726167699c51509"></a>
## clustered_by

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::clustered_by` · sqlparser 0.62.0

```rust
clustered_by: Option<ast::ClusteredBy>
```

Source: `src/ast/helpers/stmt_create_table.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `CLUSTERED BY` clause.

<a id="op-5429e3d9c3030cb49e3bb742"></a>
## columns

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::columns` · sqlparser 0.62.0

```rust
fn columns(self, columns: Vec<ColumnDef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the table column definitions.

<a id="op-56096d898e663e6300fce497"></a>
## columns

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::ColumnDef>
```

Source: `src/ast/helpers/stmt_create_table.rs:91`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column definitions for the table.

<a id="op-c1a3a55bf8972639f4e19e75"></a>
## comment

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::comment` · sqlparser 0.62.0

```rust
comment: Option<ast::CommentDef>
```

Source: `src/ast/helpers/stmt_create_table.rs:113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table comment.

<a id="op-a4518c24e6e50725b8e2830f"></a>
## comment_after_column_def

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::comment_after_column_def` · sqlparser 0.62.0

```rust
fn comment_after_column_def(self, comment: Option<CommentDef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set a comment for the table or following column definitions.

<a id="op-208d628d23b3d2bc5f27424a"></a>
## constraints

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::constraints` · sqlparser 0.62.0

```rust
constraints: Vec<ast::TableConstraint>
```

Source: `src/ast/helpers/stmt_create_table.rs:93`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table-level constraints.

<a id="op-9934c009874a2d1285b90045"></a>
## constraints

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::constraints` · sqlparser 0.62.0

```rust
fn constraints(self, constraints: Vec<TableConstraint>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set table-level constraints.

<a id="op-934e5357ea84338057da5cae"></a>
## copy_grants

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::copy_grants` · sqlparser 0.62.0

```rust
fn copy_grants(self, copy_grants: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:423`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enable copying grants from source object.

<a id="op-d49f08b85d91d22f2cfff915"></a>
## copy_grants

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::copy_grants` · sqlparser 0.62.0

```rust
copy_grants: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:137`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to copy grants from the source.

<a id="op-378a7b6cbdc6f62f72f3327c"></a>
## data_retention_time_in_days

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::data_retention_time_in_days` · sqlparser 0.62.0

```rust
fn data_retention_time_in_days(self, data_retention_time_in_days: Option<u64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set data retention time (in days).

<a id="op-b6896d7084d6d6b793c8e34b"></a>
## data_retention_time_in_days

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::data_retention_time_in_days` · sqlparser 0.62.0

```rust
data_retention_time_in_days: Option<u64>
```

Source: `src/ast/helpers/stmt_create_table.rs:143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional data retention time in days.

<a id="op-0d4e1d2ae41c637993c6cd8b"></a>
## default_ddl_collation

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::default_ddl_collation` · sqlparser 0.62.0

```rust
default_ddl_collation: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional default DDL collation.

<a id="op-1798a4388df1fed1bc9938df"></a>
## default_ddl_collation

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::default_ddl_collation` · sqlparser 0.62.0

```rust
fn default_ddl_collation(self, default_ddl_collation: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:451`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set default DDL collation.

<a id="op-1dbeb380e38ce5e0c869cfc7"></a>
## deserialize

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 49], "end": [65, 60], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/stmt_create_table.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8eca9000bc824176d3ef363d"></a>
## distkey

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::distkey` · sqlparser 0.62.0

```rust
distkey: Option<ast::Expr>
```

Source: `src/ast/helpers/stmt_create_table.rs:181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `DISTKEY` option.

<a id="op-e520dcc65c36d0d991e553bc"></a>
## distkey

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::distkey` · sqlparser 0.62.0

```rust
fn distkey(self, distkey: Option<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set Redshift `DISTKEY` option.

<a id="op-3a1e8f30744aa1bc22b05aab"></a>
## diststyle

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::diststyle` · sqlparser 0.62.0

```rust
fn diststyle(self, diststyle: Option<DistStyle>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set Redshift `DISTSTYLE` option.

<a id="op-68c5fbdcd9239ce7b489ffab"></a>
## diststyle

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::diststyle` · sqlparser 0.62.0

```rust
diststyle: Option<ast::DistStyle>
```

Source: `src/ast/helpers/stmt_create_table.rs:179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `DISTSTYLE` option.

<a id="op-2146a1c577f110a996aa7e95"></a>
## dynamic

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::dynamic` · sqlparser 0.62.0

```rust
dynamic: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `DYNAMIC` table option is set.

<a id="op-51e04b8a5b3afabfffff7f9d"></a>
## dynamic

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::dynamic` · sqlparser 0.62.0

```rust
fn dynamic(self, dynamic: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `DYNAMIC` table option.

<a id="op-6133e049551f3a9b1a530b14"></a>
## enable_schema_evolution

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::enable_schema_evolution` · sqlparser 0.62.0

```rust
enable_schema_evolution: Option<bool>
```

Source: `src/ast/helpers/stmt_create_table.rs:139`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional flag for schema evolution support.

<a id="op-cd301d88dfc20ec4807deb7f"></a>
## enable_schema_evolution

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::enable_schema_evolution` · sqlparser 0.62.0

```rust
fn enable_schema_evolution(self, enable_schema_evolution: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:428`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enable or disable schema evolution features.

<a id="op-03fdfbb74679e7d82d8d765c"></a>
## eq

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTableBuilder) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 24], "end": [64, 33], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/stmt_create_table.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-219f78d94b52314cbba6b4aa"></a>
## external

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::external` · sqlparser 0.62.0

```rust
external: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the table is `EXTERNAL`.

<a id="op-bff64337ecba9c1b9cd8acd3"></a>
## external

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::external` · sqlparser 0.62.0

```rust
fn external(self, external: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:264`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Mark the table as `EXTERNAL`.

<a id="op-5d28d7e1e781dd6711dbee3f"></a>
## external_volume

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::external_volume` · sqlparser 0.62.0

```rust
fn external_volume(self, external_volume: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set an external volume identifier.

<a id="op-95d5aaa51b5a8a798cdb4d37"></a>
## external_volume

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::external_volume` · sqlparser 0.62.0

```rust
external_volume: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional external volume identifier.

<a id="op-606c3300d71fdfaf74e05522"></a>
## file_format

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::file_format` · sqlparser 0.62.0

```rust
file_format: Option<ast::FileFormat>
```

Source: `src/ast/helpers/stmt_create_table.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional file format for storage.

<a id="op-607f5525996ec00222fdc809"></a>
## file_format

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::file_format` · sqlparser 0.62.0

```rust
fn file_format(self, file_format: Option<FileFormat>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set file format for the table (e.g., PARQUET).

<a id="op-d47364838ed1fb4e63077f87"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/stmt_create_table.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-670150522b4213fee78f0d86"></a>
## for_values

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::for_values` · sqlparser 0.62.0

```rust
for_values: Option<ast::ForValues>
```

Source: `src/ast/helpers/stmt_create_table.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Range of values associated with the partition (`FOR VALUES`)

<a id="op-ae636ec33d1cfb68132e4434"></a>
## for_values

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::for_values` · sqlparser 0.62.0

```rust
fn for_values(self, for_values: Option<ForValues>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sets the range of values associated with the partition.

<a id="op-ccf85710b06250aeef63e851"></a>
## from

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::from` · sqlparser 0.62.0

```rust
fn from(table: CreateTable) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [640, 1], "end": [704, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/helpers/stmt_create_table.rs:641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e0ebb295ac0b15aae7cb091"></a>
## global

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::global` · sqlparser 0.62.0

```rust
global: Option<bool>
```

Source: `src/ast/helpers/stmt_create_table.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `GLOBAL` flag for dialects that support it.

<a id="op-87cfa264178d35ec806442c6"></a>
## global

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::global` · sqlparser 0.62.0

```rust
fn global(self, global: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:269`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set optional `GLOBAL` flag (dialect-specific).

<a id="op-02241117159f95c2ba533312"></a>
## hash

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 39], "end": [64, 43], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/stmt_create_table.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5454f055baa3073986e5ddb8"></a>
## hive_distribution

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::hive_distribution` · sqlparser 0.62.0

```rust
hive_distribution: ast::HiveDistributionStyle
```

Source: `src/ast/helpers/stmt_create_table.rs:95`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive distribution style.

<a id="op-dc6e128628eba43841000789"></a>
## hive_distribution

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::hive_distribution` · sqlparser 0.62.0

```rust
fn hive_distribution(self, hive_distribution: HiveDistributionStyle) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set Hive distribution style.

<a id="op-02af54f5be6a85b0638be0dd"></a>
## hive_formats

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::hive_formats` · sqlparser 0.62.0

```rust
fn hive_formats(self, hive_formats: Option<HiveFormat>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:319`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set Hive-specific formats.

<a id="op-81b69cc8a36b8d365dabf871"></a>
## hive_formats

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::hive_formats` · sqlparser 0.62.0

```rust
hive_formats: Option<ast::HiveFormat>
```

Source: `src/ast/helpers/stmt_create_table.rs:97`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional Hive format settings.

<a id="op-c252fac682e71bc682765d20"></a>
## iceberg

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::iceberg` · sqlparser 0.62.0

```rust
fn iceberg(self, iceberg: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enable Iceberg table semantics.

<a id="op-d1356d494d100bc005cbafa3"></a>
## iceberg

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::iceberg` · sqlparser 0.62.0

```rust
iceberg: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:83`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Iceberg-specific table flag.

<a id="op-256337bd7fd2aa22f036c4a3"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:77`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-64b9da4ead34108c7c783080"></a>
## if_not_exists

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::if_not_exists` · sqlparser 0.62.0

```rust
fn if_not_exists(self, if_not_exists: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `IF NOT EXISTS`.

<a id="op-66b7ef36b04644647674280f"></a>
## inherits

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::inherits` · sqlparser 0.62.0

```rust
fn inherits(self, inherits: Option<Vec<ObjectName>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:400`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set parent tables via `INHERITS`.

<a id="op-f53ef7edcd9cc47a2d6f8c2a"></a>
## inherits

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::inherits` · sqlparser 0.62.0

```rust
inherits: Option<Vec<ast::ObjectName>>
```

Source: `src/ast/helpers/stmt_create_table.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional parent tables (`INHERITS`).

<a id="op-0111d0b2ca80aedbbbc8a623"></a>
## initialize

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::initialize` · sqlparser 0.62.0

```rust
initialize: Option<ast::InitializeKind>
```

Source: `src/ast/helpers/stmt_create_table.rs:175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional initialization kind for the table.

<a id="op-6c09a0138468a420fa503c93"></a>
## initialize

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::initialize` · sqlparser 0.62.0

```rust
fn initialize(self, initialize: Option<InitializeKind>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set initialization mode for the table.

<a id="op-23a2c71056626f072fa334f8"></a>
## like

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::like` · sqlparser 0.62.0

```rust
fn like(self, like: Option<CreateTableLikeKind>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `LIKE` clause for the table.

<a id="op-9af47d83be3c2e1c0030814e"></a>
## like

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::like` · sqlparser 0.62.0

```rust
like: Option<ast::CreateTableLikeKind>
```

Source: `src/ast/helpers/stmt_create_table.rs:107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `LIKE` clause kind.

<a id="op-2cddd5f922f9da1840349072"></a>
## location

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::location` · sqlparser 0.62.0

```rust
location: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage location.

<a id="op-54147b06dc739cbd00a67a83"></a>
## location

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::location` · sqlparser 0.62.0

```rust
fn location(self, location: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:329`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set storage `location` for the table.

<a id="op-2c8a659e48b59354e120273a"></a>
## max_data_extension_time_in_days

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::max_data_extension_time_in_days` · sqlparser 0.62.0

```rust
fn max_data_extension_time_in_days(self, max_data_extension_time_in_days: Option<u64>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set maximum data extension time (in days).

<a id="op-f0a15d64051f96696e0a66d2"></a>
## max_data_extension_time_in_days

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::max_data_extension_time_in_days` · sqlparser 0.62.0

```rust
max_data_extension_time_in_days: Option<u64>
```

Source: `src/ast/helpers/stmt_create_table.rs:145`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional max data extension time in days.

<a id="op-d1d98d373f9e66fe032dffb0"></a>
## name

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/helpers/stmt_create_table.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The table name.

<a id="op-9fdf749f4ff6631b917a66ef"></a>
## new

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::new` · sqlparser 0.62.0

```rust
fn new(name: ObjectName) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create a new `CreateTableBuilder` for the given table name.

<a id="op-0f867d6a84e99fb555560dda"></a>
## on_cluster

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::on_cluster` · sqlparser 0.62.0

```rust
fn on_cluster(self, on_cluster: Option<Ident>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set cluster identifier for the table.

<a id="op-a77aa60ca2fcc29af007c4d5"></a>
## on_cluster

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::on_cluster` · sqlparser 0.62.0

```rust
on_cluster: Option<ast::Ident>
```

Source: `src/ast/helpers/stmt_create_table.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional cluster identifier.

<a id="op-d8863d8cc9ca921060e6ac00"></a>
## on_commit

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::on_commit` · sqlparser 0.62.0

```rust
fn on_commit(self, on_commit: Option<OnCommit>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `ON COMMIT` behavior for temporary tables.

<a id="op-fec77177239f1b5a813ab2c4"></a>
## on_commit

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::on_commit` · sqlparser 0.62.0

```rust
on_commit: Option<ast::OnCommit>
```

Source: `src/ast/helpers/stmt_create_table.rs:115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ON COMMIT` behavior.

<a id="op-62d18b68186afa92926533d6"></a>
## or_replace

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the statement uses `OR REPLACE`.

<a id="op-d8027dd87928b06c7cadf55b"></a>
## or_replace

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::or_replace` · sqlparser 0.62.0

```rust
fn or_replace(self, or_replace: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:254`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `OR REPLACE` for the CREATE TABLE statement.

<a id="op-85928e9826d14f54eda5e11a"></a>
## order_by

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::order_by` · sqlparser 0.62.0

```rust
fn order_by(self, order_by: Option<OneOrManyWithParens<Expr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `ORDER BY` clause for clustered/sorted tables.

<a id="op-f400e342716cb56b7a900b45"></a>
## order_by

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::order_by` · sqlparser 0.62.0

```rust
order_by: Option<ast::OneOrManyWithParens<ast::Expr>>
```

Source: `src/ast/helpers/stmt_create_table.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ORDER BY` for clustering/sorting.

<a id="op-d6c70834045a72ce5a89fe05"></a>
## partition_by

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::partition_by` · sqlparser 0.62.0

```rust
fn partition_by(self, partition_by: Option<Box<Expr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `PARTITION BY` expression.

<a id="op-e998d728b7862106396c5ea6"></a>
## partition_by

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::partition_by` · sqlparser 0.62.0

```rust
partition_by: Option<Box<ast::Expr>>
```

Source: `src/ast/helpers/stmt_create_table.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `PARTITION BY` expression.

<a id="op-0632543468e542019c909c9e"></a>
## partition_of

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::partition_of` · sqlparser 0.62.0

```rust
fn partition_of(self, partition_of: Option<ObjectName>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:406`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sets the table which is partitioned to create the current table.

<a id="op-fd2ee8941eaa9520e9de28f4"></a>
## partition_of

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::partition_of` · sqlparser 0.62.0

```rust
partition_of: Option<ast::ObjectName>
```

Source: `src/ast/helpers/stmt_create_table.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partitioned table (`PARTITION OF`)

<a id="op-156c6f3b198c3909862666a0"></a>
## primary_key

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::primary_key` · sqlparser 0.62.0

```rust
primary_key: Option<Box<ast::Expr>>
```

Source: `src/ast/helpers/stmt_create_table.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional primary key expression.

<a id="op-9eeb12a37af2839f5dc634f6"></a>
## primary_key

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::primary_key` · sqlparser 0.62.0

```rust
fn primary_key(self, primary_key: Option<Box<Expr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set a primary key expression for the table.

<a id="op-3066cbfd6d99bd8aa1455d45"></a>
## query

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::query` · sqlparser 0.62.0

```rust
fn query(self, query: Option<Box<Query>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set an underlying `AS SELECT` query for the table.

<a id="op-538938abdb8af5064e2e7f4d"></a>
## query

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::query` · sqlparser 0.62.0

```rust
query: Option<Box<ast::Query>>
```

Source: `src/ast/helpers/stmt_create_table.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `AS SELECT` query for the table.

<a id="op-5d66e9d5bc9da391745b042e"></a>
## refresh_mode

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::refresh_mode` · sqlparser 0.62.0

```rust
fn refresh_mode(self, refresh_mode: Option<RefreshModeKind>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set refresh mode for materialized/managed tables.

<a id="op-fd56a72feca216b5b68b2f61"></a>
## refresh_mode

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::refresh_mode` · sqlparser 0.62.0

```rust
refresh_mode: Option<ast::RefreshModeKind>
```

Source: `src/ast/helpers/stmt_create_table.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional refresh mode for materialized tables.

<a id="op-7dfd5811b246a7dbe3020b62"></a>
## require_user

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::require_user` · sqlparser 0.62.0

```rust
fn require_user(self, require_user: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Require a user identity for table operations.

<a id="op-eab83253e60435c8d01894d0"></a>
## require_user

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::require_user` · sqlparser 0.62.0

```rust
require_user: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether operations require a user identity.

<a id="op-36e9d9ee054531dfffad54e8"></a>
## serialize

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 38], "end": [65, 47], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/stmt_create_table.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-770f11fa8baa5f200263e51d"></a>
## snapshot

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::snapshot` · sqlparser 0.62.0

```rust
fn snapshot(self, snapshot: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `SNAPSHOT` table flag (BigQuery).

<a id="op-8bd7ae546508efec5291465c"></a>
## snapshot

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::snapshot` · sqlparser 0.62.0

```rust
snapshot: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:85`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SNAPSHOT` table flag.

<a id="op-2d3c7b6e50eba26ce58a90fb"></a>
## sortkey

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::sortkey` · sqlparser 0.62.0

```rust
sortkey: Option<Vec<ast::Expr>>
```

Source: `src/ast/helpers/stmt_create_table.rs:183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `SORTKEY` option.

<a id="op-b97af97267ab208d84a153e2"></a>
## sortkey

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::sortkey` · sqlparser 0.62.0

```rust
fn sortkey(self, sortkey: Option<Vec<Expr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set Redshift `SORTKEY` option.

<a id="op-5da73ce28b8f5e54ed76a4a9"></a>
## storage_serialization_policy

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::storage_serialization_policy` · sqlparser 0.62.0

```rust
storage_serialization_policy: Option<ast::StorageSerializationPolicy>
```

Source: `src/ast/helpers/stmt_create_table.rs:165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage serialization policy.

<a id="op-b2293e47c53cfe16c43f9602"></a>
## storage_serialization_policy

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::storage_serialization_policy` · sqlparser 0.62.0

```rust
fn storage_serialization_policy(self, storage_serialization_policy: Option<StorageSerializationPolicy>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set a storage serialization policy.

<a id="op-769eac3b2c6982b3237e2ff5"></a>
## strict

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::strict` · sqlparser 0.62.0

```rust
fn strict(self, strict: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `STRICT` option.

<a id="op-f7d772b1d999568d69723070"></a>
## strict

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::strict` · sqlparser 0.62.0

```rust
strict: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`STRICT` table flag.

<a id="op-b20b292951334538d7fd4bfe"></a>
## table_options

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::table_options` · sqlparser 0.62.0

```rust
fn table_options(self, table_options: CreateTableOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set arbitrary table options parsed from the statement.

<a id="op-b6a8dbf20068334b1b144e71"></a>
## table_options

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::table_options` · sqlparser 0.62.0

```rust
table_options: ast::CreateTableOptions
```

Source: `src/ast/helpers/stmt_create_table.rs:167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parsed table options from the statement.

<a id="op-65286457f41ef8919594c7f7"></a>
## target_lag

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::target_lag` · sqlparser 0.62.0

```rust
fn target_lag(self, target_lag: Option<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set a target lag configuration (dialect-specific).

<a id="op-b484a177ca2f30ffa57c0c8b"></a>
## target_lag

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::target_lag` · sqlparser 0.62.0

```rust
target_lag: Option<String>
```

Source: `src/ast/helpers/stmt_create_table.rs:169`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional target lag configuration.

<a id="op-0047f58fb6da3700359d5f8e"></a>
## temporary

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::temporary` · sqlparser 0.62.0

```rust
fn temporary(self, temporary: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Mark the table as `TEMPORARY`.

<a id="op-874ca87575af0ecccdb2af2e"></a>
## temporary

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the table is `TEMPORARY`.

<a id="op-5fc7e4ec18d5c0a114bfcda6"></a>
## transient

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::transient` · sqlparser 0.62.0

```rust
transient: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `TRANSIENT` was specified.

<a id="op-f8912b99763fdaf1e58d0d01"></a>
## transient

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::transient` · sqlparser 0.62.0

```rust
fn transient(self, transient: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `TRANSIENT` flag.

<a id="op-459112d751d968f69ed1fb0a"></a>
## try_from

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::try_from` · sqlparser 0.62.0

```rust
fn try_from(stmt: Statement) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [625, 1], "end": [638, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/ast/helpers/stmt_create_table.rs:630`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42089ffde6c2227a9e48433f"></a>
## version

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::version` · sqlparser 0.62.0

```rust
version: Option<ast::TableVersion>
```

Source: `src/ast/helpers/stmt_create_table.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional table version.

<a id="op-ad15c55f0a1948dbc27c8014"></a>
## version

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::version` · sqlparser 0.62.0

```rust
fn version(self, version: Option<TableVersion>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set table `VERSION`.

<a id="op-1cc95bb6710e9888f682ce06"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 47], "end": [66, 55], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/stmt_create_table.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a9856be3205ddee4eb49c37"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 40], "end": [66, 45], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/stmt_create_table.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29300a3fdb2a759b89c1fe23"></a>
## volatile

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::volatile` · sqlparser 0.62.0

```rust
volatile: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:81`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `VOLATILE` was specified.

<a id="op-6536f19fb8c23bf6cf702152"></a>
## volatile

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::volatile` · sqlparser 0.62.0

```rust
fn volatile(self, volatile: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `VOLATILE` flag.

<a id="op-2f7d3b7870600a4153747d4e"></a>
## warehouse

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::warehouse` · sqlparser 0.62.0

```rust
warehouse: Option<ast::Ident>
```

Source: `src/ast/helpers/stmt_create_table.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional warehouse identifier.

<a id="op-e297f2576613211934d090e3"></a>
## warehouse

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::warehouse` · sqlparser 0.62.0

```rust
fn warehouse(self, warehouse: Option<Ident>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:520`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Associate the table with a warehouse identifier.

<a id="op-5f3dbee87c49809ff15d6a27"></a>
## with_aggregation_policy

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_aggregation_policy` · sqlparser 0.62.0

```rust
with_aggregation_policy: Option<ast::ObjectName>
```

Source: `src/ast/helpers/stmt_create_table.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional aggregation policy object name.

<a id="op-c4c3c7783568d38b0de16e40"></a>
## with_aggregation_policy

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_aggregation_policy` · sqlparser 0.62.0

```rust
fn with_aggregation_policy(self, with_aggregation_policy: Option<ObjectName>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set aggregation policy object.

<a id="op-2c0ce84f029004625af5a328"></a>
## with_row_access_policy

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_row_access_policy` · sqlparser 0.62.0

```rust
fn with_row_access_policy(self, with_row_access_policy: Option<RowAccessPolicy>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attach a row access policy to the table.

<a id="op-ed4c92ed5991cefae224e3ce"></a>
## with_row_access_policy

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_row_access_policy` · sqlparser 0.62.0

```rust
with_row_access_policy: Option<ast::RowAccessPolicy>
```

Source: `src/ast/helpers/stmt_create_table.rs:151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional row access policy applied to the table.

<a id="op-b8bc8dae963bc175f4dc45f0"></a>
## with_storage_lifecycle_policy

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_storage_lifecycle_policy` · sqlparser 0.62.0

```rust
fn with_storage_lifecycle_policy(self, with_storage_lifecycle_policy: Option<StorageLifecyclePolicy>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attach a storage lifecycle policy to the table.

<a id="op-c1fba3cda7c891e0e3725f36"></a>
## with_storage_lifecycle_policy

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_storage_lifecycle_policy` · sqlparser 0.62.0

```rust
with_storage_lifecycle_policy: Option<ast::StorageLifecyclePolicy>
```

Source: `src/ast/helpers/stmt_create_table.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional storage lifecycle policy applied to the table.

<a id="op-3314c15fd7d46425464a4482"></a>
## with_tags

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_tags` · sqlparser 0.62.0

```rust
with_tags: Option<Vec<ast::Tag>>
```

Source: `src/ast/helpers/stmt_create_table.rs:155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional tags/labels attached to the table metadata.

<a id="op-741022e7ceb0257af06b3ba9"></a>
## with_tags

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::with_tags` · sqlparser 0.62.0

```rust
fn with_tags(self, with_tags: Option<Vec<Tag>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attach tags/labels to the table metadata.

<a id="op-4334afababa7a3f707d709c8"></a>
## without_rowid

`struct_field` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::without_rowid` · sqlparser 0.62.0

```rust
without_rowid: bool
```

Source: `src/ast/helpers/stmt_create_table.rs:105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `WITHOUT ROWID` is set.

<a id="op-c4b7fa3e1a56486c50596928"></a>
## without_rowid

`function` · `sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder::without_rowid` · sqlparser 0.62.0

```rust
fn without_rowid(self, without_rowid: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder", "path": "CreateTableBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [623, 2], "filename": "src/ast/helpers/stmt_create_table.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/helpers/stmt_create_table.rs:339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set `WITHOUT ROWID` option.
