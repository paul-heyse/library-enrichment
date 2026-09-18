# `sqlparser::ast::query::NamedWindowExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.NamedWindowExpr.json).

<a id="op-7511d57ee4d7d6071a441fd2"></a>
## NamedWindowExpr

`enum` · `sqlparser::ast::query::NamedWindowExpr` · sqlparser 0.62.0

```rust
enum NamedWindowExpr
```

Source: `src/ast/query.rs:703`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An expression used in a named window declaration.

```sql
WINDOW mywindow AS [named_window_expr]
```

<a id="op-2938d762406cf800c4d99d78"></a>
## NamedWindow

`variant` · `sqlparser::ast::query::NamedWindowExpr::NamedWindow` · sqlparser 0.62.0

```rust
NamedWindow
```

Source: `src/ast/query.rs:713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A direct reference to another named window definition.
[BigQuery]

Example:
```sql
WINDOW mywindow AS prev_window
```

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/window-function-calls#ref_named_window

<a id="op-3d3b84660300bb5030969e77"></a>
## WindowSpec

`variant` · `sqlparser::ast::query::NamedWindowExpr::WindowSpec` · sqlparser 0.62.0

```rust
WindowSpec
```

Source: `src/ast/query.rs:720`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A window expression.

Example:
```sql
WINDOW mywindow AS (ORDER BY 1)
```

<a id="op-009236457b57b3b64ba728a7"></a>
## clone

`function` · `sqlparser::ast::query::NamedWindowExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NamedWindowExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 17], "end": [700, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-398046a6302f7ed450f43374"></a>
## cmp

`function` · `sqlparser::ast::query::NamedWindowExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NamedWindowExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 51], "end": [700, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4bdec9fa2f6dac400708503"></a>
## deserialize

`function` · `sqlparser::ast::query::NamedWindowExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 49], "end": [701, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3da5228a986871703339e84"></a>
## eq

`function` · `sqlparser::ast::query::NamedWindowExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NamedWindowExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 24], "end": [700, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18b6ac4e86ca3d682d9c7180"></a>
## fmt

`function` · `sqlparser::ast::query::NamedWindowExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [723, 1], "end": [735, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80bfddd6d79d1b6d599028b8"></a>
## fmt

`function` · `sqlparser::ast::query::NamedWindowExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 10], "end": [700, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7847f8c2fb5c58028c44ccb0"></a>
## hash

`function` · `sqlparser::ast::query::NamedWindowExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 56], "end": [700, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e423d6a93fa6f58675be2fa1"></a>
## partial_cmp

`function` · `sqlparser::ast::query::NamedWindowExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NamedWindowExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [700, 35], "end": [700, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:700`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9ea8b8c4d5c2b9fdf1df09d"></a>
## serialize

`function` · `sqlparser::ast::query::NamedWindowExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [701, 38], "end": [701, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:701`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a06bfb8b77b4f11ce1c2d520"></a>
## visit

`function` · `sqlparser::ast::query::NamedWindowExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [702, 47], "end": [702, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbde9c75e5c7bb440c65a22e"></a>
## visit

`function` · `sqlparser::ast::query::NamedWindowExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowExpr", "path": "NamedWindowExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [702, 40], "end": [702, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:702`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
