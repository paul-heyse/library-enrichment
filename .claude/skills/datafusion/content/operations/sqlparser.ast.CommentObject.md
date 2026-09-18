# `sqlparser::ast::CommentObject`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CommentObject.json).

<a id="op-cc8f01662036158910f2802a"></a>
## CommentObject

`enum` · `sqlparser::ast::CommentObject` · sqlparser 0.62.0

```rust
enum CommentObject
```

Source: `src/ast/mod.rs:2491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Objects that can be targeted by a `COMMENT` statement.

<a id="op-ad0f4aa6aa4d1cb7bc1aacac"></a>
## Collation

`variant` · `sqlparser::ast::CommentObject::Collation` · sqlparser 0.62.0

```rust
Collation
```

Source: `src/ast/mod.rs:2493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A collation.

<a id="op-379064473d35d99315ae1630"></a>
## Column

`variant` · `sqlparser::ast::CommentObject::Column` · sqlparser 0.62.0

```rust
Column
```

Source: `src/ast/mod.rs:2495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table column.

<a id="op-d090015ac2dce76e8c774b3b"></a>
## Database

`variant` · `sqlparser::ast::CommentObject::Database` · sqlparser 0.62.0

```rust
Database
```

Source: `src/ast/mod.rs:2497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A database.

<a id="op-69e088042c5ead7b049db41b"></a>
## Domain

`variant` · `sqlparser::ast::CommentObject::Domain` · sqlparser 0.62.0

```rust
Domain
```

Source: `src/ast/mod.rs:2499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A domain.

<a id="op-ace3101950f6047e79563fd9"></a>
## Extension

`variant` · `sqlparser::ast::CommentObject::Extension` · sqlparser 0.62.0

```rust
Extension
```

Source: `src/ast/mod.rs:2501`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An extension.

<a id="op-71ff8ff543cb1bdf3152ac80"></a>
## Function

`variant` · `sqlparser::ast::CommentObject::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/mod.rs:2503`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A function.

<a id="op-3096d84db8cc41f795ce1def"></a>
## Index

`variant` · `sqlparser::ast::CommentObject::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/mod.rs:2505`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An index.

<a id="op-376ec62541193df9a86ed0cd"></a>
## MaterializedView

`variant` · `sqlparser::ast::CommentObject::MaterializedView` · sqlparser 0.62.0

```rust
MaterializedView
```

Source: `src/ast/mod.rs:2507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A materialized view.

<a id="op-86f7fcd426ca31b30d5e4dd7"></a>
## Procedure

`variant` · `sqlparser::ast::CommentObject::Procedure` · sqlparser 0.62.0

```rust
Procedure
```

Source: `src/ast/mod.rs:2509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A procedure.

<a id="op-1e9e00acdb57e05122375930"></a>
## Role

`variant` · `sqlparser::ast::CommentObject::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/mod.rs:2511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A role.

<a id="op-4bb5021d63d9ddf58625e61c"></a>
## Schema

`variant` · `sqlparser::ast::CommentObject::Schema` · sqlparser 0.62.0

```rust
Schema
```

Source: `src/ast/mod.rs:2513`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A schema.

<a id="op-a4a6def61fffade6741b54fd"></a>
## Sequence

`variant` · `sqlparser::ast::CommentObject::Sequence` · sqlparser 0.62.0

```rust
Sequence
```

Source: `src/ast/mod.rs:2515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A sequence.

<a id="op-2c8c2d71cfa87da94487bbca"></a>
## Table

`variant` · `sqlparser::ast::CommentObject::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:2517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table.

<a id="op-8d2b4fa4bb416ec22f2c6523"></a>
## Type

`variant` · `sqlparser::ast::CommentObject::Type` · sqlparser 0.62.0

```rust
Type
```

Source: `src/ast/mod.rs:2519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A type.

<a id="op-aa6133b389c871fcacb83227"></a>
## User

`variant` · `sqlparser::ast::CommentObject::User` · sqlparser 0.62.0

```rust
User
```

Source: `src/ast/mod.rs:2521`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A user.

<a id="op-2a745473d0865c3bc08f50bf"></a>
## View

`variant` · `sqlparser::ast::CommentObject::View` · sqlparser 0.62.0

```rust
View
```

Source: `src/ast/mod.rs:2523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A view.

<a id="op-ec74228f76e773088d6c3afb"></a>
## clone

`function` · `sqlparser::ast::CommentObject::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CommentObject
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2487, 23], "end": [2487, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4023fec57b6d8251297d8a5"></a>
## cmp

`function` · `sqlparser::ast::CommentObject::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CommentObject) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2487, 57], "end": [2487, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f8970b939beb14efbdeb9e5"></a>
## deserialize

`function` · `sqlparser::ast::CommentObject::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2488, 49], "end": [2488, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4105b0f000342a72ad857dd4"></a>
## eq

`function` · `sqlparser::ast::CommentObject::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CommentObject) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2487, 30], "end": [2487, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4eb91915323d4547f3cb91d0"></a>
## fmt

`function` · `sqlparser::ast::CommentObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2526, 1], "end": [2547, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2527`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a56389ec07e44afc6c6f4537"></a>
## fmt

`function` · `sqlparser::ast::CommentObject::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2487, 10], "end": [2487, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-557542532c1e8553e250510c"></a>
## hash

`function` · `sqlparser::ast::CommentObject::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2487, 62], "end": [2487, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5793511f9046b1bcf5a82d0c"></a>
## partial_cmp

`function` · `sqlparser::ast::CommentObject::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CommentObject) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2487, 41], "end": [2487, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95c482885a0c1e9f0981bf44"></a>
## serialize

`function` · `sqlparser::ast::CommentObject::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2488, 38], "end": [2488, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ae797d071524b452dc6abb"></a>
## visit

`function` · `sqlparser::ast::CommentObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2489, 40], "end": [2489, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47920c1db17dea6bbe48aff3"></a>
## visit

`function` · `sqlparser::ast::CommentObject::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CommentObject", "path": "CommentObject"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2489, 47], "end": [2489, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
