# `sqlparser::ast::trigger::TriggerObject`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerObject.json).

<a id="op-52a5901903af9dc4957cf901"></a>
## TriggerObject

`enum` · `sqlparser::ast::trigger::TriggerObject` · sqlparser 0.62.0

```rust
enum TriggerObject
```

Source: `src/ast/trigger.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This specifies whether the trigger function should be fired once for every row affected by the trigger event, or just once per SQL statement.

<a id="op-df2e89dcf9967e0d92f16b73"></a>
## Row

`variant` · `sqlparser::ast::trigger::TriggerObject::Row` · sqlparser 0.62.0

```rust
Row
```

Source: `src/ast/trigger.rs:27`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The trigger fires once for each row affected by the triggering event

<a id="op-48a7064e994eaebe0a99c453"></a>
## Statement

`variant` · `sqlparser::ast::trigger::TriggerObject::Statement` · sqlparser 0.62.0

```rust
Statement
```

Source: `src/ast/trigger.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The trigger fires once for the triggering SQL statement

<a id="op-d04f0e4ffae7f2940c520b5d"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerObject::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 17], "end": [22, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59ea2051982e3ef4ec0cf130"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerObject::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerObject) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 57], "end": [22, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40436f0dc4bc81a5520e08c6"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerObject::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 49], "end": [23, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b709546cb53c9cc18adb54a"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerObject::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerObject) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 30], "end": [22, 39], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850654fe42afa2ec5c11895f"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 1], "end": [39, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:33`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbda293d42023aa197d23c43"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fb324e437a6aff265e13cbb"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerObject::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 62], "end": [22, 66], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cad894327120d3ec7466001"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerObject::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerObject) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 41], "end": [22, 51], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:22`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a46b052637a43ebedbe5cab1"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerObject::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 38], "end": [23, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06acac3fb1f10d3e132367da"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 47], "end": [24, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:24`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b23965d2bcb1a6a0c663a132"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerObject", "path": "TriggerObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 40], "end": [24, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:24`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
