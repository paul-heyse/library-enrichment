# `sqlparser::ast::MfaMethodKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MfaMethodKind.json).

<a id="op-19f51b7ef2bacc5d3114ea3d"></a>
## MfaMethodKind

`enum` · `sqlparser::ast::MfaMethodKind` · sqlparser 0.62.0

```rust
enum MfaMethodKind
```

Source: `src/ast/mod.rs:11621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Types of MFA methods

<a id="op-6a5b65fa2aa415e8e2ea0f07"></a>
## Duo

`variant` · `sqlparser::ast::MfaMethodKind::Duo` · sqlparser 0.62.0

```rust
Duo
```

Source: `src/ast/mod.rs:11627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Duo Security MFA method.

<a id="op-88a2b2d191b0989ff7535673"></a>
## PassKey

`variant` · `sqlparser::ast::MfaMethodKind::PassKey` · sqlparser 0.62.0

```rust
PassKey
```

Source: `src/ast/mod.rs:11623`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PassKey (hardware or platform passkey) MFA method.

<a id="op-5d28e51aa90b3c6f3e842cb7"></a>
## Totp

`variant` · `sqlparser::ast::MfaMethodKind::Totp` · sqlparser 0.62.0

```rust
Totp
```

Source: `src/ast/mod.rs:11625`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Time-based One-Time Password (TOTP) MFA method.

<a id="op-83cb4b187757b1074a43d4a1"></a>
## clone

`function` · `sqlparser::ast::MfaMethodKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MfaMethodKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11618, 17], "end": [11618, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7f17b0e796f9f6544a1b8bd"></a>
## cmp

`function` · `sqlparser::ast::MfaMethodKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MfaMethodKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11618, 51], "end": [11618, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb92843f87b7a8d4bd1c3dde"></a>
## deserialize

`function` · `sqlparser::ast::MfaMethodKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11619, 49], "end": [11619, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11619`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffe3faea9f7852479277a270"></a>
## eq

`function` · `sqlparser::ast::MfaMethodKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MfaMethodKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11618, 24], "end": [11618, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9df2b1f88fcb706e8b020d7"></a>
## fmt

`function` · `sqlparser::ast::MfaMethodKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11618, 10], "end": [11618, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4c7be4e6898dd507dadb417"></a>
## fmt

`function` · `sqlparser::ast::MfaMethodKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11630, 1], "end": [11638, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11631`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8435b01216e9e5ef0e41cd8b"></a>
## hash

`function` · `sqlparser::ast::MfaMethodKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11618, 56], "end": [11618, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1bfa5a3ed3930e79b641e48"></a>
## partial_cmp

`function` · `sqlparser::ast::MfaMethodKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MfaMethodKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11618, 35], "end": [11618, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11618`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6309eb04d8ad7d8f0d7db25d"></a>
## serialize

`function` · `sqlparser::ast::MfaMethodKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11619, 38], "end": [11619, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11619`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97b1ffcf63f66821447d2328"></a>
## visit

`function` · `sqlparser::ast::MfaMethodKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11620, 47], "end": [11620, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aae89dee869e3a9a6dd05b59"></a>
## visit

`function` · `sqlparser::ast::MfaMethodKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MfaMethodKind", "path": "MfaMethodKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11620, 40], "end": [11620, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
