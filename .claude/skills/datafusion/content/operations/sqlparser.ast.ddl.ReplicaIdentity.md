# `sqlparser::ast::ddl::ReplicaIdentity`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ReplicaIdentity.json).

<a id="op-f667dacd4575bb9b67c67a4a"></a>
## ReplicaIdentity

`enum` · `sqlparser::ast::ddl::ReplicaIdentity` · sqlparser 0.62.0

```rust
enum ReplicaIdentity
```

Source: `src/ast/ddl.rs:101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ALTER TABLE operation REPLICA IDENTITY values
See [Postgres ALTER TABLE docs](https://www.postgresql.org/docs/current/sql-altertable.html)

<a id="op-52b05924093660103e9b15b1"></a>
## Default

`variant` · `sqlparser::ast::ddl::ReplicaIdentity::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/ddl.rs:107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default replica identity (`REPLICA IDENTITY DEFAULT`).

<a id="op-4b3d2b0c27d8975e8bbdd02f"></a>
## Full

`variant` · `sqlparser::ast::ddl::ReplicaIdentity::Full` · sqlparser 0.62.0

```rust
Full
```

Source: `src/ast/ddl.rs:105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Full replica identity (`REPLICA IDENTITY FULL`).

<a id="op-e7ebfc9daab0a3c21e648bf4"></a>
## Index

`variant` · `sqlparser::ast::ddl::ReplicaIdentity::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/ddl.rs:109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use the given index as replica identity (`REPLICA IDENTITY USING INDEX`).

<a id="op-2947823770a1c339e3b204ff"></a>
## Nothing

`variant` · `sqlparser::ast::ddl::ReplicaIdentity::Nothing` · sqlparser 0.62.0

```rust
Nothing
```

Source: `src/ast/ddl.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No replica identity (`REPLICA IDENTITY NOTHING`).

<a id="op-513c9c99b30d060c35e5b25e"></a>
## clone

`function` · `sqlparser::ast::ddl::ReplicaIdentity::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ReplicaIdentity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 17], "end": [98, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-048a1ec2866d0cdbc508d5a0"></a>
## cmp

`function` · `sqlparser::ast::ddl::ReplicaIdentity::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ReplicaIdentity) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 51], "end": [98, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b531dd879095d1e5e1589d3"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ReplicaIdentity::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 49], "end": [99, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ddcbba46e09565f448d2dd5"></a>
## eq

`function` · `sqlparser::ast::ddl::ReplicaIdentity::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ReplicaIdentity) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 24], "end": [98, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ef49e75f844852cdce52382"></a>
## fmt

`function` · `sqlparser::ast::ddl::ReplicaIdentity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [121, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da9e35a74e343fd7f2906c38"></a>
## fmt

`function` · `sqlparser::ast::ddl::ReplicaIdentity::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 10], "end": [98, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14501097993332d1f7ad59b6"></a>
## hash

`function` · `sqlparser::ast::ddl::ReplicaIdentity::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 56], "end": [98, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d75f45238fd1c7a1a8fb63a"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ReplicaIdentity::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ReplicaIdentity) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 35], "end": [98, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57eb6056c16b7d15da695ae2"></a>
## serialize

`function` · `sqlparser::ast::ddl::ReplicaIdentity::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 38], "end": [99, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9af68d9b81659062363203a"></a>
## visit

`function` · `sqlparser::ast::ddl::ReplicaIdentity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 47], "end": [100, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be2b3aa1dd3f4ce24c51affc"></a>
## visit

`function` · `sqlparser::ast::ddl::ReplicaIdentity::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ReplicaIdentity", "path": "ReplicaIdentity"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 40], "end": [100, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
