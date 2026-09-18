# `sqlparser::ast::Expr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.json).

<a id="op-60bb4348f2010610c575bb63"></a>
## Expr

`enum` · `sqlparser::ast::Expr` · sqlparser 0.62.0

```rust
enum Expr
```

Source: `src/ast/mod.rs:871`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An SQL expression of any type.

# Semantics / Type Checking

The parser does not distinguish between expressions of different types
(e.g. boolean vs string). The caller is responsible for detecting and
validating types as necessary (for example  `WHERE 1` vs `SELECT 1=1`)
See the [README.md] for more details.

[README.md]: https://github.com/apache/datafusion-sqlparser-rs/blob/main/README.md#syntax-vs-semantics

# Equality and Hashing Does not Include Source Locations

The `Expr` type implements `PartialEq` and `Eq` based on the semantic value
of the expression (not bitwise comparison). This means that `Expr` instances
that are semantically equivalent but have different spans (locations in the
source tree) will compare as equal.

<a id="op-1fd641dad32834fc1b47f2f8"></a>
## AllOp

`variant` · `sqlparser::ast::Expr::AllOp` · sqlparser 0.62.0

```rust
AllOp
```

Source: `src/ast/mod.rs:1051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALL` operation e.g. `foo > ALL(bar)`, comparison operator is one of `[=, >, <, =>, =<, !=]`
<https://docs.snowflake.com/en/sql-reference/operators-subquery#all-any>

<a id="op-48a0504c590d1450498eb972"></a>
## AnyOp

`variant` · `sqlparser::ast::Expr::AnyOp` · sqlparser 0.62.0

```rust
AnyOp
```

Source: `src/ast/mod.rs:1039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ANY` operation e.g. `foo > ANY(bar)`, comparison operator is one of `[=, >, <, =>, =<, !=]`
<https://docs.snowflake.com/en/sql-reference/operators-subquery#all-any>

<a id="op-7e5ca5e54f845fbe999e2d75"></a>
## Array

`variant` · `sqlparser::ast::Expr::Array` · sqlparser 0.62.0

```rust
Array
```

Source: `src/ast/mod.rs:1318`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An array expression e.g. `ARRAY[1, 2]`

<a id="op-2318604b75b767521554939e"></a>
## AtTimeZone

`variant` · `sqlparser::ast::Expr::AtTimeZone` · sqlparser 0.62.0

```rust
AtTimeZone
```

Source: `src/ast/mod.rs:1105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

AT a timestamp to a different timezone e.g. `FROM_UNIXTIME(0) AT TIME ZONE 'UTC-06:00'`

<a id="op-5a985a5ada2fa48e910f7c52"></a>
## Between

`variant` · `sqlparser::ast::Expr::Between` · sqlparser 0.62.0

```rust
Between
```

Source: `src/ast/mod.rs:968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<expr> [ NOT ] BETWEEN <low> AND <high>`

<a id="op-ff9686970242a7d3bd02da08"></a>
## BinaryOp

`variant` · `sqlparser::ast::Expr::BinaryOp` · sqlparser 0.62.0

```rust
BinaryOp
```

Source: `src/ast/mod.rs:979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Binary operation e.g. `1 + 1` or `foo > bar`

<a id="op-87c61b34b38c2b146d245779"></a>
## Case

`variant` · `sqlparser::ast::Expr::Case` · sqlparser 0.62.0

```rust
Case
```

Source: `src/ast/mod.rs:1243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASE [<operand>] WHEN <condition> THEN <result> ... [ELSE <result>] END`

Note we only recognize a complete single expression as `<condition>`,
not `< 0` nor `1, 2, 3` as allowed in a `<simple when clause>` per
<https://jakewheat.github.io/sql-overview/sql-2011-foundation-grammar.html#simple-when-clause>

<a id="op-fc4459c70cf04f6124e83e46"></a>
## Cast

`variant` · `sqlparser::ast::Expr::Cast` · sqlparser 0.62.0

```rust
Cast
```

Source: `src/ast/mod.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CAST` an expression to a different data type e.g. `CAST(foo AS VARCHAR(123))`

<a id="op-7314b734071bd909b0007526"></a>
## Ceil

`variant` · `sqlparser::ast::Expr::Ceil` · sqlparser 0.62.0

```rust
Ceil
```

Source: `src/ast/mod.rs:1132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
CEIL(<expr> [TO DateTimeField])
```
```sql
CEIL( <input_expr> [, <scale_expr> ] )
```

<a id="op-ced026e3f888e067703bf6a7"></a>
## Collate

`variant` · `sqlparser::ast::Expr::Collate` · sqlparser 0.62.0

```rust
Collate
```

Source: `src/ast/mod.rs:1212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`expr COLLATE collation`

<a id="op-caa40f99aa3a3865bf4efc4e"></a>
## CompoundFieldAccess

`variant` · `sqlparser::ast::Expr::CompoundFieldAccess` · sqlparser 0.62.0

```rust
CompoundFieldAccess
```

Source: `src/ast/mod.rs:894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multi-part expression access.

This structure represents an access chain in structured / nested types
such as maps, arrays, and lists:
- Array
    - A 1-dim array `a[1]` will be represented like:
      `CompoundFieldAccess(Ident('a'), vec![Subscript(1)]`
    - A 2-dim array `a[1][2]` will be represented like:
      `CompoundFieldAccess(Ident('a'), vec![Subscript(1), Subscript(2)]`
- Map or Struct (Bracket-style)
    - A map `a['field1']` will be represented like:
      `CompoundFieldAccess(Ident('a'), vec![Subscript('field')]`
    - A 2-dim map `a['field1']['field2']` will be represented like:
      `CompoundFieldAccess(Ident('a'), vec![Subscript('field1'), Subscript('field2')]`
- Struct (Dot-style) (only effect when the chain contains both subscript and expr)
    - A struct access `a[field1].field2` will be represented like:
      `CompoundFieldAccess(Ident('a'), vec![Subscript('field1'), Ident('field2')]`
- If a struct access likes `a.field1.field2`, it will be represented by CompoundIdentifier([a, field1, field2])

<a id="op-e08e68e59c071d6335d7ce14"></a>
## CompoundIdentifier

`variant` · `sqlparser::ast::Expr::CompoundIdentifier` · sqlparser 0.62.0

```rust
CompoundIdentifier
```

Source: `src/ast/mod.rs:875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multi-part identifier, e.g. `table_alias.column` or `schema.table.col`

<a id="op-63c903b37bc340019241b244"></a>
## Convert

`variant` · `sqlparser::ast::Expr::Convert` · sqlparser 0.62.0

```rust
Convert
```

Source: `src/ast/mod.rs:1068`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CONVERT a value to a different data type or character encoding. e.g. `CONVERT(foo USING utf8mb4)`

<a id="op-c13f4f8fe4d198a9601519cc"></a>
## Cube

`variant` · `sqlparser::ast::Expr::Cube` · sqlparser 0.62.0

```rust
Cube
```

Source: `src/ast/mod.rs:1269`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `CUBE` expr.

<a id="op-9c642eb5ae5401f7a2739baa"></a>
## Dictionary

`variant` · `sqlparser::ast::Expr::Dictionary` · sqlparser 0.62.0

```rust
Dictionary
```

Source: `src/ast/mod.rs:1308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DuckDB` specific `Struct` literal expression [1]

Syntax:
```sql
syntax: {'field_name': expr1[, ... ]}
```
[1]: https://duckdb.org/docs/sql/data_types/struct#creating-structs

<a id="op-5f087e94c9ecb57261f05158"></a>
## Exists

`variant` · `sqlparser::ast::Expr::Exists` · sqlparser 0.62.0

```rust
Exists
```

Source: `src/ast/mod.rs:1257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An exists expression `[ NOT ] EXISTS(SELECT ...)`, used in expressions like
`WHERE [ NOT ] EXISTS (SELECT ...)`.

<a id="op-6c78d05987a94a67d284e37b"></a>
## Extract

`variant` · `sqlparser::ast::Expr::Extract` · sqlparser 0.62.0

```rust
Extract
```

Source: `src/ast/mod.rs:1118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Extract a field from a timestamp e.g. `EXTRACT(MONTH FROM foo)`
Or `EXTRACT(MONTH, foo)`

Syntax:
```sql
EXTRACT(DateTimeField FROM <expr>) | EXTRACT(DateTimeField, <expr>)
```

<a id="op-8779da9db3383ff9a62f15d0"></a>
## Floor

`variant` · `sqlparser::ast::Expr::Floor` · sqlparser 0.62.0

```rust
Floor
```

Source: `src/ast/mod.rs:1144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
FLOOR(<expr> [TO DateTimeField])
```
```sql
FLOOR( <input_expr> [, <scale_expr> ] )


<a id="op-cc98a6f6f4861b74eb6225d0"></a>
## Function

`variant` · `sqlparser::ast::Expr::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/mod.rs:1237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Scalar function call e.g. `LEFT(foo, 5)`

<a id="op-e16429f133f7f4625323a596"></a>
## GroupingSets

`variant` · `sqlparser::ast::Expr::GroupingSets` · sqlparser 0.62.0

```rust
GroupingSets
```

Source: `src/ast/mod.rs:1267`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `GROUPING SETS` expr.

<a id="op-ee9e27d1801fe11c71a0306e"></a>
## ILike

`variant` · `sqlparser::ast::Expr::ILike` · sqlparser 0.62.0

```rust
ILike
```

Source: `src/ast/mod.rs:1002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ILIKE` (case-insensitive `LIKE`)

<a id="op-4337781e9c4e4ceb38e05d93"></a>
## Identifier

`variant` · `sqlparser::ast::Expr::Identifier` · sqlparser 0.62.0

```rust
Identifier
```

Source: `src/ast/mod.rs:873`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identifier e.g. table name or column name

<a id="op-94bcfe5c719f9834e5b7f1a4"></a>
## InList

`variant` · `sqlparser::ast::Expr::InList` · sqlparser 0.62.0

```rust
InList
```

Source: `src/ast/mod.rs:941`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ NOT ] IN (val1, val2, ...)`

<a id="op-22157be30a8666e572b6c36d"></a>
## InSubquery

`variant` · `sqlparser::ast::Expr::InSubquery` · sqlparser 0.62.0

```rust
InSubquery
```

Source: `src/ast/mod.rs:950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ NOT ] IN (SELECT ...)`

<a id="op-f07bb9c0ccaf4d5a4463b40d"></a>
## InUnnest

`variant` · `sqlparser::ast::Expr::InUnnest` · sqlparser 0.62.0

```rust
InUnnest
```

Source: `src/ast/mod.rs:959`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ NOT ] IN UNNEST(array_expression)`

<a id="op-d30a0ae8a3c2f39156780ad5"></a>
## Interval

`variant` · `sqlparser::ast::Expr::Interval` · sqlparser 0.62.0

```rust
Interval
```

Source: `src/ast/mod.rs:1320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An interval expression e.g. `INTERVAL '1' YEAR`

<a id="op-dbfd65317c3968cb5ff92397"></a>
## IsDistinctFrom

`variant` · `sqlparser::ast::Expr::IsDistinctFrom` · sqlparser 0.62.0

```rust
IsDistinctFrom
```

Source: `src/ast/mod.rs:928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS DISTINCT FROM` operator

<a id="op-207618bca8b70aa723a72974"></a>
## IsFalse

`variant` · `sqlparser::ast::Expr::IsFalse` · sqlparser 0.62.0

```rust
IsFalse
```

Source: `src/ast/mod.rs:912`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS FALSE` operator

<a id="op-e64bcb262bc143d207e70521"></a>
## IsNormalized

`variant` · `sqlparser::ast::Expr::IsNormalized` · sqlparser 0.62.0

```rust
IsNormalized
```

Source: `src/ast/mod.rs:932`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<expr> IS [ NOT ] [ form ] NORMALIZED`

<a id="op-e26c1a67d4d1d852e05d8224"></a>
## IsNotDistinctFrom

`variant` · `sqlparser::ast::Expr::IsNotDistinctFrom` · sqlparser 0.62.0

```rust
IsNotDistinctFrom
```

Source: `src/ast/mod.rs:930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS NOT DISTINCT FROM` operator

<a id="op-8fd494d37200bd4b9df4e19f"></a>
## IsNotFalse

`variant` · `sqlparser::ast::Expr::IsNotFalse` · sqlparser 0.62.0

```rust
IsNotFalse
```

Source: `src/ast/mod.rs:914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS NOT FALSE` operator

<a id="op-354923896c2acd4cacb11e10"></a>
## IsNotNull

`variant` · `sqlparser::ast::Expr::IsNotNull` · sqlparser 0.62.0

```rust
IsNotNull
```

Source: `src/ast/mod.rs:922`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS NOT NULL` operator

<a id="op-ab075cc4950eb27de0ce6ff0"></a>
## IsNotTrue

`variant` · `sqlparser::ast::Expr::IsNotTrue` · sqlparser 0.62.0

```rust
IsNotTrue
```

Source: `src/ast/mod.rs:918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS NOT TRUE` operator

<a id="op-b05f07c0af0ce0676222da33"></a>
## IsNotUnknown

`variant` · `sqlparser::ast::Expr::IsNotUnknown` · sqlparser 0.62.0

```rust
IsNotUnknown
```

Source: `src/ast/mod.rs:926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS NOT UNKNOWN` operator

<a id="op-af1bcbdd72214f50aa868fcd"></a>
## IsNull

`variant` · `sqlparser::ast::Expr::IsNull` · sqlparser 0.62.0

```rust
IsNull
```

Source: `src/ast/mod.rs:920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS NULL` operator

<a id="op-929d4e4d55487984f8577134"></a>
## IsTrue

`variant` · `sqlparser::ast::Expr::IsTrue` · sqlparser 0.62.0

```rust
IsTrue
```

Source: `src/ast/mod.rs:916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS TRUE` operator

<a id="op-ee20e78538175257fbae947d"></a>
## IsUnknown

`variant` · `sqlparser::ast::Expr::IsUnknown` · sqlparser 0.62.0

```rust
IsUnknown
```

Source: `src/ast/mod.rs:924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IS UNKNOWN` operator

<a id="op-a0838210f9ad03244d038e2b"></a>
## JsonAccess

`variant` · `sqlparser::ast::Expr::JsonAccess` · sqlparser 0.62.0

```rust
JsonAccess
```

Source: `src/ast/mod.rs:905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Access data nested in a value containing semi-structured data, such as
the `VARIANT` type on Snowflake. for example `src:customer[0].name`.

See <https://docs.snowflake.com/en/user-guide/querying-semistructured>.
See <https://docs.databricks.com/en/sql/language-manual/functions/colonsign.html>.

<a id="op-681b2962ccd9799c1e0a3413"></a>
## Lambda

`variant` · `sqlparser::ast::Expr::Lambda` · sqlparser 0.62.0

```rust
Lambda
```

Source: `src/ast/mod.rs:1371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A lambda function.

Syntax:
```plaintext
param -> expr | (param1, ...) -> expr
```

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/functions#higher-order-functions---operator-and-lambdaparams-expr-function)
[Databricks](https://docs.databricks.com/en/sql/language-manual/sql-ref-lambda-functions.html)
[DuckDB](https://duckdb.org/docs/stable/sql/functions/lambda)

<a id="op-b01cf0579b91e8b95cc53264"></a>
## Like

`variant` · `sqlparser::ast::Expr::Like` · sqlparser 0.62.0

```rust
Like
```

Source: `src/ast/mod.rs:988`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[NOT] LIKE <pattern> [ESCAPE <escape_character>]`

<a id="op-9d1644425e2df0053861b94c"></a>
## Map

`variant` · `sqlparser::ast::Expr::Map` · sqlparser 0.62.0

```rust
Map
```

Source: `src/ast/mod.rs:1316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DuckDB` specific `Map` literal expression [1]

Syntax:
```sql
syntax: Map {key1: value1[, ... ]}
```
[1]: https://duckdb.org/docs/sql/data_types/map#creating-maps

<a id="op-e3bc252fa8223d32bebe3515"></a>
## MatchAgainst

`variant` · `sqlparser::ast::Expr::MatchAgainst` · sqlparser 0.62.0

```rust
MatchAgainst
```

Source: `src/ast/mod.rs:1331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MySQL` specific text search function [(1)].

Syntax:
```sql
MATCH (<col>, <col>, ...) AGAINST (<expr> [<search modifier>])

<col> = CompoundIdentifier
<expr> = String literal
```
[(1)]: https://dev.mysql.com/doc/refman/8.0/en/fulltext-search.html#function_match

<a id="op-8fbca220c03569e9a1b4601f"></a>
## MemberOf

`variant` · `sqlparser::ast::Expr::MemberOf` · sqlparser 0.62.0

```rust
MemberOf
```

Source: `src/ast/mod.rs:1373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Checks membership of a value in a JSON array

<a id="op-3d4b9b6294c430dc81d15cb5"></a>
## Named

`variant` · `sqlparser::ast::Expr::Named` · sqlparser 0.62.0

```rust
Named
```

Source: `src/ast/mod.rs:1295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BigQuery` specific: An named expression in a typeless struct [1]

Syntax
```sql
1 AS A
```
[1]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#struct_type

<a id="op-b5055ccd15fc94a7ce4cd4fc"></a>
## Nested

`variant` · `sqlparser::ast::Expr::Nested` · sqlparser 0.62.0

```rust
Nested
```

Source: `src/ast/mod.rs:1219`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Nested expression e.g. `(foo > bar)` or `(1)`

<a id="op-d15620e4ef84d7a32fc99724"></a>
## OuterJoin

`variant` · `sqlparser::ast::Expr::OuterJoin` · sqlparser 0.62.0

```rust
OuterJoin
```

Source: `src/ast/mod.rs:1358`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Some dialects support an older syntax for outer joins where columns are
marked with the `(+)` operator in the WHERE clause, for example:

```sql
SELECT t1.c1, t2.c2 FROM t1, t2 WHERE t1.c1 = t2.c2 (+)
```

which is equivalent to

```sql
SELECT t1.c1, t2.c2 FROM t1 LEFT OUTER JOIN t2 ON t1.c1 = t2.c2
```

See <https://docs.snowflake.com/en/sql-reference/constructs/where#joins-in-the-where-clause>.

<a id="op-adbdd68b92fc323e928b96b6"></a>
## Overlay

`variant` · `sqlparser::ast::Expr::Overlay` · sqlparser 0.62.0

```rust
Overlay
```

Source: `src/ast/mod.rs:1201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
OVERLAY(<expr> PLACING <expr> FROM <expr>[ FOR <expr> ]
```

<a id="op-771c841b5721d3d6b0a6339a"></a>
## Position

`variant` · `sqlparser::ast::Expr::Position` · sqlparser 0.62.0

```rust
Position
```

Source: `src/ast/mod.rs:1153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
POSITION(<expr> in <expr>)
```

<a id="op-eab6eb4c718122a49e7943ff"></a>
## Prefixed

`variant` · `sqlparser::ast::Expr::Prefixed` · sqlparser 0.62.0

```rust
Prefixed
```

Source: `src/ast/mod.rs:1225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Prefixed expression, e.g. introducer strings, projection prefix
<https://dev.mysql.com/doc/refman/8.0/en/charset-introducer.html>
<https://docs.snowflake.com/en/sql-reference/constructs/connect-by>

<a id="op-e0ea7b22d0a12630074f3bf7"></a>
## Prior

`variant` · `sqlparser::ast::Expr::Prior` · sqlparser 0.62.0

```rust
Prior
```

Source: `src/ast/mod.rs:1360`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A reference to the prior level in a CONNECT BY clause.

<a id="op-fc9b51b8d45f95c4adeb06fc"></a>
## QualifiedWildcard

`variant` · `sqlparser::ast::Expr::QualifiedWildcard` · sqlparser 0.62.0

```rust
QualifiedWildcard
```

Source: `src/ast/mod.rs:1343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Qualified wildcard, e.g. `alias.*` or `schema.table.*`.
(Same caveats apply to `QualifiedWildcard` as to `Wildcard`.)

<a id="op-f4212abd02928a46a37703e4"></a>
## RLike

`variant` · `sqlparser::ast::Expr::RLike` · sqlparser 0.62.0

```rust
RLike
```

Source: `src/ast/mod.rs:1027`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL: `RLIKE` regex or `REGEXP` regex

<a id="op-d96539ac0174bedbad1c6e6d"></a>
## Rollup

`variant` · `sqlparser::ast::Expr::Rollup` · sqlparser 0.62.0

```rust
Rollup
```

Source: `src/ast/mod.rs:1271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `ROLLUP` expr.

<a id="op-4ba3e95eed854a94ab4316ec"></a>
## SimilarTo

`variant` · `sqlparser::ast::Expr::SimilarTo` · sqlparser 0.62.0

```rust
SimilarTo
```

Source: `src/ast/mod.rs:1016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SIMILAR TO` regex

<a id="op-ee3595738aa8ae1c7c5d549a"></a>
## Struct

`variant` · `sqlparser::ast::Expr::Struct` · sqlparser 0.62.0

```rust
Struct
```

Source: `src/ast/mod.rs:1282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`Struct` literal expression
Syntax:
```sql
STRUCT<[field_name] field_type, ...>( expr1 [, ... ])

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#struct_type)
[Databricks](https://docs.databricks.com/en/sql/language-manual/functions/struct.html)
```

<a id="op-3ebe5504e5915f6218528f96"></a>
## Subquery

`variant` · `sqlparser::ast::Expr::Subquery` · sqlparser 0.62.0

```rust
Subquery
```

Source: `src/ast/mod.rs:1265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A parenthesized subquery `(SELECT ...)`, used in expression like
`SELECT (subquery) AS x` or `WHERE (subquery) = x`

<a id="op-6be73bd93f8d21a66ef26cd0"></a>
## Substring

`variant` · `sqlparser::ast::Expr::Substring` · sqlparser 0.62.0

```rust
Substring
```

Source: `src/ast/mod.rs:1166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SUBSTRING(<expr> [FROM <expr>] [FOR <expr>])
```
or
```sql
SUBSTRING(<expr>, <expr>, <expr>)
```

<a id="op-081f78bcac642ea8b3ac7898"></a>
## Trim

`variant` · `sqlparser::ast::Expr::Trim` · sqlparser 0.62.0

```rust
Trim
```

Source: `src/ast/mod.rs:1188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
TRIM([BOTH | LEADING | TRAILING] [<expr> FROM] <expr>)
TRIM(<expr>)
TRIM(<expr>, [, characters]) -- PostgreSQL, DuckDB, Snowflake, BigQuery, Generic
```

<a id="op-cfb3b802a6fcc77169f55a8d"></a>
## Tuple

`variant` · `sqlparser::ast::Expr::Tuple` · sqlparser 0.62.0

```rust
Tuple
```

Source: `src/ast/mod.rs:1273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ROW / TUPLE a single value, such as `SELECT (1, 2)`

<a id="op-cf56408ba3d451ad0d001e88"></a>
## TypedString

`variant` · `sqlparser::ast::Expr::TypedString` · sqlparser 0.62.0

```rust
TypedString
```

Source: `src/ast/mod.rs:1235`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A constant of form `<data_type> 'value'`.
This can represent ANSI SQL `DATE`, `TIME`, and `TIMESTAMP` literals (such as `DATE '2020-01-01'`),
as well as constants of other types (a non-standard PostgreSQL extension).

<a id="op-9fbf0fee698d7f438622ad27"></a>
## UnaryOp

`variant` · `sqlparser::ast::Expr::UnaryOp` · sqlparser 0.62.0

```rust
UnaryOp
```

Source: `src/ast/mod.rs:1061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unary operation e.g. `NOT foo`

<a id="op-b308556454b939d0289fc387"></a>
## Value

`variant` · `sqlparser::ast::Expr::Value` · sqlparser 0.62.0

```rust
Value
```

Source: `src/ast/mod.rs:1221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A literal value, such as string, number, date or NULL

<a id="op-8b111cdad97a96d2b57b05f7"></a>
## Wildcard

`variant` · `sqlparser::ast::Expr::Wildcard` · sqlparser 0.62.0

```rust
Wildcard
```

Source: `src/ast/mod.rs:1340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unqualified `*` wildcard token (e.g. `*`).

<a id="op-287e886215e7feb3883d1a1c"></a>
## clone

`function` · `sqlparser::ast::Expr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 17], "end": [864, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c66c13cf45b34e5b4562f13"></a>
## cmp

`function` · `sqlparser::ast::Expr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Expr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 51], "end": [864, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4858943fefc85b6579ac28ff"></a>
## deserialize

`function` · `sqlparser::ast::Expr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 49], "end": [865, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94488aa504945861274de8d3"></a>
## eq

`function` · `sqlparser::ast::Expr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Expr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 24], "end": [864, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5771fa77794b7cabb634104d"></a>
## fmt

`function` · `sqlparser::ast::Expr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1717, 1], "end": [2226, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1718`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d0cf80cd9030eb80e350ac"></a>
## fmt

`function` · `sqlparser::ast::Expr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 10], "end": [864, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7192115cc2394bdbed9c8048"></a>
## hash

`function` · `sqlparser::ast::Expr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 56], "end": [864, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e75b19dbe572e1dd357cb3af"></a>
## partial_cmp

`function` · `sqlparser::ast::Expr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Expr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 35], "end": [864, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faca4e95ec707848233b9ca7"></a>
## serialize

`function` · `sqlparser::ast::Expr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 38], "end": [865, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d887a81c9d2ca5e0f0f980"></a>
## span

`function` · `sqlparser::ast::Expr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "super::Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1463, 1], "end": [1668, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f45be67af8692f6a829d7a4"></a>
## value

`function` · `sqlparser::ast::Expr::value` · sqlparser 0.62.0

```rust
fn value(value: impl Into<ValueWithSpan>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1376, 1], "end": [1381, 2], "filename": "src/ast/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/mod.rs:1378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Creates a new [`Expr::Value`](../operations/sqlparser.ast.Expr.md#op-b308556454b939d0289fc387)

<a id="op-8947c961dfe4ad10a7892ea7"></a>
## visit

`function` · `sqlparser::ast::Expr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [868, 12], "end": [868, 17], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcd8c919dd154d346f103441"></a>
## visit

`function` · `sqlparser::ast::Expr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [868, 19], "end": [868, 27], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
