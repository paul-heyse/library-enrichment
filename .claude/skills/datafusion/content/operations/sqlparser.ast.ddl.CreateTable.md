# `sqlparser::ast::ddl::CreateTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateTable.json).

<a id="op-63d22407a880eec6ebcc698f"></a>
## CreateTable

`struct` · `sqlparser::ast::ddl::CreateTable` · sqlparser 0.62.0

```rust
struct CreateTable
```

Source: `src/ast/ddl.rs:2897`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE TABLE statement.

<a id="op-1fcc5b10778ab9ccdc8e1957"></a>
## backup

`struct_field` · `sqlparser::ast::ddl::CreateTable::backup` · sqlparser 0.62.0

```rust
backup: Option<bool>
```

Source: `src/ast/ddl.rs:3062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `BACKUP` option: `BACKUP { YES | NO }`
<https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html>

<a id="op-9de2e4d4b10db75e79c503db"></a>
## base_location

`struct_field` · `sqlparser::ast::ddl::CreateTable::base_location` · sqlparser 0.62.0

```rust
base_location: Option<String>
```

Source: `src/ast/ddl.rs:3026`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "BASE_LOCATION" clause for Iceberg tables
<https://docs.snowflake.com/en/sql-reference/sql/create-iceberg-table>

<a id="op-846a071987091b4acc93e3bf"></a>
## catalog

`struct_field` · `sqlparser::ast::ddl::CreateTable::catalog` · sqlparser 0.62.0

```rust
catalog: Option<String>
```

Source: `src/ast/ddl.rs:3029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "CATALOG" clause for Iceberg tables
<https://docs.snowflake.com/en/sql-reference/sql/create-iceberg-table>

<a id="op-2bc8056f2f66762dabf38fc4"></a>
## catalog_sync

`struct_field` · `sqlparser::ast::ddl::CreateTable::catalog_sync` · sqlparser 0.62.0

```rust
catalog_sync: Option<String>
```

Source: `src/ast/ddl.rs:3032`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "CATALOG_SYNC" clause for Iceberg tables
<https://docs.snowflake.com/en/sql-reference/sql/create-iceberg-table>

<a id="op-953fea61964eccfbab3ebc40"></a>
## change_tracking

`struct_field` · `sqlparser::ast::ddl::CreateTable::change_tracking` · sqlparser 0.62.0

```rust
change_tracking: Option<bool>
```

Source: `src/ast/ddl.rs:2999`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "CHANGE_TRACKING" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-9ed6cc3076fb3df062ee22c9"></a>
## clone

`struct_field` · `sqlparser::ast::ddl::CreateTable::clone` · sqlparser 0.62.0

```rust
clone: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:2943`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CLONE` clause

<a id="op-ac15c7ba5efb79afbacd8b6c"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateTable::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2894, 17], "end": [2894, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd57549889b217e1f33ab938"></a>
## cluster_by

`struct_field` · `sqlparser::ast::ddl::CreateTable::cluster_by` · sqlparser 0.62.0

```rust
cluster_by: Option<ast::WrappedCollection<Vec<ast::Expr>>>
```

Source: `src/ast/ddl.rs:2970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery: Table clustering column list.
<https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#table_option_list>
Snowflake: Table clustering list which contains base column, expressions on base columns.
<https://docs.snowflake.com/en/user-guide/tables-clustering-keys#defining-a-clustering-key-for-a-table>

<a id="op-0135b6645d91105aa675a1fa"></a>
## clustered_by

`struct_field` · `sqlparser::ast::ddl::CreateTable::clustered_by` · sqlparser 0.62.0

```rust
clustered_by: Option<ClusteredBy>
```

Source: `src/ast/ddl.rs:2973`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive: Table clustering column list.
<https://cwiki.apache.org/confluence/display/Hive/LanguageManual+DDL#LanguageManualDDL-CreateTable>

<a id="op-a8e1f047d3d78b9bc2a578f4"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateTable::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateTable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2894, 51], "end": [2894, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4cacd4735a31ca215e31257"></a>
## columns

`struct_field` · `sqlparser::ast::ddl::CreateTable::columns` · sqlparser 0.62.0

```rust
columns: Vec<ColumnDef>
```

Source: `src/ast/ddl.rs:2923`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column definitions

<a id="op-60c778bfde8fdac858910a01"></a>
## comment

`struct_field` · `sqlparser::ast::ddl::CreateTable::comment` · sqlparser 0.62.0

```rust
comment: Option<ast::CommentDef>
```

Source: `src/ast/ddl.rs:2949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For Hive dialect, the table comment is after the column definitions without `=`,
so the `comment` field is optional and different than the comment field in the general options list.
[Hive](https://cwiki.apache.org/confluence/display/Hive/LanguageManual+DDL#LanguageManualDDL-CreateTable)

<a id="op-9a4b750c79632fb49772c8f5"></a>
## constraints

`struct_field` · `sqlparser::ast::ddl::CreateTable::constraints` · sqlparser 0.62.0

```rust
constraints: Vec<ast::table_constraints::TableConstraint>
```

Source: `src/ast/ddl.rs:2925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table constraints

<a id="op-48338722559ffc879f5f3b43"></a>
## copy_grants

`struct_field` · `sqlparser::ast::ddl::CreateTable::copy_grants` · sqlparser 0.62.0

```rust
copy_grants: bool
```

Source: `src/ast/ddl.rs:2993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "COPY GRANTS" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-bc42918c365d6fa20a569ad2"></a>
## data_retention_time_in_days

`struct_field` · `sqlparser::ast::ddl::CreateTable::data_retention_time_in_days` · sqlparser 0.62.0

```rust
data_retention_time_in_days: Option<u64>
```

Source: `src/ast/ddl.rs:3002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "DATA_RETENTION_TIME_IN_DAYS" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-7d70e61531a4eb7a1c2d1cf8"></a>
## default_ddl_collation

`struct_field` · `sqlparser::ast::ddl::CreateTable::default_ddl_collation` · sqlparser 0.62.0

```rust
default_ddl_collation: Option<String>
```

Source: `src/ast/ddl.rs:3008`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "DEFAULT_DDL_COLLATION" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-5be01da14c8fc4f0612f7851"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateTable::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2895, 49], "end": [2895, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe0443171b4f859d70f212dd"></a>
## distkey

`struct_field` · `sqlparser::ast::ddl::CreateTable::distkey` · sqlparser 0.62.0

```rust
distkey: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:3056`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `DISTKEY` option
<https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html>

<a id="op-20bb177ca0055dd03d246e9d"></a>
## diststyle

`struct_field` · `sqlparser::ast::ddl::CreateTable::diststyle` · sqlparser 0.62.0

```rust
diststyle: Option<DistStyle>
```

Source: `src/ast/ddl.rs:3053`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `DISTSTYLE` option
<https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html>

<a id="op-9c066ad522da32f3e1e4bbc1"></a>
## dynamic

`struct_field` · `sqlparser::ast::ddl::CreateTable::dynamic` · sqlparser 0.62.0

```rust
dynamic: bool
```

Source: `src/ast/ddl.rs:2905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DYNAMIC` clause

<a id="op-10352487085ef51f5ff48a9e"></a>
## enable_schema_evolution

`struct_field` · `sqlparser::ast::ddl::CreateTable::enable_schema_evolution` · sqlparser 0.62.0

```rust
enable_schema_evolution: Option<bool>
```

Source: `src/ast/ddl.rs:2996`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "ENABLE_SCHEMA_EVOLUTION" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-53363e577195888bc6c6532b"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateTable::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2894, 24], "end": [2894, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e7a49aa6eb738a2434398fe"></a>
## external

`struct_field` · `sqlparser::ast::ddl::CreateTable::external` · sqlparser 0.62.0

```rust
external: bool
```

Source: `src/ast/ddl.rs:2903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXTERNAL` clause

<a id="op-34e6b2229674233eb78bf919"></a>
## external_volume

`struct_field` · `sqlparser::ast::ddl::CreateTable::external_volume` · sqlparser 0.62.0

```rust
external_volume: Option<String>
```

Source: `src/ast/ddl.rs:3023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "EXTERNAL_VOLUME" clause for Iceberg tables
<https://docs.snowflake.com/en/sql-reference/sql/create-iceberg-table>

<a id="op-ab44e96e879d3b3e9b2772e9"></a>
## file_format

`struct_field` · `sqlparser::ast::ddl::CreateTable::file_format` · sqlparser 0.62.0

```rust
file_format: Option<ast::FileFormat>
```

Source: `src/ast/ddl.rs:2933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

General comment for the table

<a id="op-0b5b967f0645728d30344a35"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3065, 1], "end": [3384, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3066`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b1bfc1bf6f63903143eaedb"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2894, 10], "end": [2894, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc9e389c95524177806799b"></a>
## for_values

`struct_field` · `sqlparser::ast::ddl::CreateTable::for_values` · sqlparser 0.62.0

```rust
for_values: Option<ForValues>
```

Source: `src/ast/ddl.rs:2986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL partition bound specification for PARTITION OF.
<https://www.postgresql.org/docs/current/sql-createtable.html>

<a id="op-5ce35fedccfad0b92d47f69b"></a>
## global

`struct_field` · `sqlparser::ast::ddl::CreateTable::global` · sqlparser 0.62.0

```rust
global: Option<bool>
```

Source: `src/ast/ddl.rs:2907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`GLOBAL` clause

<a id="op-d0c290b3dd4fa1c4b4bb180a"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateTable::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2894, 56], "end": [2894, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8b47ecaa03d9cbef5b5841b"></a>
## hive_distribution

`struct_field` · `sqlparser::ast::ddl::CreateTable::hive_distribution` · sqlparser 0.62.0

```rust
hive_distribution: ast::HiveDistributionStyle
```

Source: `src/ast/ddl.rs:2927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive-specific distribution style

<a id="op-76098ef18bbde46aadbf40c1"></a>
## hive_formats

`struct_field` · `sqlparser::ast::ddl::CreateTable::hive_formats` · sqlparser 0.62.0

```rust
hive_formats: Option<ast::HiveFormat>
```

Source: `src/ast/ddl.rs:2929`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive-specific formats like `ROW FORMAT DELIMITED` or `ROW FORMAT SERDE 'serde_class' WITH SERDEPROPERTIES (...)`

<a id="op-189bc339889d76b698a86582"></a>
## iceberg

`struct_field` · `sqlparser::ast::ddl::CreateTable::iceberg` · sqlparser 0.62.0

```rust
iceberg: bool
```

Source: `src/ast/ddl.rs:2915`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ICEBERG` clause

<a id="op-cb6423b7b3c6261afe1b4f37"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateTable::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:2909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF NOT EXISTS` clause

<a id="op-84411354c4a438e6bb858196"></a>
## inherits

`struct_field` · `sqlparser::ast::ddl::CreateTable::inherits` · sqlparser 0.62.0

```rust
inherits: Option<Vec<ast::ObjectName>>
```

Source: `src/ast/ddl.rs:2978`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres `INHERITs` clause, which contains the list of tables from which
the new table inherits.
<https://www.postgresql.org/docs/current/ddl-inherit.html>
<https://www.postgresql.org/docs/current/sql-createtable.html#SQL-CREATETABLE-PARMS-INHERITS>

<a id="op-1849d3dee232b4b22a21c541"></a>
## initialize

`struct_field` · `sqlparser::ast::ddl::CreateTable::initialize` · sqlparser 0.62.0

```rust
initialize: Option<ast::InitializeKind>
```

Source: `src/ast/ddl.rs:3047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "INITIALIZE" clause for dybamic tables
<https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table>

<a id="op-fd4c11eeb2484501d2992816"></a>
## like

`struct_field` · `sqlparser::ast::ddl::CreateTable::like` · sqlparser 0.62.0

```rust
like: Option<ast::CreateTableLikeKind>
```

Source: `src/ast/ddl.rs:2941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LIKE` clause

<a id="op-313b22c737602d492d64961d"></a>
## location

`struct_field` · `sqlparser::ast::ddl::CreateTable::location` · sqlparser 0.62.0

```rust
location: Option<String>
```

Source: `src/ast/ddl.rs:2935`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Location of the table data

<a id="op-a6831c76a27190e1cabfe241"></a>
## max_data_extension_time_in_days

`struct_field` · `sqlparser::ast::ddl::CreateTable::max_data_extension_time_in_days` · sqlparser 0.62.0

```rust
max_data_extension_time_in_days: Option<u64>
```

Source: `src/ast/ddl.rs:3005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "MAX_DATA_EXTENSION_TIME_IN_DAYS" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-ad37dd7e121959aab8e7eaf1"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateTable::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:2921`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table name

<a id="op-a63b3a86c542fe62cc3b6f2b"></a>
## on_cluster

`struct_field` · `sqlparser::ast::ddl::CreateTable::on_cluster` · sqlparser 0.62.0

```rust
on_cluster: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:2955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse "ON CLUSTER" clause:
<https://clickhouse.com/docs/en/sql-reference/distributed-ddl/>

<a id="op-2eb85b9c64be8aae87227c53"></a>
## on_commit

`struct_field` · `sqlparser::ast::ddl::CreateTable::on_commit` · sqlparser 0.62.0

```rust
on_commit: Option<ast::OnCommit>
```

Source: `src/ast/ddl.rs:2952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse "ON COMMIT" clause:
<https://clickhouse.com/docs/en/sql-reference/statements/create/table/>

<a id="op-b6e0ac7ac7c89644ad30cf27"></a>
## or_replace

`struct_field` · `sqlparser::ast::ddl::CreateTable::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/ddl.rs:2899`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OR REPLACE` clause

<a id="op-5bff77fe88c69fec81403865"></a>
## order_by

`struct_field` · `sqlparser::ast::ddl::CreateTable::order_by` · sqlparser 0.62.0

```rust
order_by: Option<ast::OneOrManyWithParens<ast::Expr>>
```

Source: `src/ast/ddl.rs:2962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse "ORDER BY " clause. Note that omitted ORDER BY is different
than empty (represented as ()), the latter meaning "no sorting".
<https://clickhouse.com/docs/en/sql-reference/statements/create/table/>

<a id="op-e2c52cb52611e702e4a7009f"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateTable::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateTable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2894, 35], "end": [2894, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ac82d75a1c6302f4bfc6a68"></a>
## partition_by

`struct_field` · `sqlparser::ast::ddl::CreateTable::partition_by` · sqlparser 0.62.0

```rust
partition_by: Option<Box<ast::Expr>>
```

Source: `src/ast/ddl.rs:2965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery: A partition expression for the table.
<https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#partition_expression>

<a id="op-e15e52008753c349a32801c9"></a>
## partition_of

`struct_field` · `sqlparser::ast::ddl::CreateTable::partition_of` · sqlparser 0.62.0

```rust
partition_of: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:2983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL `PARTITION OF` clause to create a partition of a parent table.
Contains the parent table name.
<https://www.postgresql.org/docs/current/sql-createtable.html>

<a id="op-90dc1152d1f2d83d3c8c0f99"></a>
## primary_key

`struct_field` · `sqlparser::ast::ddl::CreateTable::primary_key` · sqlparser 0.62.0

```rust
primary_key: Option<Box<ast::Expr>>
```

Source: `src/ast/ddl.rs:2958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse "PRIMARY KEY " clause.
<https://clickhouse.com/docs/en/sql-reference/statements/create/table/>

<a id="op-c887000ebd29fc45e2fc77e3"></a>
## query

`struct_field` · `sqlparser::ast::ddl::CreateTable::query` · sqlparser 0.62.0

```rust
query: Option<Box<ast::Query>>
```

Source: `src/ast/ddl.rs:2937`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query used to populate the table

<a id="op-47a07acfd16fab1afa10ad1d"></a>
## refresh_mode

`struct_field` · `sqlparser::ast::ddl::CreateTable::refresh_mode` · sqlparser 0.62.0

```rust
refresh_mode: Option<ast::RefreshModeKind>
```

Source: `src/ast/ddl.rs:3044`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "REFRESH_MODE" clause for dybamic tables
<https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table>

<a id="op-ae9cab6e3b5ccaa8ce21d934"></a>
## require_user

`struct_field` · `sqlparser::ast::ddl::CreateTable::require_user` · sqlparser 0.62.0

```rust
require_user: bool
```

Source: `src/ast/ddl.rs:3050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "REQUIRE USER" clause for dybamic tables
<https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table>

<a id="op-7b4f2fd94ceaa5cad81218c6"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateTable::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2895, 38], "end": [2895, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf0fd3178502bcb471f7963b"></a>
## snapshot

`struct_field` · `sqlparser::ast::ddl::CreateTable::snapshot` · sqlparser 0.62.0

```rust
snapshot: bool
```

Source: `src/ast/ddl.rs:2918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SNAPSHOT` clause
<https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_snapshot_table_statement>

<a id="op-ebb2dbeb74b8c8fa080e9f67"></a>
## sortkey

`struct_field` · `sqlparser::ast::ddl::CreateTable::sortkey` · sqlparser 0.62.0

```rust
sortkey: Option<Vec<ast::Expr>>
```

Source: `src/ast/ddl.rs:3059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `SORTKEY` option
<https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html>

<a id="op-7da446f600b29c65914d0755"></a>
## span

`function` · `sqlparser::ast::ddl::CreateTable::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "super::CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [545, 1], "end": [620, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-778a155dae789f1a1f0cadfe"></a>
## storage_serialization_policy

`struct_field` · `sqlparser::ast::ddl::CreateTable::storage_serialization_policy` · sqlparser 0.62.0

```rust
storage_serialization_policy: Option<ast::StorageSerializationPolicy>
```

Source: `src/ast/ddl.rs:3035`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "STORAGE_SERIALIZATION_POLICY" clause for Iceberg tables
<https://docs.snowflake.com/en/sql-reference/sql/create-iceberg-table>

<a id="op-cedb4dd348e605dc58dd3856"></a>
## strict

`struct_field` · `sqlparser::ast::ddl::CreateTable::strict` · sqlparser 0.62.0

```rust
strict: bool
```

Source: `src/ast/ddl.rs:2990`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQLite "STRICT" clause.
if the "STRICT" table-option keyword is added to the end, after the closing ")",
then strict typing rules apply to that table.

<a id="op-f18a92a756b970e21375d3cb"></a>
## table_options

`struct_field` · `sqlparser::ast::ddl::CreateTable::table_options` · sqlparser 0.62.0

```rust
table_options: ast::CreateTableOptions
```

Source: `src/ast/ddl.rs:2931`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table options

<a id="op-cfcc197cc40b5577f0ea7cdb"></a>
## target_lag

`struct_field` · `sqlparser::ast::ddl::CreateTable::target_lag` · sqlparser 0.62.0

```rust
target_lag: Option<String>
```

Source: `src/ast/ddl.rs:3038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "TARGET_LAG" clause for dybamic tables
<https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table>

<a id="op-3c31e6662d52299f8c6a4fc8"></a>
## temporary

`struct_field` · `sqlparser::ast::ddl::CreateTable::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/ddl.rs:2901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TEMP` or `TEMPORARY` clause

<a id="op-bce292044004300e1d4fed4c"></a>
## transient

`struct_field` · `sqlparser::ast::ddl::CreateTable::transient` · sqlparser 0.62.0

```rust
transient: bool
```

Source: `src/ast/ddl.rs:2911`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TRANSIENT` clause

<a id="op-b011b420fcfc1d6cf3b4611c"></a>
## version

`struct_field` · `sqlparser::ast::ddl::CreateTable::version` · sqlparser 0.62.0

```rust
version: Option<ast::TableVersion>
```

Source: `src/ast/ddl.rs:2945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table version (for systems that support versioned tables)

<a id="op-45bbb85182bf97eb5187447d"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2896, 47], "end": [2896, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-812ee2330329b020e4b3d40c"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2896, 40], "end": [2896, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f4af6b951f5fbc5d9ecb888"></a>
## volatile

`struct_field` · `sqlparser::ast::ddl::CreateTable::volatile` · sqlparser 0.62.0

```rust
volatile: bool
```

Source: `src/ast/ddl.rs:2913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VOLATILE` clause

<a id="op-e589bbe89e944f5b292967d5"></a>
## warehouse

`struct_field` · `sqlparser::ast::ddl::CreateTable::warehouse` · sqlparser 0.62.0

```rust
warehouse: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:3041`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "WAREHOUSE" clause for dybamic tables
<https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table>

<a id="op-bb29fb114dfc85d57233740e"></a>
## with_aggregation_policy

`struct_field` · `sqlparser::ast::ddl::CreateTable::with_aggregation_policy` · sqlparser 0.62.0

```rust
with_aggregation_policy: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:3011`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "WITH AGGREGATION POLICY" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-e246983fcd9f080bdf18af57"></a>
## with_row_access_policy

`struct_field` · `sqlparser::ast::ddl::CreateTable::with_row_access_policy` · sqlparser 0.62.0

```rust
with_row_access_policy: Option<ast::RowAccessPolicy>
```

Source: `src/ast/ddl.rs:3014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "WITH ROW ACCESS POLICY" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-f2f3c93115cefc1bb0a23e57"></a>
## with_storage_lifecycle_policy

`struct_field` · `sqlparser::ast::ddl::CreateTable::with_storage_lifecycle_policy` · sqlparser 0.62.0

```rust
with_storage_lifecycle_policy: Option<ast::StorageLifecyclePolicy>
```

Source: `src/ast/ddl.rs:3017`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `WITH STORAGE LIFECYCLE POLICY` clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-ba3b6b77aedf1ca69a7af554"></a>
## with_tags

`struct_field` · `sqlparser::ast::ddl::CreateTable::with_tags` · sqlparser 0.62.0

```rust
with_tags: Option<Vec<ast::Tag>>
```

Source: `src/ast/ddl.rs:3020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake "WITH TAG" clause
<https://docs.snowflake.com/en/sql-reference/sql/create-table>

<a id="op-72500e873be84bab01c15ba3"></a>
## without_rowid

`struct_field` · `sqlparser::ast::ddl::CreateTable::without_rowid` · sqlparser 0.62.0

```rust
without_rowid: bool
```

Source: `src/ast/ddl.rs:2939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If the table should be created without a rowid (SQLite)
