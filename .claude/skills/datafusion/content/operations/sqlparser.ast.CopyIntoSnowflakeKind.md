# `sqlparser::ast::CopyIntoSnowflakeKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopyIntoSnowflakeKind.json).

<a id="op-f354ab6bb2aa1abcb1099740"></a>
## CopyIntoSnowflakeKind

`enum` · `sqlparser::ast::CopyIntoSnowflakeKind` · sqlparser 0.62.0

```rust
enum CopyIntoSnowflakeKind
```

Source: `src/ast/mod.rs:11274`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variants of the Snowflake `COPY INTO` statement

<a id="op-44ebc71c08f30822fa1cc7c1"></a>
## Location

`variant` · `sqlparser::ast::CopyIntoSnowflakeKind::Location` · sqlparser 0.62.0

```rust
Location
```

Source: `src/ast/mod.rs:11280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unloads data from a table or query to external files
See: <https://docs.snowflake.com/en/sql-reference/sql/copy-into-location>

<a id="op-63daa6d2229e1a86b18488e3"></a>
## Table

`variant` · `sqlparser::ast::CopyIntoSnowflakeKind::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:11277`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Loads data from files to a table
See: <https://docs.snowflake.com/en/sql-reference/sql/copy-into-table>

<a id="op-7f713379fb5ff23ac5face6c"></a>
## clone

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CopyIntoSnowflakeKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11271, 23], "end": [11271, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e9d69f19f1feb9cd9eb7913"></a>
## cmp

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CopyIntoSnowflakeKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11271, 57], "end": [11271, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99bdad1eb7b3a1f6b34b14d2"></a>
## deserialize

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11272, 49], "end": [11272, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ad5a828acf1c59a6e77ca2"></a>
## eq

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CopyIntoSnowflakeKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11271, 30], "end": [11271, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cbd0f1e2904ed487c8695ac"></a>
## fmt

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11271, 10], "end": [11271, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43fad40caf0ac161b53ab784"></a>
## hash

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11271, 62], "end": [11271, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32f0d96be115cc7a064abdd7"></a>
## partial_cmp

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CopyIntoSnowflakeKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11271, 41], "end": [11271, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f6b51b7a9e7e1e3b2c76dc1"></a>
## serialize

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11272, 38], "end": [11272, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11272`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-413ea210a043388aa5501d91"></a>
## visit

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11273, 40], "end": [11273, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c6285a626084dbf3face428"></a>
## visit

`function` · `sqlparser::ast::CopyIntoSnowflakeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyIntoSnowflakeKind", "path": "CopyIntoSnowflakeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11273, 47], "end": [11273, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11273`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
