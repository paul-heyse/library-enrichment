# `sqlparser::ast::UnloadPartitionBy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.UnloadPartitionBy.json).

<a id="op-f0b25d0df445043f46c38a2c"></a>
## UnloadPartitionBy

`struct` · `sqlparser::ast::UnloadPartitionBy` · sqlparser 0.62.0

```rust
struct UnloadPartitionBy
```

Source: `src/ast/mod.rs:9629`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the partition keys for the unload operation

```sql
PARTITION BY ( column_name [, ... ] ) [ INCLUDE ]
```

<a id="op-621e70d9a5d504909b7d841f"></a>
## clone

`function` · `sqlparser::ast::UnloadPartitionBy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UnloadPartitionBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9626, 17], "end": [9626, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd97f12415add8be113da26f"></a>
## cmp

`function` · `sqlparser::ast::UnloadPartitionBy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UnloadPartitionBy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9626, 51], "end": [9626, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1f940962bdc4a159855bcae"></a>
## columns

`struct_field` · `sqlparser::ast::UnloadPartitionBy::columns` · sqlparser 0.62.0

```rust
columns: Vec<Ident>
```

Source: `src/ast/mod.rs:9631`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns used to partition the unload output.

<a id="op-abe8f13e377a596ccfec093c"></a>
## deserialize

`function` · `sqlparser::ast::UnloadPartitionBy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9627, 49], "end": [9627, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6911e3e11e9d1fa8f96403e"></a>
## eq

`function` · `sqlparser::ast::UnloadPartitionBy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UnloadPartitionBy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9626, 24], "end": [9626, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62880b5cd71917dc2db6d490"></a>
## fmt

`function` · `sqlparser::ast::UnloadPartitionBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9636, 1], "end": [9645, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8c636656e7fbaac4a7e7a2b"></a>
## fmt

`function` · `sqlparser::ast::UnloadPartitionBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9626, 10], "end": [9626, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b377d1c9b83bdd8eb3d7b7d"></a>
## hash

`function` · `sqlparser::ast::UnloadPartitionBy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9626, 56], "end": [9626, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac0cf6ab00b3960b1631cb2c"></a>
## include

`struct_field` · `sqlparser::ast::UnloadPartitionBy::include` · sqlparser 0.62.0

```rust
include: bool
```

Source: `src/ast/mod.rs:9633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to include the partition in the output.

<a id="op-8a206f2d5ced9d2c4e0a1365"></a>
## partial_cmp

`function` · `sqlparser::ast::UnloadPartitionBy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UnloadPartitionBy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9626, 35], "end": [9626, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4da104426314df604a4f090"></a>
## serialize

`function` · `sqlparser::ast::UnloadPartitionBy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9627, 38], "end": [9627, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9627`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02b173218d4a7a40981e2d98"></a>
## visit

`function` · `sqlparser::ast::UnloadPartitionBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9628, 40], "end": [9628, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529838f2d63895ad11c66bf4"></a>
## visit

`function` · `sqlparser::ast::UnloadPartitionBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnloadPartitionBy", "path": "UnloadPartitionBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9628, 47], "end": [9628, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
