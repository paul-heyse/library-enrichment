# `sqlparser::ast::query::ProjectionSelect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ProjectionSelect.json).

<a id="op-7c520eca5acbf5b257b1fe1e"></a>
## ProjectionSelect

`struct` · `sqlparser::ast::query::ProjectionSelect` · sqlparser 0.62.0

```rust
struct ProjectionSelect
```

Source: `src/ast/query.rs:122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query syntax for ClickHouse ADD PROJECTION statement.
Its syntax is similar to SELECT statement, but it is used to add a new projection to a table.
Syntax is `SELECT <COLUMN LIST EXPR> [GROUP BY] [ORDER BY]`

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/projection#add-projection)

<a id="op-29e36ee9149af00b6c752811"></a>
## clone

`function` · `sqlparser::ast::query::ProjectionSelect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ProjectionSelect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 17], "end": [119, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f17dbf473a69b65306036ed"></a>
## cmp

`function` · `sqlparser::ast::query::ProjectionSelect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ProjectionSelect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 51], "end": [119, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe6f07c434390109f6b8c574"></a>
## deserialize

`function` · `sqlparser::ast::query::ProjectionSelect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 49], "end": [120, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc82f43eae4df276122f5dfe"></a>
## eq

`function` · `sqlparser::ast::query::ProjectionSelect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ProjectionSelect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 24], "end": [119, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa6f5bc544a40fdbb256d600"></a>
## fmt

`function` · `sqlparser::ast::query::ProjectionSelect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 1], "end": [142, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd6fba62d86a8c48432fb75b"></a>
## fmt

`function` · `sqlparser::ast::query::ProjectionSelect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 10], "end": [119, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a95b10fec77e2c301906828"></a>
## group_by

`struct_field` · `sqlparser::ast::query::ProjectionSelect::group_by` · sqlparser 0.62.0

```rust
group_by: Option<GroupByExpr>
```

Source: `src/ast/query.rs:128`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `GROUP BY` clause for the projection-select.

<a id="op-0cbcdfdf605b98b13e47d744"></a>
## hash

`function` · `sqlparser::ast::query::ProjectionSelect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 56], "end": [119, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4da89e8332e60bb37f89e2c7"></a>
## order_by

`struct_field` · `sqlparser::ast::query::ProjectionSelect::order_by` · sqlparser 0.62.0

```rust
order_by: Option<OrderBy>
```

Source: `src/ast/query.rs:126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ORDER BY` clause for the projection-select.

<a id="op-36cce3b3d0d25cf1080e2939"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ProjectionSelect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ProjectionSelect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 35], "end": [119, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-355922013963ec06870d90ca"></a>
## projection

`struct_field` · `sqlparser::ast::query::ProjectionSelect::projection` · sqlparser 0.62.0

```rust
projection: Vec<SelectItem>
```

Source: `src/ast/query.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of projected select items.

<a id="op-01ce70967d40195b49f9e899"></a>
## serialize

`function` · `sqlparser::ast::query::ProjectionSelect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 38], "end": [120, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62c241f9894550ba0619cd72"></a>
## span

`function` · `sqlparser::ast::query::ProjectionSelect::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "super::ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1247, 1], "end": [1263, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22c5e89e33a0456ceebb4b23"></a>
## visit

`function` · `sqlparser::ast::query::ProjectionSelect::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 47], "end": [121, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8629d2820430eac8dee85050"></a>
## visit

`function` · `sqlparser::ast::query::ProjectionSelect::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ProjectionSelect", "path": "ProjectionSelect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 40], "end": [121, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
