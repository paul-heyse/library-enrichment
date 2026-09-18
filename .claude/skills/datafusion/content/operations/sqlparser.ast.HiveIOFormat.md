# `sqlparser::ast::HiveIOFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveIOFormat.json).

<a id="op-d5f21bb5ba78649b14a8fdee"></a>
## HiveIOFormat

`enum` · `sqlparser::ast::HiveIOFormat` · sqlparser 0.62.0

```rust
enum HiveIOFormat
```

Source: `src/ast/mod.rs:8687`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive input/output format specification used in `CREATE TABLE`.

<a id="op-fc405b55f30b5770bf0fa37a"></a>
## FileFormat

`variant` · `sqlparser::ast::HiveIOFormat::FileFormat` · sqlparser 0.62.0

```rust
FileFormat
```

Source: `src/ast/mod.rs:8696`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

File format wrapper referencing a `FileFormat` variant.

<a id="op-a47f5ebcc85d39f060007f8c"></a>
## IOF

`variant` · `sqlparser::ast::HiveIOFormat::IOF` · sqlparser 0.62.0

```rust
IOF
```

Source: `src/ast/mod.rs:8689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic IO format with separate input and output expressions.

<a id="op-84cda6240dc98945afe93bcd"></a>
## Using

`variant` · `sqlparser::ast::HiveIOFormat::Using` · sqlparser 0.62.0

```rust
Using
```

Source: `src/ast/mod.rs:8705`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USING <format>` syntax used by Spark SQL.

Example: `CREATE TABLE t (i INT) USING PARQUET`

See <https://spark.apache.org/docs/latest/sql-ref-syntax-ddl-create-table-datasource.html>

<a id="op-bd45e1ff03ed17b9f69911e7"></a>
## clone

`function` · `sqlparser::ast::HiveIOFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveIOFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8682, 17], "end": [8682, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abe5ffa0517fa1352f6c4d7b"></a>
## cmp

`function` · `sqlparser::ast::HiveIOFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveIOFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8682, 51], "end": [8682, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cee4c48fcade8d7890edda5"></a>
## deserialize

`function` · `sqlparser::ast::HiveIOFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8683, 49], "end": [8683, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-792ef3ba6d8572e07b09c683"></a>
## eq

`function` · `sqlparser::ast::HiveIOFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveIOFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8682, 24], "end": [8682, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba6cc38e754edbb2a14ba586"></a>
## fmt

`function` · `sqlparser::ast::HiveIOFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8682, 10], "end": [8682, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d3cc5eb1d9b7554544aa021"></a>
## hash

`function` · `sqlparser::ast::HiveIOFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8682, 56], "end": [8682, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36ea7683f1f3531ba1a00fdc"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveIOFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveIOFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8682, 35], "end": [8682, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8682`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-813183caf9f5ede1240a5c71"></a>
## serialize

`function` · `sqlparser::ast::HiveIOFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8683, 38], "end": [8683, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b795e7c8a58c9230e093b0a"></a>
## visit

`function` · `sqlparser::ast::HiveIOFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8684, 47], "end": [8684, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bb94c9d02784346ac84671c"></a>
## visit

`function` · `sqlparser::ast::HiveIOFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveIOFormat", "path": "HiveIOFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8684, 40], "end": [8684, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8684`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
