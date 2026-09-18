# `sqlparser::ast::query::ValueTableMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ValueTableMode.json).

<a id="op-d1e04d0e4ddddd7ee79c571f"></a>
## ValueTableMode

`enum` · `sqlparser::ast::query::ValueTableMode` · sqlparser 0.62.0

```rust
enum ValueTableMode
```

Source: `src/ast/query.rs:4159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery supports ValueTables which have 2 modes:
`SELECT [ALL | DISTINCT] AS STRUCT`
`SELECT [ALL | DISTINCT] AS VALUE`

<https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#value_tables>
<https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_list>
Mode of BigQuery value tables, e.g. `AS STRUCT` or `AS VALUE`.

<a id="op-f2c65a6006bcd7418b35c8e2"></a>
## AsStruct

`variant` · `sqlparser::ast::query::ValueTableMode::AsStruct` · sqlparser 0.62.0

```rust
AsStruct
```

Source: `src/ast/query.rs:4161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AS STRUCT`

<a id="op-3788ebff1cd49224f771aaca"></a>
## AsValue

`variant` · `sqlparser::ast::query::ValueTableMode::AsValue` · sqlparser 0.62.0

```rust
AsValue
```

Source: `src/ast/query.rs:4163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AS VALUE`

<a id="op-458447540a5b09b79fc4b437"></a>
## DistinctAsStruct

`variant` · `sqlparser::ast::query::ValueTableMode::DistinctAsStruct` · sqlparser 0.62.0

```rust
DistinctAsStruct
```

Source: `src/ast/query.rs:4165`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTINCT AS STRUCT`

<a id="op-bfcecc63f09602d10a329c1d"></a>
## DistinctAsValue

`variant` · `sqlparser::ast::query::ValueTableMode::DistinctAsValue` · sqlparser 0.62.0

```rust
DistinctAsValue
```

Source: `src/ast/query.rs:4167`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTINCT AS VALUE`

<a id="op-cfb6ab66e29514485a0ad6ed"></a>
## clone

`function` · `sqlparser::ast::query::ValueTableMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ValueTableMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4155, 23], "end": [4155, 28], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad2e101e73614e2fa24be072"></a>
## cmp

`function` · `sqlparser::ast::query::ValueTableMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ValueTableMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4155, 57], "end": [4155, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4244a9c57b50d8e32f79dbe"></a>
## deserialize

`function` · `sqlparser::ast::query::ValueTableMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4156, 49], "end": [4156, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd1bf2ce680e244501947e9f"></a>
## eq

`function` · `sqlparser::ast::query::ValueTableMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ValueTableMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4155, 30], "end": [4155, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03b153f2d8f60e2c2c35edcc"></a>
## fmt

`function` · `sqlparser::ast::query::ValueTableMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4155, 10], "end": [4155, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4102761ed5b018507c8df8a5"></a>
## fmt

`function` · `sqlparser::ast::query::ValueTableMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4170, 1], "end": [4179, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1923159c58739f61f86b307f"></a>
## hash

`function` · `sqlparser::ast::query::ValueTableMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4155, 62], "end": [4155, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f83d955ede7222886a5c9b01"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ValueTableMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ValueTableMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4155, 41], "end": [4155, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4edb03a4171acfe160e27148"></a>
## serialize

`function` · `sqlparser::ast::query::ValueTableMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4156, 38], "end": [4156, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-632ee9c620aca9c3ef3ab5e8"></a>
## visit

`function` · `sqlparser::ast::query::ValueTableMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4157, 47], "end": [4157, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1a28644ebff61c39ecb7672"></a>
## visit

`function` · `sqlparser::ast::query::ValueTableMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ValueTableMode", "path": "ValueTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4157, 40], "end": [4157, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
