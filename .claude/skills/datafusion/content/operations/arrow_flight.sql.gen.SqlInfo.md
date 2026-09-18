# `arrow_flight::sql::gen::SqlInfo`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_flight.sql.gen.SqlInfo.json).

<a id="op-49e5326d851196e72a1b878a"></a>
## SqlInfo

`enum` · `arrow_flight::sql::gen::SqlInfo` · arrow-flight 59.3.0

```rust
enum SqlInfo
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1040`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Options for CommandGetSqlInfo.

<a id="op-a6670547ffa53fde430f7262"></a>
## Error

`assoc_type` · `arrow_flight::sql::gen::SqlInfo::Error` · arrow-flight 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 68], "end": [1038, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5a962635024469b5cc4d014"></a>
## FlightSqlServerArrowVersion

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerArrowVersion` · arrow-flight 59.3.0

```rust
FlightSqlServerArrowVersion
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1046`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string with the Arrow format version of the Flight SQL Server.

<a id="op-5003f40ea4f0806ddaa91af8"></a>
## FlightSqlServerBulkIngestion

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerBulkIngestion` · arrow-flight 59.3.0

```rust
FlightSqlServerBulkIngestion
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1090`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server supports executing
bulk ingestion.

<a id="op-f8358721cc49456c0e308ae9"></a>
## FlightSqlServerCancel

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerCancel` · arrow-flight 59.3.0

```rust
FlightSqlServerCancel
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1086`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server supports explicit
query cancellation (the CancelQuery action).

<a id="op-352d454a98d9ebbd67419d07"></a>
## FlightSqlServerIngestTransactionsSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerIngestTransactionsSupported` · arrow-flight 59.3.0

```rust
FlightSqlServerIngestTransactionsSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1099`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether transactions are supported for bulk ingestion. If not, invoking
the method commit in the context of a bulk ingestion is a noop, and the isolation level is
`arrow.flight.protocol.sql.SqlTransactionIsolationLevel.TRANSACTION_NONE`.

Returns:
- false: if bulk ingestion transactions are unsupported;
- true: if bulk ingestion transactions are supported.

<a id="op-51cddf9e544acdd592c859ff"></a>
## FlightSqlServerName

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerName` · arrow-flight 59.3.0

```rust
FlightSqlServerName
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1042`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string with the name of the Flight SQL Server.

<a id="op-6dc70365f2d09f612c32eb7b"></a>
## FlightSqlServerReadOnly

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerReadOnly` · arrow-flight 59.3.0

```rust
FlightSqlServerReadOnly
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1053`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server is read only.

Returns:
- false: if read-write
- true: if read only

<a id="op-9ca9cf822de60a5dfd89792f"></a>
## FlightSqlServerSql

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerSql` · arrow-flight 59.3.0

```rust
FlightSqlServerSql
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1060`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server supports executing
SQL queries.

Note that the absence of this info (as opposed to a false value) does not necessarily
mean that SQL is not supported, as this property was not originally defined.

<a id="op-7b5338419be41ef76eb20f78"></a>
## FlightSqlServerStatementTimeout

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerStatementTimeout` · arrow-flight 59.3.0

```rust
FlightSqlServerStatementTimeout
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1104`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves an int32 indicating the timeout (in milliseconds) for prepared statement handles.

If 0, there is no timeout.  Servers should reset the timeout when the handle is used in a command.

<a id="op-312b2fddf6a7f2b12f5458cf"></a>
## FlightSqlServerSubstrait

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerSubstrait` · arrow-flight 59.3.0

```rust
FlightSqlServerSubstrait
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1064`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server supports executing
Substrait plans.

<a id="op-ad3f76a433a2dc29ee3eafdb"></a>
## FlightSqlServerSubstraitMaxVersion

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerSubstraitMaxVersion` · arrow-flight 59.3.0

```rust
FlightSqlServerSubstraitMaxVersion
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1072`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a string value indicating the maximum supported Substrait version, or null
if Substrait is not supported.

<a id="op-3ac01c8e0220edb3b3c7709b"></a>
## FlightSqlServerSubstraitMinVersion

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerSubstraitMinVersion` · arrow-flight 59.3.0

```rust
FlightSqlServerSubstraitMinVersion
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1068`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a string value indicating the minimum supported Substrait version, or null
if Substrait is not supported.

<a id="op-df25d79cc665224cbda81995"></a>
## FlightSqlServerTransaction

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerTransaction` · arrow-flight 59.3.0

```rust
FlightSqlServerTransaction
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1082`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves an int32 indicating whether the Flight SQL Server supports the
BeginTransaction/EndTransaction/BeginSavepoint/EndSavepoint actions.

Even if this is not supported, the database may still support explicit "BEGIN
TRANSACTION"/"COMMIT" SQL statements (see SQL_TRANSACTIONS_SUPPORTED); this property
is only about whether the server implements the Flight SQL API endpoints.

The possible values are listed in `SqlSupportedTransaction`.

<a id="op-65cedee482e799eb7f648ffd"></a>
## FlightSqlServerTransactionTimeout

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerTransactionTimeout` · arrow-flight 59.3.0

```rust
FlightSqlServerTransactionTimeout
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1109`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves an int32 indicating the timeout (in milliseconds) for transactions, since transactions are not tied to a connection.

If 0, there is no timeout.  Servers should reset the timeout when the handle is used in a command.

<a id="op-f0fd448d3bbb311216861381"></a>
## FlightSqlServerVersion

`variant` · `arrow_flight::sql::gen::SqlInfo::FlightSqlServerVersion` · arrow-flight 59.3.0

```rust
FlightSqlServerVersion
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1044`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string with the native version of the Flight SQL Server.

<a id="op-ee73973af79c241e44489364"></a>
## SqlAllTablesAreSelectable

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlAllTablesAreSelectable` · arrow-flight 59.3.0

```rust
SqlAllTablesAreSelectable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1149`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether all tables are selectable.

Returns:
- false: if not all tables are selectable or if none are;
- true: if all tables are selectable.

<a id="op-ed0a979c4ae99a6e30c2a5ea"></a>
## SqlAnsi92SupportedLevel

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlAnsi92SupportedLevel` · arrow-flight 59.3.0

```rust
SqlAnsi92SupportedLevel
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1291`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported ANSI92 SQL grammar level.

Returns an int32 bitmask value representing the supported ANSI92 SQL grammar level.
The returned bitmask should be parsed in order to retrieve the supported commands.

For instance:
- return 0 (\b0)   => \[\] (ANSI92 SQL grammar is unsupported);
- return 1 (\b1)   => \[ANSI92_ENTRY_SQL\];
- return 2 (\b10)  => \[ANSI92_INTERMEDIATE_SQL\];
- return 3 (\b11)  => \[ANSI92_ENTRY_SQL, ANSI92_INTERMEDIATE_SQL\];
- return 4 (\b100) => \[ANSI92_FULL_SQL\];
- return 5 (\b101) => \[ANSI92_ENTRY_SQL, ANSI92_FULL_SQL\];
- return 6 (\b110) => \[ANSI92_INTERMEDIATE_SQL, ANSI92_FULL_SQL\];
- return 7 (\b111) => \[ANSI92_ENTRY_SQL, ANSI92_INTERMEDIATE_SQL, ANSI92_FULL_SQL\].
Valid ANSI92 SQL grammar levels are described under `arrow.flight.protocol.sql.SupportedAnsi92SqlGrammarLevel`.

<a id="op-82ad650dfd6d1c6d3a953396"></a>
## SqlBatchUpdatesSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlBatchUpdatesSupported` · arrow-flight 59.3.0

```rust
SqlBatchUpdatesSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1619`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether this database supports batch updates.

- false: if this database does not support batch updates;
- true: if this database supports batch updates.

<a id="op-d7fbfa3951ab349e020ff0bd"></a>
## SqlCatalogAtStart

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlCatalogAtStart` · arrow-flight 59.3.0

```rust
SqlCatalogAtStart
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1318`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether a catalog appears at the start of a fully qualified table name.

- false: if a catalog does not appear at the start of a fully qualified table name;
- true: if a catalog appears at the start of a fully qualified table name.

<a id="op-5a2df794d8f84249a426e262"></a>
## SqlCatalogTerm

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlCatalogTerm` · arrow-flight 59.3.0

```rust
SqlCatalogTerm
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1312`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a UTF-8 string with the preferred term for "catalog".
If a empty string is returned its assumed that the server does NOT supports catalogs.

<a id="op-8f61e224f32a45dbdb05eb03"></a>
## SqlCatalogsSupportedActions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlCatalogsSupportedActions` · arrow-flight 59.3.0

```rust
SqlCatalogsSupportedActions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1352`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported actions for a SQL schema.

Returns an int32 bitmask value representing the supported actions for a SQL catalog.
The returned bitmask should be parsed in order to retrieve the supported actions for a SQL catalog.

For instance:
- return 0 (\b0)   => \[\] (no supported actions for SQL catalog);
- return 1 (\b1)   => \[SQL_ELEMENT_IN_PROCEDURE_CALLS\];
- return 2 (\b10)  => \[SQL_ELEMENT_IN_INDEX_DEFINITIONS\];
- return 3 (\b11)  => \[SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS\];
- return 4 (\b100) => \[SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\];
- return 5 (\b101) => \[SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\];
- return 6 (\b110) => \[SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\];
- return 7 (\b111) => \[SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\].
Valid actions for a SQL catalog are described under `arrow.flight.protocol.sql.SqlSupportedElementActions`.

<a id="op-aa4ef2700b5423f2f657d7c7"></a>
## SqlCorrelatedSubqueriesSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlCorrelatedSubqueriesSupported` · arrow-flight 59.3.0

```rust
SqlCorrelatedSubqueriesSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1413`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether correlated subqueries are supported.

Returns:
- false: if correlated subqueries are unsupported;
- true: if correlated subqueries are supported.

<a id="op-fad201f081ad526415124e1f"></a>
## SqlDataDefinitionCausesTransactionCommit

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDataDefinitionCausesTransactionCommit` · arrow-flight 59.3.0

```rust
SqlDataDefinitionCausesTransactionCommit
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1529`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether a data definition statement within a transaction forces
the transaction to commit.

Returns:
- false: if a data definition statement within a transaction does not force the transaction to commit;
- true: if a data definition statement within a transaction forces the transaction to commit.

<a id="op-dc481e3a680dea77f8fd1491"></a>
## SqlDataDefinitionsInTransactionsIgnored

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDataDefinitionsInTransactionsIgnored` · arrow-flight 59.3.0

```rust
SqlDataDefinitionsInTransactionsIgnored
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1536`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether a data definition statement within a transaction is ignored.

Returns:
- false: if a data definition statement within a transaction is taken into account;
- true: a data definition statement within a transaction is ignored.

<a id="op-245b7e8abd6d221c5782ea1b"></a>
## SqlDatetimeFunctions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDatetimeFunctions` · arrow-flight 59.3.0

```rust
SqlDatetimeFunctions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1165`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string list with values of the supported datetime functions.

<a id="op-7cd8be6c545f08d69520f63c"></a>
## SqlDbSchemaNameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDbSchemaNameLength` · arrow-flight 59.3.0

```rust
SqlDbSchemaNameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1452`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed in a schema name.

<a id="op-b59c768d3aab0821d35e0f9f"></a>
## SqlDdlCatalog

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDdlCatalog` · arrow-flight 59.3.0

```rust
SqlDdlCatalog
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1116`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server supports CREATE and DROP of catalogs.

Returns:
- false: if it doesn't support CREATE and DROP of catalogs.
- true: if it supports CREATE and DROP of catalogs.

<a id="op-59850ae00ce5e3867b5bb15e"></a>
## SqlDdlSchema

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDdlSchema` · arrow-flight 59.3.0

```rust
SqlDdlSchema
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1123`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the Flight SQL Server supports CREATE and DROP of schemas.

Returns:
- false: if it doesn't support CREATE and DROP of schemas.
- true: if it supports CREATE and DROP of schemas.

<a id="op-44e06dd62cf5112121d68b18"></a>
## SqlDdlTable

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDdlTable` · arrow-flight 59.3.0

```rust
SqlDdlTable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1130`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Indicates whether the Flight SQL Server supports CREATE and DROP of tables.

Returns:
- false: if it doesn't support CREATE and DROP of tables.
- true: if it supports CREATE and DROP of tables.

<a id="op-be57cdc6ef33b6a53e55d5eb"></a>
## SqlDefaultTransactionIsolation

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlDefaultTransactionIsolation` · arrow-flight 59.3.0

```rust
SqlDefaultTransactionIsolation
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1486`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves this database's default transaction isolation level as described in
`arrow.flight.protocol.sql.SqlTransactionIsolationLevel`.

Returns a int32 ordinal for the SQL transaction isolation level.

<a id="op-f70fe61f6d50e439f7727008"></a>
## SqlExtraNameCharacters

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlExtraNameCharacters` · arrow-flight 59.3.0

```rust
SqlExtraNameCharacters
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1176`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a UTF-8 string with all the "extra" characters that can be used in unquoted identifier names
(those beyond a-z, A-Z, 0-9 and _).

<a id="op-4b04bf861b93b338eeb26f05"></a>
## SqlIdentifierCase

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlIdentifierCase` · arrow-flight 59.3.0

```rust
SqlIdentifierCase
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1135`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a int32 ordinal representing the case sensitivity of catalog, table, schema and table names.

The possible values are listed in `arrow.flight.protocol.sql.SqlSupportedCaseSensitivity`.

<a id="op-79ef190cf8cb3d7223322a1a"></a>
## SqlIdentifierQuoteChar

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlIdentifierQuoteChar` · arrow-flight 59.3.0

```rust
SqlIdentifierQuoteChar
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1137`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string with the supported character(s) used to surround a delimited identifier.

<a id="op-22ca01f5c8faa9d3cee02cc5"></a>
## SqlKeywords

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlKeywords` · arrow-flight 59.3.0

```rust
SqlKeywords
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1157`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string list with values of the supported keywords.

<a id="op-3aa7ecdedfd85e1f198f0b9b"></a>
## SqlLocatorsUpdateCopy

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlLocatorsUpdateCopy` · arrow-flight 59.3.0

```rust
SqlLocatorsUpdateCopy
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1640`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether updates made to a LOB are made on a copy or directly to the LOB.

Returns:
- false: if updates made to a LOB are made directly to the LOB;
- true: if updates made to a LOB are made on a copy.

<a id="op-a005e83ab5dfb7346968b5c7"></a>
## SqlMaxBinaryLiteralLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxBinaryLiteralLength` · arrow-flight 59.3.0

```rust
SqlMaxBinaryLiteralLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1428`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of hex characters allowed in an inline binary literal.

<a id="op-e6e08da0d5c8a6f34215a730"></a>
## SqlMaxCatalogNameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxCatalogNameLength` · arrow-flight 59.3.0

```rust
SqlMaxCatalogNameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1456`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed in a catalog name.

<a id="op-6a3c57b501143f6f30eb052c"></a>
## SqlMaxCharLiteralLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxCharLiteralLength` · arrow-flight 59.3.0

```rust
SqlMaxCharLiteralLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1430`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed for a character literal.

<a id="op-d8ed6f4efeb33bcfde090540"></a>
## SqlMaxColumnNameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxColumnNameLength` · arrow-flight 59.3.0

```rust
SqlMaxColumnNameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1432`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed for a column name.

<a id="op-32e5afae921c64154abe3d94"></a>
## SqlMaxColumnsInGroupBy

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxColumnsInGroupBy` · arrow-flight 59.3.0

```rust
SqlMaxColumnsInGroupBy
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1434`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of columns allowed in a GROUP BY clause.

<a id="op-dab2ff0fd38028b8ff8bea28"></a>
## SqlMaxColumnsInIndex

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxColumnsInIndex` · arrow-flight 59.3.0

```rust
SqlMaxColumnsInIndex
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1436`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of columns allowed in an index.

<a id="op-9120da39ac5b6b2344845d7a"></a>
## SqlMaxColumnsInOrderBy

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxColumnsInOrderBy` · arrow-flight 59.3.0

```rust
SqlMaxColumnsInOrderBy
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1438`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of columns allowed in an ORDER BY clause.

<a id="op-a4fb6f70ebcbf5021bab7e2c"></a>
## SqlMaxColumnsInSelect

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxColumnsInSelect` · arrow-flight 59.3.0

```rust
SqlMaxColumnsInSelect
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1440`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of columns allowed in a SELECT list.

<a id="op-2f56c2c86f40b224ef8010c5"></a>
## SqlMaxColumnsInTable

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxColumnsInTable` · arrow-flight 59.3.0

```rust
SqlMaxColumnsInTable
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1442`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of columns allowed in a table.

<a id="op-c735e793857f39ff5dd829c4"></a>
## SqlMaxConnections

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxConnections` · arrow-flight 59.3.0

```rust
SqlMaxConnections
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1444`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of concurrent connections possible.

<a id="op-8967ea79b2ef1db023d7d399"></a>
## SqlMaxCursorNameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxCursorNameLength` · arrow-flight 59.3.0

```rust
SqlMaxCursorNameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1446`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value the maximum number of characters allowed in a cursor name.

<a id="op-8a3e5d91a7d59223d8a61978"></a>
## SqlMaxIndexLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxIndexLength` · arrow-flight 59.3.0

```rust
SqlMaxIndexLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1450`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a int64 value representing the maximum number of bytes allowed for an index,
including all of the parts of the index.

<a id="op-de12d15c4a17845b7c8ea795"></a>
## SqlMaxProcedureNameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxProcedureNameLength` · arrow-flight 59.3.0

```rust
SqlMaxProcedureNameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1454`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed in a procedure name.

<a id="op-4e96efd57e6a75f1b8baf90b"></a>
## SqlMaxRowSize

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxRowSize` · arrow-flight 59.3.0

```rust
SqlMaxRowSize
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1458`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of bytes allowed in a single row.

<a id="op-3efd57be74902b91fbeb1897"></a>
## SqlMaxRowSizeIncludesBlobs

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxRowSizeIncludesBlobs` · arrow-flight 59.3.0

```rust
SqlMaxRowSizeIncludesBlobs
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1468`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean indicating whether the return value for the JDBC method getMaxRowSize includes the SQL
data types LONGVARCHAR and LONGVARBINARY.

Returns:
- false: if return value for the JDBC method getMaxRowSize does
          not include the SQL data types LONGVARCHAR and LONGVARBINARY;
- true: if return value for the JDBC method getMaxRowSize includes
         the SQL data types LONGVARCHAR and LONGVARBINARY.

<a id="op-790ca4d84e1f54e15d011a91"></a>
## SqlMaxStatementLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxStatementLength` · arrow-flight 59.3.0

```rust
SqlMaxStatementLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1472`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a int64 value representing the maximum number of characters allowed for an SQL statement;
a result of 0 (zero) means that there is no limit or the limit is not known.

<a id="op-b22f3f51ff65ea7e2af6ecec"></a>
## SqlMaxStatements

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxStatements` · arrow-flight 59.3.0

```rust
SqlMaxStatements
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1474`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of active statements that can be open at the same time.

<a id="op-f87d46c3e4fa5daa318a165c"></a>
## SqlMaxTableNameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxTableNameLength` · arrow-flight 59.3.0

```rust
SqlMaxTableNameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1476`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed in a table name.

<a id="op-c39d1becc0d7b1934739b1e5"></a>
## SqlMaxTablesInSelect

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxTablesInSelect` · arrow-flight 59.3.0

```rust
SqlMaxTablesInSelect
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1478`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of tables allowed in a SELECT statement.

<a id="op-6e6f728587282870dc7b7ddd"></a>
## SqlMaxUsernameLength

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlMaxUsernameLength` · arrow-flight 59.3.0

```rust
SqlMaxUsernameLength
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1480`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a int64 value representing the maximum number of characters allowed in a user name.

<a id="op-fd7ceb9782489caf87088a53"></a>
## SqlNamedParametersSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlNamedParametersSupported` · arrow-flight 59.3.0

```rust
SqlNamedParametersSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1633`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether named parameters are supported in callable statements.

Returns:
- false: if named parameters in callable statements are unsupported;
- true: if named parameters in callable statements are supported.

<a id="op-76efd5d88abf1ca89a1b26b0"></a>
## SqlNullOrdering

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlNullOrdering` · arrow-flight 59.3.0

```rust
SqlNullOrdering
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1155`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the null ordering.

Returns a int32 ordinal for the null ordering being used, as described in
`arrow.flight.protocol.sql.SqlNullOrdering`.

<a id="op-9482c8ecbf4a4633a6fdb398"></a>
## SqlNullPlusNullIsNull

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlNullPlusNullIsNull` · arrow-flight 59.3.0

```rust
SqlNullPlusNullIsNull
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1193`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether concatenations between null and non-null values being
null are supported.

- Returns:
- false: if concatenations between null and non-null values being null are unsupported;
- true: if concatenations between null and non-null values being null are supported.

<a id="op-101f8f0c643e8768ce426942"></a>
## SqlNumericFunctions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlNumericFunctions` · arrow-flight 59.3.0

```rust
SqlNumericFunctions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1159`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string list with values of the supported numeric functions.

<a id="op-52812256feef922d8732efae"></a>
## SqlOuterJoinsSupportLevel

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlOuterJoinsSupportLevel` · arrow-flight 59.3.0

```rust
SqlOuterJoinsSupportLevel
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1304`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the support level for SQL OUTER JOINs.

Returns a int32 ordinal for the SQL ordering being used, as described in
`arrow.flight.protocol.sql.SqlOuterJoinsSupportLevel`.

<a id="op-4d3e6569afd09679ad5088b4"></a>
## SqlProcedureTerm

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlProcedureTerm` · arrow-flight 59.3.0

```rust
SqlProcedureTerm
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1308`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string with the preferred term for "procedure".

<a id="op-bcdcaa7a0f7dc2a392e2a3f1"></a>
## SqlQuotedIdentifierCase

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlQuotedIdentifierCase` · arrow-flight 59.3.0

```rust
SqlQuotedIdentifierCase
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1142`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a int32 describing the case sensitivity of quoted identifiers.

The possible values are listed in `arrow.flight.protocol.sql.SqlSupportedCaseSensitivity`.

<a id="op-844b107ca6adefd7ae85044d"></a>
## SqlSavepointsSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSavepointsSupported` · arrow-flight 59.3.0

```rust
SqlSavepointsSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1626`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether this database supports savepoints.

Returns:
- false: if this database does not support savepoints;
- true: if this database supports savepoints.

<a id="op-7fe068eb138c2ee4a0bab075"></a>
## SqlSchemaTerm

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSchemaTerm` · arrow-flight 59.3.0

```rust
SqlSchemaTerm
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1306`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string with the preferred term for "schema".

<a id="op-b65b4c4e70bc2467400fc2b2"></a>
## SqlSchemasSupportedActions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSchemasSupportedActions` · arrow-flight 59.3.0

```rust
SqlSchemasSupportedActions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1335`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported actions for a SQL schema.

Returns an int32 bitmask value representing the supported actions for a SQL schema.
The returned bitmask should be parsed in order to retrieve the supported actions for a SQL schema.

For instance:
- return 0 (\b0)   => \[\] (no supported actions for SQL schema);
- return 1 (\b1)   => \[SQL_ELEMENT_IN_PROCEDURE_CALLS\];
- return 2 (\b10)  => \[SQL_ELEMENT_IN_INDEX_DEFINITIONS\];
- return 3 (\b11)  => \[SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS\];
- return 4 (\b100) => \[SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\];
- return 5 (\b101) => \[SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\];
- return 6 (\b110) => \[SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\];
- return 7 (\b111) => \[SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS\].
Valid actions for a SQL schema described under `arrow.flight.protocol.sql.SqlSupportedElementActions`.

<a id="op-b0e3b3f0a5e23d622e64bd97"></a>
## SqlSearchStringEscape

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSearchStringEscape` · arrow-flight 59.3.0

```rust
SqlSearchStringEscape
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1172`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the UTF-8 string that can be used to escape wildcard characters.
This is the string that can be used to escape '_' or '%' in the catalog search parameters that are a pattern
(and therefore use one of the wildcard characters).
The '_' character represents any single character; the '%' character represents any sequence of zero or more
characters.

<a id="op-a1c289a56b7b99f254741ad8"></a>
## SqlSelectForUpdateSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSelectForUpdateSupported` · arrow-flight 59.3.0

```rust
SqlSelectForUpdateSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1372`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether SELECT FOR UPDATE statements are supported.

Returns:
- false: if SELECT FOR UPDATE statements are unsupported;
- true: if SELECT FOR UPDATE statements are supported.

<a id="op-ce488ce8a2d1f6277374acfa"></a>
## SqlStoredFunctionsUsingCallSyntaxSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlStoredFunctionsUsingCallSyntaxSupported` · arrow-flight 59.3.0

```rust
SqlStoredFunctionsUsingCallSyntaxSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1648`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether invoking user-defined or vendor functions
using the stored procedure escape syntax is supported.

Returns:
- false: if invoking user-defined or vendor functions using the stored procedure escape syntax is unsupported;
- true: if invoking user-defined or vendor functions using the stored procedure escape syntax is supported.

<a id="op-95710876d971d456d87fa8f9"></a>
## SqlStoredProceduresSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlStoredProceduresSupported` · arrow-flight 59.3.0

```rust
SqlStoredProceduresSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1380`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether stored procedure calls that use the stored procedure escape syntax
are supported.

Returns:
- false: if stored procedure calls that use the stored procedure escape syntax are unsupported;
- true: if stored procedure calls that use the stored procedure escape syntax are supported.

<a id="op-13f6b9c85ec25e3f1213eb2f"></a>
## SqlStringFunctions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlStringFunctions` · arrow-flight 59.3.0

```rust
SqlStringFunctions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1161`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string list with values of the supported string functions.

<a id="op-292f72fcd6462b30728bde1f"></a>
## SqlSupportedConcurrenciesForResultSetForwardOnly

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedConcurrenciesForResultSetForwardOnly` · arrow-flight 59.3.0

```rust
SqlSupportedConcurrenciesForResultSetForwardOnly
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1583`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Returns an int32 bitmask value concurrency types supported for
`arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_FORWARD_ONLY`.

For instance:
- return 0 (\b0)   => \[\] (no supported concurrency types for this result set type)
- return 1 (\b1)   => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED\]
- return 2 (\b10)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 3 (\b11)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 4 (\b100) => \[SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 5 (\b101) => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 6 (\b110)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 7 (\b111)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.

<a id="op-96bb0422559f3faabbab7ebb"></a>
## SqlSupportedConcurrenciesForResultSetScrollInsensitive

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedConcurrenciesForResultSetScrollInsensitive` · arrow-flight 59.3.0

```rust
SqlSupportedConcurrenciesForResultSetScrollInsensitive
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1613`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Returns an int32 bitmask value concurrency types supported for
`arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE`.

For instance:
- return 0 (\b0)   => \[\] (no supported concurrency types for this result set type)
- return 1 (\b1)   => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED\]
- return 2 (\b10)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 3 (\b11)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 4 (\b100) => \[SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 5 (\b101) => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 6 (\b110)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 7 (\b111)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.

<a id="op-8e3e76b7b2420c108b6960a4"></a>
## SqlSupportedConcurrenciesForResultSetScrollSensitive

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedConcurrenciesForResultSetScrollSensitive` · arrow-flight 59.3.0

```rust
SqlSupportedConcurrenciesForResultSetScrollSensitive
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1598`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Returns an int32 bitmask value concurrency types supported for
`arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_SCROLL_SENSITIVE`.

For instance:
- return 0 (\b0)   => \[\] (no supported concurrency types for this result set type)
- return 1 (\b1)   => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED\]
- return 2 (\b10)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 3 (\b11)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 4 (\b100) => \[SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 5 (\b101) => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 6 (\b110)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 7 (\b111)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.

<a id="op-3fbfd925a20328f1e03410b2"></a>
## SqlSupportedConcurrenciesForResultSetUnspecified

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedConcurrenciesForResultSetUnspecified` · arrow-flight 59.3.0

```rust
SqlSupportedConcurrenciesForResultSetUnspecified
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1568`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Returns an int32 bitmask value concurrency types supported for
`arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_UNSPECIFIED`.

For instance:
- return 0 (\b0)   => \[\] (no supported concurrency types for this result set type)
- return 1 (\b1)   => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED\]
- return 2 (\b10)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 3 (\b11)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY\]
- return 4 (\b100) => \[SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 5 (\b101) => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 6 (\b110)  => \[SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
- return 7 (\b111)  => \[SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE\]
Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.

<a id="op-7ccdabd32f8bfc857758ece9"></a>
## SqlSupportedGrammar

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedGrammar` · arrow-flight 59.3.0

```rust
SqlSupportedGrammar
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1274`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported SQL grammar level as per the ODBC specification.

Returns an int32 bitmask value representing the supported SQL grammar level.
The returned bitmask should be parsed in order to retrieve the supported grammar levels.

For instance:
- return 0 (\b0)   => \[\] (SQL grammar is unsupported);
- return 1 (\b1)   => \[SQL_MINIMUM_GRAMMAR\];
- return 2 (\b10)  => \[SQL_CORE_GRAMMAR\];
- return 3 (\b11)  => \[SQL_MINIMUM_GRAMMAR, SQL_CORE_GRAMMAR\];
- return 4 (\b100) => \[SQL_EXTENDED_GRAMMAR\];
- return 5 (\b101) => \[SQL_MINIMUM_GRAMMAR, SQL_EXTENDED_GRAMMAR\];
- return 6 (\b110) => \[SQL_CORE_GRAMMAR, SQL_EXTENDED_GRAMMAR\];
- return 7 (\b111) => \[SQL_MINIMUM_GRAMMAR, SQL_CORE_GRAMMAR, SQL_EXTENDED_GRAMMAR\].
Valid SQL grammar levels are described under `arrow.flight.protocol.sql.SupportedSqlGrammar`.

<a id="op-264e1615f46712d0eb766b6e"></a>
## SqlSupportedGroupBy

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedGroupBy` · arrow-flight 59.3.0

```rust
SqlSupportedGroupBy
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1243`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported GROUP BY commands;

Returns an int32 bitmask value representing the supported commands.
The returned bitmask should be parsed in order to retrieve the supported commands.

For instance:
- return 0 (\b0)   => \[\] (GROUP BY is unsupported);
- return 1 (\b1)   => \[SQL_GROUP_BY_UNRELATED\];
- return 2 (\b10)  => \[SQL_GROUP_BY_BEYOND_SELECT\];
- return 3 (\b11)  => \[SQL_GROUP_BY_UNRELATED, SQL_GROUP_BY_BEYOND_SELECT\].
Valid GROUP BY types are described under `arrow.flight.protocol.sql.SqlSupportedGroupBy`.

<a id="op-4cc02e703fbd9f94ec674da5"></a>
## SqlSupportedPositionedCommands

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedPositionedCommands` · arrow-flight 59.3.0

```rust
SqlSupportedPositionedCommands
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1365`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported SQL positioned commands.

Returns an int32 bitmask value representing the supported SQL positioned commands.
The returned bitmask should be parsed in order to retrieve the supported SQL positioned commands.

For instance:
- return 0 (\b0)   => \[\] (no supported SQL positioned commands);
- return 1 (\b1)   => \[SQL_POSITIONED_DELETE\];
- return 2 (\b10)  => \[SQL_POSITIONED_UPDATE\];
- return 3 (\b11)  => \[SQL_POSITIONED_DELETE, SQL_POSITIONED_UPDATE\].
Valid SQL positioned commands are described under `arrow.flight.protocol.sql.SqlSupportedPositionedCommands`.

<a id="op-1871bcba79207ac45bdbb840"></a>
## SqlSupportedResultSetTypes

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedResultSetTypes` · arrow-flight 59.3.0

```rust
SqlSupportedResultSetTypes
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1553`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves an int32 bitmask value representing the supported result set types.
The returned bitmask should be parsed in order to retrieve the supported result set types.

For instance:
- return 0   (\b0)     => \[\] (no supported result set types);
- return 1   (\b1)     => \[SQL_RESULT_SET_TYPE_UNSPECIFIED\];
- return 2   (\b10)    => \[SQL_RESULT_SET_TYPE_FORWARD_ONLY\];
- return 3   (\b11)    => \[SQL_RESULT_SET_TYPE_UNSPECIFIED, SQL_RESULT_SET_TYPE_FORWARD_ONLY\];
- return 4   (\b100)   => \[SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE\];
- return 5   (\b101)   => \[SQL_RESULT_SET_TYPE_UNSPECIFIED, SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE\];
- return 6   (\b110)   => \[SQL_RESULT_SET_TYPE_FORWARD_ONLY, SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE\];
- return 7   (\b111)   => \[SQL_RESULT_SET_TYPE_UNSPECIFIED, SQL_RESULT_SET_TYPE_FORWARD_ONLY, SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE\];
- return 8   (\b1000)  => \[SQL_RESULT_SET_TYPE_SCROLL_SENSITIVE\];
- ...
Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetType`.

<a id="op-cd5cad1954170bd5a6114dce"></a>
## SqlSupportedSubqueries

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedSubqueries` · arrow-flight 59.3.0

```rust
SqlSupportedSubqueries
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1406`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported SQL subqueries.

Returns an int32 bitmask value representing the supported SQL subqueries.
The returned bitmask should be parsed in order to retrieve the supported SQL subqueries.

For instance:
- return 0   (\b0)     => \[\] (no supported SQL subqueries);
- return 1   (\b1)     => \[SQL_SUBQUERIES_IN_COMPARISONS\];
- return 2   (\b10)    => \[SQL_SUBQUERIES_IN_EXISTS\];
- return 3   (\b11)    => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS\];
- return 4   (\b100)   => \[SQL_SUBQUERIES_IN_INS\];
- return 5   (\b101)   => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_INS\];
- return 6   (\b110)   => \[SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_EXISTS\];
- return 7   (\b111)   => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_INS\];
- return 8   (\b1000)  => \[SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 9   (\b1001)  => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 10  (\b1010)  => \[SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 11  (\b1011)  => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 12  (\b1100)  => \[SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 13  (\b1101)  => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 14  (\b1110)  => \[SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- return 15  (\b1111)  => \[SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS\];
- ...
Valid SQL subqueries are described under `arrow.flight.protocol.sql.SqlSupportedSubqueries`.

<a id="op-c1acf193577b29c50f4be67d"></a>
## SqlSupportedTransactionsIsolationLevels

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedTransactionsIsolationLevels` · arrow-flight 59.3.0

```rust
SqlSupportedTransactionsIsolationLevels
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1521`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported transactions isolation levels.

Returns an int32 bitmask value representing the supported transactions isolation levels.
The returned bitmask should be parsed in order to retrieve the supported transactions isolation levels.

For instance:
- return 0   (\b0)     => \[\] (no supported SQL transactions isolation levels);
- return 1   (\b1)     => \[SQL_TRANSACTION_NONE\];
- return 2   (\b10)    => \[SQL_TRANSACTION_READ_UNCOMMITTED\];
- return 3   (\b11)    => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED\];
- return 4   (\b100)   => \[SQL_TRANSACTION_REPEATABLE_READ\];
- return 5   (\b101)   => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_REPEATABLE_READ\];
- return 6   (\b110)   => \[SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ\];
- return 7   (\b111)   => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ\];
- return 8   (\b1000)  => \[SQL_TRANSACTION_REPEATABLE_READ\];
- return 9   (\b1001)  => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_REPEATABLE_READ\];
- return 10  (\b1010)  => \[SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ\];
- return 11  (\b1011)  => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ\];
- return 12  (\b1100)  => \[SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ\];
- return 13  (\b1101)  => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ\];
- return 14  (\b1110)  => \[SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ\];
- return 15  (\b1111)  => \[SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ\];
- return 16  (\b10000) => \[SQL_TRANSACTION_SERIALIZABLE\];
- ...
Valid SQL positioned commands are described under `arrow.flight.protocol.sql.SqlTransactionIsolationLevel`.

<a id="op-390b4735f9962f3a53fd498e"></a>
## SqlSupportedUnions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportedUnions` · arrow-flight 59.3.0

```rust
SqlSupportedUnions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1426`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves the supported SQL UNIONs.

Returns an int32 bitmask value representing the supported SQL UNIONs.
The returned bitmask should be parsed in order to retrieve the supported SQL UNIONs.

For instance:
- return 0 (\b0)   => \[\] (no supported SQL positioned commands);
- return 1 (\b1)   => \[SQL_UNION\];
- return 2 (\b10)  => \[SQL_UNION_ALL\];
- return 3 (\b11)  => \[SQL_UNION, SQL_UNION_ALL\].
Valid SQL positioned commands are described under `arrow.flight.protocol.sql.SqlSupportedUnions`.

<a id="op-8706bffb3518cefc7c6d72c8"></a>
## SqlSupportsColumnAliasing

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsColumnAliasing` · arrow-flight 59.3.0

```rust
SqlSupportsColumnAliasing
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1185`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether column aliasing is supported.
If so, the SQL AS clause can be used to provide names for computed columns or to provide alias names for columns
as required.

Returns:
- false: if column aliasing is unsupported;
- true: if column aliasing is supported.

<a id="op-f493f9206ac3ba0b0c8b081b"></a>
## SqlSupportsConvert

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsConvert` · arrow-flight 59.3.0

```rust
SqlSupportsConvert
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1199`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a map where the key is the type to convert from and the value is a list with the types to convert to,
indicating the supported conversions. Each key and each item on the list value is a value to a predefined type on
SqlSupportsConvert enum.
The returned map will be:  map<int32, list<int32>>

<a id="op-94e679b2b7341cf3f51b8c0d"></a>
## SqlSupportsDifferentTableCorrelationNames

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsDifferentTableCorrelationNames` · arrow-flight 59.3.0

```rust
SqlSupportsDifferentTableCorrelationNames
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1215`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether, when table correlation names are supported,
they are restricted to being different from the names of the tables.

Returns:
- false: if different table correlation names are unsupported;
- true: if different table correlation names are supported

<a id="op-9989982b1a1260f8fa11f499"></a>
## SqlSupportsExpressionsInOrderBy

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsExpressionsInOrderBy` · arrow-flight 59.3.0

```rust
SqlSupportsExpressionsInOrderBy
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1222`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether expressions in ORDER BY lists are supported.

Returns:
- false: if expressions in ORDER BY are unsupported;
- true: if expressions in ORDER BY are supported;

<a id="op-f5af83aa90b4108134d403d1"></a>
## SqlSupportsIntegrityEnhancementFacility

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsIntegrityEnhancementFacility` · arrow-flight 59.3.0

```rust
SqlSupportsIntegrityEnhancementFacility
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1298`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether the SQL Integrity Enhancement Facility is supported.

Returns:
- false: if the SQL Integrity Enhancement Facility is supported;
- true: if the SQL Integrity Enhancement Facility is supported.

<a id="op-19687ecf5a75ec1643263c1d"></a>
## SqlSupportsLikeEscapeClause

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsLikeEscapeClause` · arrow-flight 59.3.0

```rust
SqlSupportsLikeEscapeClause
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1250`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether specifying a LIKE escape clause is supported.

Returns:
- false: if specifying a LIKE escape clause is unsupported;
- true: if specifying a LIKE escape clause is supported.

<a id="op-d2d488c839e5c80715198389"></a>
## SqlSupportsNonNullableColumns

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsNonNullableColumns` · arrow-flight 59.3.0

```rust
SqlSupportsNonNullableColumns
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1257`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether columns may be defined as non-nullable.

Returns:
- false: if columns cannot be defined as non-nullable;
- true: if columns may be defined as non-nullable.

<a id="op-09840807e039e1edf19fd6ee"></a>
## SqlSupportsOrderByUnrelated

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsOrderByUnrelated` · arrow-flight 59.3.0

```rust
SqlSupportsOrderByUnrelated
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1230`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether using a column that is not in the SELECT statement in a GROUP BY
clause is supported.

Returns:
- false: if using a column that is not in the SELECT statement in a GROUP BY clause is unsupported;
- true: if using a column that is not in the SELECT statement in a GROUP BY clause is supported.

<a id="op-341ae295a3b70ba53756458a"></a>
## SqlSupportsTableCorrelationNames

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSupportsTableCorrelationNames` · arrow-flight 59.3.0

```rust
SqlSupportsTableCorrelationNames
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1207`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether, when table correlation names are supported,
they are restricted to being different from the names of the tables.

Returns:
- false: if table correlation names are unsupported;
- true: if table correlation names are supported.

<a id="op-f673f93f670d3845c6b5ee06"></a>
## SqlSystemFunctions

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlSystemFunctions` · arrow-flight 59.3.0

```rust
SqlSystemFunctions
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1163`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Retrieves a UTF-8 string list with values of the supported system functions.

<a id="op-bdf78535f3e08342a19eac1b"></a>
## SqlTransactionsSupported

`variant` · `arrow_flight::sql::gen::SqlInfo::SqlTransactionsSupported` · arrow-flight 59.3.0

```rust
SqlTransactionsSupported
```

Source: `src/sql/arrow.flight.protocol.sql.rs:1494`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).


Retrieves a boolean value indicating whether transactions are supported. If not, invoking the method commit is a
noop, and the isolation level is `arrow.flight.protocol.sql.SqlTransactionIsolationLevel.TRANSACTION_NONE`.

Returns:
- false: if transactions are unsupported;
- true: if transactions are supported.

<a id="op-8f522c44a35b4745fed2df66"></a>
## as_str_name

`function` · `arrow_flight::sql::gen::SqlInfo::as_str_name` · arrow-flight 59.3.0

```rust
fn as_str_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1650, 1], "end": [1927, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1655`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

String value of the enum field names used in the ProtoBuf definition.

The values are not transformed in any way and thus are considered stable
(if the ProtoBuf definition does not change) and safe for programmatic use.

<a id="op-7288bdf63a6c4de368173235"></a>
## clone

`function` · `arrow_flight::sql::gen::SqlInfo::clone` · arrow-flight 59.3.0

```rust
fn clone(&self) -> SqlInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 10], "end": [1038, 15], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa43abf6c6118ead9fbe1ff"></a>
## cmp

`function` · `arrow_flight::sql::gen::SqlInfo::cmp` · arrow-flight 59.3.0

```rust
fn cmp(&self, other: &SqlInfo) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 63], "end": [1038, 66], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-304b0e1b1baba166667515b6"></a>
## default

`function` · `arrow_flight::sql::gen::SqlInfo::default` · arrow-flight 59.3.0

```rust
fn default() -> SqlInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 68], "end": [1038, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7afe3bed1843b1b53602f42d"></a>
## eq

`function` · `arrow_flight::sql::gen::SqlInfo::eq` · arrow-flight 59.3.0

```rust
fn eq(&self, other: &SqlInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 30], "end": [1038, 39], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37ad23ec88e43691ac949e50"></a>
## fmt

`function` · `arrow_flight::sql::gen::SqlInfo::fmt` · arrow-flight 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 23], "end": [1038, 28], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4216b4166dcf4eaf767a0366"></a>
## from_i32

`function` · `arrow_flight::sql::gen::SqlInfo::from_i32` · arrow-flight 59.3.0

```rust
fn from_i32(value: i32) -> ::core::option::Option<SqlInfo>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 68], "end": [1038, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Converts an `i32` to a `SqlInfo`, or `None` if `value` is not a valid variant.

<a id="op-5e4aa84ca9b6b06452277f78"></a>
## from_str_name

`function` · `arrow_flight::sql::gen::SqlInfo::from_str_name` · arrow-flight 59.3.0

```rust
fn from_str_name(value: &str) -> ::core::option::Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1650, 1], "end": [1927, 2], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1787`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Creates an enum from field names used in the ProtoBuf definition.

<a id="op-90ba78b49ce077e0d5b0dc58"></a>
## hash

`function` · `arrow_flight::sql::gen::SqlInfo::hash` · arrow-flight 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 45], "end": [1038, 49], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06521cc0d6cda54c64343368"></a>
## is_valid

`function` · `arrow_flight::sql::gen::SqlInfo::is_valid` · arrow-flight 59.3.0

```rust
fn is_valid(value: i32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 68], "end": [1038, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": null, "trait_path": null}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

Returns `true` if `value` is a variant of `SqlInfo`.

<a id="op-a0c0470089cb3cce9d21cba5"></a>
## partial_cmp

`function` · `arrow_flight::sql::gen::SqlInfo::partial_cmp` · arrow-flight 59.3.0

```rust
fn partial_cmp(&self, other: &SqlInfo) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 51], "end": [1038, 61], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12856cd260630d1e75cdbfbf"></a>
## try_from

`function` · `arrow_flight::sql::gen::SqlInfo::try_from` · arrow-flight 59.3.0

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlInfo, ::prost::UnknownEnumValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_flight::sql::gen::SqlInfo", "path": "SqlInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1038, 68], "end": [1038, 88], "filename": "src/sql/arrow.flight.protocol.sql.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/sql/arrow.flight.protocol.sql.rs:1038`. [Exact documentation build](https://docs.rs/crate/arrow-flight/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
