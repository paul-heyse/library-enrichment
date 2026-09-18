# `sqlparser::ast::FileSizeUnit`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FileSizeUnit.json).

<a id="op-d4864f372352260397bc4389"></a>
## FileSizeUnit

`enum` · `sqlparser::ast::FileSizeUnit` · sqlparser 0.62.0

```rust
enum FileSizeUnit
```

Source: `src/ast/mod.rs:9605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Units for `FileSize` (MB or GB).

<a id="op-16d0bae0e0b257225c08275c"></a>
## GB

`variant` · `sqlparser::ast::FileSizeUnit::GB` · sqlparser 0.62.0

```rust
GB
```

Source: `src/ast/mod.rs:9609`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Gigabytes.

<a id="op-a6db06931f65d7be20fb49b5"></a>
## MB

`variant` · `sqlparser::ast::FileSizeUnit::MB` · sqlparser 0.62.0

```rust
MB
```

Source: `src/ast/mod.rs:9607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Megabytes.

<a id="op-dfb012ce1634b327c2e6a54e"></a>
## clone

`function` · `sqlparser::ast::FileSizeUnit::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FileSizeUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9602, 17], "end": [9602, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af896a8c06a67c7f73923c8a"></a>
## cmp

`function` · `sqlparser::ast::FileSizeUnit::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FileSizeUnit) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9602, 51], "end": [9602, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afe2e40b5da62e4e5c55756e"></a>
## deserialize

`function` · `sqlparser::ast::FileSizeUnit::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9603, 49], "end": [9603, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae7c0719f255c8c2efdcb960"></a>
## eq

`function` · `sqlparser::ast::FileSizeUnit::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FileSizeUnit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9602, 24], "end": [9602, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73fa255369df2c6609447892"></a>
## fmt

`function` · `sqlparser::ast::FileSizeUnit::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9602, 10], "end": [9602, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfa5c9d41ed46a5364cdcd38"></a>
## fmt

`function` · `sqlparser::ast::FileSizeUnit::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9612, 1], "end": [9619, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28fd9eeb6ffe557af46197d7"></a>
## hash

`function` · `sqlparser::ast::FileSizeUnit::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9602, 56], "end": [9602, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b67fecd23e9372c360d4d91e"></a>
## partial_cmp

`function` · `sqlparser::ast::FileSizeUnit::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FileSizeUnit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9602, 35], "end": [9602, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-913cb7775bdade9d3fa6d959"></a>
## serialize

`function` · `sqlparser::ast::FileSizeUnit::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9603, 38], "end": [9603, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37507ae4915fbbddf326f94f"></a>
## visit

`function` · `sqlparser::ast::FileSizeUnit::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9604, 47], "end": [9604, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b1f34982d475f3f92f90c91"></a>
## visit

`function` · `sqlparser::ast::FileSizeUnit::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSizeUnit", "path": "FileSizeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9604, 40], "end": [9604, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
