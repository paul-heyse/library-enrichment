# `sqlparser::ast::dml::MultiTableInsertIntoClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MultiTableInsertIntoClause.json).

<a id="op-d6f653543ff45cf51d07e6f2"></a>
## MultiTableInsertIntoClause

`struct` · `sqlparser::ast::dml::MultiTableInsertIntoClause` · sqlparser 0.62.0

```rust
struct MultiTableInsertIntoClause
```

Source: `src/ast/dml.rs:842`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An INTO clause in a multi-table INSERT.

Syntax:
```sql
INTO <target_table> [ ( <target_col_name> [ , ... ] ) ] [ VALUES ( { <source_col_name> | DEFAULT | NULL } [ , ... ] ) ]
```

<a id="op-40767c229b7942674cf2822c"></a>
## clone

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MultiTableInsertIntoClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 17], "end": [839, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07959150611f024395e4974a"></a>
## cmp

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MultiTableInsertIntoClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 51], "end": [839, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc3e58c5090ac250b4416860"></a>
## columns

`struct_field` · `sqlparser::ast::dml::MultiTableInsertIntoClause::columns` · sqlparser 0.62.0

```rust
columns: Vec<super::Ident>
```

Source: `src/ast/dml.rs:846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The target columns (optional)

<a id="op-ec31c364fcd6f36c4ff3d027"></a>
## deserialize

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [840, 49], "end": [840, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-858168e9619a02e374dfb329"></a>
## eq

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MultiTableInsertIntoClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 24], "end": [839, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c40993904cc5bb0ea30ce01"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 10], "end": [839, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2d901a5d047c027316a5840"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 1], "end": [862, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9cda43b558b066e22eee9e4"></a>
## hash

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 56], "end": [839, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b5228893e6d0e8e75ed37dc"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MultiTableInsertIntoClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [839, 35], "end": [839, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3696d764e879f2f7b476d21"></a>
## serialize

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [840, 38], "end": [840, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61467b65f80dac4ac304bc4e"></a>
## table_name

`struct_field` · `sqlparser::ast::dml::MultiTableInsertIntoClause::table_name` · sqlparser 0.62.0

```rust
table_name: super::ObjectName
```

Source: `src/ast/dml.rs:844`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The target table

<a id="op-2f187738cf924096da7d68bc"></a>
## values

`struct_field` · `sqlparser::ast::dml::MultiTableInsertIntoClause::values` · sqlparser 0.62.0

```rust
values: Option<MultiTableInsertValues>
```

Source: `src/ast/dml.rs:848`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The VALUES clause (optional)

<a id="op-5f196fa4c7348ac9dfbf95e0"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 47], "end": [841, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7f20e7d16f75203e14bd8bc"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertIntoClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertIntoClause", "path": "MultiTableInsertIntoClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [841, 40], "end": [841, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:841`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
