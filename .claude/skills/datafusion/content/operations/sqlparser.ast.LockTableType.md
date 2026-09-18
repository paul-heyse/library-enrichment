# `sqlparser::ast::LockTableType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LockTableType.json).

<a id="op-ace548ee0acd4135c2694dad"></a>
## LockTableType

`enum` · `sqlparser::ast::LockTableType` · sqlparser 0.62.0

```rust
enum LockTableType
```

Source: `src/ast/mod.rs:10390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of lock used in `LOCK TABLE` statements.

<a id="op-faa5123afee4f6c7f1f574ac"></a>
## Read

`variant` · `sqlparser::ast::LockTableType::Read` · sqlparser 0.62.0

```rust
Read
```

Source: `src/ast/mod.rs:10392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Shared/read lock. If `local` is true, it's a local read lock.

<a id="op-975a86657401735995bdde0a"></a>
## Write

`variant` · `sqlparser::ast::LockTableType::Write` · sqlparser 0.62.0

```rust
Write
```

Source: `src/ast/mod.rs:10397`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exclusive/write lock. If `low_priority` is true, the write is low priority.

<a id="op-a3cf07a0da18dfa24eed6f49"></a>
## clone

`function` · `sqlparser::ast::LockTableType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LockTableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10386, 17], "end": [10386, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0776bd6f101866034eb7100"></a>
## cmp

`function` · `sqlparser::ast::LockTableType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LockTableType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10386, 51], "end": [10386, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d32bc823bf10bf2f9705e010"></a>
## deserialize

`function` · `sqlparser::ast::LockTableType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10387, 49], "end": [10387, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-201210e6d2eecb13e0c276d7"></a>
## eq

`function` · `sqlparser::ast::LockTableType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LockTableType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10386, 24], "end": [10386, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-065461750d817faa26ad8713"></a>
## fmt

`function` · `sqlparser::ast::LockTableType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10403, 1], "end": [10422, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10404`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e009b564b86918bdc977009c"></a>
## fmt

`function` · `sqlparser::ast::LockTableType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10386, 10], "end": [10386, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ea46b5b07f017ed50a1019a"></a>
## hash

`function` · `sqlparser::ast::LockTableType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10386, 56], "end": [10386, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-494c0030698f71e4a56bfc50"></a>
## partial_cmp

`function` · `sqlparser::ast::LockTableType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LockTableType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10386, 35], "end": [10386, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10386`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ff7a7b4acfebbc218b9735a"></a>
## serialize

`function` · `sqlparser::ast::LockTableType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10387, 38], "end": [10387, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-907a10fd185087326e8838e8"></a>
## visit

`function` · `sqlparser::ast::LockTableType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10388, 47], "end": [10388, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb30e4b2154911b5ca93042a"></a>
## visit

`function` · `sqlparser::ast::LockTableType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableType", "path": "LockTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10388, 40], "end": [10388, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
