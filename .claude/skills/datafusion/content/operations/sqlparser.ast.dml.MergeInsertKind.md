# `sqlparser::ast::dml::MergeInsertKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MergeInsertKind.json).

<a id="op-ac04e907c4da714f5ba66d95"></a>
## MergeInsertKind

`enum` · `sqlparser::ast::dml::MergeInsertKind` · sqlparser 0.62.0

```rust
enum MergeInsertKind
```

Source: `src/ast/dml.rs:636`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of expression used to insert rows within a `MERGE` statement.

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

<a id="op-b10ad86e8c3abe0c5aa07e9f"></a>
## Row

`variant` · `sqlparser::ast::dml::MergeInsertKind::Row` · sqlparser 0.62.0

```rust
Row
```

Source: `src/ast/dml.rs:651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The insert expression is defined using only the `ROW` keyword.

Example:
```sql
INSERT ROW
```
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

<a id="op-88c5554055ca10b7a1c1a240"></a>
## Values

`variant` · `sqlparser::ast::dml::MergeInsertKind::Values` · sqlparser 0.62.0

```rust
Values
```

Source: `src/ast/dml.rs:643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The insert expression is defined from an explicit `VALUES` clause

Example:
```sql
INSERT VALUES(product, quantity)
```

<a id="op-b1d8af204fd8b1ff913a8008"></a>
## clone

`function` · `sqlparser::ast::dml::MergeInsertKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MergeInsertKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 17], "end": [633, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-479187190eef4b2bf604f231"></a>
## cmp

`function` · `sqlparser::ast::dml::MergeInsertKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MergeInsertKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 51], "end": [633, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f42b36f82c53f0c8e9436b6e"></a>
## deserialize

`function` · `sqlparser::ast::dml::MergeInsertKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [634, 49], "end": [634, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e2483ed5576e6b0866b6206"></a>
## eq

`function` · `sqlparser::ast::dml::MergeInsertKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MergeInsertKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 24], "end": [633, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-125b6e6ee09674c3e31eed01"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeInsertKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 1], "end": [665, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f814c3b5a488fb747e1f270"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeInsertKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 10], "end": [633, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0720cffb936801b67e3f6255"></a>
## hash

`function` · `sqlparser::ast::dml::MergeInsertKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 56], "end": [633, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f329f7d293638b267b082b5"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MergeInsertKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MergeInsertKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [633, 35], "end": [633, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:633`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e755fdda27afb9bfb96ab4c7"></a>
## serialize

`function` · `sqlparser::ast::dml::MergeInsertKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [634, 38], "end": [634, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:634`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b60e604c9790f5e62ed9e05"></a>
## visit

`function` · `sqlparser::ast::dml::MergeInsertKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [635, 40], "end": [635, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:635`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-740fc2a99b6a7dbbadf5a0c2"></a>
## visit

`function` · `sqlparser::ast::dml::MergeInsertKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertKind", "path": "MergeInsertKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [635, 47], "end": [635, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:635`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
