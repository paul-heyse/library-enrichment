# `sqlparser::ast::ContextModifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ContextModifier.json).

<a id="op-5685a7190fb03a66e950e2da"></a>
## ContextModifier

`enum` · `sqlparser::ast::ContextModifier` · sqlparser 0.62.0

```rust
enum ContextModifier
```

Source: `src/ast/mod.rs:9806`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional context modifier for statements that can be or `LOCAL`, `GLOBAL`, or `SESSION`.

<a id="op-f5a5e760e5c0efba03b3514a"></a>
## Global

`variant` · `sqlparser::ast::ContextModifier::Global` · sqlparser 0.62.0

```rust
Global
```

Source: `src/ast/mod.rs:9812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`GLOBAL` identifier

<a id="op-fa1caaaacfc58830cb9ac64e"></a>
## Local

`variant` · `sqlparser::ast::ContextModifier::Local` · sqlparser 0.62.0

```rust
Local
```

Source: `src/ast/mod.rs:9808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LOCAL` identifier, usually related to transactional states.

<a id="op-9e4e66face73573db889ff2f"></a>
## Session

`variant` · `sqlparser::ast::ContextModifier::Session` · sqlparser 0.62.0

```rust
Session
```

Source: `src/ast/mod.rs:9810`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SESSION` identifier

<a id="op-bbf0bf68bee401bd15857356"></a>
## clone

`function` · `sqlparser::ast::ContextModifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ContextModifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9803, 23], "end": [9803, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cef071fa24cc40c1a863816"></a>
## cmp

`function` · `sqlparser::ast::ContextModifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ContextModifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9803, 57], "end": [9803, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebdd9af48c9720b5a77cd64e"></a>
## deserialize

`function` · `sqlparser::ast::ContextModifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9804, 49], "end": [9804, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f022ee8dc8bd5c3ebc63c3d"></a>
## eq

`function` · `sqlparser::ast::ContextModifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ContextModifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9803, 30], "end": [9803, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b05fdb6872cf4d774b8dc5eb"></a>
## fmt

`function` · `sqlparser::ast::ContextModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9815, 1], "end": [9829, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4a81c929e1c9a4e8044e6d3"></a>
## fmt

`function` · `sqlparser::ast::ContextModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9803, 10], "end": [9803, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1b3efc5b36928a1a5d36bb9"></a>
## hash

`function` · `sqlparser::ast::ContextModifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9803, 62], "end": [9803, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5de885ffbc943c8a04742c4"></a>
## partial_cmp

`function` · `sqlparser::ast::ContextModifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ContextModifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9803, 41], "end": [9803, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a39a97487d1ab7a3d1b3916"></a>
## serialize

`function` · `sqlparser::ast::ContextModifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9804, 38], "end": [9804, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fed7b9f1cb5cdb2c5338f45"></a>
## visit

`function` · `sqlparser::ast::ContextModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9805, 47], "end": [9805, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edb883a319cc0f8d96ca5721"></a>
## visit

`function` · `sqlparser::ast::ContextModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ContextModifier", "path": "ContextModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9805, 40], "end": [9805, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
