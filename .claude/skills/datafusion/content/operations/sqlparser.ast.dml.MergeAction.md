# `sqlparser::ast::dml::MergeAction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MergeAction.json).

<a id="op-2e6571abbc12ea40599750b2"></a>
## MergeAction

`enum` · `sqlparser::ast::dml::MergeAction` · sqlparser 0.62.0

```rust
enum MergeAction
```

Source: `src/ast/dml.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Underlying statement of a `WHEN` clause within a `MERGE` Statement

Example
```sql
INSERT (product, quantity) VALUES(product, quantity)
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/MERGE.html)

<a id="op-edebcab5cd1c0dd32f206509"></a>
## Delete

`variant` · `sqlparser::ast::dml::MergeAction::Delete` · sqlparser 0.62.0

```rust
Delete
```

Source: `src/ast/dml.rs:607`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A plain `DELETE` clause

<a id="op-94112b0f3d4081fa94cb55c3"></a>
## Insert

`variant` · `sqlparser::ast::dml::MergeAction::Insert` · sqlparser 0.62.0

```rust
Insert
```

Source: `src/ast/dml.rs:598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `INSERT` clause

Example:
```sql
INSERT (product, quantity) VALUES(product, quantity)
```

<a id="op-3cbd420483d2fd391cdbff46"></a>
## Update

`variant` · `sqlparser::ast::dml::MergeAction::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/dml.rs:605`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `UPDATE` clause

Example:
```sql
UPDATE SET quantity = T.quantity + S.quantity
```

<a id="op-d4d469774bd47f0171d64e38"></a>
## clone

`function` · `sqlparser::ast::dml::MergeAction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MergeAction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [588, 17], "end": [588, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dc0626ac0f7629eb3d15436"></a>
## cmp

`function` · `sqlparser::ast::dml::MergeAction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MergeAction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [588, 51], "end": [588, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ff1a5751ac8205beb6e8dba"></a>
## deserialize

`function` · `sqlparser::ast::dml::MergeAction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 49], "end": [589, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15dfb0cf5244cfb174568c05"></a>
## eq

`function` · `sqlparser::ast::dml::MergeAction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MergeAction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [588, 24], "end": [588, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69f8320e93720f7290208dfb"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [588, 10], "end": [588, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b839cd36b6130944934bf5d"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeAction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [613, 1], "end": [627, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:614`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4c4d57a905b88b3ae783f65"></a>
## hash

`function` · `sqlparser::ast::dml::MergeAction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [588, 56], "end": [588, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ba450f6b4994036df5288a0"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MergeAction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MergeAction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [588, 35], "end": [588, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7b7991f247dc67db46c1957"></a>
## serialize

`function` · `sqlparser::ast::dml::MergeAction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [589, 38], "end": [589, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cbe4483cb5ca8e5037dcc19"></a>
## span

`function` · `sqlparser::ast::dml::MergeAction::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "super::MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2519, 1], "end": [2527, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2520`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1df91080055a700d4728402f"></a>
## visit

`function` · `sqlparser::ast::dml::MergeAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 40], "end": [590, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d42760e84d1e87ccc16e3247"></a>
## visit

`function` · `sqlparser::ast::dml::MergeAction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeAction", "path": "MergeAction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [590, 47], "end": [590, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
