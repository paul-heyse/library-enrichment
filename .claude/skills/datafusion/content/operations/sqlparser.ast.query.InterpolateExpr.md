# `sqlparser::ast::query::InterpolateExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.InterpolateExpr.json).

<a id="op-9a7a383651546cf4ceccd90a"></a>
## InterpolateExpr

`struct` · `sqlparser::ast::query::InterpolateExpr` · sqlparser 0.62.0

```rust
struct InterpolateExpr
```

Source: `src/ast/query.rs:2998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse `INTERPOLATE` clause for use in `ORDER BY` clause when using `WITH FILL` modifier.
Supported by [ClickHouse syntax]

[ClickHouse syntax]: <https://clickhouse.com/docs/en/sql-reference/statements/select/order-by#order-by-expr-with-fill-modifier>
An expression used by `WITH FILL`/`INTERPOLATE` to specify interpolation for a column.

<a id="op-daa44311c2f5300202c1ec9b"></a>
## clone

`function` · `sqlparser::ast::query::InterpolateExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> InterpolateExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2994, 17], "end": [2994, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-158e74da63ea54409dcb94f5"></a>
## cmp

`function` · `sqlparser::ast::query::InterpolateExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &InterpolateExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2994, 51], "end": [2994, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e34f348ae934123e3c19be4a"></a>
## column

`struct_field` · `sqlparser::ast::query::InterpolateExpr::column` · sqlparser 0.62.0

```rust
column: Ident
```

Source: `src/ast/query.rs:3000`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The column to interpolate.

<a id="op-1c85348f000767a11263520e"></a>
## deserialize

`function` · `sqlparser::ast::query::InterpolateExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2995, 49], "end": [2995, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-524670560fc0306cd8b63e37"></a>
## eq

`function` · `sqlparser::ast::query::InterpolateExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &InterpolateExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2994, 24], "end": [2994, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0db472574c7d6b3090a635a5"></a>
## expr

`struct_field` · `sqlparser::ast::query::InterpolateExpr::expr` · sqlparser 0.62.0

```rust
expr: Option<Expr>
```

Source: `src/ast/query.rs:3002`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `AS <expr>` expression specifying how to compute interpolated values.

<a id="op-bd77e64987bbb565fb3d04a3"></a>
## fmt

`function` · `sqlparser::ast::query::InterpolateExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2994, 10], "end": [2994, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c180a0a41cf966c4b70a2d3d"></a>
## fmt

`function` · `sqlparser::ast::query::InterpolateExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3014, 1], "end": [3022, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ced1d1af811e5adf68922db"></a>
## hash

`function` · `sqlparser::ast::query::InterpolateExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2994, 56], "end": [2994, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f1ef9ef28f3dc139bf49e89"></a>
## partial_cmp

`function` · `sqlparser::ast::query::InterpolateExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &InterpolateExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2994, 35], "end": [2994, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2994`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2a3284569d70b3412f352d2"></a>
## serialize

`function` · `sqlparser::ast::query::InterpolateExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2995, 38], "end": [2995, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0def7adcc76d5a8f8fd5f06"></a>
## span

`function` · `sqlparser::ast::query::InterpolateExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "super::InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1306, 1], "end": [1312, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6e75ca20a7c2ef83afe24c8"></a>
## visit

`function` · `sqlparser::ast::query::InterpolateExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2996, 47], "end": [2996, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2996`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e303a6185660ef22dcfa5520"></a>
## visit

`function` · `sqlparser::ast::query::InterpolateExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InterpolateExpr", "path": "InterpolateExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2996, 40], "end": [2996, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2996`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
