# `sqlparser::ast::dml::Update`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.Update.json).

<a id="op-a34f546464a5f6410e3ab25a"></a>
## Update

`struct` · `sqlparser::ast::dml::Update` · sqlparser 0.62.0

```rust
struct Update
```

Source: `src/ast/dml.rs:367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

UPDATE statement.

<a id="op-4c3d41775629b18c579ef53e"></a>
## assignments

`struct_field` · `sqlparser::ast::dml::Update::assignments` · sqlparser 0.62.0

```rust
assignments: Vec<super::Assignment>
```

Source: `src/ast/dml.rs:378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column assignments

<a id="op-2e386be6691a64b4a18d0344"></a>
## clone

`function` · `sqlparser::ast::dml::Update::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Update
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 17], "end": [364, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af4e73d03227b3b273ee1b5c"></a>
## cmp

`function` · `sqlparser::ast::dml::Update::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Update) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 51], "end": [364, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b373c0bb954a62914549821"></a>
## deserialize

`function` · `sqlparser::ast::dml::Update::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [365, 49], "end": [365, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e43b15fdfd7ff12d3e2ea026"></a>
## eq

`function` · `sqlparser::ast::dml::Update::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Update) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 24], "end": [364, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20ecbaf8ab5ae295a6992375"></a>
## fmt

`function` · `sqlparser::ast::dml::Update::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [451, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc56bc20f79fcbc40c90fdbe"></a>
## fmt

`function` · `sqlparser::ast::dml::Update::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 10], "end": [364, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7ffebd27f60de43c156ccd2"></a>
## from

`struct_field` · `sqlparser::ast::dml::Update::from` · sqlparser 0.62.0

```rust
from: Option<super::UpdateTableFromKind>
```

Source: `src/ast/dml.rs:380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table which provide value to be set

<a id="op-db581ff9c451f5cb86cf2385"></a>
## hash

`function` · `sqlparser::ast::dml::Update::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 56], "end": [364, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aa835ccd0abdddeaa40640b"></a>
## limit

`struct_field` · `sqlparser::ast::dml::Update::limit` · sqlparser 0.62.0

```rust
limit: Option<super::Expr>
```

Source: `src/ast/dml.rs:394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LIMIT

<a id="op-0565b5ace3064f1947b66892"></a>
## optimizer_hints

`struct_field` · `sqlparser::ast::dml::Update::optimizer_hints` · sqlparser 0.62.0

```rust
optimizer_hints: Vec<super::OptimizerHint>
```

Source: `src/ast/dml.rs:374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query optimizer hints

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/optimizer-hints.html)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Comments.html#GUID-D316D545-89E2-4D54-977F-FC97815CD62E)

<a id="op-6d34af1e49d4aaf66f3468c0"></a>
## or

`struct_field` · `sqlparser::ast::dml::Update::or` · sqlparser 0.62.0

```rust
or: Option<super::SqliteOnConflict>
```

Source: `src/ast/dml.rs:389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQLite-specific conflict resolution clause

<a id="op-72f623cb0f6a0ad53a23a39a"></a>
## order_by

`struct_field` · `sqlparser::ast::dml::Update::order_by` · sqlparser 0.62.0

```rust
order_by: Vec<super::OrderByExpr>
```

Source: `src/ast/dml.rs:392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ORDER BY (MySQL extension for single-table UPDATE)
See <https://dev.mysql.com/doc/refman/8.4/en/update.html>

<a id="op-c6d238ac7eaaf71b1b66546d"></a>
## output

`struct_field` · `sqlparser::ast::dml::Update::output` · sqlparser 0.62.0

```rust
output: Option<OutputClause>
```

Source: `src/ast/dml.rs:387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

OUTPUT (MSSQL)
See <https://learn.microsoft.com/en-us/sql/t-sql/queries/output-clause-transact-sql>

<a id="op-5c5870f5aa9ff75e8d628c65"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::Update::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Update) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [364, 35], "end": [364, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de07f0dc171841d0eebbd6e1"></a>
## returning

`struct_field` · `sqlparser::ast::dml::Update::returning` · sqlparser 0.62.0

```rust
returning: Option<Vec<super::SelectItem>>
```

Source: `src/ast/dml.rs:384`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RETURNING

<a id="op-ae61adf5251b5d0c8efe30c0"></a>
## selection

`struct_field` · `sqlparser::ast::dml::Update::selection` · sqlparser 0.62.0

```rust
selection: Option<super::Expr>
```

Source: `src/ast/dml.rs:382`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WHERE

<a id="op-6b5004ac954833b3ac8a865e"></a>
## serialize

`function` · `sqlparser::ast::dml::Update::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [365, 38], "end": [365, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b30c9d497554fd2167ecaec7"></a>
## span

`function` · `sqlparser::ast::dml::Update::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "super::Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [957, 1], "end": [985, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-973b7f440ad4692c7ac7c9da"></a>
## table

`struct_field` · `sqlparser::ast::dml::Update::table` · sqlparser 0.62.0

```rust
table: super::TableWithJoins
```

Source: `src/ast/dml.rs:376`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TABLE

<a id="op-a7cafd00ad621135c7bbf6ec"></a>
## update_token

`struct_field` · `sqlparser::ast::dml::Update::update_token` · sqlparser 0.62.0

```rust
update_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token for the `UPDATE` keyword

<a id="op-00f864c80cc169e6590d67de"></a>
## visit

`function` · `sqlparser::ast::dml::Update::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 40], "end": [366, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-855631f486122fe19a704bd0"></a>
## visit

`function` · `sqlparser::ast::dml::Update::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Update", "path": "Update"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 47], "end": [366, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
