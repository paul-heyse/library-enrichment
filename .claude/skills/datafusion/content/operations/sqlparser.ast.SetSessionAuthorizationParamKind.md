# `sqlparser::ast::SetSessionAuthorizationParamKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionAuthorizationParamKind.json).

<a id="op-995fbf36bf80cc2365889b1c"></a>
## SetSessionAuthorizationParamKind

`enum` · `sqlparser::ast::SetSessionAuthorizationParamKind` · sqlparser 0.62.0

```rust
enum SetSessionAuthorizationParamKind
```

Source: `src/ast/mod.rs:11058`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the parameter kind for SET SESSION AUTHORIZATION

<a id="op-ab67d562ea199f27f1bebed2"></a>
## Default

`variant` · `sqlparser::ast::SetSessionAuthorizationParamKind::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/mod.rs:11060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default authorization

<a id="op-959a6e2ffaf5e345487cbdae"></a>
## User

`variant` · `sqlparser::ast::SetSessionAuthorizationParamKind::User` · sqlparser 0.62.0

```rust
User
```

Source: `src/ast/mod.rs:11063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

User name

<a id="op-d7fccbfb19629f7a9c2cef6a"></a>
## clone

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionAuthorizationParamKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11055, 17], "end": [11055, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77cb862da5da6d0cf08f2ec7"></a>
## cmp

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionAuthorizationParamKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11055, 51], "end": [11055, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95744fdbc99b1567fe39c5d4"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11056, 49], "end": [11056, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11056`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78e8abaf48b3a23062246871"></a>
## eq

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionAuthorizationParamKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11055, 24], "end": [11055, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fd8fc15806590dbd122cd89"></a>
## fmt

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11066, 1], "end": [11073, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b09761ad4f73bc31ea4a8079"></a>
## fmt

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11055, 10], "end": [11055, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67bb3ee7d77c532043f83e6b"></a>
## hash

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11055, 56], "end": [11055, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-286aea2bec415d5e7f0ef656"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionAuthorizationParamKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11055, 35], "end": [11055, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b88aafd5f212ecfea82ce468"></a>
## serialize

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11056, 38], "end": [11056, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11056`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e4ede189ed0be65f211fff1"></a>
## visit

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11057, 40], "end": [11057, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d624d6b37ac5d303a4a3f19c"></a>
## visit

`function` · `sqlparser::ast::SetSessionAuthorizationParamKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionAuthorizationParamKind", "path": "SetSessionAuthorizationParamKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11057, 47], "end": [11057, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
