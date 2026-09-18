# `sqlparser::ast::TransactionMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.TransactionMode.json).

<a id="op-cc62b26c2c2ea70abe5603c1"></a>
## TransactionMode

`enum` · `sqlparser::ast::TransactionMode` · sqlparser 0.62.0

```rust
enum TransactionMode
```

Source: `src/ast/mod.rs:9040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Mode for transactions: access mode or isolation level.

<a id="op-aecf0aabf386b85d5f0f21d1"></a>
## AccessMode

`variant` · `sqlparser::ast::TransactionMode::AccessMode` · sqlparser 0.62.0

```rust
AccessMode
```

Source: `src/ast/mod.rs:9042`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Access mode for a transaction (e.g. `READ ONLY` / `READ WRITE`).

<a id="op-5caffa9f344637765489a4d9"></a>
## IsolationLevel

`variant` · `sqlparser::ast::TransactionMode::IsolationLevel` · sqlparser 0.62.0

```rust
IsolationLevel
```

Source: `src/ast/mod.rs:9044`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Isolation level for a transaction (e.g. `SERIALIZABLE`).

<a id="op-81056df9060d572057734ec1"></a>
## clone

`function` · `sqlparser::ast::TransactionMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TransactionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9036, 23], "end": [9036, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42a400bb73b30e04013a0684"></a>
## cmp

`function` · `sqlparser::ast::TransactionMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TransactionMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9036, 57], "end": [9036, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3fccb2812c985d5e31aca08"></a>
## deserialize

`function` · `sqlparser::ast::TransactionMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9037, 49], "end": [9037, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1e991e73367df4f391498b1"></a>
## eq

`function` · `sqlparser::ast::TransactionMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TransactionMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9036, 30], "end": [9036, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21b6068f2226cef4e94e9bb5"></a>
## fmt

`function` · `sqlparser::ast::TransactionMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9047, 1], "end": [9055, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d6124de01b21fd58dd5c016"></a>
## fmt

`function` · `sqlparser::ast::TransactionMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9036, 10], "end": [9036, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-821b1a414af9a6bae44df3b0"></a>
## hash

`function` · `sqlparser::ast::TransactionMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9036, 62], "end": [9036, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d428767ad59aa157584f2fe1"></a>
## partial_cmp

`function` · `sqlparser::ast::TransactionMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TransactionMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9036, 41], "end": [9036, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a95877c98d7422945c64ca86"></a>
## serialize

`function` · `sqlparser::ast::TransactionMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9037, 38], "end": [9037, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1275a1c457eecc45b0d38e63"></a>
## visit

`function` · `sqlparser::ast::TransactionMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9038, 40], "end": [9038, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5ba38e99ad2b9e0c6b4b2c0"></a>
## visit

`function` · `sqlparser::ast::TransactionMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::TransactionMode", "path": "TransactionMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9038, 47], "end": [9038, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
