# `sqlparser::ast::HiveRowDelimiter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveRowDelimiter.json).

<a id="op-be82070517e277d644092add"></a>
## HiveRowDelimiter

`struct` · `sqlparser::ast::HiveRowDelimiter` · sqlparser 0.62.0

```rust
struct HiveRowDelimiter
```

Source: `src/ast/mod.rs:8590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single row delimiter specification for Hive `ROW FORMAT`.

<a id="op-30e3dabe64ccab392f4bb5bb"></a>
## char

`struct_field` · `sqlparser::ast::HiveRowDelimiter::char` · sqlparser 0.62.0

```rust
char: Ident
```

Source: `src/ast/mod.rs:8594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The delimiter character identifier.

<a id="op-3dd2b6d5d0d87f298df6c841"></a>
## clone

`function` · `sqlparser::ast::HiveRowDelimiter::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveRowDelimiter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8586, 17], "end": [8586, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0755474615a983d1400f0a5"></a>
## cmp

`function` · `sqlparser::ast::HiveRowDelimiter::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveRowDelimiter) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8586, 51], "end": [8586, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d878d8babdde3c454ba06389"></a>
## delimiter

`struct_field` · `sqlparser::ast::HiveRowDelimiter::delimiter` · sqlparser 0.62.0

```rust
delimiter: HiveDelimiter
```

Source: `src/ast/mod.rs:8592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The delimiter kind (fields/lines/etc.).

<a id="op-cbf5e43443e0b300136964ed"></a>
## deserialize

`function` · `sqlparser::ast::HiveRowDelimiter::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8587, 49], "end": [8587, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdb2ce2780c2de14724e19b2"></a>
## eq

`function` · `sqlparser::ast::HiveRowDelimiter::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveRowDelimiter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8586, 24], "end": [8586, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-226edfe6550df30f1f0826ae"></a>
## fmt

`function` · `sqlparser::ast::HiveRowDelimiter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8597, 1], "end": [8602, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7cad6cee35acb4e5055c669"></a>
## fmt

`function` · `sqlparser::ast::HiveRowDelimiter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8586, 10], "end": [8586, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4044f937876e68cc2b016622"></a>
## hash

`function` · `sqlparser::ast::HiveRowDelimiter::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8586, 56], "end": [8586, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8ff65fca90a47009ec46a1a"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveRowDelimiter::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveRowDelimiter) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8586, 35], "end": [8586, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be367a470724bf98becd3457"></a>
## serialize

`function` · `sqlparser::ast::HiveRowDelimiter::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8587, 38], "end": [8587, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-986dd34c18914ce41f1f2144"></a>
## visit

`function` · `sqlparser::ast::HiveRowDelimiter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8588, 47], "end": [8588, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a24c2e47e1fff9a40bf25949"></a>
## visit

`function` · `sqlparser::ast::HiveRowDelimiter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveRowDelimiter", "path": "HiveRowDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8588, 40], "end": [8588, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
