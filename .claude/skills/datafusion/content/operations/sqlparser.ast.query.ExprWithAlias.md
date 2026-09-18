# `sqlparser::ast::query::ExprWithAlias`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.ExprWithAlias.json).

<a id="op-6ce26adcdcea6e1157831034"></a>
## ExprWithAlias

`struct` · `sqlparser::ast::query::ExprWithAlias` · sqlparser 0.62.0

```rust
struct ExprWithAlias
```

Source: `src/ast/query.rs:1313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression optionally followed by an alias.

Example:
```sql
42 AS myint
```

<a id="op-fba7f8c2d283c067484b8233"></a>
## alias

`struct_field` · `sqlparser::ast::query::ExprWithAlias::alias` · sqlparser 0.62.0

```rust
alias: Option<Ident>
```

Source: `src/ast/query.rs:1317`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the expression.

<a id="op-42559d8fc68c6d8f431397cc"></a>
## clone

`function` · `sqlparser::ast::query::ExprWithAlias::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExprWithAlias
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1310, 17], "end": [1310, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8da84d49611f9aef3bf88a2"></a>
## cmp

`function` · `sqlparser::ast::query::ExprWithAlias::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExprWithAlias) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1310, 51], "end": [1310, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98d2e7fcdb9843553688654d"></a>
## deserialize

`function` · `sqlparser::ast::query::ExprWithAlias::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1311, 49], "end": [1311, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf7d4619e35e81be129c5571"></a>
## eq

`function` · `sqlparser::ast::query::ExprWithAlias::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExprWithAlias) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1310, 24], "end": [1310, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86ee57709aa320eae8db71e7"></a>
## expr

`struct_field` · `sqlparser::ast::query::ExprWithAlias::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:1315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression.

<a id="op-961e160db02a3ef0a88510a6"></a>
## fmt

`function` · `sqlparser::ast::query::ExprWithAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1320, 1], "end": [1329, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f436a94dc5142d86a608e1e"></a>
## fmt

`function` · `sqlparser::ast::query::ExprWithAlias::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1310, 10], "end": [1310, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ec1c0ed687dcbb080885966"></a>
## hash

`function` · `sqlparser::ast::query::ExprWithAlias::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1310, 56], "end": [1310, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-645d0311d1ee6092280d496c"></a>
## partial_cmp

`function` · `sqlparser::ast::query::ExprWithAlias::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExprWithAlias) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1310, 35], "end": [1310, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3f55876dcdfe0d60d75edec"></a>
## serialize

`function` · `sqlparser::ast::query::ExprWithAlias::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1311, 38], "end": [1311, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-119e4f6d75c3933deb6330c6"></a>
## span

`function` · `sqlparser::ast::query::ExprWithAlias::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "super::ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2091, 1], "end": [2097, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-702c38a22ffc09c42c319ded"></a>
## visit

`function` · `sqlparser::ast::query::ExprWithAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 40], "end": [1312, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9089c01574939b04d275446"></a>
## visit

`function` · `sqlparser::ast::query::ExprWithAlias::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::ExprWithAlias", "path": "ExprWithAlias"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1312, 47], "end": [1312, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
