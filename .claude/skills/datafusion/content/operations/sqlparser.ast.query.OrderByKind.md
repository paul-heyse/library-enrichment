# `sqlparser::ast::query::OrderByKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.OrderByKind.json).

<a id="op-46fa06f9d20888eb99455db6"></a>
## OrderByKind

`enum` · `sqlparser::ast::query::OrderByKind` · sqlparser 0.62.0

```rust
enum OrderByKind
```

Source: `src/ast/query.rs:2878`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of `ORDER BY` clause: either `ALL` with modifiers or a list of expressions.

<a id="op-85683750c80779ea8499bc65"></a>
## All

`variant` · `sqlparser::ast::query::OrderByKind::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/query.rs:2883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`GROUP BY ALL`/`ORDER BY ALL` syntax with optional modifiers.

[DuckDB]:  <https://duckdb.org/docs/sql/query_syntax/orderby>
[ClickHouse]: <https://clickhouse.com/docs/en/sql-reference/statements/select/order-by>

<a id="op-6d0038ab9b563ce10b2b63da"></a>
## Expressions

`variant` · `sqlparser::ast::query::OrderByKind::Expressions` · sqlparser 0.62.0

```rust
Expressions
```

Source: `src/ast/query.rs:2886`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A standard list of ordering expressions.

<a id="op-c957400d78af17c4519917ff"></a>
## clone

`function` · `sqlparser::ast::query::OrderByKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OrderByKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2874, 17], "end": [2874, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d2c330d8ee830a35eff4889"></a>
## cmp

`function` · `sqlparser::ast::query::OrderByKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OrderByKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2874, 51], "end": [2874, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9b569a5b31073620a4f775f"></a>
## deserialize

`function` · `sqlparser::ast::query::OrderByKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2875, 49], "end": [2875, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8439ceddf91c41257b8ca88d"></a>
## eq

`function` · `sqlparser::ast::query::OrderByKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OrderByKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2874, 24], "end": [2874, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58166402685fce30243d2011"></a>
## fmt

`function` · `sqlparser::ast::query::OrderByKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2874, 10], "end": [2874, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81f685fc59e59cf530a072c2"></a>
## hash

`function` · `sqlparser::ast::query::OrderByKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2874, 56], "end": [2874, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccb2b1828d1c1b4743796b72"></a>
## partial_cmp

`function` · `sqlparser::ast::query::OrderByKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OrderByKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2874, 35], "end": [2874, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f7ddd1337fcce2133f9c5e2"></a>
## serialize

`function` · `sqlparser::ast::query::OrderByKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2875, 38], "end": [2875, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-585c59dad5aef74b7c455aa7"></a>
## visit

`function` · `sqlparser::ast::query::OrderByKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2876, 40], "end": [2876, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6de302d61be1cffd1649cb22"></a>
## visit

`function` · `sqlparser::ast::query::OrderByKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByKind", "path": "OrderByKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2876, 47], "end": [2876, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
