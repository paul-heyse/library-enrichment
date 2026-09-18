# `sqlparser::ast::ddl::IndexType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IndexType.json).

<a id="op-8a08950c8c97b3d345c24530"></a>
## IndexType

`enum` · `sqlparser::ast::ddl::IndexType` · sqlparser 0.62.0

```rust
enum IndexType
```

Source: `src/ast/ddl.rs:1412`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indexing method used by that index.

This structure isn't present on ANSI, but is found at least in [`MySQL` CREATE TABLE][1],
[`MySQL` CREATE INDEX][2], and [Postgresql CREATE INDEX][3] statements.

[1]: https://dev.mysql.com/doc/refman/8.0/en/create-table.html
[2]: https://dev.mysql.com/doc/refman/8.0/en/create-index.html
[3]: https://www.postgresql.org/docs/14/sql-createindex.html

<a id="op-c62ce25a431ee7a489a2ae51"></a>
## BRIN

`variant` · `sqlparser::ast::ddl::IndexType::BRIN` · sqlparser 0.62.0

```rust
BRIN
```

Source: `src/ast/ddl.rs:1424`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Block Range Index (BRIN).

<a id="op-a9e0cba1e78b35096bd2ea28"></a>
## BTree

`variant` · `sqlparser::ast::ddl::IndexType::BTree` · sqlparser 0.62.0

```rust
BTree
```

Source: `src/ast/ddl.rs:1414`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

B-Tree index (commonly default for many databases).

<a id="op-f584cdead07ae5037e3b01ba"></a>
## Bloom

`variant` · `sqlparser::ast::ddl::IndexType::Bloom` · sqlparser 0.62.0

```rust
Bloom
```

Source: `src/ast/ddl.rs:1426`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bloom filter based index.

<a id="op-dee3d5b7a896fedf0cceb7a4"></a>
## Custom

`variant` · `sqlparser::ast::ddl::IndexType::Custom` · sqlparser 0.62.0

```rust
Custom
```

Source: `src/ast/ddl.rs:1429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Users may define their own index types, which would
not be covered by the above variants.

<a id="op-cb254dceff4b83a27ad8f3bd"></a>
## GIN

`variant` · `sqlparser::ast::ddl::IndexType::GIN` · sqlparser 0.62.0

```rust
GIN
```

Source: `src/ast/ddl.rs:1418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generalized Inverted Index (GIN).

<a id="op-041709809893d6f83fa7c5f0"></a>
## GiST

`variant` · `sqlparser::ast::ddl::IndexType::GiST` · sqlparser 0.62.0

```rust
GiST
```

Source: `src/ast/ddl.rs:1420`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generalized Search Tree (GiST) index.

<a id="op-c0417658694a936f3e0e1c03"></a>
## Hash

`variant` · `sqlparser::ast::ddl::IndexType::Hash` · sqlparser 0.62.0

```rust
Hash
```

Source: `src/ast/ddl.rs:1416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hash index.

<a id="op-e2445cdf3470032c97d00a57"></a>
## SPGiST

`variant` · `sqlparser::ast::ddl::IndexType::SPGiST` · sqlparser 0.62.0

```rust
SPGiST
```

Source: `src/ast/ddl.rs:1422`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Space-partitioned GiST (SPGiST) index.

<a id="op-ea2964f9508dc4bdabccdf51"></a>
## clone

`function` · `sqlparser::ast::ddl::IndexType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IndexType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1409, 17], "end": [1409, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53de2024a7a087166bbcfcb4"></a>
## cmp

`function` · `sqlparser::ast::ddl::IndexType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IndexType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1409, 51], "end": [1409, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-322f610ba09ffe5041ea3b5a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IndexType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 49], "end": [1410, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8380e88a59b1d436ab4d98b7"></a>
## eq

`function` · `sqlparser::ast::ddl::IndexType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IndexType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1409, 24], "end": [1409, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03e1d19810d6e7ac69bdf6df"></a>
## fmt

`function` · `sqlparser::ast::ddl::IndexType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1409, 10], "end": [1409, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f036c39f547647831a3fd35"></a>
## fmt

`function` · `sqlparser::ast::ddl::IndexType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1432, 1], "end": [1445, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0352bad8c426e38540209700"></a>
## hash

`function` · `sqlparser::ast::ddl::IndexType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1409, 56], "end": [1409, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9f4c92c9c0461c473f77f62"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IndexType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IndexType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1409, 35], "end": [1409, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4284d7c41120d4856b6f47b8"></a>
## serialize

`function` · `sqlparser::ast::ddl::IndexType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1410, 38], "end": [1410, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-576255101a937659d676e95f"></a>
## visit

`function` · `sqlparser::ast::ddl::IndexType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1411, 47], "end": [1411, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc4fa09b0b7fec8cb6493914"></a>
## visit

`function` · `sqlparser::ast::ddl::IndexType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IndexType", "path": "IndexType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1411, 40], "end": [1411, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
