# `sqlparser::ast::DiscardObject`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DiscardObject.json).

<a id="op-2203640c7ce668f38b98e725"></a>
## DiscardObject

`enum` · `sqlparser::ast::DiscardObject` · sqlparser 0.62.0

```rust
enum DiscardObject
```

Source: `src/ast/mod.rs:9707`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Objects that can be discarded with `DISCARD`.

<a id="op-7ba0b90d7d43349d201a4c44"></a>
## ALL

`variant` · `sqlparser::ast::DiscardObject::ALL` · sqlparser 0.62.0

```rust
ALL
```

Source: `src/ast/mod.rs:9709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Discard all session state.

<a id="op-b7db48498f27045be2ce944b"></a>
## PLANS

`variant` · `sqlparser::ast::DiscardObject::PLANS` · sqlparser 0.62.0

```rust
PLANS
```

Source: `src/ast/mod.rs:9711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Discard cached plans.

<a id="op-3ea4284661706b352ed31a97"></a>
## SEQUENCES

`variant` · `sqlparser::ast::DiscardObject::SEQUENCES` · sqlparser 0.62.0

```rust
SEQUENCES
```

Source: `src/ast/mod.rs:9713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Discard sequence values.

<a id="op-746873a0eeedf4cde1015d8d"></a>
## TEMP

`variant` · `sqlparser::ast::DiscardObject::TEMP` · sqlparser 0.62.0

```rust
TEMP
```

Source: `src/ast/mod.rs:9715`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Discard temporary objects.

<a id="op-3deeb475696f09ad06a0e0e8"></a>
## clone

`function` · `sqlparser::ast::DiscardObject::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DiscardObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9704, 23], "end": [9704, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3058b745ee4b1dd61ffcc496"></a>
## cmp

`function` · `sqlparser::ast::DiscardObject::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DiscardObject) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9704, 57], "end": [9704, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-655cb586b6016229949232b8"></a>
## deserialize

`function` · `sqlparser::ast::DiscardObject::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9705, 49], "end": [9705, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75bcd10752b60ec74856102d"></a>
## eq

`function` · `sqlparser::ast::DiscardObject::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DiscardObject) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9704, 30], "end": [9704, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b5971613822e23f9f8138c4"></a>
## fmt

`function` · `sqlparser::ast::DiscardObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9718, 1], "end": [9727, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9719`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c783c06e3ec51057918b48e"></a>
## fmt

`function` · `sqlparser::ast::DiscardObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9704, 10], "end": [9704, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d51dbca3c03a7645a8b796b"></a>
## hash

`function` · `sqlparser::ast::DiscardObject::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9704, 62], "end": [9704, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e753c7d84e1fb4febac506a9"></a>
## partial_cmp

`function` · `sqlparser::ast::DiscardObject::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DiscardObject) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9704, 41], "end": [9704, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9704`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc94546f5f509169f0cf1366"></a>
## serialize

`function` · `sqlparser::ast::DiscardObject::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9705, 38], "end": [9705, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d4eef4fbd7e855f3fadbd5b"></a>
## visit

`function` · `sqlparser::ast::DiscardObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9706, 47], "end": [9706, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9706`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1c5c3bf90dccf531acdccbb"></a>
## visit

`function` · `sqlparser::ast::DiscardObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DiscardObject", "path": "DiscardObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9706, 40], "end": [9706, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9706`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
