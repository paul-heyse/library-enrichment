# `sqlparser::ast::ClusteredIndex`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ClusteredIndex.json).

<a id="op-4dfdae7ebac4b5857ef7b729"></a>
## ClusteredIndex

`struct` · `sqlparser::ast::ClusteredIndex` · sqlparser 0.62.0

```rust
struct ClusteredIndex
```

Source: `src/ast/mod.rs:8730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A clustered index column specification.

<a id="op-69a74e59fe2b4b9541039269"></a>
## asc

`struct_field` · `sqlparser::ast::ClusteredIndex::asc` · sqlparser 0.62.0

```rust
asc: Option<bool>
```

Source: `src/ast/mod.rs:8734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional sort direction: `Some(true)` for ASC, `Some(false)` for DESC, `None` for unspecified.

<a id="op-1b740e8a0762d638d3226e2a"></a>
## clone

`function` · `sqlparser::ast::ClusteredIndex::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ClusteredIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8726, 17], "end": [8726, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6875216b39dc148d9a8b9d74"></a>
## cmp

`function` · `sqlparser::ast::ClusteredIndex::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ClusteredIndex) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8726, 51], "end": [8726, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-775eda28885ac639df19287b"></a>
## deserialize

`function` · `sqlparser::ast::ClusteredIndex::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8727, 49], "end": [8727, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-013b0fbb84a8c9a09373957e"></a>
## eq

`function` · `sqlparser::ast::ClusteredIndex::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ClusteredIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8726, 24], "end": [8726, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75d0aa5853dbf76d8f6d44f3"></a>
## fmt

`function` · `sqlparser::ast::ClusteredIndex::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8737, 1], "end": [8746, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae88b2340b0c7c187a6876b0"></a>
## fmt

`function` · `sqlparser::ast::ClusteredIndex::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8726, 10], "end": [8726, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec7c08391e7d9ef7e949979d"></a>
## hash

`function` · `sqlparser::ast::ClusteredIndex::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8726, 56], "end": [8726, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a729ef91abf145ebeb98a792"></a>
## name

`struct_field` · `sqlparser::ast::ClusteredIndex::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:8732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column identifier for the clustered index entry.

<a id="op-ae617095d7a045a8db772984"></a>
## partial_cmp

`function` · `sqlparser::ast::ClusteredIndex::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ClusteredIndex) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8726, 35], "end": [8726, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e235b6385ff575650cecd6d"></a>
## serialize

`function` · `sqlparser::ast::ClusteredIndex::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8727, 38], "end": [8727, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-714458f7a5f3ce42f049542b"></a>
## span

`function` · `sqlparser::ast::ClusteredIndex::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "super::ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1066, 1], "end": [1075, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-249d8b6d527f221503260541"></a>
## visit

`function` · `sqlparser::ast::ClusteredIndex::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8728, 47], "end": [8728, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8728`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b9ef3e7f398a6da91f0ab03"></a>
## visit

`function` · `sqlparser::ast::ClusteredIndex::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ClusteredIndex", "path": "ClusteredIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8728, 40], "end": [8728, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8728`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
