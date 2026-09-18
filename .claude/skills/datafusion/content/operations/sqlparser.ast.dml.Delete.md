# `sqlparser::ast::dml::Delete`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.Delete.json).

<a id="op-6cf2cb175020874af7046e97"></a>
## Delete

`struct` · `sqlparser::ast::dml::Delete` · sqlparser 0.62.0

```rust
struct Delete
```

Source: `src/ast/dml.rs:282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DELETE statement.

<a id="op-33838df0e6b18c5f7c791863"></a>
## clone

`function` · `sqlparser::ast::dml::Delete::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Delete
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 17], "end": [279, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91c269b36382c386e0df59b7"></a>
## cmp

`function` · `sqlparser::ast::dml::Delete::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Delete) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 51], "end": [279, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde2ccbe179cae072b15f503"></a>
## delete_token

`struct_field` · `sqlparser::ast::dml::Delete::delete_token` · sqlparser 0.62.0

```rust
delete_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token for the `DELETE` keyword

<a id="op-c35a08147c7667c530520d8e"></a>
## deserialize

`function` · `sqlparser::ast::dml::Delete::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 49], "end": [280, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1222bfe43b8ee360cbbad0ff"></a>
## eq

`function` · `sqlparser::ast::dml::Delete::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Delete) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 24], "end": [279, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6237c3cd7aae1378a3d4e72c"></a>
## fmt

`function` · `sqlparser::ast::dml::Delete::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 10], "end": [279, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdec945e169ac687c4d2369b"></a>
## fmt

`function` · `sqlparser::ast::dml::Delete::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [361, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0b565d69fa61306f169c58"></a>
## from

`struct_field` · `sqlparser::ast::dml::Delete::from` · sqlparser 0.62.0

```rust
from: super::FromTable
```

Source: `src/ast/dml.rs:293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FROM

<a id="op-e3a5119495a6cf6ab7afe3d5"></a>
## hash

`function` · `sqlparser::ast::dml::Delete::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 56], "end": [279, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ea1543940239902bf22ed96"></a>
## limit

`struct_field` · `sqlparser::ast::dml::Delete::limit` · sqlparser 0.62.0

```rust
limit: Option<super::Expr>
```

Source: `src/ast/dml.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LIMIT (MySQL)

<a id="op-6d888cd77770d56daae0ccc3"></a>
## optimizer_hints

`struct_field` · `sqlparser::ast::dml::Delete::optimizer_hints` · sqlparser 0.62.0

```rust
optimizer_hints: Vec<super::OptimizerHint>
```

Source: `src/ast/dml.rs:289`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query optimizer hints

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/optimizer-hints.html)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Comments.html#GUID-D316D545-89E2-4D54-977F-FC97815CD62E)

<a id="op-08efe869ed56b207f3494f20"></a>
## order_by

`struct_field` · `sqlparser::ast::dml::Delete::order_by` · sqlparser 0.62.0

```rust
order_by: Vec<super::OrderByExpr>
```

Source: `src/ast/dml.rs:304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ORDER BY (MySQL)

<a id="op-6dfb056f6685d4fe73c28314"></a>
## output

`struct_field` · `sqlparser::ast::dml::Delete::output` · sqlparser 0.62.0

```rust
output: Option<OutputClause>
```

Source: `src/ast/dml.rs:302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

OUTPUT (MSSQL)
See <https://learn.microsoft.com/en-us/sql/t-sql/queries/output-clause-transact-sql>

<a id="op-411c3842268d1ae0de87611f"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::Delete::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Delete) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 35], "end": [279, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:279`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c76858a947e22a7670f7d68"></a>
## returning

`struct_field` · `sqlparser::ast::dml::Delete::returning` · sqlparser 0.62.0

```rust
returning: Option<Vec<super::SelectItem>>
```

Source: `src/ast/dml.rs:299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

RETURNING

<a id="op-0ab69948698334fdfdfb93a8"></a>
## selection

`struct_field` · `sqlparser::ast::dml::Delete::selection` · sqlparser 0.62.0

```rust
selection: Option<super::Expr>
```

Source: `src/ast/dml.rs:297`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WHERE

<a id="op-cb0c56c93b7a5db08c3e6df5"></a>
## serialize

`function` · `sqlparser::ast::dml::Delete::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [280, 38], "end": [280, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:280`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d87f1102e6e65492fd589c6e"></a>
## span

`function` · `sqlparser::ast::dml::Delete::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "super::Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [921, 1], "end": [955, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:922`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4249e565cdf975a415c1ed61"></a>
## tables

`struct_field` · `sqlparser::ast::dml::Delete::tables` · sqlparser 0.62.0

```rust
tables: Vec<super::ObjectName>
```

Source: `src/ast/dml.rs:291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multi tables delete are supported in mysql

<a id="op-96103183ddb820b956a0a7ce"></a>
## using

`struct_field` · `sqlparser::ast::dml::Delete::using` · sqlparser 0.62.0

```rust
using: Option<Vec<super::TableWithJoins>>
```

Source: `src/ast/dml.rs:295`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

USING (Snowflake, Postgres, MySQL)

<a id="op-521211f9d7fa531405e7b11b"></a>
## visit

`function` · `sqlparser::ast::dml::Delete::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 40], "end": [281, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:281`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eeade37eb00b8850d9675d06"></a>
## visit

`function` · `sqlparser::ast::dml::Delete::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::Delete", "path": "Delete"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 47], "end": [281, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:281`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
