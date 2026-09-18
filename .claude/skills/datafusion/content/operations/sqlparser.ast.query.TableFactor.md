# `sqlparser::ast::query::TableFactor`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.json).

<a id="op-35b85cae81505b709989d539"></a>
## TableFactor

`enum` · `sqlparser::ast::query::TableFactor` · sqlparser 0.62.0

```rust
enum TableFactor
```

Source: `src/ast/query.rs:1463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table name or a parenthesized subquery with an optional alias

<a id="op-f0eba3c2cdce192bc002480b"></a>
## Derived

`variant` · `sqlparser::ast::query::TableFactor::Derived` · sqlparser 0.62.0

```rust
Derived
```

Source: `src/ast/query.rs:1500`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A derived table (a parenthesized subquery), optionally `LATERAL`.

<a id="op-dcf569696e0d3170393efd07"></a>
## Function

`variant` · `sqlparser::ast::query::TableFactor::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/query.rs:1518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`e.g. LATERAL FLATTEN(<args>)[ AS <alias> ]`

<a id="op-9886bf1f4d6547f5b4f099e2"></a>
## JsonTable

`variant` · `sqlparser::ast::query::TableFactor::JsonTable` · sqlparser 0.62.0

```rust
JsonTable
```

Source: `src/ast/query.rs:1567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `JSON_TABLE` table-valued function.
Part of the SQL standard, but implemented only by MySQL, Oracle, and DB2.

<https://modern-sql.com/blog/2017-06/whats-new-in-sql-2016#json_table>
<https://dev.mysql.com/doc/refman/8.0/en/json-table-functions.html#function_json-table>

```sql
SELECT * FROM JSON_TABLE(
   '[{"a": 1, "b": 2}, {"a": 3, "b": 4}]',
   '$[*]' COLUMNS(
       a INT PATH '$.a' DEFAULT '0' ON EMPTY,
       b INT PATH '$.b' NULL ON ERROR
    )
) AS jt;
````

<a id="op-d9fbbce8cbff12cb6b0acf49"></a>
## MatchRecognize

`variant` · `sqlparser::ast::query::TableFactor::MatchRecognize` · sqlparser 0.62.0

```rust
MatchRecognize
```

Source: `src/ast/query.rs:1660`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `MATCH_RECOGNIZE` operation on a table.

See <https://docs.snowflake.com/en/sql-reference/constructs/match_recognize>.

<a id="op-d3cb30431706fed462100d17"></a>
## NestedJoin

`variant` · `sqlparser::ast::query::TableFactor::NestedJoin` · sqlparser 0.62.0

```rust
NestedJoin
```

Source: `src/ast/query.rs:1606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a parenthesized table factor. The SQL spec only allows a
join expression (`(foo <JOIN> bar [ <JOIN> baz ... ])`) to be nested,
possibly several times.

The parser may also accept non-standard nesting of bare tables for some
dialects, but the information about such nesting is stripped from AST.

<a id="op-e7a4cfa0b83d3c307bbbb779"></a>
## OpenJsonTable

`variant` · `sqlparser::ast::query::TableFactor::OpenJsonTable` · sqlparser 0.62.0

```rust
OpenJsonTable
```

Source: `src/ast/query.rs:1588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The MSSQL's `OPENJSON` table-valued function.

```sql
OPENJSON( jsonExpression [ , path ] )  [ <with_clause> ]

<with_clause> ::= WITH ( { colName type [ column_path ] [ AS JSON ] } [ ,...n ] )
````

Reference: <https://learn.microsoft.com/en-us/sql/t-sql/functions/openjson-transact-sql?view=sql-server-ver16#syntax>

<a id="op-827310dc9acdaf7b5ad5e69e"></a>
## Pivot

`variant` · `sqlparser::ast::query::TableFactor::Pivot` · sqlparser 0.62.0

```rust
Pivot
```

Source: `src/ast/query.rs:1618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents PIVOT operation on a table.
For example `FROM monthly_sales PIVOT(sum(amount) FOR MONTH IN ('JAN', 'FEB'))`

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#pivot_operator)
[Snowflake](https://docs.snowflake.com/en/sql-reference/constructs/pivot)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/SELECT.html#GUID-CFA006CA-6FF1-4972-821E-6996142A51C6__GUID-68257B27-1C4C-4C47-8140-5C60E0E65D35)

<a id="op-7ce1ed275c895560a605aaf0"></a>
## SemanticView

`variant` · `sqlparser::ast::query::TableFactor::SemanticView` · sqlparser 0.62.0

```rust
SemanticView
```

Source: `src/ast/query.rs:1722`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake's SEMANTIC_VIEW function for semantic models.

<https://docs.snowflake.com/en/sql-reference/constructs/semantic_view>

```sql
SELECT * FROM SEMANTIC_VIEW(
    tpch_analysis
    DIMENSIONS customer.customer_market_segment
    METRICS orders.order_average_value
);
```

<a id="op-67009b60c2bf58f73f05ddb0"></a>
## Table

`variant` · `sqlparser::ast::query::TableFactor::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/query.rs:1465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named table or relation, possibly with arguments, hints, or sampling.

<a id="op-3eb52c904ce83c2b39d449dd"></a>
## TableFunction

`variant` · `sqlparser::ast::query::TableFactor::TableFunction` · sqlparser 0.62.0

```rust
TableFunction
```

Source: `src/ast/query.rs:1511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TABLE(<expr>)[ AS <alias> ]`

<a id="op-09f539ef76985a39f91066ce"></a>
## UNNEST

`variant` · `sqlparser::ast::query::TableFactor::UNNEST` · sqlparser 0.62.0

```rust
UNNEST
```

Source: `src/ast/query.rs:1540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SELECT * FROM UNNEST ([10,20,30]) as numbers WITH OFFSET;
+---------+--------+
| numbers | offset |
+---------+--------+
| 10      | 0      |
| 20      | 1      |
| 30      | 2      |
+---------+--------+
```

<a id="op-e952cf1743023a36d9149584"></a>
## Unpivot

`variant` · `sqlparser::ast::query::TableFactor::Unpivot` · sqlparser 0.62.0

```rust
Unpivot
```

Source: `src/ast/query.rs:1643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An UNPIVOT operation on a table.

Syntax:
```sql
table UNPIVOT [ { INCLUDE | EXCLUDE } NULLS ] (value FOR name IN (column1, [ column2, ... ])) [ alias ]
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/constructs/unpivot)
[Databricks](https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-qry-select-unpivot)
[BigQuery](https://docs.cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#unpivot_operator)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/SELECT.html#GUID-CFA006CA-6FF1-4972-821E-6996142A51C6__GUID-9B4E0389-413C-4014-94A1-0A0571BDF7E1)

<a id="op-02494807a3a6bb81e41e8baa"></a>
## XmlTable

`variant` · `sqlparser::ast::query::TableFactor::XmlTable` · sqlparser 0.62.0

```rust
XmlTable
```

Source: `src/ast/query.rs:1699`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `XMLTABLE` table-valued function.
Part of the SQL standard, supported by PostgreSQL, Oracle, and DB2.

<https://www.postgresql.org/docs/15/functions-xml.html#FUNCTIONS-XML-PROCESSING>

```sql
SELECT xmltable.*
FROM xmldata,
XMLTABLE('//ROWS/ROW'
    PASSING data
    COLUMNS id int PATH '@id',
    ordinality FOR ORDINALITY,
    "COUNTRY_NAME" text,
    country_id text PATH 'COUNTRY_ID',
    size_sq_km float PATH 'SIZE[@unit = "sq_km"]',
    size_other text PATH 'concat(SIZE[@unit!="sq_km"], " ", SIZE[@unit!="sq_km"]/@unit)',
    premier_name text PATH 'PREMIER_NAME' DEFAULT 'not specified'
);
````

<a id="op-34cf5da13f9df2a4385fc9c6"></a>
## clone

`function` · `sqlparser::ast::query::TableFactor::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableFactor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 17], "end": [1459, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05acca8e89c4decfbec640a9"></a>
## cmp

`function` · `sqlparser::ast::query::TableFactor::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableFactor) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 51], "end": [1459, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe66ca975c04859ff68fa4ed"></a>
## deserialize

`function` · `sqlparser::ast::query::TableFactor::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1460, 49], "end": [1460, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b52675ca14187dcef7ca7e7"></a>
## eq

`function` · `sqlparser::ast::query::TableFactor::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableFactor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 24], "end": [1459, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85e08141c9b81f38bb275bf3"></a>
## fmt

`function` · `sqlparser::ast::query::TableFactor::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2200, 1], "end": [2521, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc99936a170bef76dd9fe6a9"></a>
## fmt

`function` · `sqlparser::ast::query::TableFactor::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 10], "end": [1459, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40c76d2139f3008724d7a8db"></a>
## hash

`function` · `sqlparser::ast::query::TableFactor::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 56], "end": [1459, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81c9f2d835b6978e67afbbde"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableFactor::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableFactor) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1459, 35], "end": [1459, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df62b36e2d48a06e43297520"></a>
## serialize

`function` · `sqlparser::ast::query::TableFactor::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1460, 38], "end": [1460, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78a0479e1d6515bba7d34eed"></a>
## span

`function` · `sqlparser::ast::query::TableFactor::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "super::TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1941, 1], "end": [2079, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1942`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06532c11ac109005c39a1373"></a>
## visit

`function` · `sqlparser::ast::query::TableFactor::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1461, 47], "end": [1461, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81c761cfdb35650ee962c1c7"></a>
## visit

`function` · `sqlparser::ast::query::TableFactor::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFactor", "path": "TableFactor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1461, 40], "end": [1461, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
