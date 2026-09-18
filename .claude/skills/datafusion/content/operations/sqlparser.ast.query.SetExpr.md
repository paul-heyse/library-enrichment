# `sqlparser::ast::query::SetExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SetExpr.json).

<a id="op-4350846adc4be3d79335e2ae"></a>
## SetExpr

`enum` · `sqlparser::ast::query::SetExpr` · sqlparser 0.62.0

```rust
enum SetExpr
```

Source: `src/ast/query.rs:150`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A node in a tree, representing a "query body" expression, roughly:
`SELECT ... [ {UNION|EXCEPT|INTERSECT} SELECT ...]`

<a id="op-9dc7d0527dd9857e4e539909"></a>
## Delete

`variant` · `sqlparser::ast::query::SetExpr::Delete` · sqlparser 0.62.0

```rust
Delete
```

Source: `src/ast/query.rs:175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DELETE` statement

<a id="op-9ed3c00006fb8783957757ab"></a>
## Insert

`variant` · `sqlparser::ast::query::SetExpr::Insert` · sqlparser 0.62.0

```rust
Insert
```

Source: `src/ast/query.rs:171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INSERT` statement

<a id="op-a3f7e2bb2d92afcdaa2db3f6"></a>
## Merge

`variant` · `sqlparser::ast::query::SetExpr::Merge` · sqlparser 0.62.0

```rust
Merge
```

Source: `src/ast/query.rs:177`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MERGE` statement

<a id="op-fea5a0b9a0e1039a781a059b"></a>
## Query

`variant` · `sqlparser::ast::query::SetExpr::Query` · sqlparser 0.62.0

```rust
Query
```

Source: `src/ast/query.rs:155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parenthesized SELECT subquery, which may include more set operations
in its body and an optional ORDER BY / LIMIT.

<a id="op-0be52453eb8e695c3d810071"></a>
## Select

`variant` · `sqlparser::ast::query::SetExpr::Select` · sqlparser 0.62.0

```rust
Select
```

Source: `src/ast/query.rs:152`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Restricted SELECT .. FROM .. HAVING (no ORDER BY or set operations)

<a id="op-9c0262b080685a0703f8f9a2"></a>
## SetOperation

`variant` · `sqlparser::ast::query::SetExpr::SetOperation` · sqlparser 0.62.0

```rust
SetOperation
```

Source: `src/ast/query.rs:158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

UNION/EXCEPT/INTERSECT of two queries
A set operation combining two query expressions.

<a id="op-db3ff1abf520433b013d70ce"></a>
## Table

`variant` · `sqlparser::ast::query::SetExpr::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/query.rs:179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TABLE` command

<a id="op-8ab51b9e75ece84bf18a5c51"></a>
## Update

`variant` · `sqlparser::ast::query::SetExpr::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/query.rs:173`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`UPDATE` statement

<a id="op-b76dbb825323492c10d5dd87"></a>
## Values

`variant` · `sqlparser::ast::query::SetExpr::Values` · sqlparser 0.62.0

```rust
Values
```

Source: `src/ast/query.rs:169`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VALUES (...)`

<a id="op-f00e7c7dedb75229ab3b8764"></a>
## as_select

`function` · `sqlparser::ast::query::SetExpr::as_select` · sqlparser 0.62.0

```rust
fn as_select(&self) -> Option<&Select>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [191, 2], "filename": "src/ast/query.rs"}, "trait": null, "trait_path": null}`

Source: `src/ast/query.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If this `SetExpr` is a `SELECT`, returns the [`Select`](../operations/sqlparser.ast.query.Select.md#op-10b575bc5d4cbdd9ae434f0d).

<a id="op-301b8db29df8e3c219c5f900"></a>
## clone

`function` · `sqlparser::ast::query::SetExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 17], "end": [147, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-857f0fc99566dc27dedecd42"></a>
## cmp

`function` · `sqlparser::ast::query::SetExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 51], "end": [147, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aa458eb1c9af213db0d114a"></a>
## deserialize

`function` · `sqlparser::ast::query::SetExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 49], "end": [148, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49b87fbf641456449da3cd78"></a>
## eq

`function` · `sqlparser::ast::query::SetExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 24], "end": [147, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1062c1875c973b5cb67987f4"></a>
## fmt

`function` · `sqlparser::ast::query::SetExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [193, 1], "end": [234, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:194`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2c2b84c7c59dffdabd155c6"></a>
## fmt

`function` · `sqlparser::ast::query::SetExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 10], "end": [147, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12c6ba08bef39f9c068a8955"></a>
## hash

`function` · `sqlparser::ast::query::SetExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 56], "end": [147, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5461c3f00f89e88737f7aa7"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SetExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [147, 35], "end": [147, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92dcf2f15ecbd730c804c7f2"></a>
## serialize

`function` · `sqlparser::ast::query::SetExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 38], "end": [148, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad5cc7eb717adcc2404916dd"></a>
## span

`function` · `sqlparser::ast::query::SetExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "super::SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [219, 1], "end": [238, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:220`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ef3bb5de500790c86ce1988"></a>
## visit

`function` · `sqlparser::ast::query::SetExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 47], "end": [149, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd80df16f5e3ea6a03468cf0"></a>
## visit

`function` · `sqlparser::ast::query::SetExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SetExpr", "path": "SetExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 40], "end": [149, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:149`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
