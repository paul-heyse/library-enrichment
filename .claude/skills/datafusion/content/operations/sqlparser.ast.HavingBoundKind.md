# `sqlparser::ast::HavingBoundKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HavingBoundKind.json).

<a id="op-f32e9ddf01803933ca9f7e9d"></a>
## HavingBoundKind

`enum` · `sqlparser::ast::HavingBoundKind` · sqlparser 0.62.0

```rust
enum HavingBoundKind
```

Source: `src/ast/mod.rs:8441`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Which bound is used in a HAVING clause for ANY_VALUE on BigQuery.

<a id="op-3e2a72bece87e56af38aa712"></a>
## Max

`variant` · `sqlparser::ast::HavingBoundKind::Max` · sqlparser 0.62.0

```rust
Max
```

Source: `src/ast/mod.rs:8445`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The maximum bound.

<a id="op-bbeded72f66e00895d57a4b6"></a>
## Min

`variant` · `sqlparser::ast::HavingBoundKind::Min` · sqlparser 0.62.0

```rust
Min
```

Source: `src/ast/mod.rs:8443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The minimum bound.

<a id="op-0e55d4660fb5e321befd466a"></a>
## clone

`function` · `sqlparser::ast::HavingBoundKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HavingBoundKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8437, 23], "end": [8437, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-756c6e266c9f75491d3c678a"></a>
## cmp

`function` · `sqlparser::ast::HavingBoundKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HavingBoundKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8437, 57], "end": [8437, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde113d300b8a5b043a5d77b"></a>
## deserialize

`function` · `sqlparser::ast::HavingBoundKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8438, 49], "end": [8438, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a3c542bbea268b91bfa1bd2"></a>
## eq

`function` · `sqlparser::ast::HavingBoundKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HavingBoundKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8437, 30], "end": [8437, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f30091dcf2f3c2ba44c578a"></a>
## fmt

`function` · `sqlparser::ast::HavingBoundKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8448, 1], "end": [8455, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8449`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7d42f4747b352f8ad53ef32"></a>
## fmt

`function` · `sqlparser::ast::HavingBoundKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8437, 10], "end": [8437, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f4ade00e8b113550d2b16d0"></a>
## hash

`function` · `sqlparser::ast::HavingBoundKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8437, 62], "end": [8437, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50689e058a44171cd34bfd99"></a>
## partial_cmp

`function` · `sqlparser::ast::HavingBoundKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HavingBoundKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8437, 41], "end": [8437, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63f69cb23a6a491bf13b1efb"></a>
## serialize

`function` · `sqlparser::ast::HavingBoundKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8438, 38], "end": [8438, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8438`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dea2209f2c6f52712a65193"></a>
## visit

`function` · `sqlparser::ast::HavingBoundKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8439, 40], "end": [8439, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72c3baa256efdbfcf0ac4c64"></a>
## visit

`function` · `sqlparser::ast::HavingBoundKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HavingBoundKind", "path": "HavingBoundKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8439, 47], "end": [8439, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
