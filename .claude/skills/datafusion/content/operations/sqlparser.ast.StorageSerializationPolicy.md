# `sqlparser::ast::StorageSerializationPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.StorageSerializationPolicy.json).

<a id="op-de4437b41b334b279831f25f"></a>
## StorageSerializationPolicy

`enum` · `sqlparser::ast::StorageSerializationPolicy` · sqlparser 0.62.0

```rust
enum StorageSerializationPolicy
```

Source: `src/ast/mod.rs:11229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake StorageSerializationPolicy for Iceberg Tables
```sql
[ STORAGE_SERIALIZATION_POLICY = { COMPATIBLE | OPTIMIZED } ]
```

<https://docs.snowflake.com/en/sql-reference/sql/create-iceberg-table>

<a id="op-fefbc59536f1dcef96803f68"></a>
## Compatible

`variant` · `sqlparser::ast::StorageSerializationPolicy::Compatible` · sqlparser 0.62.0

```rust
Compatible
```

Source: `src/ast/mod.rs:11231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use compatible serialization mode.

<a id="op-dea98f103b056ba8af02a423"></a>
## Optimized

`variant` · `sqlparser::ast::StorageSerializationPolicy::Optimized` · sqlparser 0.62.0

```rust
Optimized
```

Source: `src/ast/mod.rs:11233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use optimized serialization mode.

<a id="op-b711a65b097767619e00a6e0"></a>
## clone

`function` · `sqlparser::ast::StorageSerializationPolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StorageSerializationPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11226, 23], "end": [11226, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b49fcf5e61472dabcc93936f"></a>
## cmp

`function` · `sqlparser::ast::StorageSerializationPolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StorageSerializationPolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11226, 57], "end": [11226, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d33bd27f795cce1a554f7f6"></a>
## deserialize

`function` · `sqlparser::ast::StorageSerializationPolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11227, 49], "end": [11227, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c4aa894b51d497b6b2103b8"></a>
## eq

`function` · `sqlparser::ast::StorageSerializationPolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StorageSerializationPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11226, 30], "end": [11226, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7b0bf3680afd72f17c6a44e"></a>
## fmt

`function` · `sqlparser::ast::StorageSerializationPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11226, 10], "end": [11226, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5a4230e90305344f31dae01"></a>
## fmt

`function` · `sqlparser::ast::StorageSerializationPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11236, 1], "end": [11243, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c1230ff5ce995d1d7e75701"></a>
## hash

`function` · `sqlparser::ast::StorageSerializationPolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11226, 62], "end": [11226, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b19534153340797c2bf89cd4"></a>
## partial_cmp

`function` · `sqlparser::ast::StorageSerializationPolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StorageSerializationPolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11226, 41], "end": [11226, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11226`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e6bc4841dada3ab6087c8f0"></a>
## serialize

`function` · `sqlparser::ast::StorageSerializationPolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11227, 38], "end": [11227, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3602207137059ecb1d9795a2"></a>
## visit

`function` · `sqlparser::ast::StorageSerializationPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11228, 47], "end": [11228, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c12f43edf0f8b9bdf7038b2"></a>
## visit

`function` · `sqlparser::ast::StorageSerializationPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageSerializationPolicy", "path": "StorageSerializationPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11228, 40], "end": [11228, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
