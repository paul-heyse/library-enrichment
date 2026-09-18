# `sqlparser::ast::dml::MergeInsertExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MergeInsertExpr.json).

<a id="op-1fd0ef51278406acee0e8322"></a>
## MergeInsertExpr

`struct` · `sqlparser::ast::dml::MergeInsertExpr` · sqlparser 0.62.0

```rust
struct MergeInsertExpr
```

Source: `src/ast/dml.rs:681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression used to insert rows within a `MERGE` statement.

Examples
```sql
INSERT (product, quantity) VALUES(product, quantity)
INSERT ROW
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/MERGE.html)

<a id="op-58ceb04cfce4218ec277ac9a"></a>
## clone

`function` · `sqlparser::ast::dml::MergeInsertExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MergeInsertExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [678, 17], "end": [678, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c42e1c148c91a3a38a26f538"></a>
## cmp

`function` · `sqlparser::ast::dml::MergeInsertExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MergeInsertExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [678, 51], "end": [678, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0682f3110b98101f5ea9e456"></a>
## columns

`struct_field` · `sqlparser::ast::dml::MergeInsertExpr::columns` · sqlparser 0.62.0

```rust
columns: Vec<super::ObjectName>
```

Source: `src/ast/dml.rs:691`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns (if any) specified by the insert.

Example:
```sql
INSERT (product, quantity) VALUES(product, quantity)
INSERT (product, quantity) ROW
```

<a id="op-06878e6f42528baf07c55c88"></a>
## deserialize

`function` · `sqlparser::ast::dml::MergeInsertExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 49], "end": [679, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd49802ebfa42a499107adef"></a>
## eq

`function` · `sqlparser::ast::dml::MergeInsertExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MergeInsertExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [678, 24], "end": [678, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72dd0e17d7cf8b75e4c1de16"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeInsertExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 1], "end": [711, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a787e0ff93103e99c6538688"></a>
## fmt

`function` · `sqlparser::ast::dml::MergeInsertExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [678, 10], "end": [678, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5e9b3b4d0c319f75999f93c"></a>
## hash

`function` · `sqlparser::ast::dml::MergeInsertExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [678, 56], "end": [678, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1173aff2af68fd5c911f7e12"></a>
## insert_predicate

`struct_field` · `sqlparser::ast::dml::MergeInsertExpr::insert_predicate` · sqlparser 0.62.0

```rust
insert_predicate: Option<super::Expr>
```

Source: `src/ast/dml.rs:697`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An optional condition to restrict the insertion (Oracle specific)

<a id="op-e3e9ce12b8ba63a7bf5b0008"></a>
## insert_token

`struct_field` · `sqlparser::ast::dml::MergeInsertExpr::insert_token` · sqlparser 0.62.0

```rust
insert_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `INSERT` token that starts the sub-expression.

<a id="op-b25b40bf85d14b3ec030c01c"></a>
## kind

`struct_field` · `sqlparser::ast::dml::MergeInsertExpr::kind` · sqlparser 0.62.0

```rust
kind: MergeInsertKind
```

Source: `src/ast/dml.rs:695`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The insert type used by the statement.

<a id="op-0c7a54f5a069880992b1b06f"></a>
## kind_token

`struct_field` · `sqlparser::ast::dml::MergeInsertExpr::kind_token` · sqlparser 0.62.0

```rust
kind_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:693`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The token, `[VALUES | ROW]` starting `kind`.

<a id="op-5a12ba1685266fb76ec99645"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MergeInsertExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MergeInsertExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [678, 35], "end": [678, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:678`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5415af28d4245b37a2c46a92"></a>
## serialize

`function` · `sqlparser::ast::dml::MergeInsertExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [679, 38], "end": [679, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c3aa7ec025d309ce2c3c996"></a>
## span

`function` · `sqlparser::ast::dml::MergeInsertExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "super::MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2529, 1], "end": [2545, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bf2a6db2e44d24d5ce65343"></a>
## visit

`function` · `sqlparser::ast::dml::MergeInsertExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 47], "end": [680, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9521f2a00fabf2fc9bb5943"></a>
## visit

`function` · `sqlparser::ast::dml::MergeInsertExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MergeInsertExpr", "path": "MergeInsertExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 40], "end": [680, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:680`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
