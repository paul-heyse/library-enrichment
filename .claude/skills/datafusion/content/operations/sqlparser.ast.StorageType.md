# `sqlparser::ast::StorageType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.StorageType.json).

<a id="op-5e62b454e851a1dce7c2a7cd"></a>
## StorageType

`enum` · `sqlparser::ast::StorageType` · sqlparser 0.62.0

```rust
enum StorageType
```

Source: `src/ast/mod.rs:8905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Storage type options for a tablespace.

<a id="op-65c224ad1518b47eeddbccd1"></a>
## Disk

`variant` · `sqlparser::ast::StorageType::Disk` · sqlparser 0.62.0

```rust
Disk
```

Source: `src/ast/mod.rs:8907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Store on disk.

<a id="op-a63d6576b5f0c039faa94b2f"></a>
## Memory

`variant` · `sqlparser::ast::StorageType::Memory` · sqlparser 0.62.0

```rust
Memory
```

Source: `src/ast/mod.rs:8909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Store in memory.

<a id="op-d21cde5cf09218f009075bfd"></a>
## clone

`function` · `sqlparser::ast::StorageType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StorageType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8901, 17], "end": [8901, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5df4540d22f7b0f9098100e9"></a>
## cmp

`function` · `sqlparser::ast::StorageType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StorageType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8901, 57], "end": [8901, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac49d353c4ad55731896d70d"></a>
## deserialize

`function` · `sqlparser::ast::StorageType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8902, 49], "end": [8902, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63182c18515dfeb4bff58ac0"></a>
## eq

`function` · `sqlparser::ast::StorageType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StorageType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8901, 24], "end": [8901, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b43e9ff36388c6447cde0379"></a>
## fmt

`function` · `sqlparser::ast::StorageType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8901, 10], "end": [8901, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57609b07af491f961b1f5f02"></a>
## hash

`function` · `sqlparser::ast::StorageType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8901, 39], "end": [8901, 43], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b82cf7190a059a1c2c734053"></a>
## partial_cmp

`function` · `sqlparser::ast::StorageType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StorageType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8901, 45], "end": [8901, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8901`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f369837134a8f43d9f8a60d"></a>
## serialize

`function` · `sqlparser::ast::StorageType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8902, 38], "end": [8902, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d8e35e1691a806cb166dc63"></a>
## visit

`function` · `sqlparser::ast::StorageType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8903, 40], "end": [8903, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b117878509e195b6dec55b91"></a>
## visit

`function` · `sqlparser::ast::StorageType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StorageType", "path": "StorageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8903, 47], "end": [8903, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
