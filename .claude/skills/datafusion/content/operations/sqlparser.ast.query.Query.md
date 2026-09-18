# `sqlparser::ast::query::Query`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Query.json).

<a id="op-81d3b051c5e9810bee2041ec"></a>
## Query

`struct` · `sqlparser::ast::query::Query` · sqlparser 0.62.0

```rust
struct Query
```

Source: `src/ast/query.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The most complete variant of a `SELECT` query expression, optionally
including `WITH`, `UNION` / other set operations, and `ORDER BY`.

<a id="op-8c78de5c72fac644740b6bfc"></a>
## body

`struct_field` · `sqlparser::ast::query::Query::body` · sqlparser 0.62.0

```rust
body: Box<SetExpr>
```

Source: `src/ast/query.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SELECT or UNION / EXCEPT / INTERSECT

<a id="op-2f990ae228f1c695e0b39c34"></a>
## clone

`function` · `sqlparser::ast::query::Query::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Query
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9246417f5aa2bf5ec972be7d"></a>
## cmp

`function` · `sqlparser::ast::query::Query::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Query) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 51], "end": [36, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d99de0adeacba26c810f6ef8"></a>
## deserialize

`function` · `sqlparser::ast::query::Query::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 49], "end": [37, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:37`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2ab06e431dfb37f0e9abd37"></a>
## eq

`function` · `sqlparser::ast::query::Query::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Query) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 24], "end": [36, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5548f4999e02200ae72ed56d"></a>
## fetch

`struct_field` · `sqlparser::ast::query::Query::fetch` · sqlparser 0.62.0

```rust
fetch: Option<Fetch>
```

Source: `src/ast/query.rs:50`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FETCH { FIRST | NEXT } <N> [ PERCENT ] { ROW | ROWS } | { ONLY | WITH TIES }`

<a id="op-307d42f943a77f7860b3e0b0"></a>
## fmt

`function` · `sqlparser::ast::query::Query::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [112, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-650746de846c231140069684"></a>
## fmt

`function` · `sqlparser::ast::query::Query::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2b16c36c5b85a6f776bf6f9"></a>
## for_clause

`struct_field` · `sqlparser::ast::query::Query::for_clause` · sqlparser 0.62.0

```rust
for_clause: Option<ForClause>
```

Source: `src/ast/query.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR XML { RAW | AUTO | EXPLICIT | PATH } [ , ELEMENTS ]`
`FOR JSON { AUTO | PATH } [ , INCLUDE_NULL_VALUES ]`
(MSSQL-specific)

<a id="op-ba9e02a6a04e3ac95481d846"></a>
## format_clause

`struct_field` · `sqlparser::ast::query::Query::format_clause` · sqlparser 0.62.0

```rust
format_clause: Option<FormatClause>
```

Source: `src/ast/query.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SELECT * FROM t FORMAT JSONCompact`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/format)
(ClickHouse-specific)

<a id="op-18a9afc0c18a0aecfebb3525"></a>
## hash

`function` · `sqlparser::ast::query::Query::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 56], "end": [36, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c9fd9c47f70ee4e66d94da1"></a>
## limit_clause

`struct_field` · `sqlparser::ast::query::Query::limit_clause` · sqlparser 0.62.0

```rust
limit_clause: Option<LimitClause>
```

Source: `src/ast/query.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LIMIT ... OFFSET ... | LIMIT <offset>, <limit>`

<a id="op-015db5fec4a299f199b70284"></a>
## locks

`struct_field` · `sqlparser::ast::query::Query::locks` · sqlparser 0.62.0

```rust
locks: Vec<LockClause>
```

Source: `src/ast/query.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR { UPDATE | SHARE } [ OF table_name ] [ SKIP LOCKED | NOWAIT ]`

<a id="op-9a16dca7c4a940b80d8167c3"></a>
## order_by

`struct_field` · `sqlparser::ast::query::Query::order_by` · sqlparser 0.62.0

```rust
order_by: Option<OrderBy>
```

Source: `src/ast/query.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ORDER BY

<a id="op-3e63c11b634a041d309245df"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Query::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Query) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 35], "end": [36, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54e14f593a7e50a49678a52e"></a>
## pipe_operators

`struct_field` · `sqlparser::ast::query::Query::pipe_operators` · sqlparser 0.62.0

```rust
pipe_operators: Vec<PipeOperator>
```

Source: `src/ast/query.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pipe operator

<a id="op-d886ce3c37b9113e66fd4782"></a>
## serialize

`function` · `sqlparser::ast::query::Query::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 38], "end": [37, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:37`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df5efbb4d09c3457c803edc4"></a>
## settings

`struct_field` · `sqlparser::ast::query::Query::settings` · sqlparser 0.62.0

```rust
settings: Option<Vec<Setting>>
```

Source: `src/ast/query.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse syntax: `SELECT * FROM t SETTINGS key1 = value1, key2 = value2`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select#settings-in-select-query)

<a id="op-0c4d5d163825c5ae442766f9"></a>
## span

`function` · `sqlparser::ast::query::Query::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "super::Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [115, 1], "end": [139, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:116`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f8ba2a5b022a29d6a2b75d6"></a>
## visit

`function` · `sqlparser::ast::query::Query::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 40], "end": [38, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee14f0ec6f7b6d20428e1ef4"></a>
## visit

`function` · `sqlparser::ast::query::Query::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Query", "path": "Query"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 47], "end": [38, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bc45b9d455e2b458ea95c8b"></a>
## with

`struct_field` · `sqlparser::ast::query::Query::with` · sqlparser 0.62.0

```rust
with: Option<With>
```

Source: `src/ast/query.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WITH (common table expressions, or CTEs)
