# `sqlparser::ast::query::GroupByExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.GroupByExpr.json).

<a id="op-8c033e79f778acf1f3c0f2be"></a>
## GroupByExpr

`enum` · `sqlparser::ast::query::GroupByExpr` · sqlparser 0.62.0

```rust
enum GroupByExpr
```

Source: `src/ast/query.rs:3746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the two syntactic forms that `GROUP BY` can take, including
`GROUP BY ALL` with optional modifiers and ordinary `GROUP BY <exprs>`.

<a id="op-f35d3eef6e7690f878ac58eb"></a>
## All

`variant` · `sqlparser::ast::query::GroupByExpr::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/query.rs:3756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ALL syntax of [Snowflake], [DuckDB] and [ClickHouse].

[Snowflake]: <https://docs.snowflake.com/en/sql-reference/constructs/group-by#label-group-by-all-columns>
[DuckDB]:  <https://duckdb.org/docs/sql/query_syntax/groupby.html>
[ClickHouse]: <https://clickhouse.com/docs/en/sql-reference/statements/select/group-by#group-by-all>

ClickHouse also supports WITH modifiers after GROUP BY ALL and expressions.

[ClickHouse]: <https://clickhouse.com/docs/en/sql-reference/statements/select/group-by#rollup-modifier>

<a id="op-c558a4638691a0080839f963"></a>
## Expressions

`variant` · `sqlparser::ast::query::GroupByExpr::Expressions` · sqlparser 0.62.0

```rust
Expressions
```

Source: `src/ast/query.rs:3758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`GROUP BY <expressions>` with optional modifiers.

<a id="op-3c22010c70252020904666f2"></a>
## clone

`function` · `sqlparser::ast::query::GroupByExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GroupByExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 17], "end": [3741, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8547ed35ab9a3f3072367af5"></a>
## cmp

`function` · `sqlparser::ast::query::GroupByExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GroupByExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 51], "end": [3741, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2248f9130e6a95802d51d007"></a>
## deserialize

`function` · `sqlparser::ast::query::GroupByExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3742, 49], "end": [3742, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3742`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db4611fe95587cb63bfe2a3c"></a>
## eq

`function` · `sqlparser::ast::query::GroupByExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GroupByExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 24], "end": [3741, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d05642bee6ad7bc90c369b51"></a>
## fmt

`function` · `sqlparser::ast::query::GroupByExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3761, 1], "end": [3782, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f72beb138a1e1b1c9b65bc4e"></a>
## fmt

`function` · `sqlparser::ast::query::GroupByExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 10], "end": [3741, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7ca20fb6e00e92a4ae576b0"></a>
## hash

`function` · `sqlparser::ast::query::GroupByExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 56], "end": [3741, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59072ec77bf9bdb02a1d17e2"></a>
## partial_cmp

`function` · `sqlparser::ast::query::GroupByExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GroupByExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3741, 35], "end": [3741, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e56f3dd5878e9b7eeb222ea"></a>
## serialize

`function` · `sqlparser::ast::query::GroupByExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3742, 38], "end": [3742, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3742`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c1500b825087d65e33181a5"></a>
## span

`function` · `sqlparser::ast::query::GroupByExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "super::GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1287, 1], "end": [1296, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91b687964e377f896a7eae9c"></a>
## visit

`function` · `sqlparser::ast::query::GroupByExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3743, 40], "end": [3743, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea0ca977ea74e23cce933961"></a>
## visit

`function` · `sqlparser::ast::query::GroupByExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByExpr", "path": "GroupByExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3743, 47], "end": [3743, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
