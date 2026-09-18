# `sqlparser::ast::query::TableWithJoins`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableWithJoins.json).

<a id="op-1185b974291f410266fff12c"></a>
## TableWithJoins

`struct` · `sqlparser::ast::query::TableWithJoins` · sqlparser 0.62.0

```rust
struct TableWithJoins
```

Source: `src/ast/query.rs:1209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A left table followed by zero or more joins.

<a id="op-84bfca8cee1d1adb0330324a"></a>
## clone

`function` · `sqlparser::ast::query::TableWithJoins::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableWithJoins
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 17], "end": [1205, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a28094158bcd6e687833591c"></a>
## cmp

`function` · `sqlparser::ast::query::TableWithJoins::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableWithJoins) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 51], "end": [1205, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b6323a9719ce8d3bdff450e"></a>
## deserialize

`function` · `sqlparser::ast::query::TableWithJoins::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1206, 49], "end": [1206, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d86a9a4891f79242e5c0c6f"></a>
## eq

`function` · `sqlparser::ast::query::TableWithJoins::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableWithJoins) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 24], "end": [1205, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a79bff13b15b05db28d6ce5"></a>
## fmt

`function` · `sqlparser::ast::query::TableWithJoins::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1216, 1], "end": [1225, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1217`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6984ebf8538505d83da4ea5c"></a>
## fmt

`function` · `sqlparser::ast::query::TableWithJoins::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 10], "end": [1205, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a1004da1e2d056a47968c62"></a>
## hash

`function` · `sqlparser::ast::query::TableWithJoins::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 56], "end": [1205, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d3f7808da12fd72784a5d45"></a>
## joins

`struct_field` · `sqlparser::ast::query::TableWithJoins::joins` · sqlparser 0.62.0

```rust
joins: Vec<Join>
```

Source: `src/ast/query.rs:1213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The sequence of joins applied to the relation.

<a id="op-0a8a39d1b4a0021f599e900c"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableWithJoins::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableWithJoins) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1205, 35], "end": [1205, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2efc5ece96da3147ec919e9"></a>
## relation

`struct_field` · `sqlparser::ast::query::TableWithJoins::relation` · sqlparser 0.62.0

```rust
relation: TableFactor
```

Source: `src/ast/query.rs:1211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The starting table factor (left side) of the join chain.

<a id="op-0d18d28aba1671a794e09959"></a>
## serialize

`function` · `sqlparser::ast::query::TableWithJoins::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1206, 38], "end": [1206, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d6c5431f5915339823f45c0"></a>
## span

`function` · `sqlparser::ast::query::TableWithJoins::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "super::TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2278, 1], "end": [2284, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b60bd972b9c61910fb323e9a"></a>
## visit

`function` · `sqlparser::ast::query::TableWithJoins::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 47], "end": [1207, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de615d442d7ea6da2b8d5e4a"></a>
## visit

`function` · `sqlparser::ast::query::TableWithJoins::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableWithJoins", "path": "TableWithJoins"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1207, 40], "end": [1207, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
