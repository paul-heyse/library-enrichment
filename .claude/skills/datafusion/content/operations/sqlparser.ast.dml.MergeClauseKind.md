# `sqlparser::ast::dml::MergeClauseKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MergeClauseKind.json).

<a id="op-6ceecaebf0922e43d397e610"></a>
## MergeClauseKind

`enum` · `sqlparser::ast::dml::MergeClauseKind` · sqlparser 0.62.0

```rust
enum MergeClauseKind
```

Source: `src/ast/dml.rs:552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variant of `WHEN` clause used within a `MERGE` Statement.

Example:
```sql
MERGE INTO T USING U ON FALSE WHEN MATCHED THEN DELETE
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

<a id="op-a0aa8eaaf8cdd48b6569446f"></a>
## Matched

`variant` · `sqlparser::ast::dml::MergeClauseKind::Matched` · sqlparser 0.62.0

```rust
Matched
```

Source: `src/ast/dml.rs:554`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WHEN MATCHED`

<a id="op-66bbc1fab75ba31838ce18b5"></a>
## NotMatched

`variant` · `sqlparser::ast::dml::MergeClauseKind::NotMatched` · sqlparser 0.62.0

```rust
NotMatched
```

Source: `src/ast/dml.rs:556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WHEN NOT MATCHED`

<a id="op-67b9a77a5bcfe3d0ced79e55"></a>
## NotMatchedBySource

`variant` · `sqlparser::ast::dml::MergeClauseKind::NotMatchedBySource` · sqlparser 0.62.0

```rust
NotMatchedBySource
```

Source: `src/ast/dml.rs:564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WHEN MATCHED BY SOURCE`

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

<a id="op-f09b072eced6e05f4bc321c9"></a>
## NotMatchedByTarget

`variant` · `sqlparser::ast::dml::MergeClauseKind::NotMatchedByTarget` · sqlparser 0.62.0

```rust
NotMatchedByTarget
```

Source: `src/ast/dml.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WHEN MATCHED BY TARGET`

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

<a id="op-36cfe4c64c120c1e691b7736"></a>
## clone

`function` · `sqlparser::ast::dml::MergeClauseKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MergeClauseKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 17], "end": [549, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07275ce8f2d18691bfeeea52"></a>
## cmp

`function` · `sqlparser::ast::dml::MergeClauseKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MergeClauseKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 57], "end": [549, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac081c33776d7acf25dfea20"></a>
## deserialize

`function` · `sqlparser::ast::dml::MergeClauseKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 49], "end": [550, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90eea845ed13debb59fcc103"></a>
## eq

`function` · `sqlparser::ast::dml::MergeClauseKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MergeClauseKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 30], "end": [549, 39], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cf2a5c331b71c06e6e0701e"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeClauseKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 10], "end": [549, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b02e574b2bd7fb845c29c156"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeClauseKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [576, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:568`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d6ec0d6f06e035c9826eb80"></a>
## hash

`function` · `sqlparser::ast::dml::MergeClauseKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 62], "end": [549, 66], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-725baf0428c47cc7b44a232b"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MergeClauseKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MergeClauseKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 41], "end": [549, 51], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:549`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1099dd69bced9a8fabd1719e"></a>
## serialize

`function` · `sqlparser::ast::dml::MergeClauseKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 38], "end": [550, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113fe5b87b6d9ab1cff54b6b"></a>
## visit

`function` · `sqlparser::ast::dml::MergeClauseKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 47], "end": [551, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-865012248d55fbffaf44626d"></a>
## visit

`function` · `sqlparser::ast::dml::MergeClauseKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClauseKind", "path": "MergeClauseKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 40], "end": [551, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
