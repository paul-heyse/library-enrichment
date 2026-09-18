# `sqlparser::ast::ddl::ColumnOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnOption.json).

<a id="op-ff25115c2a6301ade2d4d22c"></a>
## ColumnOption

`enum` · `sqlparser::ast::ddl::ColumnOption` · sqlparser 0.62.0

```rust
enum ColumnOption
```

Source: `src/ast/ddl.rs:1894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ColumnOption`s are modifiers that follow a column definition in a `CREATE
TABLE` statement.

<a id="op-765c99a48c0f13849e04bea8"></a>
## Alias

`variant` · `sqlparser::ast::ddl::ColumnOption::Alias` · sqlparser 0.62.0

```rust
Alias
```

Source: `src/ast/ddl.rs:1914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALIAS <expr>`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/create/table#default_values)

<a id="op-a0c0862fca1cc37d93de1315"></a>
## CharacterSet

`variant` · `sqlparser::ast::ddl::ColumnOption::CharacterSet` · sqlparser 0.62.0

```rust
CharacterSet
```

Source: `src/ast/ddl.rs:1935`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CHARACTER SET <name>` column option

<a id="op-b4f5e37b0d73d8b4830fbb01"></a>
## Check

`variant` · `sqlparser::ast::ddl::ColumnOption::Check` · sqlparser 0.62.0

```rust
Check
```

Source: `src/ast/ddl.rs:1929`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CHECK (<expr>)`

<a id="op-62bb99ea8b429491b9852de7"></a>
## Collation

`variant` · `sqlparser::ast::ddl::ColumnOption::Collation` · sqlparser 0.62.0

```rust
Collation
```

Source: `src/ast/ddl.rs:1937`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COLLATE <name>` column option

<a id="op-05623c0e1e712beac2c89131"></a>
## Comment

`variant` · `sqlparser::ast::ddl::ColumnOption::Comment` · sqlparser 0.62.0

```rust
Comment
```

Source: `src/ast/ddl.rs:1939`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COMMENT '<text>'` column option

<a id="op-c4ae509b4dc292ff1c5e2a44"></a>
## Default

`variant` · `sqlparser::ast::ddl::ColumnOption::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/ddl.rs:1900`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DEFAULT <restricted-expr>`

<a id="op-8e9ce9b5c2e8838ec5048244"></a>
## DialectSpecific

`variant` · `sqlparser::ast::ddl::ColumnOption::DialectSpecific` · sqlparser 0.62.0

```rust
DialectSpecific
```

Source: `src/ast/ddl.rs:1933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dialect-specific options, such as:
- MySQL's `AUTO_INCREMENT` or SQLite's `AUTOINCREMENT`
- ...

<a id="op-58a476e5fbc1841ca35dec70"></a>
## Ephemeral

`variant` · `sqlparser::ast::ddl::ColumnOption::Ephemeral` · sqlparser 0.62.0

```rust
Ephemeral
```

Source: `src/ast/ddl.rs:1910`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EPHEMERAL [<expr>]`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/create/table#default_values)

<a id="op-daf88ded311431ab7432900d"></a>
## ForeignKey

`variant` · `sqlparser::ast::ddl::ColumnOption::ForeignKey` · sqlparser 0.62.0

```rust
ForeignKey
```

Source: `src/ast/ddl.rs:1927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A referential integrity constraint (`REFERENCES <foreign_table> (<referred_columns>)
[ MATCH { FULL | PARTIAL | SIMPLE } ]
{ [ON DELETE <referential_action>] [ON UPDATE <referential_action>] |
  [ON UPDATE <referential_action>] [ON DELETE <referential_action>]
}
[<constraint_characteristics>]
`).

<a id="op-8bef7e3d30aa1c470fe05e0c"></a>
## Generated

`variant` · `sqlparser::ast::ddl::ColumnOption::Generated` · sqlparser 0.62.0

```rust
Generated
```

Source: `src/ast/ddl.rs:1944`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`Generated`s are modifiers that follow a column definition in a `CREATE
TABLE` statement.

<a id="op-0399e3a171ff3de9e8ed178b"></a>
## Identity

`variant` · `sqlparser::ast::ddl::ColumnOption::Identity` · sqlparser 0.62.0

```rust
Identity
```

Source: `src/ast/ddl.rs:1971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Creates an identity or an autoincrement column in a table.
Syntax
```sql
{ IDENTITY | AUTOINCREMENT } [ (seed , increment) | START num INCREMENT num ] [ ORDER | NOORDER ]
```
[MS SQL Server]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-transact-sql-identity-property
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-24ac50e13daf13adeb6d8d17"></a>
## Invisible

`variant` · `sqlparser::ast::ddl::ColumnOption::Invisible` · sqlparser 0.62.0

```rust
Invisible
```

Source: `src/ast/ddl.rs:2003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL specific: Column is invisible via SELECT *
Syntax:
```sql
CREATE TABLE t (foo INT, bar INT INVISIBLE);
```
[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/invisible-columns.html

<a id="op-3abd29d21abf22bc389d0469"></a>
## Materialized

`variant` · `sqlparser::ast::ddl::ColumnOption::Materialized` · sqlparser 0.62.0

```rust
Materialized
```

Source: `src/ast/ddl.rs:1906`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MATERIALIZE <expr>`
Syntax: `b INT MATERIALIZE (a + 1)`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/create/table#default_values)

<a id="op-acc058b34739710f6772870c"></a>
## NotNull

`variant` · `sqlparser::ast::ddl::ColumnOption::NotNull` · sqlparser 0.62.0

```rust
NotNull
```

Source: `src/ast/ddl.rs:1898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NOT NULL`

<a id="op-4b33e64887b3d460656677b3"></a>
## Null

`variant` · `sqlparser::ast::ddl::ColumnOption::Null` · sqlparser 0.62.0

```rust
Null
```

Source: `src/ast/ddl.rs:1896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NULL`

<a id="op-264295f492077ef3a72a753b"></a>
## OnConflict

`variant` · `sqlparser::ast::ddl::ColumnOption::OnConflict` · sqlparser 0.62.0

```rust
OnConflict
```

Source: `src/ast/ddl.rs:1974`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQLite specific: ON CONFLICT option on column definition
<https://www.sqlite.org/lang_conflict.html>

<a id="op-23f6a9ebe0577083c4734e40"></a>
## OnUpdate

`variant` · `sqlparser::ast::ddl::ColumnOption::OnUpdate` · sqlparser 0.62.0

```rust
OnUpdate
```

Source: `src/ast/ddl.rs:1941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ON UPDATE <expr>` column option

<a id="op-23c20fb342119ab27f2d1741"></a>
## Options

`variant` · `sqlparser::ast::ddl::ColumnOption::Options` · sqlparser 0.62.0

```rust
Options
```

Source: `src/ast/ddl.rs:1963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery specific: Explicit column options in a view [1] or table [2]
Syntax
```sql
OPTIONS(description="field desc")
```
[1]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#view_column_option_list
[2]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#column_option_list

<a id="op-935f9f4c653e0a0d1333ce4e"></a>
## Policy

`variant` · `sqlparser::ast::ddl::ColumnOption::Policy` · sqlparser 0.62.0

```rust
Policy
```

Source: `src/ast/ddl.rs:1982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake specific: an option of specifying security masking or projection policy to set on a column.
Syntax:
```sql
[ WITH ] MASKING POLICY <policy_name> [ USING ( <col_name> , <cond_col1> , ... ) ]
[ WITH ] PROJECTION POLICY <policy_name>
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-00b29a7a0cff13db9986c9a8"></a>
## PrimaryKey

`variant` · `sqlparser::ast::ddl::ColumnOption::PrimaryKey` · sqlparser 0.62.0

```rust
PrimaryKey
```

Source: `src/ast/ddl.rs:1917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PRIMARY KEY [<constraint_characteristics>]`

<a id="op-3e80d55a8122f967f9eff4ed"></a>
## Srid

`variant` · `sqlparser::ast::ddl::ColumnOption::Srid` · sqlparser 0.62.0

```rust
Srid
```

Source: `src/ast/ddl.rs:1996`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL specific: Spatial reference identifier
Syntax:
```sql
CREATE TABLE geom (g GEOMETRY NOT NULL SRID 4326);
```
[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/creating-spatial-indexes.html

<a id="op-e2be4ec9c636f4b8146f6c8a"></a>
## Tags

`variant` · `sqlparser::ast::ddl::ColumnOption::Tags` · sqlparser 0.62.0

```rust
Tags
```

Source: `src/ast/ddl.rs:1989`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake specific: Specifies the tag name and the tag string value.
Syntax:
```sql
[ WITH ] TAG ( <tag_name> = '<tag_value>' [ , <tag_name> = '<tag_value>' , ... ] )
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-9d6a4c1267646e2823ad77c7"></a>
## Unique

`variant` · `sqlparser::ast::ddl::ColumnOption::Unique` · sqlparser 0.62.0

```rust
Unique
```

Source: `src/ast/ddl.rs:1919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UNIQUE [<constraint_characteristics>]`

<a id="op-01a2eca5512bd5d3845fe7d3"></a>
## clone

`function` · `sqlparser::ast::ddl::ColumnOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ColumnOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1891, 17], "end": [1891, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5003711c56a6d74eb876ca03"></a>
## cmp

`function` · `sqlparser::ast::ddl::ColumnOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ColumnOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1891, 51], "end": [1891, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dd04debc4dbf5970d7839df"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ColumnOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1892, 49], "end": [1892, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7562fd055e58ee613b4b36f1"></a>
## eq

`function` · `sqlparser::ast::ddl::ColumnOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ColumnOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1891, 24], "end": [1891, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11293d7f4ffeea70812beba0"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1891, 10], "end": [1891, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-749d458bee455d56185c1621"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2029, 1], "end": [2155, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0140bf9efb88a633208d43c0"></a>
## from

`function` · `sqlparser::ast::ddl::ColumnOption::from` · sqlparser 0.62.0

```rust
fn from(c: CheckConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2018, 1], "end": [2022, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:2019`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ca8684b2fa2bf1f7687da1a"></a>
## from

`function` · `sqlparser::ast::ddl::ColumnOption::from` · sqlparser 0.62.0

```rust
fn from(c: UniqueConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2006, 1], "end": [2010, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:2007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f6b203519a9a70ffea1e4c6"></a>
## from

`function` · `sqlparser::ast::ddl::ColumnOption::from` · sqlparser 0.62.0

```rust
fn from(c: PrimaryKeyConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2012, 1], "end": [2016, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:2013`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8894aada4418e6ef490c30d"></a>
## from

`function` · `sqlparser::ast::ddl::ColumnOption::from` · sqlparser 0.62.0

```rust
fn from(fk: ForeignKeyConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2023, 1], "end": [2027, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/ddl.rs:2024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12e110bbab65775820784840"></a>
## hash

`function` · `sqlparser::ast::ddl::ColumnOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1891, 56], "end": [1891, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2e1bd39301f969df58d41d2"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ColumnOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ColumnOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1891, 35], "end": [1891, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff1dcdcbff67a7671b3c5e4e"></a>
## serialize

`function` · `sqlparser::ast::ddl::ColumnOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1892, 38], "end": [1892, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a5fae1e28c2cb5a45a68fa7"></a>
## span

`function` · `sqlparser::ast::ddl::ColumnOption::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "super::ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [817, 1], "end": [845, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:818`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34520444214e3404d4f122db"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1893, 40], "end": [1893, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e490214d8cbaa3359a1e6913"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOption", "path": "ColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1893, 47], "end": [1893, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
