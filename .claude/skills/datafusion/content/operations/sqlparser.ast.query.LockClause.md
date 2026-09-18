# `sqlparser::ast::query::LockClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.LockClause.json).

<a id="op-f69c638cd9ee3c343f8d97d9"></a>
## LockClause

`struct` · `sqlparser::ast::query::LockClause` · sqlparser 0.62.0

```rust
struct LockClause
```

Source: `src/ast/query.rs:3512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FOR ...` locking clause.

<a id="op-51edf7bf57d64f1eab3cf221"></a>
## clone

`function` · `sqlparser::ast::query::LockClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LockClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 17], "end": [3508, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20047c18a6d4757682aaf0c4"></a>
## cmp

`function` · `sqlparser::ast::query::LockClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LockClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 51], "end": [3508, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aae69f7da2da8f4060824c0c"></a>
## deserialize

`function` · `sqlparser::ast::query::LockClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3509, 49], "end": [3509, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c922d03c77f09ebf8c1f8c0e"></a>
## eq

`function` · `sqlparser::ast::query::LockClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LockClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 24], "end": [3508, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ccf1d4ea388547c44594b9b"></a>
## fmt

`function` · `sqlparser::ast::query::LockClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 10], "end": [3508, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-974c6adff505f18089d00fa9"></a>
## fmt

`function` · `sqlparser::ast::query::LockClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3521, 1], "end": [3532, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-163f5465605a83f5c4f024aa"></a>
## hash

`function` · `sqlparser::ast::query::LockClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 56], "end": [3508, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6d7f9b622277067961b21e3"></a>
## lock_type

`struct_field` · `sqlparser::ast::query::LockClause::lock_type` · sqlparser 0.62.0

```rust
lock_type: LockType
```

Source: `src/ast/query.rs:3514`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of lock requested (e.g. `SHARE`, `UPDATE`).

<a id="op-45fe26c483ada7e75a570a33"></a>
## nonblock

`struct_field` · `sqlparser::ast::query::LockClause::nonblock` · sqlparser 0.62.0

```rust
nonblock: Option<NonBlock>
```

Source: `src/ast/query.rs:3518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional non-blocking behavior (`NOWAIT` / `SKIP LOCKED`).

<a id="op-e623f442cd2694acddfbb548"></a>
## of

`struct_field` · `sqlparser::ast::query::LockClause::of` · sqlparser 0.62.0

```rust
of: Option<ObjectName>
```

Source: `src/ast/query.rs:3516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional object name after `OF` (e.g. `FOR UPDATE OF t1`).

<a id="op-662bf136b7fa7a0c754f0039"></a>
## partial_cmp

`function` · `sqlparser::ast::query::LockClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LockClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 35], "end": [3508, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cb30d2d0504831f69e11639"></a>
## serialize

`function` · `sqlparser::ast::query::LockClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3509, 38], "end": [3509, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1292fcf57cec39d0145a3c15"></a>
## visit

`function` · `sqlparser::ast::query::LockClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3510, 40], "end": [3510, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a98a025605904bb620b5086"></a>
## visit

`function` · `sqlparser::ast::query::LockClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LockClause", "path": "LockClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3510, 47], "end": [3510, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
