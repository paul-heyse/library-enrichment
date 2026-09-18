# `sqlparser::ast::Password`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Password.json).

<a id="op-ad6ee563ee89bd247c25c472"></a>
## Password

`enum` · `sqlparser::ast::Password` · sqlparser 0.62.0

```rust
enum Password
```

Source: `src/ast/mod.rs:2553`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Password specification variants used in user-related statements.

<a id="op-b54e99624cdfc956c8e108b3"></a>
## NullPassword

`variant` · `sqlparser::ast::Password::NullPassword` · sqlparser 0.62.0

```rust
NullPassword
```

Source: `src/ast/mod.rs:2557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a `NULL` password.

<a id="op-dc3688f9cbbeabe903594010"></a>
## Password

`variant` · `sqlparser::ast::Password::Password` · sqlparser 0.62.0

```rust
Password
```

Source: `src/ast/mod.rs:2555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A concrete password expression.

<a id="op-3bf130ce6c3b17d66d98633f"></a>
## clone

`function` · `sqlparser::ast::Password::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Password
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2549, 17], "end": [2549, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6be603709e155911b9865e6b"></a>
## cmp

`function` · `sqlparser::ast::Password::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Password) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2549, 51], "end": [2549, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba7d230e2ab6530127e65fc2"></a>
## deserialize

`function` · `sqlparser::ast::Password::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2550, 49], "end": [2550, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cdcfd1d8b234aec9acec9bf"></a>
## eq

`function` · `sqlparser::ast::Password::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Password) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2549, 24], "end": [2549, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1b1c1832e3b21f15e199de6"></a>
## fmt

`function` · `sqlparser::ast::Password::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2549, 10], "end": [2549, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be37259e28d544c63217c60c"></a>
## hash

`function` · `sqlparser::ast::Password::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2549, 56], "end": [2549, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-145c126b20116c0972b4a38e"></a>
## partial_cmp

`function` · `sqlparser::ast::Password::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Password) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2549, 35], "end": [2549, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c997a5d692b848eb93431244"></a>
## serialize

`function` · `sqlparser::ast::Password::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2550, 38], "end": [2550, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78adcef916a68600f314f92d"></a>
## visit

`function` · `sqlparser::ast::Password::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2551, 47], "end": [2551, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d05b521ac56fd093ffdce340"></a>
## visit

`function` · `sqlparser::ast::Password::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Password", "path": "Password"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2551, 40], "end": [2551, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
