# `sqlparser::ast::query::SelectItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SelectItem.json).

<a id="op-345b8b94c18591ea120ebb9c"></a>
## SelectItem

`enum` · `sqlparser::ast::query::SelectItem` · sqlparser 0.62.0

```rust
enum SelectItem
```

Source: `src/ast/query.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One item of the comma-separated list following `SELECT`

<a id="op-08d5a98ecd0be443e89d8567"></a>
## ExprWithAlias

`variant` · `sqlparser::ast::query::SelectItem::ExprWithAlias` · sqlparser 0.62.0

```rust
ExprWithAlias
```

Source: `src/ast/query.rs:869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression, followed by `[ AS ] alias`

<a id="op-5cb20138adfb4c72263dc624"></a>
## ExprWithAliases

`variant` · `sqlparser::ast::query::SelectItem::ExprWithAliases` · sqlparser 0.62.0

```rust
ExprWithAliases
```

Source: `src/ast/query.rs:878`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression, followed by `[ AS ] (alias1, alias2, ...)`

[Spark SQL](https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select.html)

<a id="op-22c21f8e2ba865add5a9dd01"></a>
## QualifiedWildcard

`variant` · `sqlparser::ast::query::SelectItem::QualifiedWildcard` · sqlparser 0.62.0

```rust
QualifiedWildcard
```

Source: `src/ast/query.rs:886`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression, followed by a wildcard expansion.
e.g. `alias.*`, `STRUCT<STRING>('foo').*`

<a id="op-d0015a419b404526403b73a5"></a>
## UnnamedExpr

`variant` · `sqlparser::ast::query::SelectItem::UnnamedExpr` · sqlparser 0.62.0

```rust
UnnamedExpr
```

Source: `src/ast/query.rs:867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Any expression, not followed by `[ AS ] alias`

<a id="op-8536db4509ae4f3faeb65477"></a>
## Wildcard

`variant` · `sqlparser::ast::query::SelectItem::Wildcard` · sqlparser 0.62.0

```rust
Wildcard
```

Source: `src/ast/query.rs:888`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unqualified `*`

<a id="op-46aa4e61059f2e4c1b32083b"></a>
## clone

`function` · `sqlparser::ast::query::SelectItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SelectItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [862, 17], "end": [862, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37913540a847ca4d00a81823"></a>
## cmp

`function` · `sqlparser::ast::query::SelectItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SelectItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [862, 51], "end": [862, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d02b62f00bf5406383f7b27d"></a>
## deserialize

`function` · `sqlparser::ast::query::SelectItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [863, 49], "end": [863, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f8044080df02c71565a0eac"></a>
## eq

`function` · `sqlparser::ast::query::SelectItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SelectItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [862, 24], "end": [862, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58614d62a935ecd572116c93"></a>
## fmt

`function` · `sqlparser::ast::query::SelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1177, 1], "end": [1203, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e397245111ab3f84a0197e3a"></a>
## fmt

`function` · `sqlparser::ast::query::SelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [862, 10], "end": [862, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67eceb8e45a9eedd54f10708"></a>
## hash

`function` · `sqlparser::ast::query::SelectItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [862, 56], "end": [862, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-251a4a63778382d72bac6e7d"></a>
## partial_cmp

`function` · `sqlparser::ast::query::SelectItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SelectItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [862, 35], "end": [862, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b7cae645d24134a85789293"></a>
## serialize

`function` · `sqlparser::ast::query::SelectItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [863, 38], "end": [863, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c41b2b160b84296ed5eeaf98"></a>
## span

`function` · `sqlparser::ast::query::SelectItem::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "super::SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1835, 1], "end": [1851, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fdb4aa3deed7168e6232cb2"></a>
## visit

`function` · `sqlparser::ast::query::SelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 40], "end": [864, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b100ebc6ff13345967ffd8a"></a>
## visit

`function` · `sqlparser::ast::query::SelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::SelectItem", "path": "SelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [864, 47], "end": [864, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
