# `arrow_flight::sql::gen`

Crate `arrow-flight` · 54 public items · structured records in [`model/arrow_flight.sql.gen.json`](../model/arrow_flight.sql.gen.json)

## Nullable

`enum` · `arrow_flight::sql::gen::Nullable`

Also reachable as `arrow_flight::sql::Nullable`

```rust
enum Nullable
```

**Variants**: `NullabilityNoNulls`, `NullabilityNullable`, `NullabilityUnknown`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<Nullable>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<Nullable, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.Nullable.md).


---

## Searchable

`enum` · `arrow_flight::sql::gen::Searchable`

Also reachable as `arrow_flight::sql::Searchable`

```rust
enum Searchable
```

**Variants**: `None`, `Char`, `Basic`, `Full`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<Searchable>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<Searchable, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.Searchable.md).


---

## SqlInfo

`enum` · `arrow_flight::sql::gen::SqlInfo`

Also reachable as `arrow_flight::sql::SqlInfo`

```rust
enum SqlInfo
```

**Variants**: `FlightSqlServerName`, `FlightSqlServerVersion`, `FlightSqlServerArrowVersion`, `FlightSqlServerReadOnly`, `FlightSqlServerSql`, `FlightSqlServerSubstrait`, `FlightSqlServerSubstraitMinVersion`, `FlightSqlServerSubstraitMaxVersion`, `FlightSqlServerTransaction`, `FlightSqlServerCancel`, `FlightSqlServerBulkIngestion`, `FlightSqlServerIngestTransactionsSupported`, `FlightSqlServerStatementTimeout`, `FlightSqlServerTransactionTimeout`, `SqlDdlCatalog`, `SqlDdlSchema`, `SqlDdlTable`, `SqlIdentifierCase`, `SqlIdentifierQuoteChar`, `SqlQuotedIdentifierCase`, `SqlAllTablesAreSelectable`, `SqlNullOrdering`, `SqlKeywords`, `SqlNumericFunctions`, `SqlStringFunctions`, `SqlSystemFunctions`, `SqlDatetimeFunctions`, `SqlSearchStringEscape`, `SqlExtraNameCharacters`, `SqlSupportsColumnAliasing`, `SqlNullPlusNullIsNull`, `SqlSupportsConvert`, `SqlSupportsTableCorrelationNames`, `SqlSupportsDifferentTableCorrelationNames`, `SqlSupportsExpressionsInOrderBy`, `SqlSupportsOrderByUnrelated`, `SqlSupportedGroupBy`, `SqlSupportsLikeEscapeClause`, `SqlSupportsNonNullableColumns`, `SqlSupportedGrammar`, `SqlAnsi92SupportedLevel`, `SqlSupportsIntegrityEnhancementFacility`, `SqlOuterJoinsSupportLevel`, `SqlSchemaTerm`, `SqlProcedureTerm`, `SqlCatalogTerm`, `SqlCatalogAtStart`, `SqlSchemasSupportedActions`, `SqlCatalogsSupportedActions`, `SqlSupportedPositionedCommands`, `SqlSelectForUpdateSupported`, `SqlStoredProceduresSupported`, `SqlSupportedSubqueries`, `SqlCorrelatedSubqueriesSupported`, `SqlSupportedUnions`, `SqlMaxBinaryLiteralLength`, `SqlMaxCharLiteralLength`, `SqlMaxColumnNameLength`, `SqlMaxColumnsInGroupBy`, `SqlMaxColumnsInIndex`, `SqlMaxColumnsInOrderBy`, `SqlMaxColumnsInSelect`, `SqlMaxColumnsInTable`, `SqlMaxConnections`, `SqlMaxCursorNameLength`, `SqlMaxIndexLength`, `SqlDbSchemaNameLength`, `SqlMaxProcedureNameLength`, `SqlMaxCatalogNameLength`, `SqlMaxRowSize`, `SqlMaxRowSizeIncludesBlobs`, `SqlMaxStatementLength`, `SqlMaxStatements`, `SqlMaxTableNameLength`, `SqlMaxTablesInSelect`, `SqlMaxUsernameLength`, `SqlDefaultTransactionIsolation`, `SqlTransactionsSupported`, `SqlSupportedTransactionsIsolationLevels`, `SqlDataDefinitionCausesTransactionCommit`, `SqlDataDefinitionsInTransactionsIgnored`, `SqlSupportedResultSetTypes`, `SqlSupportedConcurrenciesForResultSetUnspecified`, `SqlSupportedConcurrenciesForResultSetForwardOnly`, `SqlSupportedConcurrenciesForResultSetScrollSensitive`, `SqlSupportedConcurrenciesForResultSetScrollInsensitive`, `SqlBatchUpdatesSupported`, `SqlSavepointsSupported`, `SqlNamedParametersSupported`, `SqlLocatorsUpdateCopy`, `SqlStoredFunctionsUsingCallSyntaxSupported`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlInfo>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlInfo, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlInfo.md).


Options for CommandGetSqlInfo.

---

## SqlNullOrdering

`enum` · `arrow_flight::sql::gen::SqlNullOrdering`

Also reachable as `arrow_flight::sql::SqlNullOrdering`

```rust
enum SqlNullOrdering
```

**Variants**: `SqlNullsSortedHigh`, `SqlNullsSortedLow`, `SqlNullsSortedAtStart`, `SqlNullsSortedAtEnd`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlNullOrdering>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlNullOrdering, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlNullOrdering.md).


---

## SqlOuterJoinsSupportLevel

`enum` · `arrow_flight::sql::gen::SqlOuterJoinsSupportLevel`

Also reachable as `arrow_flight::sql::SqlOuterJoinsSupportLevel`

```rust
enum SqlOuterJoinsSupportLevel
```

**Variants**: `SqlJoinsUnsupported`, `SqlLimitedOuterJoins`, `SqlFullOuterJoins`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlOuterJoinsSupportLevel>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlOuterJoinsSupportLevel, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlOuterJoinsSupportLevel.md).


---

## SqlSupportedCaseSensitivity

`enum` · `arrow_flight::sql::gen::SqlSupportedCaseSensitivity`

Also reachable as `arrow_flight::sql::SqlSupportedCaseSensitivity`

```rust
enum SqlSupportedCaseSensitivity
```

**Variants**: `SqlCaseSensitivityUnknown`, `SqlCaseSensitivityCaseInsensitive`, `SqlCaseSensitivityUppercase`, `SqlCaseSensitivityLowercase`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedCaseSensitivity>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedCaseSensitivity, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedCaseSensitivity.md).


---

## SqlSupportedElementActions

`enum` · `arrow_flight::sql::gen::SqlSupportedElementActions`

Also reachable as `arrow_flight::sql::SqlSupportedElementActions`

```rust
enum SqlSupportedElementActions
```

**Variants**: `SqlElementInProcedureCalls`, `SqlElementInIndexDefinitions`, `SqlElementInPrivilegeDefinitions`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedElementActions>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedElementActions, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedElementActions.md).


---

## SqlSupportedGroupBy

`enum` · `arrow_flight::sql::gen::SqlSupportedGroupBy`

Also reachable as `arrow_flight::sql::SqlSupportedGroupBy`

```rust
enum SqlSupportedGroupBy
```

**Variants**: `SqlGroupByUnrelated`, `SqlGroupByBeyondSelect`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedGroupBy>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedGroupBy, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedGroupBy.md).


---

## SqlSupportedPositionedCommands

`enum` · `arrow_flight::sql::gen::SqlSupportedPositionedCommands`

Also reachable as `arrow_flight::sql::SqlSupportedPositionedCommands`

```rust
enum SqlSupportedPositionedCommands
```

**Variants**: `SqlPositionedDelete`, `SqlPositionedUpdate`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedPositionedCommands>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedPositionedCommands, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedPositionedCommands.md).


---

## SqlSupportedResultSetConcurrency

`enum` · `arrow_flight::sql::gen::SqlSupportedResultSetConcurrency`

Also reachable as `arrow_flight::sql::SqlSupportedResultSetConcurrency`

```rust
enum SqlSupportedResultSetConcurrency
```

**Variants**: `SqlResultSetConcurrencyUnspecified`, `SqlResultSetConcurrencyReadOnly`, `SqlResultSetConcurrencyUpdatable`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedResultSetConcurrency>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedResultSetConcurrency, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedResultSetConcurrency.md).


---

## SqlSupportedResultSetType

`enum` · `arrow_flight::sql::gen::SqlSupportedResultSetType`

Also reachable as `arrow_flight::sql::SqlSupportedResultSetType`

```rust
enum SqlSupportedResultSetType
```

**Variants**: `SqlResultSetTypeUnspecified`, `SqlResultSetTypeForwardOnly`, `SqlResultSetTypeScrollInsensitive`, `SqlResultSetTypeScrollSensitive`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedResultSetType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedResultSetType, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedResultSetType.md).


---

## SqlSupportedSubqueries

`enum` · `arrow_flight::sql::gen::SqlSupportedSubqueries`

Also reachable as `arrow_flight::sql::SqlSupportedSubqueries`

```rust
enum SqlSupportedSubqueries
```

**Variants**: `SqlSubqueriesInComparisons`, `SqlSubqueriesInExists`, `SqlSubqueriesInIns`, `SqlSubqueriesInQuantifieds`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedSubqueries>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedSubqueries, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedSubqueries.md).


---

## SqlSupportedTransaction

`enum` · `arrow_flight::sql::gen::SqlSupportedTransaction`

Also reachable as `arrow_flight::sql::SqlSupportedTransaction`

```rust
enum SqlSupportedTransaction
```

**Variants**: `None`, `Transaction`, `Savepoint`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedTransaction>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedTransaction, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedTransaction.md).


The level of support for Flight SQL transaction RPCs.

---

## SqlSupportedTransactions

`enum` · `arrow_flight::sql::gen::SqlSupportedTransactions`

Also reachable as `arrow_flight::sql::SqlSupportedTransactions`

```rust
enum SqlSupportedTransactions
```

**Variants**: `SqlTransactionUnspecified`, `SqlDataDefinitionTransactions`, `SqlDataManipulationTransactions`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedTransactions>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedTransactions, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedTransactions.md).


---

## SqlSupportedUnions

`enum` · `arrow_flight::sql::gen::SqlSupportedUnions`

Also reachable as `arrow_flight::sql::SqlSupportedUnions`

```rust
enum SqlSupportedUnions
```

**Variants**: `SqlUnion`, `SqlUnionAll`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportedUnions>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportedUnions, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportedUnions.md).


---

## SqlSupportsConvert

`enum` · `arrow_flight::sql::gen::SqlSupportsConvert`

Also reachable as `arrow_flight::sql::SqlSupportsConvert`

```rust
enum SqlSupportsConvert
```

**Variants**: `SqlConvertBigint`, `SqlConvertBinary`, `SqlConvertBit`, `SqlConvertChar`, `SqlConvertDate`, `SqlConvertDecimal`, `SqlConvertFloat`, `SqlConvertInteger`, `SqlConvertIntervalDayTime`, `SqlConvertIntervalYearMonth`, `SqlConvertLongvarbinary`, `SqlConvertLongvarchar`, `SqlConvertNumeric`, `SqlConvertReal`, `SqlConvertSmallint`, `SqlConvertTime`, `SqlConvertTimestamp`, `SqlConvertTinyint`, `SqlConvertVarbinary`, `SqlConvertVarchar`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlSupportsConvert>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlSupportsConvert, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlSupportsConvert.md).


---

## SqlTransactionIsolationLevel

`enum` · `arrow_flight::sql::gen::SqlTransactionIsolationLevel`

Also reachable as `arrow_flight::sql::SqlTransactionIsolationLevel`

```rust
enum SqlTransactionIsolationLevel
```

**Variants**: `SqlTransactionNone`, `SqlTransactionReadUncommitted`, `SqlTransactionReadCommitted`, `SqlTransactionRepeatableRead`, `SqlTransactionSerializable`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SqlTransactionIsolationLevel>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SqlTransactionIsolationLevel, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SqlTransactionIsolationLevel.md).


---

## SupportedAnsi92SqlGrammarLevel

`enum` · `arrow_flight::sql::gen::SupportedAnsi92SqlGrammarLevel`

```rust
enum SupportedAnsi92SqlGrammarLevel
```

**Variants**: `Ansi92EntrySql`, `Ansi92IntermediateSql`, `Ansi92FullSql`

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SupportedAnsi92SqlGrammarLevel.md).


---

## SupportedSqlGrammar

`enum` · `arrow_flight::sql::gen::SupportedSqlGrammar`

Also reachable as `arrow_flight::sql::SupportedSqlGrammar`

```rust
enum SupportedSqlGrammar
```

**Variants**: `SqlMinimumGrammar`, `SqlCoreGrammar`, `SqlExtendedGrammar`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<SupportedSqlGrammar>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<SupportedSqlGrammar, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SupportedSqlGrammar.md).


---

## UpdateDeleteRules

`enum` · `arrow_flight::sql::gen::UpdateDeleteRules`

Also reachable as `arrow_flight::sql::UpdateDeleteRules`

```rust
enum UpdateDeleteRules
```

**Variants**: `Cascade`, `Restrict`, `SetNull`, `NoAction`, `SetDefault`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<UpdateDeleteRules>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<UpdateDeleteRules, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.UpdateDeleteRules.md).


---

## XdbcDataType

`enum` · `arrow_flight::sql::gen::XdbcDataType`

Also reachable as `arrow_flight::sql::XdbcDataType`

```rust
enum XdbcDataType
```

**Variants**: `XdbcUnknownType`, `XdbcChar`, `XdbcNumeric`, `XdbcDecimal`, `XdbcInteger`, `XdbcSmallint`, `XdbcFloat`, `XdbcReal`, `XdbcDouble`, `XdbcDatetime`, `XdbcInterval`, `XdbcVarchar`, `XdbcDate`, `XdbcTime`, `XdbcTimestamp`, `XdbcLongvarchar`, `XdbcBinary`, `XdbcVarbinary`, `XdbcLongvarbinary`, `XdbcBigint`, `XdbcTinyint`, `XdbcBit`, `XdbcWchar`, `XdbcWvarchar`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<XdbcDataType>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<XdbcDataType, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.XdbcDataType.md).


*
The JDBC/ODBC-defined type of any object.
All the values here are the same as in the JDBC and ODBC specs.

---

## XdbcDatetimeSubcode

`enum` · `arrow_flight::sql::gen::XdbcDatetimeSubcode`

Also reachable as `arrow_flight::sql::XdbcDatetimeSubcode`

```rust
enum XdbcDatetimeSubcode
```

**Variants**: `XdbcSubcodeUnknown`, `XdbcSubcodeYear`, `XdbcSubcodeTime`, `XdbcSubcodeTimestamp`, `XdbcSubcodeTimeWithTimezone`, `XdbcSubcodeTimestampWithTimezone`, `XdbcSubcodeSecond`, `XdbcSubcodeYearToMonth`, `XdbcSubcodeDayToHour`, `XdbcSubcodeDayToMinute`, `XdbcSubcodeDayToSecond`, `XdbcSubcodeHourToMinute`, `XdbcSubcodeHourToSecond`, `XdbcSubcodeMinuteToSecond`, `XdbcSubcodeIntervalYear`, `XdbcSubcodeIntervalMonth`, `XdbcSubcodeIntervalDay`, `XdbcSubcodeIntervalHour`, `XdbcSubcodeIntervalMinute`, `XdbcSubcodeIntervalSecond`, `XdbcSubcodeIntervalYearToMonth`, `XdbcSubcodeIntervalDayToHour`, `XdbcSubcodeIntervalDayToMinute`, `XdbcSubcodeIntervalDayToSecond`, `XdbcSubcodeIntervalHourToMinute`, `XdbcSubcodeIntervalHourToSecond`, `XdbcSubcodeIntervalMinuteToSecond`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (4)

```rust
fn as_str_name(&self) -> &'static str
fn from_i32(value: i32) -> ::core::option::Option<XdbcDatetimeSubcode>
fn from_str_name(value: &str) -> ::core::option::Option<Self>
fn is_valid(value: i32) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: i32) -> ::core::result::Result<XdbcDatetimeSubcode, ::prost::UnknownEnumValue>
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.XdbcDatetimeSubcode.md).


*
Detailed subtype information for XDBC_TYPE_DATETIME and XDBC_TYPE_INTERVAL.

---

## ActionBeginSavepointRequest

`struct` · `arrow_flight::sql::gen::ActionBeginSavepointRequest`

Also reachable as `arrow_flight::sql::ActionBeginSavepointRequest`

```rust
struct ActionBeginSavepointRequest
```

**Fields**: `transaction_id`, `name`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionBeginSavepointRequest.md).



Request message for the "BeginSavepoint" action.
Creates a savepoint within a transaction.

Only supported if FLIGHT_SQL_TRANSACTION is
FLIGHT_SQL_TRANSACTION_SUPPORT_SAVEPOINT.

---

## ActionBeginSavepointResult

`struct` · `arrow_flight::sql::gen::ActionBeginSavepointResult`

Also reachable as `arrow_flight::sql::ActionBeginSavepointResult`

```rust
struct ActionBeginSavepointResult
```

**Fields**: `savepoint_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionBeginSavepointResult.md).



The result of a "BeginSavepoint" action.

The transaction can be manipulated with the "EndSavepoint" action.
If the associated transaction is committed, rolled back, or times
out, then the savepoint is also invalidated.

The result should be wrapped in a google.protobuf.Any message.

---

## ActionBeginTransactionRequest

`struct` · `arrow_flight::sql::gen::ActionBeginTransactionRequest`

Also reachable as `arrow_flight::sql::ActionBeginTransactionRequest`

```rust
struct ActionBeginTransactionRequest
```

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionBeginTransactionRequest.md).



Request message for the "BeginTransaction" action.
Begins a transaction.

---

## ActionBeginTransactionResult

`struct` · `arrow_flight::sql::gen::ActionBeginTransactionResult`

Also reachable as `arrow_flight::sql::ActionBeginTransactionResult`

```rust
struct ActionBeginTransactionResult
```

**Fields**: `transaction_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionBeginTransactionResult.md).



The result of a "BeginTransaction" action.

The transaction can be manipulated with the "EndTransaction" action, or
automatically via server timeout. If the transaction times out, then it is
automatically rolled back.

The result should be wrapped in a google.protobuf.Any message.

---

## ActionCancelQueryRequest

`struct` · `arrow_flight::sql::gen::ActionCancelQueryRequest`

Also reachable as `arrow_flight::sql::ActionCancelQueryRequest`

```rust
struct ActionCancelQueryRequest
```

**Fields**: `info`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionCancelQueryRequest.md).



Request message for the "CancelQuery" action.

Explicitly cancel a running query.

This lets a single client explicitly cancel work, no matter how many clients
are involved/whether the query is distributed or not, given server support.
The transaction/statement is not rolled back; it is the application's job to
commit or rollback as appropriate. This only indicates the client no longer
wishes to read the remainder of the query results or continue submitting
data.

This command is idempotent.

This command is deprecated since 13.0.0. Use the "CancelFlightInfo"
action with DoAction instead.

---

## ActionCancelQueryResult

`struct` · `arrow_flight::sql::gen::ActionCancelQueryResult`

Also reachable as `arrow_flight::sql::ActionCancelQueryResult`

```rust
struct ActionCancelQueryResult
```

**Fields**: `result`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn result(&self) -> action_cancel_query_result::CancelResult
fn set_result(&mut self, value: action_cancel_query_result::CancelResult)
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionCancelQueryResult.md).



The result of cancelling a query.

The result should be wrapped in a google.protobuf.Any message.

This command is deprecated since 13.0.0. Use the "CancelFlightInfo"
action with DoAction instead.

---

## ActionClosePreparedStatementRequest

`struct` · `arrow_flight::sql::gen::ActionClosePreparedStatementRequest`

Also reachable as `arrow_flight::sql::ActionClosePreparedStatementRequest`

```rust
struct ActionClosePreparedStatementRequest
```

**Fields**: `prepared_statement_handle`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionClosePreparedStatementRequest.md).



Request message for the "ClosePreparedStatement" action on a Flight SQL enabled backend.
Closes server resources associated with the prepared statement handle.

---

## ActionCreatePreparedStatementRequest

`struct` · `arrow_flight::sql::gen::ActionCreatePreparedStatementRequest`

Also reachable as `arrow_flight::sql::ActionCreatePreparedStatementRequest`

```rust
struct ActionCreatePreparedStatementRequest
```

**Fields**: `query`, `transaction_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn transaction_id(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionCreatePreparedStatementRequest.md).



Request message for the "CreatePreparedStatement" action on a Flight SQL enabled backend.

---

## ActionCreatePreparedStatementResult

`struct` · `arrow_flight::sql::gen::ActionCreatePreparedStatementResult`

Also reachable as `arrow_flight::sql::ActionCreatePreparedStatementResult`

```rust
struct ActionCreatePreparedStatementResult
```

**Fields**: `prepared_statement_handle`, `dataset_schema`, `parameter_schema`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionCreatePreparedStatementResult.md).



Wrap the result of a "CreatePreparedStatement" or "CreatePreparedSubstraitPlan" action.

The resultant PreparedStatement can be closed either:
- Manually, through the "ClosePreparedStatement" action;
- Automatically, by a server timeout.

The result should be wrapped in a google.protobuf.Any message.

---

## ActionCreatePreparedSubstraitPlanRequest

`struct` · `arrow_flight::sql::gen::ActionCreatePreparedSubstraitPlanRequest`

Also reachable as `arrow_flight::sql::ActionCreatePreparedSubstraitPlanRequest`

```rust
struct ActionCreatePreparedSubstraitPlanRequest
```

**Fields**: `plan`, `transaction_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn transaction_id(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionCreatePreparedSubstraitPlanRequest.md).



Request message for the "CreatePreparedSubstraitPlan" action on a Flight SQL enabled backend.

---

## ActionEndSavepointRequest

`struct` · `arrow_flight::sql::gen::ActionEndSavepointRequest`

Also reachable as `arrow_flight::sql::ActionEndSavepointRequest`

```rust
struct ActionEndSavepointRequest
```

**Fields**: `savepoint_id`, `action`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn action(&self) -> action_end_savepoint_request::EndSavepoint
fn set_action(&mut self, value: action_end_savepoint_request::EndSavepoint)
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionEndSavepointRequest.md).



Request message for the "EndSavepoint" action.

Release (RELEASE) the savepoint or rollback (ROLLBACK) to the
savepoint.

Releasing a savepoint invalidates that savepoint.  Rolling back to
a savepoint does not invalidate the savepoint, but invalidates all
savepoints created after the current savepoint.

---

## ActionEndTransactionRequest

`struct` · `arrow_flight::sql::gen::ActionEndTransactionRequest`

Also reachable as `arrow_flight::sql::ActionEndTransactionRequest`

```rust
struct ActionEndTransactionRequest
```

**Fields**: `transaction_id`, `action`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn action(&self) -> action_end_transaction_request::EndTransaction
fn set_action(&mut self, value: action_end_transaction_request::EndTransaction)
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.ActionEndTransactionRequest.md).



Request message for the "EndTransaction" action.

Commit (COMMIT) or rollback (ROLLBACK) the transaction.

If the action completes successfully, the transaction handle is
invalidated, as are all associated savepoints.

---

## CommandGetCatalogs

`struct` · `arrow_flight::sql::gen::CommandGetCatalogs`

Also reachable as `arrow_flight::sql::CommandGetCatalogs`

```rust
struct CommandGetCatalogs
```

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn into_builder(self) -> GetCatalogsBuilder
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetCatalogs.md).



Represents a request to retrieve the list of catalogs on a Flight SQL enabled backend.
The definition of a catalog depends on vendor/implementation. It is usually the database itself
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8 not null
>
The returned data should be ordered by catalog_name.

---

## CommandGetCrossReference

`struct` · `arrow_flight::sql::gen::CommandGetCrossReference`

Also reachable as `arrow_flight::sql::CommandGetCrossReference`

```rust
struct CommandGetCrossReference
```

**Fields**: `pk_catalog`, `pk_db_schema`, `pk_table`, `fk_catalog`, `fk_db_schema`, `fk_table`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn fk_catalog(&self) -> &str
fn fk_db_schema(&self) -> &str
fn pk_catalog(&self) -> &str
fn pk_db_schema(&self) -> &str
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetCrossReference.md).



Represents a request to retrieve a description of the foreign key columns in the given foreign key table that
reference the primary key or the columns representing a unique constraint of the parent table (could be the same
or a different table) on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  pk_catalog_name: utf8,
  pk_db_schema_name: utf8,
  pk_table_name: utf8 not null,
  pk_column_name: utf8 not null,
  fk_catalog_name: utf8,
  fk_db_schema_name: utf8,
  fk_table_name: utf8 not null,
  fk_column_name: utf8 not null,
  key_sequence: int32 not null,
  fk_key_name: utf8,
  pk_key_name: utf8,
  update_rule: uint8 not null,
  delete_rule: uint8 not null
>
The returned data should be ordered by pk_catalog_name, pk_db_schema_name, pk_table_name, pk_key_name, then key_sequence.
update_rule and delete_rule returns a byte that is equivalent to actions:
    - 0 = CASCADE
    - 1 = RESTRICT
    - 2 = SET NULL
    - 3 = NO ACTION
    - 4 = SET DEFAULT

---

## CommandGetDbSchemas

`struct` · `arrow_flight::sql::gen::CommandGetDbSchemas`

Also reachable as `arrow_flight::sql::CommandGetDbSchemas`

```rust
struct CommandGetDbSchemas
```

**Fields**: `catalog`, `db_schema_filter_pattern`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn catalog(&self) -> &str
fn db_schema_filter_pattern(&self) -> &str
fn into_builder(self) -> GetDbSchemasBuilder
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetDbSchemas.md).



Represents a request to retrieve the list of database schemas on a Flight SQL enabled backend.
The definition of a database schema depends on vendor/implementation. It is usually a collection of tables.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8,
  db_schema_name: utf8 not null
>
The returned data should be ordered by catalog_name, then db_schema_name.

---

## CommandGetExportedKeys

`struct` · `arrow_flight::sql::gen::CommandGetExportedKeys`

Also reachable as `arrow_flight::sql::CommandGetExportedKeys`

```rust
struct CommandGetExportedKeys
```

**Fields**: `catalog`, `db_schema`, `table`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn catalog(&self) -> &str
fn db_schema(&self) -> &str
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetExportedKeys.md).



Represents a request to retrieve a description of the foreign key columns that reference the given table's
primary key columns (the foreign keys exported by a table) of a table on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  pk_catalog_name: utf8,
  pk_db_schema_name: utf8,
  pk_table_name: utf8 not null,
  pk_column_name: utf8 not null,
  fk_catalog_name: utf8,
  fk_db_schema_name: utf8,
  fk_table_name: utf8 not null,
  fk_column_name: utf8 not null,
  key_sequence: int32 not null,
  fk_key_name: utf8,
  pk_key_name: utf8,
  update_rule: uint8 not null,
  delete_rule: uint8 not null
>
The returned data should be ordered by fk_catalog_name, fk_db_schema_name, fk_table_name, fk_key_name, then key_sequence.
update_rule and delete_rule returns a byte that is equivalent to actions declared on UpdateDeleteRules enum.

---

## CommandGetImportedKeys

`struct` · `arrow_flight::sql::gen::CommandGetImportedKeys`

Also reachable as `arrow_flight::sql::CommandGetImportedKeys`

```rust
struct CommandGetImportedKeys
```

**Fields**: `catalog`, `db_schema`, `table`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn catalog(&self) -> &str
fn db_schema(&self) -> &str
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetImportedKeys.md).



Represents a request to retrieve the foreign keys of a table on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  pk_catalog_name: utf8,
  pk_db_schema_name: utf8,
  pk_table_name: utf8 not null,
  pk_column_name: utf8 not null,
  fk_catalog_name: utf8,
  fk_db_schema_name: utf8,
  fk_table_name: utf8 not null,
  fk_column_name: utf8 not null,
  key_sequence: int32 not null,
  fk_key_name: utf8,
  pk_key_name: utf8,
  update_rule: uint8 not null,
  delete_rule: uint8 not null
>
The returned data should be ordered by pk_catalog_name, pk_db_schema_name, pk_table_name, pk_key_name, then key_sequence.
update_rule and delete_rule returns a byte that is equivalent to actions:
    - 0 = CASCADE
    - 1 = RESTRICT
    - 2 = SET NULL
    - 3 = NO ACTION
    - 4 = SET DEFAULT

---

## CommandGetPrimaryKeys

`struct` · `arrow_flight::sql::gen::CommandGetPrimaryKeys`

Also reachable as `arrow_flight::sql::CommandGetPrimaryKeys`

```rust
struct CommandGetPrimaryKeys
```

**Fields**: `catalog`, `db_schema`, `table`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn catalog(&self) -> &str
fn db_schema(&self) -> &str
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetPrimaryKeys.md).



Represents a request to retrieve the primary keys of a table on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8,
  db_schema_name: utf8,
  table_name: utf8 not null,
  column_name: utf8 not null,
  key_name: utf8,
  key_sequence: int32 not null
>
The returned data should be ordered by catalog_name, db_schema_name, table_name, key_name, then key_sequence.

---

## CommandGetSqlInfo

`struct` · `arrow_flight::sql::gen::CommandGetSqlInfo`

Also reachable as `arrow_flight::sql::CommandGetSqlInfo`

```rust
struct CommandGetSqlInfo
```

**Fields**: `info`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn into_builder(self, infos: &SqlInfoData) -> GetSqlInfoBuilder<'_>
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetSqlInfo.md).



Represents a metadata request. Used in the command member of FlightDescriptor
for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the metadata request.

The returned Arrow schema will be:
<
  info_name: uint32 not null,
  value: dense_union<
              string_value: utf8,
              bool_value: bool,
              bigint_value: int64,
              int32_bitmask: int32,
              string_list: list<string_data: utf8>
              int32_to_int32_list_map: map<key: int32, value: list<$data$: int32>>
>
where there is one row per requested piece of metadata information.

---

## CommandGetTableTypes

`struct` · `arrow_flight::sql::gen::CommandGetTableTypes`

Also reachable as `arrow_flight::sql::CommandGetTableTypes`

```rust
struct CommandGetTableTypes
```

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn into_builder(self) -> GetTableTypesBuilder
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetTableTypes.md).



Represents a request to retrieve the list of table types on a Flight SQL enabled backend.
The table types depend on vendor/implementation. It is usually used to separate tables from views or system tables.
TABLE, VIEW, and SYSTEM TABLE are commonly supported.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  table_type: utf8 not null
>
The returned data should be ordered by table_type.

---

## CommandGetTables

`struct` · `arrow_flight::sql::gen::CommandGetTables`

Also reachable as `arrow_flight::sql::CommandGetTables`

```rust
struct CommandGetTables
```

**Fields**: `catalog`, `db_schema_filter_pattern`, `table_name_filter_pattern`, `table_types`, `include_schema`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (4)

```rust
fn catalog(&self) -> &str
fn db_schema_filter_pattern(&self) -> &str
fn into_builder(self) -> GetTablesBuilder
fn table_name_filter_pattern(&self) -> &str
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetTables.md).



Represents a request to retrieve the list of tables, and optionally their schemas, on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned Arrow schema will be:
<
  catalog_name: utf8,
  db_schema_name: utf8,
  table_name: utf8 not null,
  table_type: utf8 not null,
  \[optional\] table_schema: bytes not null (schema of the table as described in Schema.fbs::Schema,
                                           it is serialized as an IPC message.)
>
Fields on table_schema may contain the following metadata:
  - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
  - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
  - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
  - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
  - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
  - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
  - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
  - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
  - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
  - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
The returned data should be ordered by catalog_name, db_schema_name, table_name, then table_type, followed by table_schema if requested.

---

## CommandGetXdbcTypeInfo

`struct` · `arrow_flight::sql::gen::CommandGetXdbcTypeInfo`

Also reachable as `arrow_flight::sql::CommandGetXdbcTypeInfo`

```rust
struct CommandGetXdbcTypeInfo
```

**Fields**: `data_type`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn data_type(&self) -> i32
fn into_builder(self, infos: &XdbcTypeInfoData) -> GetXdbcTypeInfoBuilder<'_>
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandGetXdbcTypeInfo.md).



Represents a request to retrieve information about data type supported on a Flight SQL enabled backend.
Used in the command member of FlightDescriptor for the following RPC calls:
  - GetSchema: return the schema of the query.
  - GetFlightInfo: execute the catalog metadata request.

The returned schema will be:
<
   type_name: utf8 not null (The name of the data type, for example: VARCHAR, INTEGER, etc),
   data_type: int32 not null (The SQL data type),
   column_size: int32 (The maximum size supported by that column.
                       In case of exact numeric types, this represents the maximum precision.
                       In case of string types, this represents the character length.
                       In case of datetime data types, this represents the length in characters of the string representation.
                       NULL is returned for data types where column size is not applicable.),
   literal_prefix: utf8 (Character or characters used to prefix a literal, NULL is returned for
                         data types where a literal prefix is not applicable.),
   literal_suffix: utf8 (Character or characters used to terminate a literal,
                         NULL is returned for data types where a literal suffix is not applicable.),
   create_params: list<utf8 not null>
                        (A list of keywords corresponding to which parameters can be used when creating
                         a column for that specific type.
                         NULL is returned if there are no parameters for the data type definition.),
   nullable: int32 not null (Shows if the data type accepts a NULL value. The possible values can be seen in the
                             Nullable enum.),
   case_sensitive: bool not null (Shows if a character data type is case-sensitive in collations and comparisons),
   searchable: int32 not null (Shows how the data type is used in a WHERE clause. The possible values can be seen in the
                               Searchable enum.),
   unsigned_attribute: bool (Shows if the data type is unsigned. NULL is returned if the attribute is
                             not applicable to the data type or the data type is not numeric.),
   fixed_prec_scale: bool not null (Shows if the data type has predefined fixed precision and scale.),
   auto_increment: bool (Shows if the data type is auto incremental. NULL is returned if the attribute
                         is not applicable to the data type or the data type is not numeric.),
   local_type_name: utf8 (Localized version of the data source-dependent name of the data type. NULL
                          is returned if a localized name is not supported by the data source),
   minimum_scale: int32 (The minimum scale of the data type on the data source.
                         If a data type has a fixed scale, the MINIMUM_SCALE and MAXIMUM_SCALE
                         columns both contain this value. NULL is returned if scale is not applicable.),
   maximum_scale: int32 (The maximum scale of the data type on the data source.
                         NULL is returned if scale is not applicable.),
   sql_data_type: int32 not null (The value of the SQL DATA TYPE which has the same values
                                  as data_type value. Except for interval and datetime, which
                                  uses generic values. More info about those types can be
                                  obtained through datetime_subcode. The possible values can be seen
                                  in the XdbcDataType enum.),
   datetime_subcode: int32 (Only used when the SQL DATA TYPE is interval or datetime. It contains
                            its sub types. For type different from interval and datetime, this value
                            is NULL. The possible values can be seen in the XdbcDatetimeSubcode enum.),
   num_prec_radix: int32 (If the data type is an approximate numeric type, this column contains
                          the value 2 to indicate that COLUMN_SIZE specifies a number of bits. For
                          exact numeric types, this column contains the value 10 to indicate that
                          column size specifies a number of decimal digits. Otherwise, this column is NULL.),
   interval_precision: int32 (If the data type is an interval data type, then this column contains the value
                              of the interval leading precision. Otherwise, this column is NULL. This fields
                              is only relevant to be used by ODBC).
>
The returned data should be ordered by data_type and then by type_name.

---

## CommandPreparedStatementQuery

`struct` · `arrow_flight::sql::gen::CommandPreparedStatementQuery`

Also reachable as `arrow_flight::sql::CommandPreparedStatementQuery`

```rust
struct CommandPreparedStatementQuery
```

**Fields**: `prepared_statement_handle`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandPreparedStatementQuery.md).



Represents an instance of executing a prepared statement. Used in the command member of FlightDescriptor for
the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
    Fields on this schema may contain the following metadata:
    - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
    - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
    - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
    - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
    - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
    - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
    - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.

    If the schema is retrieved after parameter values have been bound with DoPut, then the server should account
    for the parameters when determining the schema.
  - DoPut: bind parameter values. All of the bound parameter sets will be executed as a single atomic execution.
  - GetFlightInfo: execute the prepared statement instance.

---

## CommandPreparedStatementUpdate

`struct` · `arrow_flight::sql::gen::CommandPreparedStatementUpdate`

Also reachable as `arrow_flight::sql::CommandPreparedStatementUpdate`

```rust
struct CommandPreparedStatementUpdate
```

**Fields**: `prepared_statement_handle`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandPreparedStatementUpdate.md).



Represents a SQL update query. Used in the command member of FlightDescriptor
for the RPC call DoPut to cause the server to execute the included
prepared statement handle as an update.

---

## CommandStatementIngest

`struct` · `arrow_flight::sql::gen::CommandStatementIngest`

Also reachable as `arrow_flight::sql::CommandStatementIngest`

```rust
struct CommandStatementIngest
```

**Fields**: `table_definition_options`, `table`, `schema`, `catalog`, `temporary`, `transaction_id`, `options`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn catalog(&self) -> &str
fn schema(&self) -> &str
fn transaction_id(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandStatementIngest.md).



Represents a bulk ingestion request. Used in the command member of FlightDescriptor
for the the RPC call DoPut to cause the server load the contents of the stream's
FlightData into the target destination.

---

## CommandStatementQuery

`struct` · `arrow_flight::sql::gen::CommandStatementQuery`

Also reachable as `arrow_flight::sql::CommandStatementQuery`

```rust
struct CommandStatementQuery
```

**Fields**: `query`, `transaction_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn transaction_id(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandStatementQuery.md).



Represents a SQL query. Used in the command member of FlightDescriptor
for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
    Fields on this schema may contain the following metadata:
    - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
    - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
    - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
    - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
    - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
    - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
    - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
  - GetFlightInfo: execute the query.

---

## CommandStatementSubstraitPlan

`struct` · `arrow_flight::sql::gen::CommandStatementSubstraitPlan`

Also reachable as `arrow_flight::sql::CommandStatementSubstraitPlan`

```rust
struct CommandStatementSubstraitPlan
```

**Fields**: `plan`, `transaction_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn transaction_id(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandStatementSubstraitPlan.md).



Represents a Substrait plan. Used in the command member of FlightDescriptor
for the following RPC calls:
  - GetSchema: return the Arrow schema of the query.
    Fields on this schema may contain the following metadata:
    - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
    - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
    - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
    - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
    - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
    - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
    - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
    - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
  - GetFlightInfo: execute the query.
  - DoPut: execute the query.

---

## CommandStatementUpdate

`struct` · `arrow_flight::sql::gen::CommandStatementUpdate`

Also reachable as `arrow_flight::sql::CommandStatementUpdate`

```rust
struct CommandStatementUpdate
```

**Fields**: `query`, `transaction_id`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn transaction_id(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.CommandStatementUpdate.md).



Represents a SQL update query. Used in the command member of FlightDescriptor
for the RPC call DoPut to cause the server to execute the included SQL update.

---

## DoPutPreparedStatementResult

`struct` · `arrow_flight::sql::gen::DoPutPreparedStatementResult`

Also reachable as `arrow_flight::sql::DoPutPreparedStatementResult`

```rust
struct DoPutPreparedStatementResult
```

**Fields**: `prepared_statement_handle`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn prepared_statement_handle(&self) -> &[u8]
```

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.DoPutPreparedStatementResult.md).


An *optional* response returned when `DoPut` is called with `CommandPreparedStatementQuery`.

*Note on legacy behavior*: previous versions of the protocol did not return any result for
this command, and that behavior should still be supported by clients. In that case, the client
can continue as though the fields in this message were not provided or set to sensible default values.

---

## DoPutUpdateResult

`struct` · `arrow_flight::sql::gen::DoPutUpdateResult`

Also reachable as `arrow_flight::sql::DoPutUpdateResult`

```rust
struct DoPutUpdateResult
```

**Fields**: `record_count`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.DoPutUpdateResult.md).



Returned from the RPC call DoPut when a CommandStatementUpdate,
CommandPreparedStatementUpdate, or CommandStatementIngest was
in the request, containing results from the update.

---

## SubstraitPlan

`struct` · `arrow_flight::sql::gen::SubstraitPlan`

Also reachable as `arrow_flight::sql::SubstraitPlan`

```rust
struct SubstraitPlan
```

**Fields**: `plan`, `version`

**Implements**: `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.SubstraitPlan.md).



An embedded message describing a Substrait plan to execute.

---

## TicketStatementQuery

`struct` · `arrow_flight::sql::gen::TicketStatementQuery`

Also reachable as `arrow_flight::sql::TicketStatementQuery`

```rust
struct TicketStatementQuery
```

**Fields**: `statement_handle`

**Implements**: `arrow_flight::sql::ProstMessageExt`, `prost::message::Message`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**via `arrow_flight::sql::ProstMessageExt`**

```rust
fn as_any(&self) -> Any
fn type_url() -> &'static str
```

**via `prost::message::Message`**

```rust
fn clear(&mut self)
fn encoded_len(&self) -> usize
```

[Full member, field, variant and typed contracts](../operations/arrow_flight.sql.gen.TicketStatementQuery.md).


*
Represents a ticket resulting from GetFlightInfo with a CommandStatementQuery.
This should be used only once and treated as an opaque value, that is, clients should not attempt to parse this.

---
