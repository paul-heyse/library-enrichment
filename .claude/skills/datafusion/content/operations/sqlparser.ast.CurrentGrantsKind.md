# `sqlparser::ast::CurrentGrantsKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CurrentGrantsKind.json).

<a id="op-28bdd0e90d6067647d04b1e4"></a>
## CurrentGrantsKind

`enum` · `sqlparser::ast::CurrentGrantsKind` · sqlparser 0.62.0

```rust
enum CurrentGrantsKind
```

Source: `src/ast/mod.rs:4968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
{COPY | REVOKE} CURRENT GRANTS
```

- [Snowflake](https://docs.snowflake.com/en/sql-reference/sql/grant-ownership#optional-parameters)

<a id="op-42d40e05373f58c5c3ad89f8"></a>
## CopyCurrentGrants

`variant` · `sqlparser::ast::CurrentGrantsKind::CopyCurrentGrants` · sqlparser 0.62.0

```rust
CopyCurrentGrants
```

Source: `src/ast/mod.rs:4970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COPY CURRENT GRANTS` (copy current grants to target).

<a id="op-574a38bae3e9e994223d9c6d"></a>
## RevokeCurrentGrants

`variant` · `sqlparser::ast::CurrentGrantsKind::RevokeCurrentGrants` · sqlparser 0.62.0

```rust
RevokeCurrentGrants
```

Source: `src/ast/mod.rs:4972`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`REVOKE CURRENT GRANTS` (revoke current grants from target).

<a id="op-5b703eb87a399132d77c6e11"></a>
## clone

`function` · `sqlparser::ast::CurrentGrantsKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CurrentGrantsKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4965, 17], "end": [4965, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:4965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4444e6b764f519891dfd2a63"></a>
## cmp

`function` · `sqlparser::ast::CurrentGrantsKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CurrentGrantsKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4965, 51], "end": [4965, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:4965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f408b4c5729de9514671011b"></a>
## deserialize

`function` · `sqlparser::ast::CurrentGrantsKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4966, 49], "end": [4966, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:4966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71a1e6af876c48e536a69b27"></a>
## eq

`function` · `sqlparser::ast::CurrentGrantsKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CurrentGrantsKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4965, 24], "end": [4965, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:4965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d9fd07a508feec3c2b52afb"></a>
## fmt

`function` · `sqlparser::ast::CurrentGrantsKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4965, 10], "end": [4965, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:4965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dba574cd7a04100ed5660969"></a>
## fmt

`function` · `sqlparser::ast::CurrentGrantsKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4975, 1], "end": [4982, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:4976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfba0c4b559e0f148e01ef79"></a>
## hash

`function` · `sqlparser::ast::CurrentGrantsKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4965, 56], "end": [4965, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:4965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23b446365d9ee954c300fa1f"></a>
## partial_cmp

`function` · `sqlparser::ast::CurrentGrantsKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CurrentGrantsKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4965, 35], "end": [4965, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:4965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d567c715721c19574169455"></a>
## serialize

`function` · `sqlparser::ast::CurrentGrantsKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4966, 38], "end": [4966, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:4966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c9bf7fec21fdef06e4d4af2"></a>
## visit

`function` · `sqlparser::ast::CurrentGrantsKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4967, 40], "end": [4967, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:4967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da295b50d0b502594f84af37"></a>
## visit

`function` · `sqlparser::ast::CurrentGrantsKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CurrentGrantsKind", "path": "CurrentGrantsKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4967, 47], "end": [4967, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:4967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
