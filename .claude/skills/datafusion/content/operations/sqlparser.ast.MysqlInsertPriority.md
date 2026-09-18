# `sqlparser::ast::MysqlInsertPriority`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.MysqlInsertPriority.json).

<a id="op-6ec7ac0fe4c4ba221d05555c"></a>
## MysqlInsertPriority

`enum` · `sqlparser::ast::MysqlInsertPriority` · sqlparser 0.62.0

```rust
enum MysqlInsertPriority
```

Source: `src/ast/mod.rs:9230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Mysql specific syntax

See [Mysql documentation](https://dev.mysql.com/doc/refman/8.0/en/replace.html)
See [Mysql documentation](https://dev.mysql.com/doc/refman/8.0/en/insert.html)
for more details.

<a id="op-f128a72efa5118db42cd672b"></a>
## Delayed

`variant` · `sqlparser::ast::MysqlInsertPriority::Delayed` · sqlparser 0.62.0

```rust
Delayed
```

Source: `src/ast/mod.rs:9234`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DELAYED modifier for INSERT/REPLACE.

<a id="op-dea444e471dc0989453e7f89"></a>
## HighPriority

`variant` · `sqlparser::ast::MysqlInsertPriority::HighPriority` · sqlparser 0.62.0

```rust
HighPriority
```

Source: `src/ast/mod.rs:9236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

HIGH_PRIORITY modifier for INSERT/REPLACE.

<a id="op-04b56338aa8b0f8e55906df2"></a>
## LowPriority

`variant` · `sqlparser::ast::MysqlInsertPriority::LowPriority` · sqlparser 0.62.0

```rust
LowPriority
```

Source: `src/ast/mod.rs:9232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LOW_PRIORITY modifier for INSERT/REPLACE.

<a id="op-ad5934b256f51581958cc5b0"></a>
## clone

`function` · `sqlparser::ast::MysqlInsertPriority::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MysqlInsertPriority
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9227, 23], "end": [9227, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-856dca68f327cfd5ea88a87f"></a>
## cmp

`function` · `sqlparser::ast::MysqlInsertPriority::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MysqlInsertPriority) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9227, 57], "end": [9227, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94349dfb2112334ddb69ec8e"></a>
## deserialize

`function` · `sqlparser::ast::MysqlInsertPriority::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9228, 49], "end": [9228, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f81211e1eb2882487f1d9ce5"></a>
## eq

`function` · `sqlparser::ast::MysqlInsertPriority::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MysqlInsertPriority) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9227, 30], "end": [9227, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c532697d3f7fa51a9c9421d"></a>
## fmt

`function` · `sqlparser::ast::MysqlInsertPriority::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9227, 10], "end": [9227, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c05ca70f4e75d8886289d289"></a>
## fmt

`function` · `sqlparser::ast::MysqlInsertPriority::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "crate::ast::MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9239, 1], "end": [9248, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9240`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73f0c410ceb7481273e6b51b"></a>
## hash

`function` · `sqlparser::ast::MysqlInsertPriority::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9227, 62], "end": [9227, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f69ee5cb284007f30d78cfcd"></a>
## partial_cmp

`function` · `sqlparser::ast::MysqlInsertPriority::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MysqlInsertPriority) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9227, 41], "end": [9227, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1040b29a9ccaf266e92976f4"></a>
## serialize

`function` · `sqlparser::ast::MysqlInsertPriority::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9228, 38], "end": [9228, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9228`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-396f168ee5c6cca89fe09174"></a>
## visit

`function` · `sqlparser::ast::MysqlInsertPriority::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9229, 40], "end": [9229, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a98ca84755ff26998307e6fc"></a>
## visit

`function` · `sqlparser::ast::MysqlInsertPriority::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::MysqlInsertPriority", "path": "MysqlInsertPriority"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9229, 47], "end": [9229, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9229`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
