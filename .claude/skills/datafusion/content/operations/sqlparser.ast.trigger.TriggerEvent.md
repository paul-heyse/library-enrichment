# `sqlparser::ast::trigger::TriggerEvent`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerEvent.json).

<a id="op-e3973755588232f7badbd2c6"></a>
## TriggerEvent

`enum` · `sqlparser::ast::trigger::TriggerEvent` · sqlparser 0.62.0

```rust
enum TriggerEvent
```

Source: `src/ast/trigger.rs:90`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Used to describe trigger events

<a id="op-e38d2a31dd5073fc2636edc3"></a>
## Delete

`variant` · `sqlparser::ast::trigger::TriggerEvent::Delete` · sqlparser 0.62.0

```rust
Delete
```

Source: `src/ast/trigger.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger on DELETE event

<a id="op-fe674a9dd265ceb46aa74b90"></a>
## Insert

`variant` · `sqlparser::ast::trigger::TriggerEvent::Insert` · sqlparser 0.62.0

```rust
Insert
```

Source: `src/ast/trigger.rs:92`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger on INSERT event

<a id="op-8e68a554096706a2be156c1c"></a>
## Truncate

`variant` · `sqlparser::ast::trigger::TriggerEvent::Truncate` · sqlparser 0.62.0

```rust
Truncate
```

Source: `src/ast/trigger.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger on TRUNCATE event

<a id="op-17be7a8283295a50eafca921"></a>
## Update

`variant` · `sqlparser::ast::trigger::TriggerEvent::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/trigger.rs:94`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger on UPDATE event, with optional list of columns

<a id="op-c13536cf74e3ea127318cfe0"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerEvent::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerEvent
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 17], "end": [87, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-376a59801493c6f90bd8b8e9"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerEvent::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerEvent) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 51], "end": [87, 54], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-137a85caa4a73974b82e3c22"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerEvent::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 49], "end": [88, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64ee763fbf01c77dbfc8bfa6"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerEvent::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerEvent) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 24], "end": [87, 33], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-426b52ae4c1e2c6b274615e0"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerEvent::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 10], "end": [87, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2204e209e8c2d6179d35689"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerEvent::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [117, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c32e06b094c5cfc69db9c0fe"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerEvent::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 56], "end": [87, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03332dfc6e81ff2bc1e5c25b"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerEvent::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerEvent) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 35], "end": [87, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:87`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5d95ef16143773cbb71c3d7"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerEvent::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 38], "end": [88, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:88`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f2e7364e65348db8df5a7dc"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerEvent::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 40], "end": [89, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fc8a06fee549dde25ec51ab"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerEvent::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerEvent", "path": "TriggerEvent"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 47], "end": [89, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
