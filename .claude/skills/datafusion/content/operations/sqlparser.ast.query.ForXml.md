# `sqlparser::ast::query::ForXml`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ForXml.json).

<a id="op-0db242afed6982f441ba27fd"></a>
## ForXml

`enum` · `sqlparser::ast::query::ForXml` · sqlparser 0.62.0

```rust
enum ForXml
```

Source: `src/ast/query.rs:3918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modes for `FOR XML` clause.

<a id="op-f6a8d0f267f273fc9f216733"></a>
## Auto

`variant` · `sqlparser::ast::query::ForXml::Auto` · sqlparser 0.62.0

```rust
Auto
```

Source: `src/ast/query.rs:3922`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AUTO` mode.

<a id="op-bda1604e3a645d878aafed7c"></a>
## Explicit

`variant` · `sqlparser::ast::query::ForXml::Explicit` · sqlparser 0.62.0

```rust
Explicit
```

Source: `src/ast/query.rs:3924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXPLICIT` mode.

<a id="op-f879595ac14795c9d4016814"></a>
## Path

`variant` · `sqlparser::ast::query::ForXml::Path` · sqlparser 0.62.0

```rust
Path
```

Source: `src/ast/query.rs:3926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PATH` mode with optional root: `PATH('root')`.

<a id="op-704d2b4687aafcaf31a02409"></a>
## Raw

`variant` · `sqlparser::ast::query::ForXml::Raw` · sqlparser 0.62.0

```rust
Raw
```

Source: `src/ast/query.rs:3920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RAW` mode with optional root name: `RAW('root')`.

<a id="op-717d7235d1ca9d64175ad19c"></a>
## clone

`function` · `sqlparser::ast::query::ForXml::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ForXml
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3914, 17], "end": [3914, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36c1eff0d25ac540ab04d864"></a>
## cmp

`function` · `sqlparser::ast::query::ForXml::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ForXml) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3914, 51], "end": [3914, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ae488ec52911ee29654513a"></a>
## deserialize

`function` · `sqlparser::ast::query::ForXml::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3915, 49], "end": [3915, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3915`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79bc5c1d748c9103c6b1e8f8"></a>
## eq

`function` · `sqlparser::ast::query::ForXml::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ForXml) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3914, 24], "end": [3914, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-411ef9bbe1429492e35f36ea"></a>
## fmt

`function` · `sqlparser::ast::query::ForXml::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3929, 1], "end": [3950, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7de36bb61f16170ea961a6f9"></a>
## fmt

`function` · `sqlparser::ast::query::ForXml::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3914, 10], "end": [3914, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-495bd62484c297c52bf73507"></a>
## hash

`function` · `sqlparser::ast::query::ForXml::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3914, 56], "end": [3914, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d3da2677521a48c1a64e43c"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ForXml::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ForXml) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3914, 35], "end": [3914, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3914`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2965ecafa9cc3a77fe039fc7"></a>
## serialize

`function` · `sqlparser::ast::query::ForXml::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3915, 38], "end": [3915, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3915`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-901f14c20ffc93b7c76b527a"></a>
## visit

`function` · `sqlparser::ast::query::ForXml::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 47], "end": [3916, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c1df2bd63beb240e3808605"></a>
## visit

`function` · `sqlparser::ast::query::ForXml::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForXml", "path": "ForXml"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 40], "end": [3916, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
