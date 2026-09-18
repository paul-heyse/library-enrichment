# `sqlparser::ast::ddl::CreateIndex`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateIndex.json).

<a id="op-5f775865a93018da6eacdce5"></a>
## CreateIndex

`struct` · `sqlparser::ast::ddl::CreateIndex` · sqlparser 0.62.0

```rust
struct CreateIndex
```

Source: `src/ast/ddl.rs:2806`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE INDEX statement.

<a id="op-23ede4f1218e045f86196d50"></a>
## alter_options

`struct_field` · `sqlparser::ast::ddl::CreateIndex::alter_options` · sqlparser 0.62.0

```rust
alter_options: Vec<AlterTableOperation>
```

Source: `src/ast/ddl.rs:2839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] allows a subset of options normally used for `ALTER TABLE`:

- `ALGORITHM`
- `LOCK`

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/create-index.html

<a id="op-4df4d81107cc9fdfca4d3cf5"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateIndex::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2803, 17], "end": [2803, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73da384c8dfce07494422d63"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateIndex::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateIndex) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2803, 51], "end": [2803, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fdb11d44de2f159e2c51fd"></a>
## columns

`struct_field` · `sqlparser::ast::ddl::CreateIndex::columns` · sqlparser 0.62.0

```rust
columns: Vec<IndexColumn>
```

Source: `src/ast/ddl.rs:2816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

columns included in the index

<a id="op-901682010c1adb6e46c3c945"></a>
## concurrently

`struct_field` · `sqlparser::ast::ddl::CreateIndex::concurrently` · sqlparser 0.62.0

```rust
concurrently: bool
```

Source: `src/ast/ddl.rs:2820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

whether the index is created concurrently

<a id="op-e5543d029e5d85c9dff2c46c"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateIndex::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2804, 49], "end": [2804, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1effcbd70fc25cc96ca998f"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateIndex::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2803, 24], "end": [2803, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bf47dd6264d882227514c8e"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateIndex::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2803, 10], "end": [2803, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff2d43431d0fea4f346b8565"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateIndex::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2842, 1], "end": [2891, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2843`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d56a3603992b6e0f11da0246"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateIndex::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2803, 56], "end": [2803, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f08bb8992a2036f2bb082f5"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateIndex::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:2822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

IF NOT EXISTS clause

<a id="op-0d28742ae34b647ab83332aa"></a>
## include

`struct_field` · `sqlparser::ast::ddl::CreateIndex::include` · sqlparser 0.62.0

```rust
include: Vec<ast::Ident>
```

Source: `src/ast/ddl.rs:2824`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

INCLUDE clause: <https://www.postgresql.org/docs/current/sql-createindex.html>

<a id="op-fcdbdf375f6767346422af75"></a>
## index_options

`struct_field` · `sqlparser::ast::ddl::CreateIndex::index_options` · sqlparser 0.62.0

```rust
index_options: Vec<IndexOption>
```

Source: `src/ast/ddl.rs:2832`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index options: <https://www.postgresql.org/docs/current/sql-createindex.html>

<a id="op-6cb75a7bb0fdcb5f2eb21f92"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateIndex::name` · sqlparser 0.62.0

```rust
name: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:2808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

index name

<a id="op-fe358e3c6ede5ae7dccfe058"></a>
## nulls_distinct

`struct_field` · `sqlparser::ast::ddl::CreateIndex::nulls_distinct` · sqlparser 0.62.0

```rust
nulls_distinct: Option<bool>
```

Source: `src/ast/ddl.rs:2826`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

NULLS DISTINCT / NOT DISTINCT clause: <https://www.postgresql.org/docs/current/sql-createindex.html>

<a id="op-2b479511ea97070cd0cc5b81"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateIndex::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateIndex) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2803, 35], "end": [2803, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a610294376becdd3bb30754"></a>
## predicate

`struct_field` · `sqlparser::ast::ddl::CreateIndex::predicate` · sqlparser 0.62.0

```rust
predicate: Option<ast::Expr>
```

Source: `src/ast/ddl.rs:2830`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WHERE clause: <https://www.postgresql.org/docs/current/sql-createindex.html>

<a id="op-20839b0350f803bda56354f7"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateIndex::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2804, 38], "end": [2804, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2804`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d6647b6065cf9dfd70e4134"></a>
## span

`function` · `sqlparser::ast::ddl::CreateIndex::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "super::CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [684, 1], "end": [713, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:685`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59cfde8793b4512b8505d893"></a>
## table_name

`struct_field` · `sqlparser::ast::ddl::CreateIndex::table_name` · sqlparser 0.62.0

```rust
table_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:2811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

table name

<a id="op-779fc31c5fd116a6d3c38ba6"></a>
## unique

`struct_field` · `sqlparser::ast::ddl::CreateIndex::unique` · sqlparser 0.62.0

```rust
unique: bool
```

Source: `src/ast/ddl.rs:2818`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

whether the index is unique

<a id="op-dd69e61931c6cce8bdd397c6"></a>
## using

`struct_field` · `sqlparser::ast::ddl::CreateIndex::using` · sqlparser 0.62.0

```rust
using: Option<IndexType>
```

Source: `src/ast/ddl.rs:2814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index type used in the statement. Can also be found inside [`CreateIndex::index_options`](../operations/sqlparser.ast.ddl.CreateIndex.md#op-fcdbdf375f6767346422af75)
depending on the position of the option within the statement.

<a id="op-c437c0bb4763dffd275e8945"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateIndex::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2805, 47], "end": [2805, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e23b1f34f3ccbfd0f92d75cd"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateIndex::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2805, 40], "end": [2805, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d15ae3684a0456f86b2ace6"></a>
## with

`struct_field` · `sqlparser::ast::ddl::CreateIndex::with` · sqlparser 0.62.0

```rust
with: Vec<ast::Expr>
```

Source: `src/ast/ddl.rs:2828`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WITH clause: <https://www.postgresql.org/docs/current/sql-createindex.html>
