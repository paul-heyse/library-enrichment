# `sqlparser::ast::ShowCreateObject`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowCreateObject.json).

<a id="op-a5fd7b903272f5f54b804fa8"></a>
## ShowCreateObject

`enum` · `sqlparser::ast::ShowCreateObject` · sqlparser 0.62.0

```rust
enum ShowCreateObject
```

Source: `src/ast/mod.rs:2459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Object kinds supported by `SHOW CREATE` statements.

<a id="op-03bb5321bbbf60cced576a61"></a>
## Event

`variant` · `sqlparser::ast::ShowCreateObject::Event` · sqlparser 0.62.0

```rust
Event
```

Source: `src/ast/mod.rs:2461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An event object for `SHOW CREATE EVENT`.

<a id="op-c24eb3774ba789be0d574cc0"></a>
## Function

`variant` · `sqlparser::ast::ShowCreateObject::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/mod.rs:2463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A function object for `SHOW CREATE FUNCTION`.

<a id="op-7a1c82e2895c6b3fc0342299"></a>
## Procedure

`variant` · `sqlparser::ast::ShowCreateObject::Procedure` · sqlparser 0.62.0

```rust
Procedure
```

Source: `src/ast/mod.rs:2465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A procedure object for `SHOW CREATE PROCEDURE`.

<a id="op-fc83058f2cf8694bf7cd62ea"></a>
## Table

`variant` · `sqlparser::ast::ShowCreateObject::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:2467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table object for `SHOW CREATE TABLE`.

<a id="op-5c237d21aded2d7f5731f598"></a>
## Trigger

`variant` · `sqlparser::ast::ShowCreateObject::Trigger` · sqlparser 0.62.0

```rust
Trigger
```

Source: `src/ast/mod.rs:2469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A trigger object for `SHOW CREATE TRIGGER`.

<a id="op-d37df9e331fd9f8fa2a48358"></a>
## View

`variant` · `sqlparser::ast::ShowCreateObject::View` · sqlparser 0.62.0

```rust
View
```

Source: `src/ast/mod.rs:2471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A view object for `SHOW CREATE VIEW`.

<a id="op-dbd0267add8310ae67be61d0"></a>
## clone

`function` · `sqlparser::ast::ShowCreateObject::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowCreateObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2455, 23], "end": [2455, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8aa80b596b2ab9b32ee8a66"></a>
## cmp

`function` · `sqlparser::ast::ShowCreateObject::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowCreateObject) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2455, 57], "end": [2455, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1379309ffe0054fb4b3f73fb"></a>
## deserialize

`function` · `sqlparser::ast::ShowCreateObject::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2456, 49], "end": [2456, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef6dafaf056ae795aef71496"></a>
## eq

`function` · `sqlparser::ast::ShowCreateObject::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowCreateObject) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2455, 30], "end": [2455, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26dba35892fca079c6479971"></a>
## fmt

`function` · `sqlparser::ast::ShowCreateObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2455, 10], "end": [2455, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c70e59704a20cb6c4088ee1"></a>
## fmt

`function` · `sqlparser::ast::ShowCreateObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2474, 1], "end": [2485, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d6251c44235961e0f464b38"></a>
## hash

`function` · `sqlparser::ast::ShowCreateObject::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2455, 62], "end": [2455, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b68999855deec0cd6fd28a0"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowCreateObject::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowCreateObject) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2455, 41], "end": [2455, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae7b2bef16980e6c08e83e0a"></a>
## serialize

`function` · `sqlparser::ast::ShowCreateObject::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2456, 38], "end": [2456, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ed3a731544edb1605b5364a"></a>
## visit

`function` · `sqlparser::ast::ShowCreateObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2457, 40], "end": [2457, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dd846b058dc8317d743fe60"></a>
## visit

`function` · `sqlparser::ast::ShowCreateObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowCreateObject", "path": "ShowCreateObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2457, 47], "end": [2457, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
