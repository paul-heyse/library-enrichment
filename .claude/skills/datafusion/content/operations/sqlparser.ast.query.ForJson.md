# `sqlparser::ast::query::ForJson`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ForJson.json).

<a id="op-2bd3efd6d87919d34ac0d26b"></a>
## ForJson

`enum` · `sqlparser::ast::query::ForJson` · sqlparser 0.62.0

```rust
enum ForJson
```

Source: `src/ast/query.rs:3956`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modes for `FOR JSON` clause.

<a id="op-d3f2479a499e86408084d02e"></a>
## Auto

`variant` · `sqlparser::ast::query::ForJson::Auto` · sqlparser 0.62.0

```rust
Auto
```

Source: `src/ast/query.rs:3958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AUTO` mode.

<a id="op-f228596ebb33f04ec4c015eb"></a>
## Path

`variant` · `sqlparser::ast::query::ForJson::Path` · sqlparser 0.62.0

```rust
Path
```

Source: `src/ast/query.rs:3960`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PATH` mode.

<a id="op-c77c55438e9ed14d891228e6"></a>
## clone

`function` · `sqlparser::ast::query::ForJson::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ForJson
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3952, 17], "end": [3952, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92783fd4e6aafc1839b1872d"></a>
## cmp

`function` · `sqlparser::ast::query::ForJson::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ForJson) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3952, 51], "end": [3952, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21c600746f2f60b224598808"></a>
## deserialize

`function` · `sqlparser::ast::query::ForJson::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3954, 49], "end": [3954, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f559fe49ca37e949552b035"></a>
## eq

`function` · `sqlparser::ast::query::ForJson::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ForJson) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3952, 24], "end": [3952, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2da85b6c37b30a06d04edfb8"></a>
## fmt

`function` · `sqlparser::ast::query::ForJson::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3952, 10], "end": [3952, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da9b97ffb71e0d13b6ca3610"></a>
## fmt

`function` · `sqlparser::ast::query::ForJson::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3963, 1], "end": [3970, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86b838bf7710af4b9f17c5bc"></a>
## hash

`function` · `sqlparser::ast::query::ForJson::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3952, 56], "end": [3952, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb51048007aee47844f0371b"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ForJson::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ForJson) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3952, 35], "end": [3952, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-facec48a53d89496be2fde09"></a>
## serialize

`function` · `sqlparser::ast::query::ForJson::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3954, 38], "end": [3954, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d718107d9acffecdb2e50a0"></a>
## visit

`function` · `sqlparser::ast::query::ForJson::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3953, 47], "end": [3953, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f068eb2efa1827685792541b"></a>
## visit

`function` · `sqlparser::ast::query::ForJson::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ForJson", "path": "ForJson"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3953, 40], "end": [3953, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
