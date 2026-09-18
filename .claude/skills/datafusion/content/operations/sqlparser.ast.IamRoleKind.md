# `sqlparser::ast::IamRoleKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.IamRoleKind.json).

<a id="op-ac2f12c795c1916fe90d9868"></a>
## IamRoleKind

`enum` · `sqlparser::ast::IamRoleKind` · sqlparser 0.62.0

```rust
enum IamRoleKind
```

Source: `src/ast/mod.rs:9653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `IAM_ROLE` option in the AWS ecosystem

[Redshift COPY](https://docs.aws.amazon.com/redshift/latest/dg/copy-parameters-authorization.html#copy-iam-role)

<a id="op-24a9877bb3a0c0ec882070f2"></a>
## Arn

`variant` · `sqlparser::ast::IamRoleKind::Arn` · sqlparser 0.62.0

```rust
Arn
```

Source: `src/ast/mod.rs:9657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specific role ARN, for example: `arn:aws:iam::123456789:role/role1`

<a id="op-d76d426fe20b6c5004804f51"></a>
## Default

`variant` · `sqlparser::ast::IamRoleKind::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/mod.rs:9655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default role

<a id="op-8e9a565acb89b0b5ff2cd890"></a>
## clone

`function` · `sqlparser::ast::IamRoleKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IamRoleKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9650, 17], "end": [9650, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c8a3b53ddd303e8fcd00e18"></a>
## cmp

`function` · `sqlparser::ast::IamRoleKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IamRoleKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9650, 51], "end": [9650, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde81011c84d67f0efe5263a"></a>
## deserialize

`function` · `sqlparser::ast::IamRoleKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9651, 49], "end": [9651, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4072e0570a8be10643c95df5"></a>
## eq

`function` · `sqlparser::ast::IamRoleKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IamRoleKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9650, 24], "end": [9650, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-467803297a1141a70ba9a630"></a>
## fmt

`function` · `sqlparser::ast::IamRoleKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9650, 10], "end": [9650, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7a5c1952d4d2881466f9ced"></a>
## fmt

`function` · `sqlparser::ast::IamRoleKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9660, 1], "end": [9667, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0428143f96c0f8baa30987cd"></a>
## hash

`function` · `sqlparser::ast::IamRoleKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9650, 56], "end": [9650, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65545a8e4dccf2f83a7a3272"></a>
## partial_cmp

`function` · `sqlparser::ast::IamRoleKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IamRoleKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9650, 35], "end": [9650, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecbdc897a0aec2fb148c0174"></a>
## serialize

`function` · `sqlparser::ast::IamRoleKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9651, 38], "end": [9651, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-339c1f304802f841fd498aae"></a>
## visit

`function` · `sqlparser::ast::IamRoleKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9652, 47], "end": [9652, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6abe871903fa6681e9987ca1"></a>
## visit

`function` · `sqlparser::ast::IamRoleKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::IamRoleKind", "path": "IamRoleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9652, 40], "end": [9652, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
