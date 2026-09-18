# `sqlparser::ast::trigger::TriggerReferencingType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerReferencingType.json).

<a id="op-6567ab2568d70e072f72114d"></a>
## TriggerReferencingType

`enum` · `sqlparser::ast::trigger::TriggerReferencingType` · sqlparser 0.62.0

```rust
enum TriggerReferencingType
```

Source: `src/ast/trigger.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This clause indicates whether the following relation name is for the before-image transition relation or the after-image transition relation

<a id="op-4a905ef26465391a4a7c9724"></a>
## NewTable

`variant` · `sqlparser::ast::trigger::TriggerReferencingType::NewTable` · sqlparser 0.62.0

```rust
NewTable
```

Source: `src/ast/trigger.rs:49`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The transition relation containing the new rows affected by the triggering statement

<a id="op-6f30521687c735149281dbd1"></a>
## OldTable

`variant` · `sqlparser::ast::trigger::TriggerReferencingType::OldTable` · sqlparser 0.62.0

```rust
OldTable
```

Source: `src/ast/trigger.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The transition relation containing the old rows affected by the triggering statement

<a id="op-9b2b416fd833680a72f9101c"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerReferencingType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerReferencingType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78bf6fc45aa074aed2a347f7"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerReferencingType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerReferencingType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 57], "end": [41, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea903cb0b3bebe06bd3c5aca"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerReferencingType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 49], "end": [42, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afe014e83354ec317d4de3f5"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerReferencingType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerReferencingType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 30], "end": [41, 39], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a825ba5bbe2d3446b4a891f"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerReferencingType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d975d861d8cf98d44d7bf5"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerReferencingType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [59, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-579b114c5898f8d6be046c1c"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerReferencingType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 62], "end": [41, 66], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cbf92e7089b6d257689dc5e"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerReferencingType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerReferencingType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 41], "end": [41, 51], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0677896f4ae197763fc58047"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerReferencingType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 38], "end": [42, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c7aeef688946ad3d8b78eaa"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerReferencingType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 40], "end": [43, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df0560d979fb1609995d4aa2"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerReferencingType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencingType", "path": "TriggerReferencingType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 47], "end": [43, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
