# `sqlparser::ast::OnInsert`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OnInsert.json).

<a id="op-cad724a99294f243d2da42ef"></a>
## OnInsert

`enum` · `sqlparser::ast::OnInsert` · sqlparser 0.62.0

```rust
enum OnInsert
```

Source: `src/ast/mod.rs:6694`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Behavior to apply for `INSERT` when a conflict occurs.

<a id="op-21d6333dbf46eaf4069ff408"></a>
## DuplicateKeyUpdate

`variant` · `sqlparser::ast::OnInsert::DuplicateKeyUpdate` · sqlparser 0.62.0

```rust
DuplicateKeyUpdate
```

Source: `src/ast/mod.rs:6696`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ON DUPLICATE KEY UPDATE (MySQL when the key already exists, then execute an update instead)

<a id="op-ad6cf2a92c9dad3f5e1ede60"></a>
## OnConflict

`variant` · `sqlparser::ast::OnInsert::OnConflict` · sqlparser 0.62.0

```rust
OnConflict
```

Source: `src/ast/mod.rs:6698`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ON CONFLICT is a PostgreSQL and Sqlite extension

<a id="op-a2ee0c13697ba13dadec93ef"></a>
## clone

`function` · `sqlparser::ast::OnInsert::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OnInsert
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6689, 17], "end": [6689, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3ad14fcac20a602e3cef432"></a>
## cmp

`function` · `sqlparser::ast::OnInsert::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OnInsert) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6689, 51], "end": [6689, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f75dd571a072014a5a0578a"></a>
## deserialize

`function` · `sqlparser::ast::OnInsert::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6690, 49], "end": [6690, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcb4f1eb75448af83083d2c9"></a>
## eq

`function` · `sqlparser::ast::OnInsert::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OnInsert) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6689, 24], "end": [6689, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0afa6e32b85af1264299a1e0"></a>
## fmt

`function` · `sqlparser::ast::OnInsert::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6765, 1], "end": [6776, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a48aef4d6af7658da783f924"></a>
## fmt

`function` · `sqlparser::ast::OnInsert::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6689, 10], "end": [6689, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf031470bc10072739b31c1d"></a>
## hash

`function` · `sqlparser::ast::OnInsert::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6689, 56], "end": [6689, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94afe177001826eba9e1cbe6"></a>
## partial_cmp

`function` · `sqlparser::ast::OnInsert::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OnInsert) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6689, 35], "end": [6689, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2ac93a1d2b32a20da63dbac"></a>
## serialize

`function` · `sqlparser::ast::OnInsert::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6690, 38], "end": [6690, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6690`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00e87ec0e695258472eb9b13"></a>
## span

`function` · `sqlparser::ast::OnInsert::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "super::OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1373, 1], "end": [1380, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7d3fb2d9036c47ee72348dc"></a>
## visit

`function` · `sqlparser::ast::OnInsert::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6691, 40], "end": [6691, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6691`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa786bcab18101493e679e5e"></a>
## visit

`function` · `sqlparser::ast::OnInsert::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnInsert", "path": "OnInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6691, 47], "end": [6691, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6691`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
