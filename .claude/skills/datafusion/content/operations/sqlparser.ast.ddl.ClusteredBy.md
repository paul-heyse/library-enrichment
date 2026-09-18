# `sqlparser::ast::ddl::ClusteredBy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ClusteredBy.json).

<a id="op-b307d8c93533ce87b0365d57"></a>
## ClusteredBy

`struct` · `sqlparser::ast::ddl::ClusteredBy` · sqlparser 0.62.0

```rust
struct ClusteredBy
```

Source: `src/ast/ddl.rs:2779`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive supports `CLUSTERED BY` statement in `CREATE TABLE`.
Syntax: `CLUSTERED BY (col_name, ...) [SORTED BY (col_name [ASC|DESC], ...)] INTO num_buckets BUCKETS`

[Hive](https://cwiki.apache.org/confluence/display/Hive/LanguageManual+DDL#LanguageManualDDL-CreateTable)

<a id="op-68d23babe174cb7cd99038f1"></a>
## clone

`function` · `sqlparser::ast::ddl::ClusteredBy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ClusteredBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2776, 17], "end": [2776, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eef5142e75eed7abf07a1e01"></a>
## cmp

`function` · `sqlparser::ast::ddl::ClusteredBy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ClusteredBy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2776, 51], "end": [2776, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0ab738048ba534eecc30f2d"></a>
## columns

`struct_field` · `sqlparser::ast::ddl::ClusteredBy::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::Ident>
```

Source: `src/ast/ddl.rs:2781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

columns used for clustering

<a id="op-0da9376dea96dc7fea585d4c"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ClusteredBy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2777, 49], "end": [2777, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e61b0197516ed69ce507fb28"></a>
## eq

`function` · `sqlparser::ast::ddl::ClusteredBy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ClusteredBy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2776, 24], "end": [2776, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d5d2d2346ab481a13be5500"></a>
## fmt

`function` · `sqlparser::ast::ddl::ClusteredBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2776, 10], "end": [2776, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39b23de11c9bfb0bb990bb67"></a>
## fmt

`function` · `sqlparser::ast::ddl::ClusteredBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2788, 1], "end": [2800, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd90c8156f7e2b5014c36f70"></a>
## hash

`function` · `sqlparser::ast::ddl::ClusteredBy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2776, 56], "end": [2776, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0047133d68dd32d15d70c90e"></a>
## num_buckets

`struct_field` · `sqlparser::ast::ddl::ClusteredBy::num_buckets` · sqlparser 0.62.0

```rust
num_buckets: ast::Value
```

Source: `src/ast/ddl.rs:2785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

number of buckets

<a id="op-9142b3a3137c3ca3045726dc"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ClusteredBy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ClusteredBy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2776, 35], "end": [2776, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebde02e5b7640588cf9b6825"></a>
## serialize

`function` · `sqlparser::ast::ddl::ClusteredBy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2777, 38], "end": [2777, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe285d14524319b731f939e1"></a>
## sorted_by

`struct_field` · `sqlparser::ast::ddl::ClusteredBy::sorted_by` · sqlparser 0.62.0

```rust
sorted_by: Option<Vec<ast::OrderByExpr>>
```

Source: `src/ast/ddl.rs:2783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

optional sorted by expressions

<a id="op-e135a08683fa23a4915c338d"></a>
## visit

`function` · `sqlparser::ast::ddl::ClusteredBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2778, 47], "end": [2778, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2778`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f47148f39f8b7df1b41f3abb"></a>
## visit

`function` · `sqlparser::ast::ddl::ClusteredBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ClusteredBy", "path": "ClusteredBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2778, 40], "end": [2778, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2778`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
