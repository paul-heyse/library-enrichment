# `sqlparser::ast::ddl::AlterTableAlgorithm`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableAlgorithm.json).

<a id="op-2ea20d1b1cbcfcaaa017edc8"></a>
## AlterTableAlgorithm

`enum` · `sqlparser::ast::ddl::AlterTableAlgorithm` · sqlparser 0.62.0

```rust
enum AlterTableAlgorithm
```

Source: `src/ast/ddl.rs:595`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] `ALTER TABLE` algorithm.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html
Algorithm option for `ALTER TABLE` operations (MySQL-specific).

<a id="op-0d627236964903c4f2e40c0f"></a>
## Copy

`variant` · `sqlparser::ast::ddl::AlterTableAlgorithm::Copy` · sqlparser 0.62.0

```rust
Copy
```

Source: `src/ast/ddl.rs:603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COPY` algorithm.

<a id="op-df9b8abf05d828af4657fc66"></a>
## Default

`variant` · `sqlparser::ast::ddl::AlterTableAlgorithm::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/ddl.rs:597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default algorithm selection.

<a id="op-811e688205c635f03e0aebaf"></a>
## Inplace

`variant` · `sqlparser::ast::ddl::AlterTableAlgorithm::Inplace` · sqlparser 0.62.0

```rust
Inplace
```

Source: `src/ast/ddl.rs:601`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INPLACE` algorithm.

<a id="op-0f66141e8efbcf04d099ac0c"></a>
## Instant

`variant` · `sqlparser::ast::ddl::AlterTableAlgorithm::Instant` · sqlparser 0.62.0

```rust
Instant
```

Source: `src/ast/ddl.rs:599`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INSTANT` algorithm.

<a id="op-41606434091a4c57eff16b90"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTableAlgorithm
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 17], "end": [591, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17793875a628a3899d0c5f1e"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTableAlgorithm) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 57], "end": [591, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aae565073832035c219e066f"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 49], "end": [592, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bd6544f9eae5f78e71c7bd7"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTableAlgorithm) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 30], "end": [591, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-492f06a7fa296ea8a481a843"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [606, 1], "end": [615, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa5ad3e4239ce2ce953bb7ef"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 10], "end": [591, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96e808faf2ec1863c11c5068"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 62], "end": [591, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d5285ed471260f65a4d3a5"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTableAlgorithm) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 41], "end": [591, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5d1ab647e829a676695c848"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 38], "end": [592, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70606c9e04cda1129b48f3d6"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 47], "end": [593, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876bc84e27e6abb6ac653d2b"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableAlgorithm::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableAlgorithm", "path": "AlterTableAlgorithm"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 40], "end": [593, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
