# `sqlparser::ast::trigger::TriggerExecBodyType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerExecBodyType.json).

<a id="op-600874278df5dfd4d5a4895c"></a>
## TriggerExecBodyType

`enum` · `sqlparser::ast::trigger::TriggerExecBodyType` · sqlparser 0.62.0

```rust
enum TriggerExecBodyType
```

Source: `src/ast/trigger.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Types of trigger body execution body.

<a id="op-523895593e5c2908ac793dc0"></a>
## Function

`variant` · `sqlparser::ast::trigger::TriggerExecBodyType::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/trigger.rs:151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Execute a function

<a id="op-02b2f4094d674b507b635084"></a>
## Procedure

`variant` · `sqlparser::ast::trigger::TriggerExecBodyType::Procedure` · sqlparser 0.62.0

```rust
Procedure
```

Source: `src/ast/trigger.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Execute a procedure

<a id="op-410e94b4d3bce7e3fc1eddff"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerExecBodyType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 17], "end": [146, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe8b9e5f157789bf1fa7557"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerExecBodyType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 57], "end": [146, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65aad22fd055844cb96bf34c"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 49], "end": [147, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-495e51b2b04729abd0a941ee"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerExecBodyType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 30], "end": [146, 39], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bea81876d9367f89fa15f52b"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 10], "end": [146, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d0df03231df52b2d1a54b9"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [163, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad7c83db6553f8bb8b1d01dd"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 62], "end": [146, 66], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9eb238741853065a91a8252"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerExecBodyType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 41], "end": [146, 51], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45d79c649566fe9dd6fc0693"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 38], "end": [147, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bd2c71e1c790f8a846880f4"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 40], "end": [148, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ecdbedf2cc2aa50751bbc00"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerExecBodyType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerExecBodyType", "path": "TriggerExecBodyType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 47], "end": [148, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
