# `sqlparser::ast::FunctionArgumentClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArgumentClause.json).

<a id="op-c6fdd5ab2d6ccfa51a8d9633"></a>
## FunctionArgumentClause

`enum` · `sqlparser::ast::FunctionArgumentClause` · sqlparser 0.62.0

```rust
enum FunctionArgumentClause
```

Source: `src/ast/mod.rs:8193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Clauses that can appear inside a function argument list.

<a id="op-a9698758d4a6bd87a58f80cd"></a>
## Having

`variant` · `sqlparser::ast::FunctionArgumentClause::Having` · sqlparser 0.62.0

```rust
Having
```

Source: `src/ast/mod.rs:8221`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies a minimum or maximum bound on the input to [`ANY_VALUE`] on BigQuery.

Syntax:
```plaintext
HAVING { MAX | MIN } expression
```

[`ANY_VALUE`]: https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#any_value

<a id="op-7be9eac321f410db9ee7bfb5"></a>
## IgnoreOrRespectNulls

`variant` · `sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls` · sqlparser 0.62.0

```rust
IgnoreOrRespectNulls
```

Source: `src/ast/mod.rs:8202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates how `NULL`s should be handled in the calculation, e.g. in `FIRST_VALUE` on [BigQuery].

Syntax:
```plaintext
{ IGNORE | RESPECT } NULLS ]
```

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/navigation_functions#first_value

<a id="op-1326b553aeca2a0447d1746c"></a>
## JsonNullClause

`variant` · `sqlparser::ast::FunctionArgumentClause::JsonNullClause` · sqlparser 0.62.0

```rust
JsonNullClause
```

Source: `src/ast/mod.rs:8231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `ON NULL` clause for some JSON functions.

[MSSQL `JSON_ARRAY`](https://learn.microsoft.com/en-us/sql/t-sql/functions/json-array-transact-sql?view=sql-server-ver16)
[MSSQL `JSON_OBJECT`](https://learn.microsoft.com/en-us/sql/t-sql/functions/json-object-transact-sql?view=sql-server-ver16>)
[PostgreSQL JSON functions](https://www.postgresql.org/docs/current/functions-json.html#FUNCTIONS-JSON-PROCESSING)

<a id="op-778ed1c1bf040d1b05175651"></a>
## JsonReturningClause

`variant` · `sqlparser::ast::FunctionArgumentClause::JsonReturningClause` · sqlparser 0.62.0

```rust
JsonReturningClause
```

Source: `src/ast/mod.rs:8235`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `RETURNING` clause for some JSON functions in PostgreSQL

[`JSON_OBJECT`](https://www.postgresql.org/docs/current/functions-json.html#:~:text=json_object)

<a id="op-a7ef533112860eb135b89d33"></a>
## Limit

`variant` · `sqlparser::ast::FunctionArgumentClause::Limit` · sqlparser 0.62.0

```rust
Limit
```

Source: `src/ast/mod.rs:8208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies a limit for the `ARRAY_AGG` and `ARRAY_CONCAT_AGG` functions on BigQuery.

<a id="op-43f0f5a20b6ec981210342fc"></a>
## OnOverflow

`variant` · `sqlparser::ast::FunctionArgumentClause::OnOverflow` · sqlparser 0.62.0

```rust
OnOverflow
```

Source: `src/ast/mod.rs:8212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the behavior on overflow of the `LISTAGG` function.

See <https://trino.io/docs/current/functions/aggregate.html>.

<a id="op-ac045cee30aa57242f27c587"></a>
## OrderBy

`variant` · `sqlparser::ast::FunctionArgumentClause::OrderBy` · sqlparser 0.62.0

```rust
OrderBy
```

Source: `src/ast/mod.rs:8206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the the ordering for some ordered set aggregates, e.g. `ARRAY_AGG` on [BigQuery].

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/aggregate_functions#array_agg

<a id="op-eb37282f3df7db2d197c9cc1"></a>
## Separator

`variant` · `sqlparser::ast::FunctionArgumentClause::Separator` · sqlparser 0.62.0

```rust
Separator
```

Source: `src/ast/mod.rs:8225`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `SEPARATOR` clause to the [`GROUP_CONCAT`] function in MySQL.

[`GROUP_CONCAT`]: https://dev.mysql.com/doc/refman/8.0/en/aggregate-functions.html#function_group-concat

<a id="op-fe1bee6c7179aa9ebcde0fe8"></a>
## clone

`function` · `sqlparser::ast::FunctionArgumentClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionArgumentClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8189, 17], "end": [8189, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-637e3c68e653844d68901cf9"></a>
## cmp

`function` · `sqlparser::ast::FunctionArgumentClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionArgumentClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8189, 51], "end": [8189, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c8831d24b7db2312e4aff73"></a>
## deserialize

`function` · `sqlparser::ast::FunctionArgumentClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8190, 49], "end": [8190, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8205ee2ae9d99b904ff60cc6"></a>
## eq

`function` · `sqlparser::ast::FunctionArgumentClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionArgumentClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8189, 24], "end": [8189, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f1cad37056d8d71dfcaeb1d"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgumentClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8189, 10], "end": [8189, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c838bbeff97f961c979719"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgumentClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8238, 1], "end": [8257, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85dd5c45d9067e0cb3c7e862"></a>
## hash

`function` · `sqlparser::ast::FunctionArgumentClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8189, 56], "end": [8189, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-423904b292672a197fc4e5aa"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionArgumentClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionArgumentClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8189, 35], "end": [8189, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8e3c0ea43464942213f3af8"></a>
## serialize

`function` · `sqlparser::ast::FunctionArgumentClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8190, 38], "end": [8190, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb6c9db3c24bc73306a9640f"></a>
## span

`function` · `sqlparser::ast::FunctionArgumentClause::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "super::FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1786, 1], "end": [1799, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7033e7b2501e2864e03d5273"></a>
## visit

`function` · `sqlparser::ast::FunctionArgumentClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8191, 40], "end": [8191, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc7fdf01bc4e92ae31718905"></a>
## visit

`function` · `sqlparser::ast::FunctionArgumentClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentClause", "path": "FunctionArgumentClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8191, 47], "end": [8191, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8191`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
