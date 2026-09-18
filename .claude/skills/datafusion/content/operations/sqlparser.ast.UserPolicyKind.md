# `sqlparser::ast::UserPolicyKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.UserPolicyKind.json).

<a id="op-17b833e6f2a93eb39063d5e0"></a>
## UserPolicyKind

`enum` · `sqlparser::ast::UserPolicyKind` · sqlparser 0.62.0

```rust
enum UserPolicyKind
```

Source: `src/ast/mod.rs:11657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Types of user-based policies

<a id="op-a85414d006fda3f09516908b"></a>
## Authentication

`variant` · `sqlparser::ast::UserPolicyKind::Authentication` · sqlparser 0.62.0

```rust
Authentication
```

Source: `src/ast/mod.rs:11659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Authentication policy.

<a id="op-ce05b1d32dd52b57896754af"></a>
## Password

`variant` · `sqlparser::ast::UserPolicyKind::Password` · sqlparser 0.62.0

```rust
Password
```

Source: `src/ast/mod.rs:11661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Password policy.

<a id="op-78e825f2960ee58d857780fd"></a>
## Session

`variant` · `sqlparser::ast::UserPolicyKind::Session` · sqlparser 0.62.0

```rust
Session
```

Source: `src/ast/mod.rs:11663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Session policy.

<a id="op-bd2addf97b13acac2df470b1"></a>
## clone

`function` · `sqlparser::ast::UserPolicyKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserPolicyKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11654, 17], "end": [11654, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cb6cb34c1197c11aaab448b"></a>
## cmp

`function` · `sqlparser::ast::UserPolicyKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserPolicyKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11654, 51], "end": [11654, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04e08f01b2239528cc95e423"></a>
## deserialize

`function` · `sqlparser::ast::UserPolicyKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11655, 49], "end": [11655, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81417f60df736ee578245efc"></a>
## eq

`function` · `sqlparser::ast::UserPolicyKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserPolicyKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11654, 24], "end": [11654, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27497e3382b268be87a8e7a6"></a>
## fmt

`function` · `sqlparser::ast::UserPolicyKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11666, 1], "end": [11674, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9b833039f1ff61b3f7066fe"></a>
## fmt

`function` · `sqlparser::ast::UserPolicyKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11654, 10], "end": [11654, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66400298aef7456d8c69a5f1"></a>
## hash

`function` · `sqlparser::ast::UserPolicyKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11654, 56], "end": [11654, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db6fd32613f8f3175aa9bda4"></a>
## partial_cmp

`function` · `sqlparser::ast::UserPolicyKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserPolicyKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11654, 35], "end": [11654, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bdd5ab687a710a1a5501de6"></a>
## serialize

`function` · `sqlparser::ast::UserPolicyKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11655, 38], "end": [11655, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-185fced91dd929428a08d399"></a>
## visit

`function` · `sqlparser::ast::UserPolicyKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11656, 47], "end": [11656, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c8702d0b3c0e4a8de3933e8"></a>
## visit

`function` · `sqlparser::ast::UserPolicyKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UserPolicyKind", "path": "UserPolicyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11656, 40], "end": [11656, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11656`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
