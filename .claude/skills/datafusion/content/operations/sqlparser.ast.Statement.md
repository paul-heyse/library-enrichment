# `sqlparser::ast::Statement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Statement.json).

<a id="op-a6bf150ce3eb740dc90fe9e6"></a>
## Statement

`enum` · `sqlparser::ast::Statement` · sqlparser 0.62.0

```rust
enum Statement
```

Source: `src/ast/mod.rs:3540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A top-level statement (SELECT, INSERT, CREATE, etc.)

<a id="op-de7822fcc4ff7132ab1fb732"></a>
## AlterCollation

`variant` · `sqlparser::ast::Statement::AlterCollation` · sqlparser 0.62.0

```rust
AlterCollation
```

Source: `src/ast/mod.rs:3812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER COLLATION
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-altercollation.html)

<a id="op-8b14bb8b72d4a17aace79909"></a>
## AlterConnector

`variant` · `sqlparser::ast::Statement::AlterConnector` · sqlparser 0.62.0

```rust
AlterConnector
```

Source: `src/ast/mod.rs:3850`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER CONNECTOR connector_name SET DCPROPERTIES(property_name=property_value, ...);
or
ALTER CONNECTOR connector_name SET URL new_url;
or
ALTER CONNECTOR connector_name SET OWNER [USER|ROLE] user_or_role;
```
(Hive-specific)

<a id="op-7b033d87e9ad54d365934c13"></a>
## AlterFunction

`variant` · `sqlparser::ast::Statement::AlterFunction` · sqlparser 0.62.0

```rust
AlterFunction
```

Source: `src/ast/mod.rs:3802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER FUNCTION
ALTER AGGREGATE
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-alterfunction.html)
and [PostgreSQL](https://www.postgresql.org/docs/current/sql-alteraggregate.html)

<a id="op-b9ba7a9636ce87c1354ad1b4"></a>
## AlterIndex

`variant` · `sqlparser::ast::Statement::AlterIndex` · sqlparser 0.62.0

```rust
AlterIndex
```

Source: `src/ast/mod.rs:3776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER INDEX
```

<a id="op-bb5e5f50a7484323de74c575"></a>
## AlterOperator

`variant` · `sqlparser::ast::Statement::AlterOperator` · sqlparser 0.62.0

```rust
AlterOperator
```

Source: `src/ast/mod.rs:3817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER OPERATOR
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-alteroperator.html)

<a id="op-d0db681889e4019485562269"></a>
## AlterOperatorClass

`variant` · `sqlparser::ast::Statement::AlterOperatorClass` · sqlparser 0.62.0

```rust
AlterOperatorClass
```

Source: `src/ast/mod.rs:3827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER OPERATOR CLASS
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-alteropclass.html)

<a id="op-b408103e53cc7018f2046cbb"></a>
## AlterOperatorFamily

`variant` · `sqlparser::ast::Statement::AlterOperatorFamily` · sqlparser 0.62.0

```rust
AlterOperatorFamily
```

Source: `src/ast/mod.rs:3822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER OPERATOR FAMILY
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-alteropfamily.html)

<a id="op-547c8f5f086cafe65e0f9ea0"></a>
## AlterPolicy

`variant` · `sqlparser::ast::Statement::AlterPolicy` · sqlparser 0.62.0

```rust
AlterPolicy
```

Source: `src/ast/mod.rs:3841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER POLICY <NAME> ON <TABLE NAME> [<OPERATION>]
```
(Postgresql-specific)

<a id="op-239ac67fab29072005b4636a"></a>
## AlterRole

`variant` · `sqlparser::ast::Statement::AlterRole` · sqlparser 0.62.0

```rust
AlterRole
```

Source: `src/ast/mod.rs:3831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER ROLE
```

<a id="op-9434a1fbe6cd6077f8727a30"></a>
## AlterSchema

`variant` · `sqlparser::ast::Statement::AlterSchema` · sqlparser 0.62.0

```rust
AlterSchema
```

Source: `src/ast/mod.rs:3772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER SCHEMA
```
See [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#alter_schema_collate_statement)

<a id="op-753e11271ec70183563998d6"></a>
## AlterSession

`variant` · `sqlparser::ast::Statement::AlterSession` · sqlparser 0.62.0

```rust
AlterSession
```

Source: `src/ast/mod.rs:3865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER SESSION SET sessionParam
ALTER SESSION UNSET <param_name> [ , <param_name> , ... ]
```
See <https://docs.snowflake.com/en/sql-reference/sql/alter-session>

<a id="op-ac3d08e4a9088c30c40aec02"></a>
## AlterTable

`variant` · `sqlparser::ast::Statement::AlterTable` · sqlparser 0.62.0

```rust
AlterTable
```

Source: `src/ast/mod.rs:3767`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER TABLE
```

<a id="op-92d99d94952bf4d680c4e886"></a>
## AlterType

`variant` · `sqlparser::ast::Statement::AlterType` · sqlparser 0.62.0

```rust
AlterType
```

Source: `src/ast/mod.rs:3807`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER TYPE
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-altertype.html)
```

<a id="op-a400d0db877efb6ceb42bd78"></a>
## AlterUser

`variant` · `sqlparser::ast::Statement::AlterUser` · sqlparser 0.62.0

```rust
AlterUser
```

Source: `src/ast/mod.rs:4918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER USER \[ IF EXISTS \] \[ <name> \]
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/alter-user)

<a id="op-339efd811020d3ba69909067"></a>
## AlterView

`variant` · `sqlparser::ast::Statement::AlterView` · sqlparser 0.62.0

```rust
AlterView
```

Source: `src/ast/mod.rs:3785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ALTER VIEW
```

<a id="op-793a2087326deba93c85972e"></a>
## Analyze

`variant` · `sqlparser::ast::Statement::Analyze` · sqlparser 0.62.0

```rust
Analyze
```

Source: `src/ast/mod.rs:3545`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ANALYZE
```
Analyze (Hive)

<a id="op-90731f49964ab4ef9d660e90"></a>
## Assert

`variant` · `sqlparser::ast::Statement::Assert` · sqlparser 0.62.0

```rust
Assert
```

Source: `src/ast/mod.rs:4498`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ASSERT <condition> [AS <message>]
```

<a id="op-2be2b62d9a6382d4a9c4f6b5"></a>
## AttachDatabase

`variant` · `sqlparser::ast::Statement::AttachDatabase` · sqlparser 0.62.0

```rust
AttachDatabase
```

Source: `src/ast/mod.rs:3875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ATTACH DATABASE 'path/to/file' AS alias
```
(SQLite-specific)

<a id="op-9ab74026fe37f19a073085e0"></a>
## AttachDuckDBDatabase

`variant` · `sqlparser::ast::Statement::AttachDuckDBDatabase` · sqlparser 0.62.0

```rust
AttachDuckDBDatabase
```

Source: `src/ast/mod.rs:3888`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

(DuckDB-specific)
```sql
ATTACH 'sqlite_file.db' AS sqlite_db (READ_ONLY, TYPE SQLITE);
```
See <https://duckdb.org/docs/sql/statements/attach.html>

<a id="op-36cf591a62b8b419f304ef0f"></a>
## Cache

`variant` · `sqlparser::ast::Statement::Cache` · sqlparser 0.62.0

```rust
Cache
```

Source: `src/ast/mod.rs:4655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CACHE [ FLAG ] TABLE <table_name> [ OPTIONS('K1' = 'V1', 'K2' = V2) ] [ AS ] [ <query> ]
```

See [Spark SQL docs] for more details.

[Spark SQL docs]: https://docs.databricks.com/spark/latest/spark-sql/language-manual/sql-ref-syntax-aux-cache-cache-table.html

<a id="op-294ff1f4be4c26ba5569b425"></a>
## Call

`variant` · `sqlparser::ast::Statement::Call` · sqlparser 0.62.0

```rust
Call
```

Source: `src/ast/mod.rs:3605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CALL <function>
```

<a id="op-fa906e2a69b8ff6269f8f9eb"></a>
## Case

`variant` · `sqlparser::ast::Statement::Case` · sqlparser 0.62.0

```rust
Case
```

Source: `src/ast/mod.rs:3595`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CASE` statement.

<a id="op-a2a40cf43f32a122e4173257"></a>
## Close

`variant` · `sqlparser::ast::Statement::Close` · sqlparser 0.62.0

```rust
Close
```

Source: `src/ast/mod.rs:3673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CLOSE
```
Closes the portal underlying an open cursor.

<a id="op-4fd39b7f4e0f72ac2bdc2ce6"></a>
## Comment

`variant` · `sqlparser::ast::Statement::Comment` · sqlparser 0.62.0

```rust
Comment
```

Source: `src/ast/mod.rs:4295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
COMMENT ON ...
```

Note: this is a PostgreSQL-specific statement.

<a id="op-88a1a58ce803ea2d8381a8da"></a>
## Commit

`variant` · `sqlparser::ast::Statement::Commit` · sqlparser 0.62.0

```rust
Commit
```

Source: `src/ast/mod.rs:4315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
COMMIT [ TRANSACTION | WORK ] [ AND [ NO ] CHAIN ]
```
If `end` is false

```sql
END [ TRY | CATCH ]
```
If `end` is true

<a id="op-11bd93dd5010bed64498b159"></a>
## Copy

`variant` · `sqlparser::ast::Statement::Copy` · sqlparser 0.62.0

```rust
Copy
```

Source: `src/ast/mod.rs:3609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
COPY [TO | FROM] ...
```

<a id="op-7517451e85ee565faf1a8dfe"></a>
## CopyIntoSnowflake

`variant` · `sqlparser::ast::Statement::CopyIntoSnowflake` · sqlparser 0.62.0

```rust
CopyIntoSnowflake
```

Source: `src/ast/mod.rs:3634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
COPY INTO <table> | <location>
```
See:
<https://docs.snowflake.com/en/sql-reference/sql/copy-into-table>
<https://docs.snowflake.com/en/sql-reference/sql/copy-into-location>

Copy Into syntax available for Snowflake is different than the one implemented in
Postgres. Although they share common prefix, it is reasonable to implement them
in different enums. This can be refactored later once custom dialects
are allowed to have custom Statements.

<a id="op-e9aab561217ad70fe591afd2"></a>
## CreateCollation

`variant` · `sqlparser::ast::Statement::CreateCollation` · sqlparser 0.62.0

```rust
CreateCollation
```

Source: `src/ast/mod.rs:4014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE COLLATION
```
Note: this is a PostgreSQL-specific statement.
<https://www.postgresql.org/docs/current/sql-createcollation.html>

<a id="op-251b6db49b668ead6782b024"></a>
## CreateConnector

`variant` · `sqlparser::ast::Statement::CreateConnector` · sqlparser 0.62.0

```rust
CreateConnector
```

Source: `src/ast/mod.rs:3748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE CONNECTOR
```
See [Hive](https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362034#LanguageManualDDL-CreateDataConnectorCreateConnector)

<a id="op-3412c0327fb7cc37285693ac"></a>
## CreateDatabase

`variant` · `sqlparser::ast::Statement::CreateDatabase` · sqlparser 0.62.0

```rust
CreateDatabase
```

Source: `src/ast/mod.rs:4378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE DATABASE
```
See:
<https://docs.snowflake.com/en/sql-reference/sql/create-database>

<a id="op-8ed2596db5e348526bb90eae"></a>
## CreateDomain

`variant` · `sqlparser::ast::Statement::CreateDomain` · sqlparser 0.62.0

```rust
CreateDomain
```

Source: `src/ast/mod.rs:4697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CREATE DOMAIN` statement.

<a id="op-256f161f417e2aed46863c37"></a>
## CreateExtension

`variant` · `sqlparser::ast::Statement::CreateExtension` · sqlparser 0.62.0

```rust
CreateExtension
```

Source: `src/ast/mod.rs:4008`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE EXTENSION [ IF NOT EXISTS ] extension_name
    [ WITH ] [ SCHEMA schema_name ]
             [ VERSION version ]
             [ CASCADE ]
```

Note: this is a PostgreSQL-specific statement,

<a id="op-93a71ee268befe396d8e0d06"></a>
## CreateFunction

`variant` · `sqlparser::ast::Statement::CreateFunction` · sqlparser 0.62.0

```rust
CreateFunction
```

Source: `src/ast/mod.rs:4433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE FUNCTION
```

Supported variants:
1. [Hive](https://cwiki.apache.org/confluence/display/hive/languagemanual+ddl#LanguageManualDDL-Create/Drop/ReloadFunction)
2. [PostgreSQL](https://www.postgresql.org/docs/15/sql-createfunction.html)
3. [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_function_statement)
4. [MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql)

<a id="op-859d9dffa489c9b524a57bbf"></a>
## CreateIndex

`variant` · `sqlparser::ast::Statement::CreateIndex` · sqlparser 0.62.0

```rust
CreateIndex
```

Source: `src/ast/mod.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
`CREATE INDEX`
```

<a id="op-86ab2c622c4282ed4e3509a8"></a>
## CreateMacro

`variant` · `sqlparser::ast::Statement::CreateMacro` · sqlparser 0.62.0

```rust
CreateMacro
```

Source: `src/ast/mod.rs:4459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE MACRO
```

Supported variants:
1. [DuckDB](https://duckdb.org/docs/sql/statements/create_macro)

<a id="op-9b9a66342ea929c87d229082"></a>
## CreateOperator

`variant` · `sqlparser::ast::Statement::CreateOperator` · sqlparser 0.62.0

```rust
CreateOperator
```

Source: `src/ast/mod.rs:3753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE OPERATOR
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createoperator.html)

<a id="op-eec4fb913ffebe198a915599"></a>
## CreateOperatorClass

`variant` · `sqlparser::ast::Statement::CreateOperatorClass` · sqlparser 0.62.0

```rust
CreateOperatorClass
```

Source: `src/ast/mod.rs:3763`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE OPERATOR CLASS
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createopclass.html)

<a id="op-2a94caaa39492547e2fcdb44"></a>
## CreateOperatorFamily

`variant` · `sqlparser::ast::Statement::CreateOperatorFamily` · sqlparser 0.62.0

```rust
CreateOperatorFamily
```

Source: `src/ast/mod.rs:3758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE OPERATOR FAMILY
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createopfamily.html)

<a id="op-b6c48b922343806b5096271d"></a>
## CreatePolicy

`variant` · `sqlparser::ast::Statement::CreatePolicy` · sqlparser 0.62.0

```rust
CreatePolicy
```

Source: `src/ast/mod.rs:3743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE POLICY
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createpolicy.html)

<a id="op-5511e0eb57932a7c8c890e7c"></a>
## CreateProcedure

`variant` · `sqlparser::ast::Statement::CreateProcedure` · sqlparser 0.62.0

```rust
CreateProcedure
```

Source: `src/ast/mod.rs:4441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE PROCEDURE
```

<a id="op-6e3aabb3563ae2fac8fa223d"></a>
## CreateRole

`variant` · `sqlparser::ast::Statement::CreateRole` · sqlparser 0.62.0

```rust
CreateRole
```

Source: `src/ast/mod.rs:3716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE ROLE
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createrole.html)

<a id="op-5abe14812e970af2703fedc8"></a>
## CreateSchema

`variant` · `sqlparser::ast::Statement::CreateSchema` · sqlparser 0.62.0

```rust
CreateSchema
```

Source: `src/ast/mod.rs:4335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE SCHEMA
```

<a id="op-b9c5a2514eb3c7db48364072"></a>
## CreateSecret

`variant` · `sqlparser::ast::Statement::CreateSecret` · sqlparser 0.62.0

```rust
CreateSecret
```

Source: `src/ast/mod.rs:3721`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE SECRET
```
See [DuckDB](https://duckdb.org/docs/sql/statements/create_secret.html)

<a id="op-1cd0f238a5244a08dd14f265"></a>
## CreateSequence

`variant` · `sqlparser::ast::Statement::CreateSequence` · sqlparser 0.62.0

```rust
CreateSequence
```

Source: `src/ast/mod.rs:4682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE [ { TEMPORARY | TEMP } ] SEQUENCE [ IF NOT EXISTS ] <sequence_name>
```
Define a new sequence:

<a id="op-2d1a84ea83272b81724b3f5a"></a>
## CreateServer

`variant` · `sqlparser::ast::Statement::CreateServer` · sqlparser 0.62.0

```rust
CreateServer
```

Source: `src/ast/mod.rs:3738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CREATE SERVER` statement.

<a id="op-71fd3ef587ddbb92735058c6"></a>
## CreateStage

`variant` · `sqlparser::ast::Statement::CreateStage` · sqlparser 0.62.0

```rust
CreateStage
```

Source: `src/ast/mod.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE STAGE
```
See <https://docs.snowflake.com/en/sql-reference/sql/create-stage>

<a id="op-66797855036ed510f97e1458"></a>
## CreateTable

`variant` · `sqlparser::ast::Statement::CreateTable` · sqlparser 0.62.0

```rust
CreateTable
```

Source: `src/ast/mod.rs:3692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE TABLE
```

<a id="op-17a2b091ea6c0fe43ac0c8c8"></a>
## CreateTrigger

`variant` · `sqlparser::ast::Statement::CreateTrigger` · sqlparser 0.62.0

```rust
CreateTrigger
```

Source: `src/ast/mod.rs:4435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE TRIGGER statement. See struct [CreateTrigger](../operations/sqlparser.ast.ddl.CreateTrigger.md#op-0d20726f7c1a342ab578756d) for details.

<a id="op-6f1984bca01130739d231819"></a>
## CreateType

`variant` · `sqlparser::ast::Statement::CreateType` · sqlparser 0.62.0

```rust
CreateType
```

Source: `src/ast/mod.rs:4701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE TYPE <name>
```

<a id="op-e733b592c47096e6108750d3"></a>
## CreateUser

`variant` · `sqlparser::ast::Statement::CreateUser` · sqlparser 0.62.0

```rust
CreateUser
```

Source: `src/ast/mod.rs:4913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE [OR REPLACE] USER <user> [IF NOT EXISTS]
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-user)

<a id="op-caf2538649fdd081e855dd47"></a>
## CreateView

`variant` · `sqlparser::ast::Statement::CreateView` · sqlparser 0.62.0

```rust
CreateView
```

Source: `src/ast/mod.rs:3688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE VIEW
```

<a id="op-a7d99ef7aa7d71ab3aa7198c"></a>
## CreateVirtualTable

`variant` · `sqlparser::ast::Statement::CreateVirtualTable` · sqlparser 0.62.0

```rust
CreateVirtualTable
```

Source: `src/ast/mod.rs:3697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CREATE VIRTUAL TABLE .. USING <module_name> (<module_args>)`
```
Sqlite specific statement

<a id="op-f97679655b0cb4227256be36"></a>
## Deallocate

`variant` · `sqlparser::ast::Statement::Deallocate` · sqlparser 0.62.0

```rust
Deallocate
```

Source: `src/ast/mod.rs:4521`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DEALLOCATE [ PREPARE ] { name | ALL }
```

Note: this is a PostgreSQL-specific statement.

<a id="op-34146f1116c1f0e8e4d54c2c"></a>
## Declare

`variant` · `sqlparser::ast::Statement::Declare` · sqlparser 0.62.0

```rust
Declare
```

Source: `src/ast/mod.rs:3996`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DECLARE
```
Declare Cursor Variables

Note: this is a PostgreSQL-specific statement,
but may also compatible with other SQL.

<a id="op-6108ccda98f0df7345c123be"></a>
## Delete

`variant` · `sqlparser::ast::Statement::Delete` · sqlparser 0.62.0

```rust
Delete
```

Source: `src/ast/mod.rs:3684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DELETE
```

<a id="op-f75c18e3651a8a998cb200af"></a>
## Deny

`variant` · `sqlparser::ast::Statement::Deny` · sqlparser 0.62.0

```rust
Deny
```

Source: `src/ast/mod.rs:4511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DENY privileges ON object TO grantees
```

<a id="op-bb437c6648decc7dc060cacf"></a>
## DetachDuckDBDatabase

`variant` · `sqlparser::ast::Statement::DetachDuckDBDatabase` · sqlparser 0.62.0

```rust
DetachDuckDBDatabase
```

Source: `src/ast/mod.rs:3905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

(DuckDB-specific)
```sql
DETACH db_alias;
```
See <https://duckdb.org/docs/sql/statements/attach.html>

<a id="op-d92d2aa0539424c942994c4e"></a>
## Directory

`variant` · `sqlparser::ast::Statement::Directory` · sqlparser 0.62.0

```rust
Directory
```

Source: `src/ast/mod.rs:3582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LOAD DATA from a directory or query source.

<a id="op-faeee603d06506b617770d04"></a>
## Discard

`variant` · `sqlparser::ast::Statement::Discard` · sqlparser 0.62.0

```rust
Discard
```

Source: `src/ast/mod.rs:4082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DISCARD [ ALL | PLANS | SEQUENCES | TEMPORARY | TEMP ]
```

Note: this is a PostgreSQL-specific statement,
but may also compatible with other SQL.

<a id="op-5c1a3404bc54548b742461fe"></a>
## Drop

`variant` · `sqlparser::ast::Statement::Drop` · sqlparser 0.62.0

```rust
Drop
```

Source: `src/ast/mod.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP [TABLE, VIEW, ...]
```

<a id="op-3dc6861281442e96b13b3948"></a>
## DropConnector

`variant` · `sqlparser::ast::Statement::DropConnector` · sqlparser 0.62.0

```rust
DropConnector
```

Source: `src/ast/mod.rs:3983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP CONNECTOR
```
See [Hive](https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362034#LanguageManualDDL-DropConnector)

<a id="op-79eb702642bab9882118b651"></a>
## DropDomain

`variant` · `sqlparser::ast::Statement::DropDomain` · sqlparser 0.62.0

```rust
DropDomain
```

Source: `src/ast/mod.rs:3949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP DOMAIN
```
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-dropdomain.html)

DROP DOMAIN [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]


<a id="op-f5e3dc6b49db3cd6bbf1ff47"></a>
## DropExtension

`variant` · `sqlparser::ast::Statement::DropExtension` · sqlparser 0.62.0

```rust
DropExtension
```

Source: `src/ast/mod.rs:4020`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]
```
Note: this is a PostgreSQL-specific statement.
<https://www.postgresql.org/docs/current/sql-dropextension.html>

<a id="op-bceced16054080e3c3c5429f"></a>
## DropFunction

`variant` · `sqlparser::ast::Statement::DropFunction` · sqlparser 0.62.0

```rust
DropFunction
```

Source: `src/ast/mod.rs:3941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP FUNCTION
```

<a id="op-751fbc98a8130398200fafec"></a>
## DropOperator

`variant` · `sqlparser::ast::Statement::DropOperator` · sqlparser 0.62.0

```rust
DropOperator
```

Source: `src/ast/mod.rs:4026`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP OPERATOR [ IF EXISTS ] name ( { left_type | NONE } , right_type ) [, ...] [ CASCADE | RESTRICT ]
```
Note: this is a PostgreSQL-specific statement.
<https://www.postgresql.org/docs/current/sql-dropoperator.html>

<a id="op-7ea95a0b0fdfa0420f395d4d"></a>
## DropOperatorClass

`variant` · `sqlparser::ast::Statement::DropOperatorClass` · sqlparser 0.62.0

```rust
DropOperatorClass
```

Source: `src/ast/mod.rs:4038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP OPERATOR CLASS [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
```
Note: this is a PostgreSQL-specific statement.
<https://www.postgresql.org/docs/current/sql-dropopclass.html>

<a id="op-41986f97f33bde7fb9288349"></a>
## DropOperatorFamily

`variant` · `sqlparser::ast::Statement::DropOperatorFamily` · sqlparser 0.62.0

```rust
DropOperatorFamily
```

Source: `src/ast/mod.rs:4032`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP OPERATOR FAMILY [ IF EXISTS ] name USING index_method [ CASCADE | RESTRICT ]
```
Note: this is a PostgreSQL-specific statement.
<https://www.postgresql.org/docs/current/sql-dropopfamily.html>

<a id="op-d82117ff42ced67c2b43ec89"></a>
## DropPolicy

`variant` · `sqlparser::ast::Statement::DropPolicy` · sqlparser 0.62.0

```rust
DropPolicy
```

Source: `src/ast/mod.rs:3978`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
 DROP POLICY
 ```
 See [PostgreSQL](https://www.postgresql.org/docs/current/sql-droppolicy.html)

<a id="op-b5ac726fbf2fdffc06c500a7"></a>
## DropProcedure

`variant` · `sqlparser::ast::Statement::DropProcedure` · sqlparser 0.62.0

```rust
DropProcedure
```

Source: `src/ast/mod.rs:3953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP PROCEDURE
```

<a id="op-a87323354df84ff8eb813fc2"></a>
## DropSecret

`variant` · `sqlparser::ast::Statement::DropSecret` · sqlparser 0.62.0

```rust
DropSecret
```

Source: `src/ast/mod.rs:3964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
DROP SECRET
```

<a id="op-3adf16f42fa20120a607557f"></a>
## DropTrigger

`variant` · `sqlparser::ast::Statement::DropTrigger` · sqlparser 0.62.0

```rust
DropTrigger
```

Source: `src/ast/mod.rs:4437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DROP TRIGGER statement. See struct [DropTrigger](../operations/sqlparser.ast.ddl.DropTrigger.md#op-9596571eea72374765c7d35c) for details.

<a id="op-439cb5ea234d77b573f00933"></a>
## Execute

`variant` · `sqlparser::ast::Statement::Execute` · sqlparser 0.62.0

```rust
Execute
```

Source: `src/ast/mod.rs:4535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
An `EXECUTE` statement
```

Postgres: <https://www.postgresql.org/docs/current/sql-execute.html>
MSSQL: <https://learn.microsoft.com/en-us/sql/relational-databases/stored-procedures/execute-a-stored-procedure>
BigQuery: <https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#execute_immediate>
Snowflake: <https://docs.snowflake.com/en/sql-reference/sql/execute-immediate>

<a id="op-b615f46f6c2c5ff3893803b5"></a>
## Explain

`variant` · `sqlparser::ast::Statement::Explain` · sqlparser 0.62.0

```rust
Explain
```

Source: `src/ast/mod.rs:4602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
[EXPLAIN | DESC | DESCRIBE]  <statement>
```

<a id="op-23ca82cf92faa87ec27fc2e6"></a>
## ExplainTable

`variant` · `sqlparser::ast::Statement::ExplainTable` · sqlparser 0.62.0

```rust
ExplainTable
```

Source: `src/ast/mod.rs:4585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
[EXPLAIN | DESC | DESCRIBE] TABLE
```
Note: this is a MySQL-specific statement. See <https://dev.mysql.com/doc/refman/8.0/en/explain.html>

<a id="op-d5bbfe746fd3aef2dde05828"></a>
## ExportData

`variant` · `sqlparser::ast::Statement::ExportData` · sqlparser 0.62.0

```rust
ExportData
```

Source: `src/ast/mod.rs:4908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Export data statement

Example:
```sql
EXPORT DATA OPTIONS(uri='gs://bucket/folder/*', format='PARQUET', overwrite=true) AS
SELECT field1, field2 FROM mydataset.table1 ORDER BY field1 LIMIT 10
```
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/export-statements)

<a id="op-a6216658d71b715c1845ec77"></a>
## Fetch

`variant` · `sqlparser::ast::Statement::Fetch` · sqlparser 0.62.0

```rust
Fetch
```

Source: `src/ast/mod.rs:4046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
FETCH
```
Retrieve rows from a query using a cursor

Note: this is a PostgreSQL-specific statement,
but may also compatible with other SQL.

<a id="op-936cf7530a19367acae86066"></a>
## Flush

`variant` · `sqlparser::ast::Statement::Flush` · sqlparser 0.62.0

```rust
Flush
```

Source: `src/ast/mod.rs:4062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
FLUSH [NO_WRITE_TO_BINLOG | LOCAL] flush_option [, flush_option] ... | tables_option
```

Note: this is a Mysql-specific statement,
but may also compatible with other SQL.

<a id="op-6f8345e2b47f45b0e36549e2"></a>
## Grant

`variant` · `sqlparser::ast::Statement::Grant` · sqlparser 0.62.0

```rust
Grant
```

Source: `src/ast/mod.rs:4507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
GRANT privileges ON objects TO grantees
```

<a id="op-91544b25fd3e9a33e5de011c"></a>
## If

`variant` · `sqlparser::ast::Statement::If` · sqlparser 0.62.0

```rust
If
```

Source: `src/ast/mod.rs:3597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `IF` statement.

<a id="op-a05ad82ade782a0aa427f228"></a>
## Insert

`variant` · `sqlparser::ast::Statement::Insert` · sqlparser 0.62.0

```rust
Insert
```

Source: `src/ast/mod.rs:3565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
INSERT
```

<a id="op-d99c17fc49d259266ce4de39"></a>
## Install

`variant` · `sqlparser::ast::Statement::Install` · sqlparser 0.62.0

```rust
Install
```

Source: `src/ast/mod.rs:3569`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
INSTALL
```

<a id="op-9f7f61d658302ef4a1588250"></a>
## Kill

`variant` · `sqlparser::ast::Statement::Kill` · sqlparser 0.62.0

```rust
Kill
```

Source: `src/ast/mod.rs:4574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
KILL [CONNECTION | QUERY | MUTATION]
```

See <https://clickhouse.com/docs/en/sql-reference/statements/kill/>
See <https://dev.mysql.com/doc/refman/8.0/en/kill.html>

<a id="op-f7c93500e9b81e96406d0803"></a>
## LISTEN

`variant` · `sqlparser::ast::Statement::LISTEN` · sqlparser 0.62.0

```rust
LISTEN
```

Source: `src/ast/mod.rs:4803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
LISTEN
```
listen for a notification channel

See Postgres <https://www.postgresql.org/docs/current/sql-listen.html>

<a id="op-6f7e33fdd494572cd21ef8ab"></a>
## List

`variant` · `sqlparser::ast::Statement::List` · sqlparser 0.62.0

```rust
List
```

Source: `src/ast/mod.rs:4860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `LIST`
See: <https://docs.snowflake.com/en/sql-reference/sql/list>

<a id="op-024ed0355f83f0a376f92ccd"></a>
## Load

`variant` · `sqlparser::ast::Statement::Load` · sqlparser 0.62.0

```rust
Load
```

Source: `src/ast/mod.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
LOAD
```

<a id="op-2919c685058b62355385ba04"></a>
## LoadData

`variant` · `sqlparser::ast::Statement::LoadData` · sqlparser 0.62.0

```rust
LoadData
```

Source: `src/ast/mod.rs:4837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
LOAD DATA [LOCAL] INPATH 'filepath' [OVERWRITE] INTO TABLE tablename
[PARTITION (partcol1=val1, partcol2=val2 ...)]
[INPUTFORMAT 'inputformat' SERDE 'serde']
```
Loading files into tables

See Hive <https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362036#LanguageManualDML-Loadingfilesintotables>

<a id="op-0808eed7149093e56214aabf"></a>
## Lock

`variant` · `sqlparser::ast::Statement::Lock` · sqlparser 0.62.0

```rust
Lock
```

Source: `src/ast/mod.rs:4723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
LOCK [ TABLE ] [ ONLY ] name [ * ] [, ...] [ IN lockmode MODE ] [ NOWAIT ]
```

See <https://www.postgresql.org/docs/current/sql-lock.html>

<a id="op-8de8fb216dd62a58b3840eb1"></a>
## LockTables

`variant` · `sqlparser::ast::Statement::LockTables` · sqlparser 0.62.0

```rust
LockTables
```

Source: `src/ast/mod.rs:4728`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
LOCK TABLES <table_name> [READ [LOCAL] | [LOW_PRIORITY] WRITE]
```
Note: this is a MySQL-specific statement. See <https://dev.mysql.com/doc/refman/8.0/en/lock-tables.html>

<a id="op-f144cca9b42d48004945b62d"></a>
## Merge

`variant` · `sqlparser::ast::Statement::Merge` · sqlparser 0.62.0

```rust
Merge
```

Source: `src/ast/mod.rs:4647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `MERGE` statement.

```sql
MERGE INTO <target_table> USING <source> ON <join_expr> { matchedClause | notMatchedClause } [ ... ]
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[MSSQL](https://learn.microsoft.com/en-us/sql/t-sql/statements/merge-transact-sql?view=sql-server-ver16)

<a id="op-996ae33292db54757ed0bbd6"></a>
## Msck

`variant` · `sqlparser::ast::Statement::Msck` · sqlparser 0.62.0

```rust
Msck
```

Source: `src/ast/mod.rs:3557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
MSCK
```
Msck (Hive)

<a id="op-52c9dc18d8e690c554924ee9"></a>
## NOTIFY

`variant` · `sqlparser::ast::Statement::NOTIFY` · sqlparser 0.62.0

```rust
NOTIFY
```

Source: `src/ast/mod.rs:4823`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
NOTIFY channel [ , payload ]
```
send a notification event together with an optional "payload" string to channel

See Postgres <https://www.postgresql.org/docs/current/sql-notify.html>

<a id="op-441970ac6ef6af1522a5a6c7"></a>
## Open

`variant` · `sqlparser::ast::Statement::Open` · sqlparser 0.62.0

```rust
Open
```

Source: `src/ast/mod.rs:3668`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
OPEN cursor_name
```
Opens a cursor.

<a id="op-e4f16fde96e7ed38095d9cc2"></a>
## OptimizeTable

`variant` · `sqlparser::ast::Statement::OptimizeTable` · sqlparser 0.62.0

```rust
OptimizeTable
```

Source: `src/ast/mod.rs:4773`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse:
```sql
OPTIMIZE TABLE [db.]name [ON CLUSTER cluster] [PARTITION partition | PARTITION ID 'partition_id'] [FINAL] [DEDUPLICATE [BY expression]]
```
See ClickHouse <https://clickhouse.com/docs/en/sql-reference/statements/optimize>

Databricks:
```sql
OPTIMIZE table_name [WHERE predicate] [ZORDER BY (col_name1 [, ...])]
```
See Databricks <https://docs.databricks.com/en/sql/language-manual/delta-optimize.html>

<a id="op-32e0c161ebcf1a1811953ed9"></a>
## Pragma

`variant` · `sqlparser::ast::Statement::Pragma` · sqlparser 0.62.0

```rust
Pragma
```

Source: `src/ast/mod.rs:4710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
PRAGMA <schema-name>.<pragma-name> = <pragma-value>
```

<a id="op-0e16acef3dc32a6591242708"></a>
## Prepare

`variant` · `sqlparser::ast::Statement::Prepare` · sqlparser 0.62.0

```rust
Prepare
```

Source: `src/ast/mod.rs:4560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
PREPARE name [ ( data_type [, ...] ) ] AS statement
```

Note: this is a PostgreSQL-specific statement.

<a id="op-2d27a61dac578c1f7a588d2f"></a>
## Print

`variant` · `sqlparser::ast::Statement::Print` · sqlparser 0.62.0

```rust
Print
```

Source: `src/ast/mod.rs:4889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
PRINT msg_str | @local_variable | string_expr
```

See: <https://learn.microsoft.com/en-us/sql/t-sql/statements/print-transact-sql>

<a id="op-4e80d56a55278b70d916623a"></a>
## Query

`variant` · `sqlparser::ast::Statement::Query` · sqlparser 0.62.0

```rust
Query
```

Source: `src/ast/mod.rs:3561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SELECT
```

<a id="op-da42a0db5e85535e8078f00b"></a>
## RaisError

`variant` · `sqlparser::ast::Statement::RaisError` · sqlparser 0.62.0

```rust
RaisError
```

Source: `src/ast/mod.rs:4870`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RaiseError (MSSQL)
RAISERROR ( { msg_id | msg_str | @local_variable }
{ , severity , state }
[ , argument [ , ...n ] ] )
[ WITH option [ , ...n ] ]
See <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/raiserror-transact-sql?view=sql-server-ver16>

<a id="op-bfa924658e1fc1460d587b9f"></a>
## Raise

`variant` · `sqlparser::ast::Statement::Raise` · sqlparser 0.62.0

```rust
Raise
```

Source: `src/ast/mod.rs:3601`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `RAISE` statement.

<a id="op-c93e3e271d00d4c8e1408a12"></a>
## ReleaseSavepoint

`variant` · `sqlparser::ast::Statement::ReleaseSavepoint` · sqlparser 0.62.0

```rust
ReleaseSavepoint
```

Source: `src/ast/mod.rs:4635`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
RELEASE [ SAVEPOINT ] savepoint_name
```

<a id="op-5b9cd754b696d657dd4f896a"></a>
## Remove

`variant` · `sqlparser::ast::Statement::Remove` · sqlparser 0.62.0

```rust
Remove
```

Source: `src/ast/mod.rs:4863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake `REMOVE`
See: <https://docs.snowflake.com/en/sql-reference/sql/remove>

<a id="op-6f4f036d679d804ccedfb6ab"></a>
## RenameTable

`variant` · `sqlparser::ast::Statement::RenameTable` · sqlparser 0.62.0

```rust
RenameTable
```

Source: `src/ast/mod.rs:4857`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
Rename TABLE tbl_name TO new_tbl_name[, tbl_name2 TO new_tbl_name2] ...
```
Renames one or more tables

See Mysql <https://dev.mysql.com/doc/refman/9.1/en/rename-table.html>

<a id="op-9081c568c1df97248c4f8377"></a>
## Reset

`variant` · `sqlparser::ast::Statement::Reset` · sqlparser 0.62.0

```rust
Reset
```

Source: `src/ast/mod.rs:4933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Restore the value of a run-time parameter to the default value.

```sql
RESET configuration_parameter;
RESET ALL;
```
[PostgreSQL](https://www.postgresql.org/docs/current/sql-reset.html)

<a id="op-db0db45799115881752cbe67"></a>
## Return

`variant` · `sqlparser::ast::Statement::Return` · sqlparser 0.62.0

```rust
Return
```

Source: `src/ast/mod.rs:4899`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
RETURN [ expression ]
```

See [ReturnStatement](../operations/sqlparser.ast.ReturnStatement.md#op-7da02e9c4e773dfbb56635e9)

<a id="op-bc9e3e9dc3a65a40576bf90b"></a>
## Revoke

`variant` · `sqlparser::ast::Statement::Revoke` · sqlparser 0.62.0

```rust
Revoke
```

Source: `src/ast/mod.rs:4515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
REVOKE privileges ON objects FROM grantees
```

<a id="op-ed92edf997a2985d7296ffbf"></a>
## Rollback

`variant` · `sqlparser::ast::Statement::Rollback` · sqlparser 0.62.0

```rust
Rollback
```

Source: `src/ast/mod.rs:4326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
ROLLBACK [ TRANSACTION | WORK ] [ AND [ NO ] CHAIN ] [ TO [ SAVEPOINT ] savepoint_name ]
```

<a id="op-16c91e26d889cbd43055c668"></a>
## Savepoint

`variant` · `sqlparser::ast::Statement::Savepoint` · sqlparser 0.62.0

```rust
Savepoint
```

Source: `src/ast/mod.rs:4628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SAVEPOINT
```
Define a new savepoint within the current transaction

<a id="op-72127d18e804ab2c82a285d9"></a>
## Set

`variant` · `sqlparser::ast::Statement::Set` · sqlparser 0.62.0

```rust
Set
```

Source: `src/ast/mod.rs:3547`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SET` statements (session, transaction, timezone, etc.).

<a id="op-39306127fca40543be587435"></a>
## ShowCatalogs

`variant` · `sqlparser::ast::Statement::ShowCatalogs` · sqlparser 0.62.0

```rust
ShowCatalogs
```

Source: `src/ast/mod.rs:4153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW CATALOGS
```

<a id="op-a6851fa0a921851bb44692eb"></a>
## ShowCharset

`variant` · `sqlparser::ast::Statement::ShowCharset` · sqlparser 0.62.0

```rust
ShowCharset
```

Source: `src/ast/mod.rs:4198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Show the available character sets (alias `CHARSET`).

<a id="op-58a7d5c3e93ad9d8a3f75486"></a>
## ShowCollation

`variant` · `sqlparser::ast::Statement::ShowCollation` · sqlparser 0.62.0

```rust
ShowCollation
```

Source: `src/ast/mod.rs:4238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW COLLATION
```

Note: this is a MySQL-specific statement.

<a id="op-de3b2a310ebdf7334d06d85b"></a>
## ShowColumns

`variant` · `sqlparser::ast::Statement::ShowColumns` · sqlparser 0.62.0

```rust
ShowColumns
```

Source: `src/ast/mod.rs:4142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW COLUMNS
```

<a id="op-b4b28527a59144b8780ee89b"></a>
## ShowCreate

`variant` · `sqlparser::ast::Statement::ShowCreate` · sqlparser 0.62.0

```rust
ShowCreate
```

Source: `src/ast/mod.rs:4133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW CREATE TABLE
```

Note: this is a MySQL-specific statement.

<a id="op-48bd04cb87d17517f8852b32"></a>
## ShowDatabases

`variant` · `sqlparser::ast::Statement::ShowDatabases` · sqlparser 0.62.0

```rust
ShowDatabases
```

Source: `src/ast/mod.rs:4164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW DATABASES
```

<a id="op-2ab63df5fb1d7f19313d2a3d"></a>
## ShowFunctions

`variant` · `sqlparser::ast::Statement::ShowFunctions` · sqlparser 0.62.0

```rust
ShowFunctions
```

Source: `src/ast/mod.rs:4089`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHOW FUNCTIONS`

Note: this is a Presto-specific statement.

<a id="op-e9a407abda4590f2fd0c4736"></a>
## ShowObjects

`variant` · `sqlparser::ast::Statement::ShowObjects` · sqlparser 0.62.0

```rust
ShowObjects
```

Source: `src/ast/mod.rs:4204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW OBJECTS LIKE 'line%' IN mydb.public
```
Snowflake-specific statement
<https://docs.snowflake.com/en/sql-reference/sql/show-objects>

<a id="op-05bd21598e58a9c7c5057948"></a>
## ShowProcessList

`variant` · `sqlparser::ast::Statement::ShowProcessList` · sqlparser 0.62.0

```rust
ShowProcessList
```

Source: `src/ast/mod.rs:4177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW [FULL] PROCESSLIST
```

Note: this is a MySQL-specific statement.

<a id="op-97b4a369cfc8ea5323350d37"></a>
## ShowSchemas

`variant` · `sqlparser::ast::Statement::ShowSchemas` · sqlparser 0.62.0

```rust
ShowSchemas
```

Source: `src/ast/mod.rs:4184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW SCHEMAS
```

<a id="op-77473aa01d8a42991b0e4ddd"></a>
## ShowStatus

`variant` · `sqlparser::ast::Statement::ShowStatus` · sqlparser 0.62.0

```rust
ShowStatus
```

Source: `src/ast/mod.rs:4107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW [GLOBAL | SESSION] STATUS [LIKE 'pattern' | WHERE expr]
```

Note: this is a MySQL-specific statement.

<a id="op-59981fee447ad9a07ed3fe59"></a>
## ShowTables

`variant` · `sqlparser::ast::Statement::ShowTables` · sqlparser 0.62.0

```rust
ShowTables
```

Source: `src/ast/mod.rs:4208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW TABLES
```

<a id="op-617ed9af19e768e6451477cb"></a>
## ShowVariable

`variant` · `sqlparser::ast::Statement::ShowVariable` · sqlparser 0.62.0

```rust
ShowVariable
```

Source: `src/ast/mod.rs:4098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW <variable>
```

Note: this is a PostgreSQL-specific statement.

<a id="op-67b94dccebaea7df32e0fff3"></a>
## ShowVariables

`variant` · `sqlparser::ast::Statement::ShowVariables` · sqlparser 0.62.0

```rust
ShowVariables
```

Source: `src/ast/mod.rs:4120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW VARIABLES
```

Note: this is a MySQL-specific statement.

<a id="op-2ec5ce0dbb689044c8125f61"></a>
## ShowViews

`variant` · `sqlparser::ast::Statement::ShowViews` · sqlparser 0.62.0

```rust
ShowViews
```

Source: `src/ast/mod.rs:4225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SHOW VIEWS
```

<a id="op-2950c361c288a395362dbf2f"></a>
## StartTransaction

`variant` · `sqlparser::ast::Statement::StartTransaction` · sqlparser 0.62.0

```rust
StartTransaction
```

Source: `src/ast/mod.rs:4255`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
START  [ TRANSACTION | WORK ] | START TRANSACTION } ...
```
If `begin` is false.

```sql
`BEGIN  [ TRANSACTION | WORK ] | START TRANSACTION } ...`
```
If `begin` is true

<a id="op-649bdc33fd7832279e6a10a1"></a>
## Throw

`variant` · `sqlparser::ast::Statement::Throw` · sqlparser 0.62.0

```rust
Throw
```

Source: `src/ast/mod.rs:4883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A MSSQL `THROW` statement.

<a id="op-a9420316877be47095fdca83"></a>
## Truncate

`variant` · `sqlparser::ast::Statement::Truncate` · sqlparser 0.62.0

```rust
Truncate
```

Source: `src/ast/mod.rs:3552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
TRUNCATE
```
Truncate (Hive)

<a id="op-f0b56ed3b24a2c42a4e16a1d"></a>
## UNCache

`variant` · `sqlparser::ast::Statement::UNCache` · sqlparser 0.62.0

```rust
UNCache
```

Source: `src/ast/mod.rs:4671`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
UNCACHE TABLE [ IF EXISTS ]  <table_name>
```

<a id="op-3d0669dd706cf0d10043760e"></a>
## UNLISTEN

`variant` · `sqlparser::ast::Statement::UNLISTEN` · sqlparser 0.62.0

```rust
UNLISTEN
```

Source: `src/ast/mod.rs:4813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
UNLISTEN
```
stop listening for a notification

See Postgres <https://www.postgresql.org/docs/current/sql-unlisten.html>

<a id="op-859636d38f8151a19b3ae037"></a>
## Unload

`variant` · `sqlparser::ast::Statement::Unload` · sqlparser 0.62.0

```rust
Unload
```

Source: `src/ast/mod.rs:4748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unloads the result of a query to file

[Athena](https://docs.aws.amazon.com/athena/latest/ug/unload.html):
```sql
UNLOAD(statement) TO <destination> [ WITH options ]
```

[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_UNLOAD.html):
```sql
UNLOAD('statement') TO <destination> [ OPTIONS ]
```

<a id="op-dcbf2118502c372365387e31"></a>
## UnlockTables

`variant` · `sqlparser::ast::Statement::UnlockTables` · sqlparser 0.62.0

```rust
UnlockTables
```

Source: `src/ast/mod.rs:4736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
UNLOCK TABLES
```
Note: this is a MySQL-specific statement. See <https://dev.mysql.com/doc/refman/8.0/en/lock-tables.html>

<a id="op-a4156d3ec8bdb87d800efd52"></a>
## Update

`variant` · `sqlparser::ast::Statement::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/mod.rs:3680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
UPDATE
```

<a id="op-9876ba517c843458e6934889"></a>
## Use

`variant` · `sqlparser::ast::Statement::Use` · sqlparser 0.62.0

```rust
Use
```

Source: `src/ast/mod.rs:4245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
`USE ...`
```

<a id="op-eb63c3703720042660fb4532"></a>
## Vacuum

`variant` · `sqlparser::ast::Statement::Vacuum` · sqlparser 0.62.0

```rust
Vacuum
```

Source: `src/ast/mod.rs:4925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Re-sorts rows and reclaims space in either a specified table or all tables in the current database

```sql
VACUUM tbl
```
[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_VACUUM_command.html)

<a id="op-50f07b71e60b0de48141f2dc"></a>
## WaitFor

`variant` · `sqlparser::ast::Statement::WaitFor` · sqlparser 0.62.0

```rust
WaitFor
```

Source: `src/ast/mod.rs:4893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL `WAITFOR` statement.

See: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/waitfor-transact-sql>

<a id="op-a61a71e42600cd57ed03ba56"></a>
## While

`variant` · `sqlparser::ast::Statement::While` · sqlparser 0.62.0

```rust
While
```

Source: `src/ast/mod.rs:3599`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `WHILE` statement.

<a id="op-ee89c9197862eef3f9096b4e"></a>
## clone

`function` · `sqlparser::ast::Statement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Statement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3533, 17], "end": [3533, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c709e982a4063d3d3c105e1a"></a>
## cmp

`function` · `sqlparser::ast::Statement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Statement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3533, 51], "end": [3533, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86f89af0970c3e66b2fd1a61"></a>
## deserialize

`function` · `sqlparser::ast::Statement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 49], "end": [3534, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c66ffd414f46684e3313374"></a>
## eq

`function` · `sqlparser::ast::Statement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Statement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3533, 24], "end": [3533, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89182ddf2785d7add9e5f7c1"></a>
## fmt

`function` · `sqlparser::ast::Statement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5008, 1], "end": [6391, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:5034`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Formats a SQL statement with support for pretty printing.

When using the alternate flag (`{:#}`), the statement will be formatted with proper
indentation and line breaks. For example:

```
# use sqlparser::dialect::GenericDialect;
# use sqlparser::parser::Parser;
let sql = "SELECT a, b FROM table_1";
let ast = Parser::parse_sql(&GenericDialect, sql).unwrap();

// Regular formatting
assert_eq!(format!("{}", ast[0]), "SELECT a, b FROM table_1");

// Pretty printing
assert_eq!(format!("{:#}", ast[0]),
r#"SELECT
  a,
  b
FROM
  table_1"#);
```

<a id="op-98ee35997ec55a205d4eca88"></a>
## fmt

`function` · `sqlparser::ast::Statement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3533, 10], "end": [3533, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0083bb2b918912e15a79d0c1"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateUser) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12363, 1], "end": [12367, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-060495353a00e73efb3abaee"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(r: Vec<RenameTable>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12339, 1], "end": [12343, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e834309c8db6d01341e09c1"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterOperatorFamily) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12237, 1], "end": [12241, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorFamily", "path": "AlterOperatorFamily"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14d6eb78ca402206619be177"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(s: ShowCharset) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12267, 1], "end": [12271, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCharset", "path": "ShowCharset"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-167bef8462911be8df61d9b6"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(v: Grant) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "crate::ast::Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [482, 1], "end": [486, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Grant", "path": "Grant"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/dcl.rs:483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a85e058dae4cf1fd4992a91"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(f: Function) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12147, 1], "end": [12151, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Function", "path": "Function"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3295011d4ebefb3d74f71fe6"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: DropOperatorFamily) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12309, 1], "end": [12313, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorFamily", "path": "DropOperatorFamily"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-353d2d45afb253d367a28ccc"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(s: Set) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12045, 1], "end": [12049, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35cd6a5e3b092f8cfacc1f9c"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(v: DropPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "crate::ast::Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5718, 1], "end": [5722, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropPolicy", "path": "DropPolicy"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:5719`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35e134c6b1db2f882e0ee52b"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(r: ReturnStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12351, 1], "end": [12355, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a1523b715261f92166d3335"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(v: Revoke) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "crate::ast::Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [524, 1], "end": [528, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Revoke", "path": "Revoke"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/dcl.rs:525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e7aecc649b06d247acdb392"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(w: WhileStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12129, 1], "end": [12133, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::WhileStatement", "path": "WhileStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f45755e4c5d6618f8d0030a"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterSchema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12207, 1], "end": [12211, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f9969c24fbfce3466cee766"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterCollation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12225, 1], "end": [12229, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollation", "path": "AlterCollation"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-427105461325e0aa4f8e9087"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(q: Query) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12051, 1], "end": [12055, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-432efc6e19648bf4be8f79ea"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: DropTrigger) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12297, 1], "end": [12301, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropTrigger", "path": "DropTrigger"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a34bdebeb7146f0793293f2"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateFunction) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12285, 1], "end": [12289, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateFunction", "path": "CreateFunction"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c7f70924869befb97e57a3b"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(q: Box<Query>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12057, 1], "end": [12061, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}}], "constraints": []}}, "id": "alloc::boxed::Box", "path": "Box"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12058`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cbce156d736ebf6d3051214"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(s: ShowObjects) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12273, 1], "end": [12277, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fffbfcb66bac3b800294dea"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(i: IfStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12123, 1], "end": [12127, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::IfStatement", "path": "IfStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-502a3974bae4faf682446881"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(r: ResetStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12375, 1], "end": [12379, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ResetStatement", "path": "ResetStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12376`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50951c11eac7a1fb2ed46ba1"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(v: CreatePolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "crate::ast::Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5712, 1], "end": [5716, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicy", "path": "CreatePolicy"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:5713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52c650e429ec442d3ff752f0"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateTrigger) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12291, 1], "end": [12295, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTrigger", "path": "CreateTrigger"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56d1fcc7e6186d366c6ba9df"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(u: Update) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12069, 1], "end": [12073, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12070`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aaef062d82d94389cff96a9"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterUser) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12255, 1], "end": [12259, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::AlterUser", "path": "AlterUser"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c056977ef4126117e215621"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: DropOperatorClass) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12315, 1], "end": [12319, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c4a5964b4c32b151d206e8c"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateTable) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12165, 1], "end": [12169, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateTable", "path": "CreateTable"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ef0bf7139a7e52819911b92"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: DenyStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12321, 1], "end": [12325, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::DenyStatement", "path": "DenyStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f024da016451bc2bc7e9cc7"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(at: AlterTable) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12087, 1], "end": [12091, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTable", "path": "AlterTable"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64f779e5821202ea6b9a66ab"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(t: ThrowStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12141, 1], "end": [12145, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ThrowStatement", "path": "ThrowStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c351de92fc264bd0913e82e"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(df: DropFunction) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12093, 1], "end": [12097, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12094`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e241c56cb14da9c34bbb4b8"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(o: OpenStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12153, 1], "end": [12157, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f6ba0329d4f65343754330f"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateOperator) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12189, 1], "end": [12193, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperator", "path": "CreateOperator"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72db1d10cec6809dc7085b0f"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(ce: CreateExtension) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12099, 1], "end": [12103, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72fe67ab998b7efcb21cc7fd"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(e: ExportData) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12357, 1], "end": [12361, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12358`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ba4c531b4fc14686339b542"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterFunction) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12213, 1], "end": [12217, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80c48ff5ab0997934c67410a"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateIndex) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12171, 1], "end": [12175, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-836d8cc1bc571449c687e673"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(de: DropExtension) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12111, 1], "end": [12115, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c5f3361278f97f590590dfc"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateOperatorClass) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12201, 1], "end": [12205, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorClass", "path": "CreateOperatorClass"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-937cbc5acad151e47b66c1a3"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterOperatorClass) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12243, 1], "end": [12247, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperatorClass", "path": "AlterOperatorClass"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97a1b81a3696d3814b2e4f59"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12219, 1], "end": [12223, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-994ad846376e640bad1855f7"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(cv: CreateView) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12075, 1], "end": [12079, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12076`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cc1ad53cd3c0dae0c9562ac"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(m: Merge) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12249, 1], "end": [12253, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Merge", "path": "Merge"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e553ddb10e3db00adcfec2"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: DropDomain) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12261, 1], "end": [12265, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4f4b43beae837499fc57175"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(u: Use) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12279, 1], "end": [12283, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a97390d4a82394bca183bf55"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CaseStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12117, 1], "end": [12121, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::CaseStatement", "path": "CaseStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad294d0b211064f46709a4a7"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: DropOperator) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12303, 1], "end": [12307, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperator", "path": "DropOperator"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b42459100769776bec64619d"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(v: VacuumStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12369, 1], "end": [12373, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::VacuumStatement", "path": "VacuumStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7a0b968fbd7a72eb78bc663"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateConnector) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12183, 1], "end": [12187, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateConnector", "path": "CreateConnector"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8234299969dc3f99bdad0c9"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(a: AlterOperator) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12231, 1], "end": [12235, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterOperator", "path": "AlterOperator"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1f8e7728cf8f5986e3a53ab"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateDomain) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12327, 1], "end": [12331, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateDomain", "path": "CreateDomain"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8b0d094f5465804c53f7c3f"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(v: AlterPolicy) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "crate::ast::Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5755, 1], "end": [5759, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterPolicy", "path": "AlterPolicy"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:5756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb15fd589b0a320e1b9d6d58"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(r: RaiseStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12135, 1], "end": [12139, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatement", "path": "RaiseStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc6af7b9fbd24a1d3489e328"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateServerStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12177, 1], "end": [12181, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateServerStatement", "path": "CreateServerStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d498a7c9b8c3b62a3825763e"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateOperatorFamily) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12195, 1], "end": [12199, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateOperatorFamily", "path": "CreateOperatorFamily"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6792de04df1d7a954d81eae"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(d: Delete) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12159, 1], "end": [12163, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc37744ee9e63f57cae29f5e"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(i: Insert) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12063, 1], "end": [12067, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12064`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcab46a04aa1b9b20f7951d0"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(c: CreateCollation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12105, 1], "end": [12109, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1d6c87900152cbd474d4849"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(cr: CreateRole) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12081, 1], "end": [12085, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee0fb81b0a098151b2e5667a"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(p: PrintStatement) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12345, 1], "end": [12349, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::PrintStatement", "path": "PrintStatement"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f01f43563d9f91ce750ad414"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(lock: Lock) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4948, 1], "end": [4952, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:4949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f41b0865acbe1314ef95c213"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(r: RenameTable) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12333, 1], "end": [12337, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:12334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f572b92ecdd0c4bfba7c0f64"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(msck: ddl::Msck) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4954, 1], "end": [4958, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Msck", "path": "Msck"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:4955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa80d28629ecf6e5f81463a1"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(analyze: Analyze) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4936, 1], "end": [4940, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:4937`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbec0797e2a88868208580f5"></a>
## from

`function` · `sqlparser::ast::Statement::from` · sqlparser 0.62.0

```rust
fn from(truncate: ddl::Truncate) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4942, 1], "end": [4946, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:4943`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a383c9daeb3c57c4b87d6d4"></a>
## hash

`function` · `sqlparser::ast::Statement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3533, 56], "end": [3533, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dbd26708fb1b74e3c5815ab"></a>
## partial_cmp

`function` · `sqlparser::ast::Statement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Statement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3533, 35], "end": [3533, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3533`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7381caf33e8aac03b184e4aa"></a>
## serialize

`function` · `sqlparser::ast::Statement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 38], "end": [3534, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37af9ae06b172b62a54846e9"></a>
## span

`function` · `sqlparser::ast::Statement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "super::Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [321, 1], "end": [523, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f2886d4b8b62737c6203b08"></a>
## visit

`function` · `sqlparser::ast::Statement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3537, 19], "end": [3537, 27], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d5da0c951466515b905023"></a>
## visit

`function` · `sqlparser::ast::Statement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Statement", "path": "Statement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3537, 12], "end": [3537, 17], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
