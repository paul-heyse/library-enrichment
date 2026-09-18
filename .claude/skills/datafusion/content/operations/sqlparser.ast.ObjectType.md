# `sqlparser::ast::ObjectType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ObjectType.json).

<a id="op-82a442ac74cda157517ac3f1"></a>
## ObjectType

`enum` · `sqlparser::ast::ObjectType` · sqlparser 0.62.0

```rust
enum ObjectType
```

Source: `src/ast/mod.rs:8461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Types of database objects referenced by DDL statements.

<a id="op-59f4c1e440fb8fa19d2134a5"></a>
## Collation

`variant` · `sqlparser::ast::ObjectType::Collation` · sqlparser 0.62.0

```rust
Collation
```

Source: `src/ast/mod.rs:8463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A collation.

<a id="op-4c72823c2420820066e07acd"></a>
## Database

`variant` · `sqlparser::ast::ObjectType::Database` · sqlparser 0.62.0

```rust
Database
```

Source: `src/ast/mod.rs:8475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A database.

<a id="op-7bcdf34a2d32e41a96c402e8"></a>
## Index

`variant` · `sqlparser::ast::ObjectType::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/mod.rs:8471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An index.

<a id="op-f5c4d496cd7a6c4959b628bd"></a>
## MaterializedView

`variant` · `sqlparser::ast::ObjectType::MaterializedView` · sqlparser 0.62.0

```rust
MaterializedView
```

Source: `src/ast/mod.rs:8469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A materialized view.

<a id="op-ae0dd23dc6cb71a7ac345f1a"></a>
## Role

`variant` · `sqlparser::ast::ObjectType::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/mod.rs:8477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A role.

<a id="op-a809281a7272f855ab42ef34"></a>
## Schema

`variant` · `sqlparser::ast::ObjectType::Schema` · sqlparser 0.62.0

```rust
Schema
```

Source: `src/ast/mod.rs:8473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A schema.

<a id="op-1283fdab924ee417b1869af7"></a>
## Sequence

`variant` · `sqlparser::ast::ObjectType::Sequence` · sqlparser 0.62.0

```rust
Sequence
```

Source: `src/ast/mod.rs:8479`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A sequence.

<a id="op-db6f08df97a779f7fdf39db0"></a>
## Stage

`variant` · `sqlparser::ast::ObjectType::Stage` · sqlparser 0.62.0

```rust
Stage
```

Source: `src/ast/mod.rs:8481`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A stage.

<a id="op-69cef2ed7866fc71a03a3e43"></a>
## Stream

`variant` · `sqlparser::ast::ObjectType::Stream` · sqlparser 0.62.0

```rust
Stream
```

Source: `src/ast/mod.rs:8487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A stream.

<a id="op-0bc652466beb6585e8d68b0a"></a>
## Table

`variant` · `sqlparser::ast::ObjectType::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:8465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table.

<a id="op-72130be20a030e7e4b3d24f8"></a>
## Type

`variant` · `sqlparser::ast::ObjectType::Type` · sqlparser 0.62.0

```rust
Type
```

Source: `src/ast/mod.rs:8483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A type definition.

<a id="op-e5a80a57be5eb275c9f36c11"></a>
## User

`variant` · `sqlparser::ast::ObjectType::User` · sqlparser 0.62.0

```rust
User
```

Source: `src/ast/mod.rs:8485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A user.

<a id="op-3f1284304d5a07cb208b1126"></a>
## View

`variant` · `sqlparser::ast::ObjectType::View` · sqlparser 0.62.0

```rust
View
```

Source: `src/ast/mod.rs:8467`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A view.

<a id="op-f8a2eaa4be8df480155fa351"></a>
## clone

`function` · `sqlparser::ast::ObjectType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ObjectType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8457, 23], "end": [8457, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad1275d1cbbb082f660b2e97"></a>
## cmp

`function` · `sqlparser::ast::ObjectType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ObjectType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8457, 57], "end": [8457, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87c637d37648455780be117c"></a>
## deserialize

`function` · `sqlparser::ast::ObjectType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8458, 49], "end": [8458, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8458`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b656bf2c4354e8530d600a79"></a>
## eq

`function` · `sqlparser::ast::ObjectType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ObjectType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8457, 30], "end": [8457, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d2b6e87b2e7e723d24034ea"></a>
## fmt

`function` · `sqlparser::ast::ObjectType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8490, 1], "end": [8508, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c66b037df3b6cd8aef453e77"></a>
## fmt

`function` · `sqlparser::ast::ObjectType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8457, 10], "end": [8457, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8931bd3eda1ff1ca274c674"></a>
## hash

`function` · `sqlparser::ast::ObjectType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8457, 62], "end": [8457, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2a4c998169860ed68b18105"></a>
## partial_cmp

`function` · `sqlparser::ast::ObjectType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ObjectType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8457, 41], "end": [8457, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8457`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c7c833ae19fd52bceb760dd"></a>
## serialize

`function` · `sqlparser::ast::ObjectType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8458, 38], "end": [8458, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8458`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d2b56cdff09a8c4128a031a"></a>
## visit

`function` · `sqlparser::ast::ObjectType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8459, 40], "end": [8459, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94b93f449784c83305957479"></a>
## visit

`function` · `sqlparser::ast::ObjectType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ObjectType", "path": "ObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8459, 47], "end": [8459, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
