# `sqlparser::ast::TableOptionsClustered`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TableOptionsClustered.json).

<a id="op-9cd16e4659074e70f0167bef"></a>
## TableOptionsClustered

`enum` · `sqlparser::ast::TableOptionsClustered` · sqlparser 0.62.0

```rust
enum TableOptionsClustered
```

Source: `src/ast/mod.rs:8752`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Clustered options used for `CREATE TABLE` clustered/indexed storage.

<a id="op-602849cca583c317a9ea42d1"></a>
## ColumnstoreIndex

`variant` · `sqlparser::ast::TableOptionsClustered::ColumnstoreIndex` · sqlparser 0.62.0

```rust
ColumnstoreIndex
```

Source: `src/ast/mod.rs:8754`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use a columnstore index.

<a id="op-b38afe93a8138da5d49360c2"></a>
## ColumnstoreIndexOrder

`variant` · `sqlparser::ast::TableOptionsClustered::ColumnstoreIndexOrder` · sqlparser 0.62.0

```rust
ColumnstoreIndexOrder
```

Source: `src/ast/mod.rs:8756`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columnstore index with an explicit ordering of columns.

<a id="op-098d4226f4792fa7dac4c4da"></a>
## Index

`variant` · `sqlparser::ast::TableOptionsClustered::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/mod.rs:8758`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named clustered index with one or more columns.

<a id="op-d762492fc6c8156dcfa89532"></a>
## clone

`function` · `sqlparser::ast::TableOptionsClustered::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableOptionsClustered
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8748, 17], "end": [8748, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89e877d93f6e042b71d76ece"></a>
## cmp

`function` · `sqlparser::ast::TableOptionsClustered::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableOptionsClustered) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8748, 51], "end": [8748, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19d38188d43592f19347b855"></a>
## deserialize

`function` · `sqlparser::ast::TableOptionsClustered::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8749, 49], "end": [8749, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dfa66bc2f134d03b9183c76"></a>
## eq

`function` · `sqlparser::ast::TableOptionsClustered::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableOptionsClustered) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8748, 24], "end": [8748, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cbcea5809fa68e23d5efa29"></a>
## fmt

`function` · `sqlparser::ast::TableOptionsClustered::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8761, 1], "end": [8779, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a8e83b7d325bc38b4312df3"></a>
## fmt

`function` · `sqlparser::ast::TableOptionsClustered::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8748, 10], "end": [8748, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db3ae2eed590a620df093056"></a>
## hash

`function` · `sqlparser::ast::TableOptionsClustered::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8748, 56], "end": [8748, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df89796e2606bebc5b896d60"></a>
## partial_cmp

`function` · `sqlparser::ast::TableOptionsClustered::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableOptionsClustered) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8748, 35], "end": [8748, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8748`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7199c1d24f7f2c830f72ea15"></a>
## serialize

`function` · `sqlparser::ast::TableOptionsClustered::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8749, 38], "end": [8749, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01bd1f1f45062a0ff778186b"></a>
## span

`function` · `sqlparser::ast::TableOptionsClustered::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "super::TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1054, 1], "end": [1064, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cb2dcf00d7cdce86766b0fd"></a>
## visit

`function` · `sqlparser::ast::TableOptionsClustered::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8750, 40], "end": [8750, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6bde259ff4daf1acb81895f"></a>
## visit

`function` · `sqlparser::ast::TableOptionsClustered::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TableOptionsClustered", "path": "TableOptionsClustered"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8750, 47], "end": [8750, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
