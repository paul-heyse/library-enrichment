# `sqlparser::ast::query::TableIndexHints`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableIndexHints.json).

<a id="op-ab69de3826079b786d9a9f04"></a>
## TableIndexHints

`struct` · `sqlparser::ast::query::TableIndexHints` · sqlparser 0.62.0

```rust
struct TableIndexHints
```

Source: `src/ast/query.rs:1437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-style index hints attached to a table (e.g., `USE INDEX(...)`).

<a id="op-cde536426ac0cfa49fe34ec3"></a>
## clone

`function` · `sqlparser::ast::query::TableIndexHints::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableIndexHints
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 17], "end": [1433, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31e1b002f7920b45f2d3ae64"></a>
## cmp

`function` · `sqlparser::ast::query::TableIndexHints::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableIndexHints) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 51], "end": [1433, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-720d46acf3177165874ed16f"></a>
## deserialize

`function` · `sqlparser::ast::query::TableIndexHints::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1434, 49], "end": [1434, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1434`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77c3af9c61ebe4114c6c99b4"></a>
## eq

`function` · `sqlparser::ast::query::TableIndexHints::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableIndexHints) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 24], "end": [1433, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d70b0a9ee09786f583e613e2"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexHints::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 10], "end": [1433, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6f06f1e3b33bb69c21683b6"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexHints::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1448, 1], "end": [1456, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f932ced700b277962c801281"></a>
## for_clause

`struct_field` · `sqlparser::ast::query::TableIndexHints::for_clause` · sqlparser 0.62.0

```rust
for_clause: Option<TableIndexHintForClause>
```

Source: `src/ast/query.rs:1443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `FOR` clause specifying the scope (JOIN / ORDER BY / GROUP BY).

<a id="op-48dd2e0c907a6de9a3444ba6"></a>
## hash

`function` · `sqlparser::ast::query::TableIndexHints::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 56], "end": [1433, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-170825d0f12d965020442952"></a>
## hint_type

`struct_field` · `sqlparser::ast::query::TableIndexHints::hint_type` · sqlparser 0.62.0

```rust
hint_type: TableIndexHintType
```

Source: `src/ast/query.rs:1439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type of hint (e.g., `USE`, `FORCE`, or `IGNORE`).

<a id="op-443c204444b3d59f4e852779"></a>
## index_names

`struct_field` · `sqlparser::ast::query::TableIndexHints::index_names` · sqlparser 0.62.0

```rust
index_names: Vec<Ident>
```

Source: `src/ast/query.rs:1445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of index names referred to by the hint.

<a id="op-ecf5c2f9c9cf86759a4d8322"></a>
## index_type

`struct_field` · `sqlparser::ast::query::TableIndexHints::index_type` · sqlparser 0.62.0

```rust
index_type: TableIndexType
```

Source: `src/ast/query.rs:1441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The index type (e.g., `INDEX`).

<a id="op-e391b7a89b3360ff0358cd6a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableIndexHints::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableIndexHints) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1433, 35], "end": [1433, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55a532aa4cc7b043ec7de622"></a>
## serialize

`function` · `sqlparser::ast::query::TableIndexHints::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1434, 38], "end": [1434, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1434`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1243952aa27dd81a89b2d182"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexHints::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1435, 40], "end": [1435, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f621019e63890ecc8db3d68"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexHints::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexHints", "path": "TableIndexHints"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1435, 47], "end": [1435, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
