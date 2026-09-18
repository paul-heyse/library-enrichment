# `sqlparser::ast::GranteesType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.GranteesType.json).

<a id="op-81d789171c6565f98d65c1e8"></a>
## GranteesType

`enum` · `sqlparser::ast::GranteesType` · sqlparser 0.62.0

```rust
enum GranteesType
```

Source: `src/ast/mod.rs:7458`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of principal receiving privileges.

<a id="op-fb78aeae702abdfd4383c4d9"></a>
## Application

`variant` · `sqlparser::ast::GranteesType::Application` · sqlparser 0.62.0

```rust
Application
```

Source: `src/ast/mod.rs:7472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An application principal.

<a id="op-70d222942c5acae679d9064b"></a>
## ApplicationRole

`variant` · `sqlparser::ast::GranteesType::ApplicationRole` · sqlparser 0.62.0

```rust
ApplicationRole
```

Source: `src/ast/mod.rs:7474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An application role principal.

<a id="op-b56e5a6522782c1422df2a43"></a>
## DatabaseRole

`variant` · `sqlparser::ast::GranteesType::DatabaseRole` · sqlparser 0.62.0

```rust
DatabaseRole
```

Source: `src/ast/mod.rs:7470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A database role principal.

<a id="op-34fdee95b13763eb8280a9a2"></a>
## Group

`variant` · `sqlparser::ast::GranteesType::Group` · sqlparser 0.62.0

```rust
Group
```

Source: `src/ast/mod.rs:7466`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A group principal.

<a id="op-3c2eb5a0f6b4f43e4110b62f"></a>
## None

`variant` · `sqlparser::ast::GranteesType::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/mod.rs:7476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No specific principal (e.g. `NONE`).

<a id="op-437f67e116ce31663573d1cb"></a>
## Public

`variant` · `sqlparser::ast::GranteesType::Public` · sqlparser 0.62.0

```rust
Public
```

Source: `src/ast/mod.rs:7468`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The public principal.

<a id="op-08e9b455420f80c60b985b7e"></a>
## Role

`variant` · `sqlparser::ast::GranteesType::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/mod.rs:7460`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A role principal.

<a id="op-f75526e2fe4d052a7fff150e"></a>
## Share

`variant` · `sqlparser::ast::GranteesType::Share` · sqlparser 0.62.0

```rust
Share
```

Source: `src/ast/mod.rs:7462`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A share principal.

<a id="op-b70b1ed6d1b03d6514475114"></a>
## User

`variant` · `sqlparser::ast::GranteesType::User` · sqlparser 0.62.0

```rust
User
```

Source: `src/ast/mod.rs:7464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A user principal.

<a id="op-c283f4f276531a8de41a6748"></a>
## clone

`function` · `sqlparser::ast::GranteesType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GranteesType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7454, 17], "end": [7454, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a041d08837c6995b0e04107"></a>
## cmp

`function` · `sqlparser::ast::GranteesType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GranteesType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7454, 51], "end": [7454, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0810571deeff06e23b1e519c"></a>
## deserialize

`function` · `sqlparser::ast::GranteesType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7455, 49], "end": [7455, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa9a9dbccc0ddf97c0fcfe72"></a>
## eq

`function` · `sqlparser::ast::GranteesType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GranteesType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7454, 24], "end": [7454, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0142c2cb8490acee73a63f"></a>
## fmt

`function` · `sqlparser::ast::GranteesType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7454, 10], "end": [7454, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3daffb1c0921a8660ee654a7"></a>
## hash

`function` · `sqlparser::ast::GranteesType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7454, 56], "end": [7454, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6c0cae14fe7cc28755f1efa"></a>
## partial_cmp

`function` · `sqlparser::ast::GranteesType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GranteesType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7454, 35], "end": [7454, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7454`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b74562dcda9940ed1b863ab4"></a>
## serialize

`function` · `sqlparser::ast::GranteesType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7455, 38], "end": [7455, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a15ac8de12a3346fe584787"></a>
## visit

`function` · `sqlparser::ast::GranteesType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7456, 47], "end": [7456, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94055a60ec692935b5df8e79"></a>
## visit

`function` · `sqlparser::ast::GranteesType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GranteesType", "path": "GranteesType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7456, 40], "end": [7456, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7456`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
