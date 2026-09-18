# `sqlparser::ast::ddl::AlterTableOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableOperation.json).

<a id="op-faf8320214a9c4ad71ce0174"></a>
## AlterTableOperation

`enum` · `sqlparser::ast::ddl::AlterTableOperation` · sqlparser 0.62.0

```rust
enum AlterTableOperation
```

Source: `src/ast/ddl.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER TABLE` (`Statement::AlterTable`) operation

<a id="op-16a3c35f22b187bfc0fd3640"></a>
## AddColumn

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AddColumn` · sqlparser 0.62.0

```rust
AddColumn
```

Source: `src/ast/ddl.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ADD [COLUMN] [IF NOT EXISTS] <column_def>`

<a id="op-2b220785634ae113c1e78fe3"></a>
## AddConstraint

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AddConstraint` · sqlparser 0.62.0

```rust
AddConstraint
```

Source: `src/ast/ddl.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ADD <table_constraint> [NOT VALID]`

<a id="op-218690260d51017ff4a2c0c0"></a>
## AddPartitions

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AddPartitions` · sqlparser 0.62.0

```rust
AddPartitions
```

Source: `src/ast/ddl.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Add Partitions

<a id="op-60ddb1ebcbba1939574a249d"></a>
## AddProjection

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AddProjection` · sqlparser 0.62.0

```rust
AddProjection
```

Source: `src/ast/ddl.rs:150`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ADD PROJECTION [IF NOT EXISTS] name ( SELECT <COLUMN LIST EXPR> [GROUP BY] [ORDER BY])`

Note: this is a ClickHouse-specific operation.
Please refer to [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/projection#add-projection)

<a id="op-72ddfde9fa08c107be2f6f12"></a>
## Algorithm

`variant` · `sqlparser::ast::ddl::AlterTableOperation::Algorithm` · sqlparser 0.62.0

```rust
Algorithm
```

Source: `src/ast/ddl.rs:492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALGORITHM [=] { DEFAULT | INSTANT | INPLACE | COPY }`

[MySQL]-specific table alter algorithm.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html

<a id="op-c5864cc29e9accc8b24031bd"></a>
## AlterColumn

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AlterColumn` · sqlparser 0.62.0

```rust
AlterColumn
```

Source: `src/ast/ddl.rs:427`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALTER [ COLUMN ]`
Alter a specific column with the provided operation.

<a id="op-8627724717cd4cef2dcac17b"></a>
## AlterSortKey

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AlterSortKey` · sqlparser 0.62.0

```rust
AlterSortKey
```

Source: `src/ast/ddl.rs:462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Redshift `ALTER SORTKEY (column_list)`
<https://docs.aws.amazon.com/redshift/latest/dg/r_ALTER_TABLE.html>

<a id="op-fe049f7e5f9851a4540b0abd"></a>
## AttachPartition

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AttachPartition` · sqlparser 0.62.0

```rust
AttachPartition
```

Source: `src/ast/ddl.rs:234`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ATTACH PART|PARTITION <partition_expr>`
Note: this is a ClickHouse-specific operation, please refer to
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/partition#attach-partitionpart)

<a id="op-359968ff04804fd1c94530ff"></a>
## AutoIncrement

`variant` · `sqlparser::ast::ddl::AlterTableOperation::AutoIncrement` · sqlparser 0.62.0

```rust
AutoIncrement
```

Source: `src/ast/ddl.rs:515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AUTO_INCREMENT [=] <value>`

[MySQL]-specific table option for raising current auto increment value.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html

<a id="op-740dba6feeacb3d6c525b806"></a>
## ChangeColumn

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ChangeColumn` · sqlparser 0.62.0

```rust
ChangeColumn
```

Source: `src/ast/ddl.rs:391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Change an existing column's name, type, and options.

<a id="op-a666e9aef5355dd1563f92ef"></a>
## ClearProjection

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ClearProjection` · sqlparser 0.62.0

```rust
ClearProjection
```

Source: `src/ast/ddl.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CLEAR PROJECTION [IF EXISTS] name [IN PARTITION partition_name]`

Note: this is a ClickHouse-specific operation.
Please refer to [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/projection#clear-projection)

<a id="op-bfcd8e75ac5372258678f097"></a>
## ClusterBy

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ClusterBy` · sqlparser 0.62.0

```rust
ClusterBy
```

Source: `src/ast/ddl.rs:454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake table clustering options
<https://docs.snowflake.com/en/sql-reference/sql/alter-table#clustering-actions-clusteringaction>

<a id="op-0902ffb697ec5a72b97809b6"></a>
## DetachPartition

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DetachPartition` · sqlparser 0.62.0

```rust
DetachPartition
```

Source: `src/ast/ddl.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DETACH PART|PARTITION <partition_expr>`
Note: this is a ClickHouse-specific operation, please refer to
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/partition#detach-partitionpart)

<a id="op-cf765ac1427cc52e51ae1e6a"></a>
## DisableRowLevelSecurity

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DisableRowLevelSecurity` · sqlparser 0.62.0

```rust
DisableRowLevelSecurity
```

Source: `src/ast/ddl.rs:196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISABLE ROW LEVEL SECURITY`

Note: this is a PostgreSQL-specific operation.
Please refer to [PostgreSQL documentation](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-d4f0a81bafb0b95764de1f05"></a>
## DisableRule

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DisableRule` · sqlparser 0.62.0

```rust
DisableRule
```

Source: `src/ast/ddl.rs:200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISABLE RULE rewrite_rule_name`

Note: this is a PostgreSQL-specific operation.

<a id="op-85d40936c30117de466839c1"></a>
## DisableTrigger

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DisableTrigger` · sqlparser 0.62.0

```rust
DisableTrigger
```

Source: `src/ast/ddl.rs:207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISABLE TRIGGER [ trigger_name | ALL | USER ]`

Note: this is a PostgreSQL-specific operation.

<a id="op-e279f17f0479a29d4a2b0739"></a>
## DropClusteringKey

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropClusteringKey` · sqlparser 0.62.0

```rust
DropClusteringKey
```

Source: `src/ast/ddl.rs:459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Remove the clustering key from the table.

<a id="op-a7be52d63189a187b42076ab"></a>
## DropColumn

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropColumn` · sqlparser 0.62.0

```rust
DropColumn
```

Source: `src/ast/ddl.rs:221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP [ COLUMN ] [ IF EXISTS ] <column_name> [ , <column_name>, ... ] [ CASCADE ]`

<a id="op-b04aba08f6d68df0fa7f9753"></a>
## DropConstraint

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropConstraint` · sqlparser 0.62.0

```rust
DropConstraint
```

Source: `src/ast/ddl.rs:212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP CONSTRAINT [ IF EXISTS ] <name>`

<a id="op-05456b2a105e25b7ed3e7980"></a>
## DropForeignKey

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropForeignKey` · sqlparser 0.62.0

```rust
DropForeignKey
```

Source: `src/ast/ddl.rs:278`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP FOREIGN KEY <fk_symbol>`

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/alter-table.html)
[Snowflake](https://docs.snowflake.com/en/sql-reference/constraints-drop)

<a id="op-f94ce7bb30a9452d9e1f8d7c"></a>
## DropIndex

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropIndex` · sqlparser 0.62.0

```rust
DropIndex
```

Source: `src/ast/ddl.rs:287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP INDEX <index_name>`

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html

<a id="op-1336ac9efe6e494edf9bdda0"></a>
## DropPartitions

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropPartitions` · sqlparser 0.62.0

```rust
DropPartitions
```

Source: `src/ast/ddl.rs:371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP PARTITIONS ...` / drop partitions from the table.

<a id="op-e5da15efe059118b3696c4c4"></a>
## DropPrimaryKey

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropPrimaryKey` · sqlparser 0.62.0

```rust
DropPrimaryKey
```

Source: `src/ast/ddl.rs:270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP PRIMARY KEY`

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/alter-table.html)
[Snowflake](https://docs.snowflake.com/en/sql-reference/constraints-drop)

<a id="op-fec29ba84e746cab85e7ca47"></a>
## DropProjection

`variant` · `sqlparser::ast::ddl::AlterTableOperation::DropProjection` · sqlparser 0.62.0

```rust
DropProjection
```

Source: `src/ast/ddl.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP PROJECTION [IF EXISTS] name`

Note: this is a ClickHouse-specific operation.
Please refer to [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/projection#drop-projection)

<a id="op-1c28963b1e7d44c8adfc6c9e"></a>
## EnableAlwaysRule

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableAlwaysRule` · sqlparser 0.62.0

```rust
EnableAlwaysRule
```

Source: `src/ast/ddl.rs:294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE ALWAYS RULE rewrite_rule_name`

Note: this is a PostgreSQL-specific operation.

<a id="op-3c30f3d7648a4c318450d8c2"></a>
## EnableAlwaysTrigger

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableAlwaysTrigger` · sqlparser 0.62.0

```rust
EnableAlwaysTrigger
```

Source: `src/ast/ddl.rs:301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE ALWAYS TRIGGER trigger_name`

Note: this is a PostgreSQL-specific operation.

<a id="op-ee2a99b154366d10b44925bc"></a>
## EnableReplicaRule

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableReplicaRule` · sqlparser 0.62.0

```rust
EnableReplicaRule
```

Source: `src/ast/ddl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE REPLICA RULE rewrite_rule_name`

Note: this is a PostgreSQL-specific operation.

<a id="op-bdabe4ac8ce2893ad127f297"></a>
## EnableReplicaTrigger

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableReplicaTrigger` · sqlparser 0.62.0

```rust
EnableReplicaTrigger
```

Source: `src/ast/ddl.rs:315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE REPLICA TRIGGER trigger_name`

Note: this is a PostgreSQL-specific operation.

<a id="op-1af6b3916dee5f0f22bc1668"></a>
## EnableRowLevelSecurity

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableRowLevelSecurity` · sqlparser 0.62.0

```rust
EnableRowLevelSecurity
```

Source: `src/ast/ddl.rs:323`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE ROW LEVEL SECURITY`

Note: this is a PostgreSQL-specific operation.
Please refer to [PostgreSQL documentation](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-ec0f092f058c96f4be942549"></a>
## EnableRule

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableRule` · sqlparser 0.62.0

```rust
EnableRule
```

Source: `src/ast/ddl.rs:337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE RULE rewrite_rule_name`

Note: this is a PostgreSQL-specific operation.

<a id="op-61034826b7c1152e094cb684"></a>
## EnableTrigger

`variant` · `sqlparser::ast::ddl::AlterTableOperation::EnableTrigger` · sqlparser 0.62.0

```rust
EnableTrigger
```

Source: `src/ast/ddl.rs:344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ENABLE TRIGGER [ trigger_name | ALL | USER ]`

Note: this is a PostgreSQL-specific operation.

<a id="op-5640f1e13dcbb1bf2f92e325"></a>
## ForceRowLevelSecurity

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ForceRowLevelSecurity` · sqlparser 0.62.0

```rust
ForceRowLevelSecurity
```

Source: `src/ast/ddl.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FORCE ROW LEVEL SECURITY`

Note: this is a PostgreSQL-specific operation.
Please refer to [PostgreSQL documentation](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-8c3140f3316050b62d886d27"></a>
## FreezePartition

`variant` · `sqlparser::ast::ddl::AlterTableOperation::FreezePartition` · sqlparser 0.62.0

```rust
FreezePartition
```

Source: `src/ast/ddl.rs:251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FREEZE PARTITION <partition_expr>`
Note: this is a ClickHouse-specific operation, please refer to
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/partition#freeze-partition)

<a id="op-a0093cf75d891f1e4b05db2e"></a>
## Lock

`variant` · `sqlparser::ast::ddl::AlterTableOperation::Lock` · sqlparser 0.62.0

```rust
Lock
```

Source: `src/ast/ddl.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LOCK [=] { DEFAULT | NONE | SHARED | EXCLUSIVE }`

[MySQL]-specific table alter lock.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html

<a id="op-d98633bb65a0da27ef7412d6"></a>
## MaterializeProjection

`variant` · `sqlparser::ast::ddl::AlterTableOperation::MaterializeProjection` · sqlparser 0.62.0

```rust
MaterializeProjection
```

Source: `src/ast/ddl.rs:172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MATERIALIZE PROJECTION [IF EXISTS] name [IN PARTITION partition_name]`

 Note: this is a ClickHouse-specific operation.
Please refer to [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/projection#materialize-projection)

<a id="op-44ccba810103006700aa7800"></a>
## ModifyColumn

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ModifyColumn` · sqlparser 0.62.0

```rust
ModifyColumn
```

Source: `src/ast/ddl.rs:405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify an existing column's type and options.

<a id="op-3b9781dcfc763615c32d90ec"></a>
## NoForceRowLevelSecurity

`variant` · `sqlparser::ast::ddl::AlterTableOperation::NoForceRowLevelSecurity` · sqlparser 0.62.0

```rust
NoForceRowLevelSecurity
```

Source: `src/ast/ddl.rs:333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NO FORCE ROW LEVEL SECURITY`

Note: this is a PostgreSQL-specific operation.
Please refer to [PostgreSQL documentation](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-83c0c7fc0b02fad95cf2005d"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterTableOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OWNER TO { <new_owner> | CURRENT_ROLE | CURRENT_USER | SESSION_USER }`

Note: this is PostgreSQL-specific <https://www.postgresql.org/docs/current/sql-altertable.html>

<a id="op-8fc404d9d08dae947e91ec82"></a>
## Refresh

`variant` · `sqlparser::ast::ddl::AlterTableOperation::Refresh` · sqlparser 0.62.0

```rust
Refresh
```

Source: `src/ast/ddl.rs:475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`REFRESH [ '<subpath>' ]`

Note: this is Snowflake specific for dynamic/external tables
<https://docs.snowflake.com/en/sql-reference/sql/alter-dynamic-table>
<https://docs.snowflake.com/en/sql-reference/sql/alter-external-table>

<a id="op-77ace3554bff1b1754f95341"></a>
## RenameColumn

`variant` · `sqlparser::ast::ddl::AlterTableOperation::RenameColumn` · sqlparser 0.62.0

```rust
RenameColumn
```

Source: `src/ast/ddl.rs:378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME [ COLUMN ] <old_column_name> TO <new_column_name>`

<a id="op-2a8d1cb0683f0ddecc6c9ce5"></a>
## RenameConstraint

`variant` · `sqlparser::ast::ddl::AlterTableOperation::RenameConstraint` · sqlparser 0.62.0

```rust
RenameConstraint
```

Source: `src/ast/ddl.rs:419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME CONSTRAINT <old_constraint_name> TO <new_constraint_name>`

Note: this is a PostgreSQL-specific operation.
Rename a constraint on the table.

<a id="op-37bebe01d96f6a55b939e269"></a>
## RenamePartitions

`variant` · `sqlparser::ast::ddl::AlterTableOperation::RenamePartitions` · sqlparser 0.62.0

```rust
RenamePartitions
```

Source: `src/ast/ddl.rs:349`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME TO PARTITION (partition=val)`

<a id="op-1c77a34d0740c9c68becaa97"></a>
## RenameTable

`variant` · `sqlparser::ast::ddl::AlterTableOperation::RenameTable` · sqlparser 0.62.0

```rust
RenameTable
```

Source: `src/ast/ddl.rs:385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RENAME TO <table_name>`

<a id="op-4d0e47e81785e641053dd0bd"></a>
## ReplicaIdentity

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ReplicaIdentity` · sqlparser 0.62.0

```rust
ReplicaIdentity
```

Source: `src/ast/ddl.rs:359`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

REPLICA IDENTITY { DEFAULT | USING INDEX index_name | FULL | NOTHING }

Note: this is a PostgreSQL-specific operation.
Please refer to [PostgreSQL documentation](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-047d262aec10cd0bec79c118"></a>
## Resume

`variant` · `sqlparser::ast::ddl::AlterTableOperation::Resume` · sqlparser 0.62.0

```rust
Resume
```

Source: `src/ast/ddl.rs:486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RESUME`

Note: this is Snowflake specific for dynamic tables <https://docs.snowflake.com/en/sql-reference/sql/alter-table>

<a id="op-cb173362ee201f69e8604302"></a>
## ResumeRecluster

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ResumeRecluster` · sqlparser 0.62.0

```rust
ResumeRecluster
```

Source: `src/ast/ddl.rs:469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Resume background reclustering operations.

<a id="op-d1b7cc9b8b75d52c11071729"></a>
## SetOptionsParens

`variant` · `sqlparser::ast::ddl::AlterTableOperation::SetOptionsParens` · sqlparser 0.62.0

```rust
SetOptionsParens
```

Source: `src/ast/ddl.rs:533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arbitrary parenthesized `SET` options.

Example:
```sql
SET (scale_factor = 0.01, threshold = 500)`
```
[PostgreSQL](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-0ec825c3f9eab771ca3375eb"></a>
## SetTblProperties

`variant` · `sqlparser::ast::ddl::AlterTableOperation::SetTblProperties` · sqlparser 0.62.0

```rust
SetTblProperties
```

Source: `src/ast/ddl.rs:441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

'SET TBLPROPERTIES ( { property_key [ = ] property_val } [, ...] )'

<a id="op-5306c675432688ccf936b87a"></a>
## Suspend

`variant` · `sqlparser::ast::ddl::AlterTableOperation::Suspend` · sqlparser 0.62.0

```rust
Suspend
```

Source: `src/ast/ddl.rs:482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SUSPEND`

Note: this is Snowflake specific for dynamic tables <https://docs.snowflake.com/en/sql-reference/sql/alter-table>

<a id="op-fc206555b77bcf4a0cefc544"></a>
## SuspendRecluster

`variant` · `sqlparser::ast::ddl::AlterTableOperation::SuspendRecluster` · sqlparser 0.62.0

```rust
SuspendRecluster
```

Source: `src/ast/ddl.rs:467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Suspend background reclustering operations.

<a id="op-16b5a3b048c6a1800fab7a98"></a>
## SwapWith

`variant` · `sqlparser::ast::ddl::AlterTableOperation::SwapWith` · sqlparser 0.62.0

```rust
SwapWith
```

Source: `src/ast/ddl.rs:436`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

'SWAP WITH <table_name>'

Note: this is Snowflake specific <https://docs.snowflake.com/en/sql-reference/sql/alter-table>

<a id="op-3f2412e836edcecda6fe9442"></a>
## UnfreezePartition

`variant` · `sqlparser::ast::ddl::AlterTableOperation::UnfreezePartition` · sqlparser 0.62.0

```rust
UnfreezePartition
```

Source: `src/ast/ddl.rs:260`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UNFREEZE PARTITION <partition_expr>`
Note: this is a ClickHouse-specific operation, please refer to
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/partition#unfreeze-partition)

<a id="op-c7f229766e8a3d1c02656c35"></a>
## ValidateConstraint

`variant` · `sqlparser::ast::ddl::AlterTableOperation::ValidateConstraint` · sqlparser 0.62.0

```rust
ValidateConstraint
```

Source: `src/ast/ddl.rs:522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VALIDATE CONSTRAINT <name>`

<a id="op-b4b98d91c23a4c51c34bcba9"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTableOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTableOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 17], "end": [124, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f159d49ffe177d06f5845246"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTableOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTableOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 51], "end": [124, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f79b9f2c5d24de18ed563c7a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTableOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 49], "end": [125, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-188ee3129e884a63bb1fd943"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTableOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTableOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 24], "end": [124, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b187e3977e4935b9ddcb85f"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [704, 1], "end": [1049, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63047793fcbabdc0a9c93651"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 10], "end": [124, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-156bd7c4ce92692f3a1ce173"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTableOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 56], "end": [124, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ac71319934fe14b095d4747"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTableOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTableOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 35], "end": [124, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0a26599c5b0fc727bb076fd"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTableOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 38], "end": [125, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10d8610878db5325add7c2f3"></a>
## span

`function` · `sqlparser::ast::ddl::AlterTableOperation::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "super::AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 1], "end": [1234, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6075bcad74b7d633637dcee0"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 47], "end": [126, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9e4b2f27c469029bbb1a60c"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableOperation", "path": "AlterTableOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 40], "end": [126, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
