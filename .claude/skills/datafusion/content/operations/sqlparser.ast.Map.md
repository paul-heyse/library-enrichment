# `sqlparser::ast::Map`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Map.json).

<a id="op-bcedfd9429a65edff6ba01e1"></a>
## Map

`struct` · `sqlparser::ast::Map` · sqlparser 0.62.0

```rust
struct Map
```

Source: `src/ast/mod.rs:630`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a Map expression.

<a id="op-18cab9f2ad2f52184ce1a0c0"></a>
## clone

`function` · `sqlparser::ast::Map::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Map
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [627, 17], "end": [627, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71f8dd6385ba100f1cb985fc"></a>
## cmp

`function` · `sqlparser::ast::Map::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Map) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [627, 51], "end": [627, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529bfb7b30b6fe6c576a2a41"></a>
## deserialize

`function` · `sqlparser::ast::Map::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 49], "end": [628, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9bd58e11e2fe86d572c8965"></a>
## entries

`struct_field` · `sqlparser::ast::Map::entries` · sqlparser 0.62.0

```rust
entries: Vec<MapEntry>
```

Source: `src/ast/mod.rs:632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Entries of the map as key/value pairs.

<a id="op-7a05c6f06aa876ada7d3af91"></a>
## eq

`function` · `sqlparser::ast::Map::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Map) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [627, 24], "end": [627, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e4d847824a1302bc58689e4"></a>
## fmt

`function` · `sqlparser::ast::Map::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [627, 10], "end": [627, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b16304a6bc5c493b066837eb"></a>
## fmt

`function` · `sqlparser::ast::Map::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [635, 1], "end": [639, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:636`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4add7de4c415d954391da05c"></a>
## hash

`function` · `sqlparser::ast::Map::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [627, 56], "end": [627, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-334272140e7976350c4e29f9"></a>
## partial_cmp

`function` · `sqlparser::ast::Map::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Map) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [627, 35], "end": [627, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f6ae5040ed8555b57a86b13"></a>
## serialize

`function` · `sqlparser::ast::Map::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [628, 38], "end": [628, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ed9c548b781cb8265183ba5"></a>
## visit

`function` · `sqlparser::ast::Map::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 40], "end": [629, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2cea968cd639fbbe4bbc261"></a>
## visit

`function` · `sqlparser::ast::Map::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [629, 47], "end": [629, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
