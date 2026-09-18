# `sqlparser::ast::dml::MergeUpdateExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MergeUpdateExpr.json).

<a id="op-79efd6b83672087896347607"></a>
## MergeUpdateExpr

`struct` · `sqlparser::ast::dml::MergeUpdateExpr` · sqlparser 0.62.0

```rust
struct MergeUpdateExpr
```

Source: `src/ast/dml.rs:726`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression used to update rows within a `MERGE` statement.

Examples
```sql
UPDATE SET quantity = T.quantity + S.quantity
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/MERGE.html)

<a id="op-01a90e1e1eeba28afca23c94"></a>
## assignments

`struct_field` · `sqlparser::ast::dml::MergeUpdateExpr::assignments` · sqlparser 0.62.0

```rust
assignments: Vec<super::Assignment>
```

Source: `src/ast/dml.rs:730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The update assiment expressions

<a id="op-fddd76de24893d2a9b65f880"></a>
## clone

`function` · `sqlparser::ast::dml::MergeUpdateExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MergeUpdateExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 17], "end": [723, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2f75378852664b50d3da9c2"></a>
## cmp

`function` · `sqlparser::ast::dml::MergeUpdateExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MergeUpdateExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 51], "end": [723, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73c210bb894cc7fd63e30f43"></a>
## delete_predicate

`struct_field` · `sqlparser::ast::dml::MergeUpdateExpr::delete_predicate` · sqlparser 0.62.0

```rust
delete_predicate: Option<super::Expr>
```

Source: `src/ast/dml.rs:734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`delete_clause` for the update "delete where" (Oracle specific)

<a id="op-8f1092cd4fcd3317efe3a88c"></a>
## deserialize

`function` · `sqlparser::ast::dml::MergeUpdateExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [724, 49], "end": [724, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d58d58ea5e0e57d99eb8b9b5"></a>
## eq

`function` · `sqlparser::ast::dml::MergeUpdateExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MergeUpdateExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 24], "end": [723, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cf786ee5eb3da3a6fa33a59"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeUpdateExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 1], "end": [748, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be8b4f545615035fcefebec6"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeUpdateExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 10], "end": [723, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbd80d4011a50675fd7855de"></a>
## hash

`function` · `sqlparser::ast::dml::MergeUpdateExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 56], "end": [723, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf6e10f8deb2d894c02b4d4a"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MergeUpdateExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MergeUpdateExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 35], "end": [723, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c435cdffc4698f6968ff33c"></a>
## serialize

`function` · `sqlparser::ast::dml::MergeUpdateExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [724, 38], "end": [724, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-718aa4791b639b03b2c34279"></a>
## span

`function` · `sqlparser::ast::dml::MergeUpdateExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "super::MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2547, 1], "end": [2556, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0f001a4972dd490944c2a0d"></a>
## update_predicate

`struct_field` · `sqlparser::ast::dml::MergeUpdateExpr::update_predicate` · sqlparser 0.62.0

```rust
update_predicate: Option<super::Expr>
```

Source: `src/ast/dml.rs:732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`where_clause` for the update (Oralce specific)

<a id="op-dac1224bb30ce020c4e445bc"></a>
## update_token

`struct_field` · `sqlparser::ast::dml::MergeUpdateExpr::update_token` · sqlparser 0.62.0

```rust
update_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:728`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `UPDATE` token that starts the sub-expression.

<a id="op-729ff6b4fe3796b8d09ebe7e"></a>
## visit

`function` · `sqlparser::ast::dml::MergeUpdateExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 47], "end": [725, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8827e51920db5aa67486b70"></a>
## visit

`function` · `sqlparser::ast::dml::MergeUpdateExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeUpdateExpr", "path": "MergeUpdateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [725, 40], "end": [725, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
