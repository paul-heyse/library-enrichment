# `sqlparser::ast::MapEntry`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MapEntry.json).

<a id="op-24f1723d82b28078770ef506"></a>
## MapEntry

`struct` · `sqlparser::ast::MapEntry` · sqlparser 0.62.0

```rust
struct MapEntry
```

Source: `src/ast/mod.rs:647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A map field within a map.

[DuckDB]: https://duckdb.org/docs/sql/data_types/map.html#creating-maps

<a id="op-060f28980d4d54e92e24a2ff"></a>
## clone

`function` · `sqlparser::ast::MapEntry::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MapEntry
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 17], "end": [644, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f80b709b68c7a6f44d02334"></a>
## cmp

`function` · `sqlparser::ast::MapEntry::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MapEntry) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 51], "end": [644, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-151b7a7b66ec0e8f17533ff2"></a>
## deserialize

`function` · `sqlparser::ast::MapEntry::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 49], "end": [645, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83ba86600749ab1eae662700"></a>
## eq

`function` · `sqlparser::ast::MapEntry::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MapEntry) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 24], "end": [644, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-783698b4874e3b3d94091847"></a>
## fmt

`function` · `sqlparser::ast::MapEntry::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 10], "end": [644, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1fe5ff55bba7ef80d048f67"></a>
## fmt

`function` · `sqlparser::ast::MapEntry::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [658, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46308d7fdc52cc02841c683e"></a>
## hash

`function` · `sqlparser::ast::MapEntry::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 56], "end": [644, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f87aa71974ef55e9f59aef57"></a>
## key

`struct_field` · `sqlparser::ast::MapEntry::key` · sqlparser 0.62.0

```rust
key: Box<Expr>
```

Source: `src/ast/mod.rs:649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Key expression of the map entry.

<a id="op-ff431ca67c2324aa5107f311"></a>
## partial_cmp

`function` · `sqlparser::ast::MapEntry::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MapEntry) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [644, 35], "end": [644, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3aec18de7aecd6b41e081309"></a>
## serialize

`function` · `sqlparser::ast::MapEntry::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [645, 38], "end": [645, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-205c84a4bafb7837b1f8ad73"></a>
## value

`struct_field` · `sqlparser::ast::MapEntry::value` · sqlparser 0.62.0

```rust
value: Box<Expr>
```

Source: `src/ast/mod.rs:651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value expression of the map entry.

<a id="op-380f03a685107ad1be30518e"></a>
## visit

`function` · `sqlparser::ast::MapEntry::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 40], "end": [646, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e3d11c1588ceabae5529784"></a>
## visit

`function` · `sqlparser::ast::MapEntry::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MapEntry", "path": "MapEntry"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [646, 47], "end": [646, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
