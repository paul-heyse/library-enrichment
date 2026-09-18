# `sqlparser::ast::query::SelectItemQualifiedWildcardKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SelectItemQualifiedWildcardKind.json).

<a id="op-f6d41e285bc8e0e69c15f885"></a>
## SelectItemQualifiedWildcardKind

`enum` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind` · sqlparser 0.62.0

```rust
enum SelectItemQualifiedWildcardKind
```

Source: `src/ast/query.rs:852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an expression behind a wildcard expansion in a projection.
`SELECT T.* FROM T;

<a id="op-802de7f2f2090384cf5e3d2f"></a>
## Expr

`variant` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/query.rs:858`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Select star on an arbitrary expression.
e.g. `STRUCT<STRING>('foo').*`

<a id="op-703ea3d82403db1df76ebf2b"></a>
## ObjectName

`variant` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::ObjectName` · sqlparser 0.62.0

```rust
ObjectName
```

Source: `src/ast/query.rs:855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression is an object name.
e.g. `alias.*` or even `schema.table.*`

<a id="op-0c4c47af8b3d5cd875e24635"></a>
## clone

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SelectItemQualifiedWildcardKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 17], "end": [849, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99fe7563791f3610c9f5672f"></a>
## cmp

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SelectItemQualifiedWildcardKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 51], "end": [849, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867ee86f544b67105778533e"></a>
## deserialize

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [850, 49], "end": [850, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:850`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a388967d406ae318ecf45960"></a>
## eq

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SelectItemQualifiedWildcardKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 24], "end": [849, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c5e53282af5e79dc9a13422"></a>
## fmt

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [900, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5480acfa6cae3a8aa401723"></a>
## fmt

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 10], "end": [849, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-038c13d6b2254b9053729551"></a>
## hash

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 56], "end": [849, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08f51ec39a0c4591ddf656bd"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SelectItemQualifiedWildcardKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [849, 35], "end": [849, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e02929cafa56517265295899"></a>
## serialize

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [850, 38], "end": [850, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:850`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fc43d03ac74b4dfe456e1dc"></a>
## span

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "crate::ast::query::SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1826, 1], "end": [1833, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6e111e83b653ba4903a0472"></a>
## visit

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 47], "end": [851, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dce06a8e07d20cd6b66fdf42"></a>
## visit

`function` · `sqlparser::ast::query::SelectItemQualifiedWildcardKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItemQualifiedWildcardKind", "path": "SelectItemQualifiedWildcardKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [851, 40], "end": [851, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
