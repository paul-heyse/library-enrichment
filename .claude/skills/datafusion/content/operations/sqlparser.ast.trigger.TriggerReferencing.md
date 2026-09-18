# `sqlparser::ast::trigger::TriggerReferencing`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.trigger.TriggerReferencing.json).

<a id="op-ef3e90680d7fd31edf2f23a8"></a>
## TriggerReferencing

`struct` · `sqlparser::ast::trigger::TriggerReferencing` · sqlparser 0.62.0

```rust
struct TriggerReferencing
```

Source: `src/ast/trigger.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This keyword immediately precedes the declaration of one or two relation names that provide access to the transition relations of the triggering statement

<a id="op-63443c9a7c0dffa1b7f7b0c0"></a>
## clone

`function` · `sqlparser::ast::trigger::TriggerReferencing::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerReferencing
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 22], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/trigger.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88844d38702616f1bf114b01"></a>
## cmp

`function` · `sqlparser::ast::trigger::TriggerReferencing::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerReferencing) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 51], "end": [62, 54], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/trigger.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9559a4ffef7f4701ad8db555"></a>
## deserialize

`function` · `sqlparser::ast::trigger::TriggerReferencing::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 49], "end": [63, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/trigger.rs:63`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c08263a9e12ee5fcae03167"></a>
## eq

`function` · `sqlparser::ast::trigger::TriggerReferencing::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerReferencing) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 24], "end": [62, 33], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/trigger.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f6a02f07a79ae26f49848f"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerReferencing::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [84, 2], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/trigger.rs:75`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f559469d42345f1774edd557"></a>
## fmt

`function` · `sqlparser::ast::trigger::TriggerReferencing::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/trigger.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1b0fb4f2b06aaf850ad79e2"></a>
## hash

`function` · `sqlparser::ast::trigger::TriggerReferencing::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 56], "end": [62, 60], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/trigger.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc3d3977d0312a0dfa733f6"></a>
## is_as

`struct_field` · `sqlparser::ast::trigger::TriggerReferencing::is_as` · sqlparser 0.62.0

```rust
is_as: bool
```

Source: `src/ast/trigger.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if the `AS` keyword is present in the referencing clause.

<a id="op-4c8ff7a396aa7bb04203f2d4"></a>
## partial_cmp

`function` · `sqlparser::ast::trigger::TriggerReferencing::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerReferencing) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 35], "end": [62, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/trigger.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5349de63bd04c5fd4e458945"></a>
## refer_type

`struct_field` · `sqlparser::ast::trigger::TriggerReferencing::refer_type` · sqlparser 0.62.0

```rust
refer_type: TriggerReferencingType
```

Source: `src/ast/trigger.rs:67`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The referencing type (`OLD TABLE` or `NEW TABLE`).

<a id="op-0412babb045810a1e719a975"></a>
## serialize

`function` · `sqlparser::ast::trigger::TriggerReferencing::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 38], "end": [63, 47], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/trigger.rs:63`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7081822e3e305a6e64a19b80"></a>
## transition_relation_name

`struct_field` · `sqlparser::ast::trigger::TriggerReferencing::transition_relation_name` · sqlparser 0.62.0

```rust
transition_relation_name: ObjectName
```

Source: `src/ast/trigger.rs:71`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The transition relation name provided by the referencing clause.

<a id="op-4ff0a97ad712ea702e511089"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerReferencing::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 47], "end": [64, 55], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/trigger.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7a132fb47108105ac047bf6"></a>
## visit

`function` · `sqlparser::ast::trigger::TriggerReferencing::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::trigger::TriggerReferencing", "path": "TriggerReferencing"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 40], "end": [64, 45], "filename": "src/ast/trigger.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/trigger.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
