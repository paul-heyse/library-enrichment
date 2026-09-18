# `sqlparser::ast::query::TableIndexType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableIndexType.json).

<a id="op-e26bf671bd1eb39d841b1e48"></a>
## TableIndexType

`enum` · `sqlparser::ast::query::TableIndexType` · sqlparser 0.62.0

```rust
enum TableIndexType
```

Source: `src/ast/query.rs:1394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of index referenced by an index hint (e.g. `USE INDEX`).

<a id="op-39817dd72f48b039e814ff00"></a>
## Index

`variant` · `sqlparser::ast::query::TableIndexType::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/query.rs:1396`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `INDEX` kind.

<a id="op-f31b0b3b997412e051439900"></a>
## Key

`variant` · `sqlparser::ast::query::TableIndexType::Key` · sqlparser 0.62.0

```rust
Key
```

Source: `src/ast/query.rs:1398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `KEY` kind.

<a id="op-439fb4a56e052c50ca348e88"></a>
## clone

`function` · `sqlparser::ast::query::TableIndexType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableIndexType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1390, 17], "end": [1390, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f82973d0d2d982a962993c47"></a>
## cmp

`function` · `sqlparser::ast::query::TableIndexType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableIndexType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1390, 57], "end": [1390, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a23ced5c1f9a7dc71a07304"></a>
## deserialize

`function` · `sqlparser::ast::query::TableIndexType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1391, 49], "end": [1391, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef8f427888f30b9dc7ac3a00"></a>
## eq

`function` · `sqlparser::ast::query::TableIndexType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableIndexType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1390, 30], "end": [1390, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48f7d4c4c925fa217f71d212"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1401, 1], "end": [1408, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1402`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c68ba0fb54ac78f40e20131e"></a>
## fmt

`function` · `sqlparser::ast::query::TableIndexType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1390, 10], "end": [1390, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0bc75ca15c0ed9f25c37496"></a>
## hash

`function` · `sqlparser::ast::query::TableIndexType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1390, 62], "end": [1390, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d05796df1418d86d3eedbd74"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableIndexType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableIndexType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1390, 41], "end": [1390, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60875e10f87662a689ffc815"></a>
## serialize

`function` · `sqlparser::ast::query::TableIndexType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1391, 38], "end": [1391, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d25391a30c6eb0748938a0d"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1392, 40], "end": [1392, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42e87dcec77d4e1bf334a5d3"></a>
## visit

`function` · `sqlparser::ast::query::TableIndexType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableIndexType", "path": "TableIndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1392, 47], "end": [1392, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
