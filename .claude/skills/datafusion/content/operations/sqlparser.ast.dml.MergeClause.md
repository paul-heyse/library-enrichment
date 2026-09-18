# `sqlparser::ast::dml::MergeClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MergeClause.json).

<a id="op-ece1e27435d8331e4dfcb1be"></a>
## MergeClause

`struct` · `sqlparser::ast::dml::MergeClause` · sqlparser 0.62.0

```rust
struct MergeClause
```

Source: `src/ast/dml.rs:513`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `WHEN` clause within a `MERGE` Statement

Example:
```sql
WHEN NOT MATCHED BY SOURCE AND product LIKE '%washer%' THEN DELETE
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)

<a id="op-2a45a49a192f6e200168bfcf"></a>
## action

`struct_field` · `sqlparser::ast::dml::MergeClause::action` · sqlparser 0.62.0

```rust
action: MergeAction
```

Source: `src/ast/dml.rs:521`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The action to perform when the clause is matched.

<a id="op-08d080d6df26b9e51999d7bd"></a>
## clause_kind

`struct_field` · `sqlparser::ast::dml::MergeClause::clause_kind` · sqlparser 0.62.0

```rust
clause_kind: MergeClauseKind
```

Source: `src/ast/dml.rs:517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of `WHEN` clause.

<a id="op-fc7b64e6ab58b5ef60896116"></a>
## clone

`function` · `sqlparser::ast::dml::MergeClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MergeClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 17], "end": [510, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8556a0562fe4ec089035e4a"></a>
## cmp

`function` · `sqlparser::ast::dml::MergeClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MergeClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 51], "end": [510, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26d14117831fe34656d47046"></a>
## deserialize

`function` · `sqlparser::ast::dml::MergeClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 49], "end": [511, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2855fcc69f314961ab14a861"></a>
## eq

`function` · `sqlparser::ast::dml::MergeClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MergeClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 24], "end": [510, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8b6cbcc54cf1de3a34a2310"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 10], "end": [510, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0a8ed908a79177585572442"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [524, 1], "end": [539, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-389fd715fa461784fb1b7e8f"></a>
## hash

`function` · `sqlparser::ast::dml::MergeClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 56], "end": [510, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8398e3663ad3754e011739f"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MergeClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MergeClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [510, 35], "end": [510, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ebed578b671c2988f8b5535"></a>
## predicate

`struct_field` · `sqlparser::ast::dml::MergeClause::predicate` · sqlparser 0.62.0

```rust
predicate: Option<super::Expr>
```

Source: `src/ast/dml.rs:519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An optional predicate to further restrict the clause.

<a id="op-3ed637e14198a40cac743465"></a>
## serialize

`function` · `sqlparser::ast::dml::MergeClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 38], "end": [511, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffaa4f6ebf8e0bddf61d7528"></a>
## span

`function` · `sqlparser::ast::dml::MergeClause::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "super::MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2513, 1], "end": [2517, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2514`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bbf61d406555015ff76ded7"></a>
## visit

`function` · `sqlparser::ast::dml::MergeClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [512, 40], "end": [512, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-801f2ba1c79e657f8a9adee3"></a>
## visit

`function` · `sqlparser::ast::dml::MergeClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeClause", "path": "MergeClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [512, 47], "end": [512, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7cef4c9c8931380545c0fd1"></a>
## when_token

`struct_field` · `sqlparser::ast::dml::MergeClause::when_token` · sqlparser 0.62.0

```rust
when_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:515`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `WHEN` token that starts the sub-expression.
