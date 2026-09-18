# `sqlparser::ast::dml::Insert`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.Insert.json).

<a id="op-176a18910d593b24e56e3ccf"></a>
## Insert

`struct` · `sqlparser::ast::dml::Insert` · sqlparser 0.62.0

```rust
struct Insert
```

Source: `src/ast/dml.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

INSERT statement.

<a id="op-f5641ffdc5da0cb1aa9f7039"></a>
## after_columns

`struct_field` · `sqlparser::ast::dml::Insert::after_columns` · sqlparser 0.62.0

```rust
after_columns: Vec<super::Ident>
```

Source: `src/ast/dml.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns defined after PARTITION

<a id="op-b985645027c43d034ee3a156"></a>
## assignments

`struct_field` · `sqlparser::ast::dml::Insert::assignments` · sqlparser 0.62.0

```rust
assignments: Vec<super::Assignment>
```

Source: `src/ast/dml.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL `INSERT INTO ... SET`
See: <https://dev.mysql.com/doc/refman/8.4/en/insert.html>

<a id="op-e5b0cbb2086f67d7b3f7e7f4"></a>
## clone

`function` · `sqlparser::ast::dml::Insert::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Insert
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6a62a2a5c65aaa93ada90ae"></a>
## cmp

`function` · `sqlparser::ast::dml::Insert::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Insert) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 51], "end": [41, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5829a3d88d2862f951528a5e"></a>
## columns

`struct_field` · `sqlparser::ast::dml::Insert::columns` · sqlparser 0.62.0

```rust
columns: Vec<super::ObjectName>
```

Source: `src/ast/dml.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

COLUMNS

<a id="op-6f2b679d5bd20db2dd19b2ea"></a>
## deserialize

`function` · `sqlparser::ast::dml::Insert::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 49], "end": [42, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c8474526f9673a4746aa783"></a>
## eq

`function` · `sqlparser::ast::dml::Insert::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Insert) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 24], "end": [41, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-750515615fc64b3304c19a2a"></a>
## fmt

`function` · `sqlparser::ast::dml::Insert::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-887e01bbbb3e6b8cfe1fcd2e"></a>
## fmt

`function` · `sqlparser::ast::dml::Insert::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 1], "end": [276, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d108803037f576962e3c34d0"></a>
## format_clause

`struct_field` · `sqlparser::ast::dml::Insert::format_clause` · sqlparser 0.62.0

```rust
format_clause: Option<super::query::InputFormatClause>
```

Source: `src/ast/dml.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Format for `INSERT` statement when not using standard SQL format. Can be e.g. `CSV`,
`JSON`, `JSONAsString`, `LineAsString` and more.

ClickHouse syntax: `INSERT INTO tbl FORMAT JSONEachRow {"foo": 1, "bar": 2}, {"foo": 3}`

[ClickHouse formats JSON insert](https://clickhouse.com/docs/en/interfaces/formats#json-inserting-data)

<a id="op-2a7ee505397f6654f1d0f0ea"></a>
## has_table_keyword

`struct_field` · `sqlparser::ast::dml::Insert::has_table_keyword` · sqlparser 0.62.0

```rust
has_table_keyword: bool
```

Source: `src/ast/dml.rs:77`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

whether the insert has the table keyword (Hive)

<a id="op-b48522ad911ff48864e80130"></a>
## hash

`function` · `sqlparser::ast::dml::Insert::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 56], "end": [41, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3c09ec39e7172b692000239"></a>
## ignore

`struct_field` · `sqlparser::ast::dml::Insert::ignore` · sqlparser 0.62.0

```rust
ignore: bool
```

Source: `src/ast/dml.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only for mysql

<a id="op-75138ec614bd7a6c61e8b20d"></a>
## insert_alias

`struct_field` · `sqlparser::ast::dml::Insert::insert_alias` · sqlparser 0.62.0

```rust
insert_alias: Option<super::InsertAliases>
```

Source: `src/ast/dml.rs:90`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only for mysql

<a id="op-199ddc8167f5f64cea112de2"></a>
## insert_token

`struct_field` · `sqlparser::ast::dml::Insert::insert_token` · sqlparser 0.62.0

```rust
insert_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token for the `INSERT` keyword (or its substitutes)

<a id="op-e6ebcd6771f9341e5019f039"></a>
## into

`struct_field` · `sqlparser::ast::dml::Insert::into` · sqlparser 0.62.0

```rust
into: bool
```

Source: `src/ast/dml.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

INTO - optional keyword

<a id="op-31eb08a4871c5082f74c5f3d"></a>
## multi_table_else_clause

`struct_field` · `sqlparser::ast::dml::Insert::multi_table_else_clause` · sqlparser 0.62.0

```rust
multi_table_else_clause: Option<Vec<MultiTableInsertIntoClause>>
```

Source: `src/ast/dml.rs:127`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For conditional multi-table insert: ELSE clause

See: <https://docs.snowflake.com/en/sql-reference/sql/insert-multi-table>

<a id="op-af75444a88810869ec8ae42a"></a>
## multi_table_insert_type

`struct_field` · `sqlparser::ast::dml::Insert::multi_table_insert_type` · sqlparser 0.62.0

```rust
multi_table_insert_type: Option<MultiTableInsertType>
```

Source: `src/ast/dml.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For Snowflake multi-table insert: specifies the type (`ALL` or `FIRST`)

- `None` means this is a regular single-table INSERT
- `Some(All)` means `INSERT ALL` (all matching WHEN clauses are executed)
- `Some(First)` means `INSERT FIRST` (only the first matching WHEN clause is executed)

See: <https://docs.snowflake.com/en/sql-reference/sql/insert-multi-table>

<a id="op-27ff2b6838ee10d0eb79894a"></a>
## multi_table_into_clauses

`struct_field` · `sqlparser::ast::dml::Insert::multi_table_into_clauses` · sqlparser 0.62.0

```rust
multi_table_into_clauses: Vec<MultiTableInsertIntoClause>
```

Source: `src/ast/dml.rs:117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For multi-table insert: additional INTO clauses (unconditional)

Used for `INSERT ALL INTO t1 INTO t2 ... SELECT ...`

See: <https://docs.snowflake.com/en/sql-reference/sql/insert-multi-table>

<a id="op-34a0de7055e7eabeb2c2280c"></a>
## multi_table_when_clauses

`struct_field` · `sqlparser::ast::dml::Insert::multi_table_when_clauses` · sqlparser 0.62.0

```rust
multi_table_when_clauses: Vec<MultiTableInsertWhenClause>
```

Source: `src/ast/dml.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

For conditional multi-table insert: WHEN clauses

Used for `INSERT ALL/FIRST WHEN cond THEN INTO t1 ... SELECT ...`

See: <https://docs.snowflake.com/en/sql-reference/sql/insert-multi-table>

<a id="op-0757c8b9df7a8860dcfa4f38"></a>
## on

`struct_field` · `sqlparser::ast::dml::Insert::on` · sqlparser 0.62.0

```rust
on: Option<super::OnInsert>
```

Source: `src/ast/dml.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ON INSERT

<a id="op-77b7094ab65f86e373314d75"></a>
## optimizer_hints

`struct_field` · `sqlparser::ast::dml::Insert::optimizer_hints` · sqlparser 0.62.0

```rust
optimizer_hints: Vec<super::OptimizerHint>
```

Source: `src/ast/dml.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query optimizer hints

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/optimizer-hints.html)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Comments.html#GUID-D316D545-89E2-4D54-977F-FC97815CD62E)

<a id="op-f1564eee9d30f55e42e85155"></a>
## or

`struct_field` · `sqlparser::ast::dml::Insert::or` · sqlparser 0.62.0

```rust
or: Option<super::SqliteOnConflict>
```

Source: `src/ast/dml.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only for Sqlite

<a id="op-8195bd8cbd1ff4377ec64e61"></a>
## output

`struct_field` · `sqlparser::ast::dml::Insert::output` · sqlparser 0.62.0

```rust
output: Option<OutputClause>
```

Source: `src/ast/dml.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

OUTPUT (MSSQL)
See <https://learn.microsoft.com/en-us/sql/t-sql/queries/output-clause-transact-sql>

<a id="op-a3cda6206f20056d49f22176"></a>
## overwrite

`struct_field` · `sqlparser::ast::dml::Insert::overwrite` · sqlparser 0.62.0

```rust
overwrite: bool
```

Source: `src/ast/dml.rs:66`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Overwrite (Hive)

<a id="op-7cdf206f46d485b2eacb5eb8"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::Insert::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Insert) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 35], "end": [41, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7ff3b5730f4c36b41124faa"></a>
## partitioned

`struct_field` · `sqlparser::ast::dml::Insert::partitioned` · sqlparser 0.62.0

```rust
partitioned: Option<Vec<super::Expr>>
```

Source: `src/ast/dml.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

partitioned insert (Hive)

<a id="op-8ec6faf60f05f3f85ec7967d"></a>
## priority

`struct_field` · `sqlparser::ast::dml::Insert::priority` · sqlparser 0.62.0

```rust
priority: Option<super::MysqlInsertPriority>
```

Source: `src/ast/dml.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only for mysql

<a id="op-8fa26e2d05e194aaa60982cd"></a>
## replace_into

`struct_field` · `sqlparser::ast::dml::Insert::replace_into` · sqlparser 0.62.0

```rust
replace_into: bool
```

Source: `src/ast/dml.rs:86`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only for mysql

<a id="op-a515a6916e913fba4205ccca"></a>
## returning

`struct_field` · `sqlparser::ast::dml::Insert::returning` · sqlparser 0.62.0

```rust
returning: Option<Vec<super::SelectItem>>
```

Source: `src/ast/dml.rs:81`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RETURNING

<a id="op-01cc11aed8a346d403df53e0"></a>
## serialize

`function` · `sqlparser::ast::dml::Insert::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 38], "end": [42, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f072e2a510763796a931d0f"></a>
## settings

`struct_field` · `sqlparser::ast::dml::Insert::settings` · sqlparser 0.62.0

```rust
settings: Option<Vec<super::Setting>>
```

Source: `src/ast/dml.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Settings used for ClickHouse.

ClickHouse syntax: `INSERT INTO tbl SETTINGS format_template_resultset = '/some/path/resultset.format'`

[ClickHouse `INSERT INTO`](https://clickhouse.com/docs/en/sql-reference/statements/insert-into)

<a id="op-d836fce8c8c8ace2a3762439"></a>
## source

`struct_field` · `sqlparser::ast::dml::Insert::source` · sqlparser 0.62.0

```rust
source: Option<Box<super::Query>>
```

Source: `src/ast/dml.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A SQL query that specifies what to insert

<a id="op-44d5dc4d333d6a9d6abc9e8f"></a>
## span

`function` · `sqlparser::ast::dml::Insert::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "super::Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1326, 1], "end": [1371, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a74f0b16922dbed98e4a8b52"></a>
## table

`struct_field` · `sqlparser::ast::dml::Insert::table` · sqlparser 0.62.0

```rust
table: super::TableObject
```

Source: `src/ast/dml.rs:59`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TABLE

<a id="op-49c151776fbc721eb139c033"></a>
## table_alias

`struct_field` · `sqlparser::ast::dml::Insert::table_alias` · sqlparser 0.62.0

```rust
table_alias: Option<super::TableAliasWithoutColumns>
```

Source: `src/ast/dml.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`table_name as foo` (for PostgreSQL)
`table_name foo` (for Oracle)

<a id="op-2954bcf63c91cb8cc662c325"></a>
## visit

`function` · `sqlparser::ast::dml::Insert::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 40], "end": [43, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e67aaf0cbf0149c6fc6390c9"></a>
## visit

`function` · `sqlparser::ast::dml::Insert::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Insert", "path": "Insert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 47], "end": [43, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
