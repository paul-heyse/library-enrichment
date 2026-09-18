# `sqlparser::ast::query::LockType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.LockType.json).

<a id="op-01611ef42473c95dabfeea71"></a>
## LockType

`enum` · `sqlparser::ast::query::LockType` · sqlparser 0.62.0

```rust
enum LockType
```

Source: `src/ast/query.rs:3538`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The lock type used in `FOR <lock>` clauses (e.g. `FOR SHARE`, `FOR UPDATE`).

<a id="op-806ac414c33ed3dcbcc33806"></a>
## Share

`variant` · `sqlparser::ast::query::LockType::Share` · sqlparser 0.62.0

```rust
Share
```

Source: `src/ast/query.rs:3540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHARE` lock (shared lock).

<a id="op-1efe59240f9d9351705f87c5"></a>
## Update

`variant` · `sqlparser::ast::query::LockType::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/query.rs:3542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UPDATE` lock (exclusive/update lock).

<a id="op-3401d1b36ff5e080dfc25bef"></a>
## clone

`function` · `sqlparser::ast::query::LockType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LockType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 23], "end": [3534, 28], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc1a63accd53cf4f257fcb74"></a>
## cmp

`function` · `sqlparser::ast::query::LockType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LockType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 57], "end": [3534, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ef2fbe2ebb4762abeab2295"></a>
## deserialize

`function` · `sqlparser::ast::query::LockType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3535, 49], "end": [3535, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4a0d8782e1d3c52f64bcd12"></a>
## eq

`function` · `sqlparser::ast::query::LockType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LockType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 30], "end": [3534, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a4935ee3a548edff2335aa4"></a>
## fmt

`function` · `sqlparser::ast::query::LockType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3545, 1], "end": [3553, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d73f95a73b365dcc7e5f54d"></a>
## fmt

`function` · `sqlparser::ast::query::LockType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 10], "end": [3534, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b19e507f289beaa95882620b"></a>
## hash

`function` · `sqlparser::ast::query::LockType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 62], "end": [3534, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-326ebe405c4f35b7a7bf5905"></a>
## partial_cmp

`function` · `sqlparser::ast::query::LockType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LockType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3534, 41], "end": [3534, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3534`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76f5fada543a39898c9dae04"></a>
## serialize

`function` · `sqlparser::ast::query::LockType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3535, 38], "end": [3535, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce8a8d9f584bb36da9c628ad"></a>
## visit

`function` · `sqlparser::ast::query::LockType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3536, 47], "end": [3536, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3536`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1472cf94fd296e5aff0d054"></a>
## visit

`function` · `sqlparser::ast::query::LockType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockType", "path": "LockType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3536, 40], "end": [3536, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3536`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
