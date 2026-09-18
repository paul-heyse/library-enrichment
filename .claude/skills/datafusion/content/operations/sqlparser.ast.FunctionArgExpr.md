# `sqlparser::ast::FunctionArgExpr`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArgExpr.json).

<a id="op-d8e94ac358e3bb403826f64f"></a>
## FunctionArgExpr

`enum` · `sqlparser::ast::FunctionArgExpr` · sqlparser 0.62.0

```rust
enum FunctionArgExpr
```

Source: `src/ast/mod.rs:7850`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression forms allowed as a function argument.

<a id="op-697eb77670e578090b0778c8"></a>
## Expr

`variant` · `sqlparser::ast::FunctionArgExpr::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/mod.rs:7852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A normal expression argument.

<a id="op-276738a514f0e5f6fcb1f910"></a>
## QualifiedWildcard

`variant` · `sqlparser::ast::FunctionArgExpr::QualifiedWildcard` · sqlparser 0.62.0

```rust
QualifiedWildcard
```

Source: `src/ast/mod.rs:7854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Qualified wildcard, e.g. `alias.*` or `schema.table.*`.

<a id="op-430291351564fb8f81515b01"></a>
## Wildcard

`variant` · `sqlparser::ast::FunctionArgExpr::Wildcard` · sqlparser 0.62.0

```rust
Wildcard
```

Source: `src/ast/mod.rs:7856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unqualified `*` wildcard.

<a id="op-4db2ba88076536c178376f4d"></a>
## WildcardWithOptions

`variant` · `sqlparser::ast::FunctionArgExpr::WildcardWithOptions` · sqlparser 0.62.0

```rust
WildcardWithOptions
```

Source: `src/ast/mod.rs:7860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An unqualified `*` wildcard with additional options, e.g. `* EXCLUDE(col)`.

Used in Snowflake to support expressions like `HASH(* EXCLUDE(col))`.

<a id="op-437d3b2f11ab5c9728eb4da5"></a>
## clone

`function` · `sqlparser::ast::FunctionArgExpr::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionArgExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7846, 17], "end": [7846, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13087b213e4a34fb8f2ecf11"></a>
## cmp

`function` · `sqlparser::ast::FunctionArgExpr::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionArgExpr) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7846, 51], "end": [7846, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-185b0222ae3a2e340491c490"></a>
## deserialize

`function` · `sqlparser::ast::FunctionArgExpr::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7847, 49], "end": [7847, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7847`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf272eb570b3fe41c9d51753"></a>
## eq

`function` · `sqlparser::ast::FunctionArgExpr::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionArgExpr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7846, 24], "end": [7846, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-693de6d3f75809f7197bc13c"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7846, 10], "end": [7846, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8156e079a98cdd256c3dbec"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgExpr::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7873, 1], "end": [7882, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-333d87410b8e8393b9b3edf8"></a>
## from

`function` · `sqlparser::ast::FunctionArgExpr::from` · sqlparser 0.62.0

```rust
fn from(wildcard_expr: Expr) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7863, 1], "end": [7871, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::Expr", "path": "Expr"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/mod.rs:7864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-112caf0c7440ea4f1b97b25a"></a>
## hash

`function` · `sqlparser::ast::FunctionArgExpr::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7846, 56], "end": [7846, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24abd12c5c08ef3e03f3dee3"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionArgExpr::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionArgExpr) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7846, 35], "end": [7846, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03f5e4bf0dbaccb4b6b2f20f"></a>
## serialize

`function` · `sqlparser::ast::FunctionArgExpr::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7847, 38], "end": [7847, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7847`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cf516b300666af54891028a"></a>
## span

`function` · `sqlparser::ast::FunctionArgExpr::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "super::FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2170, 1], "end": [2181, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2171`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c40b0462b4cbff9a4ef3a3a"></a>
## visit

`function` · `sqlparser::ast::FunctionArgExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7848, 47], "end": [7848, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7848`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd3b2afedf8d46b186046c0a"></a>
## visit

`function` · `sqlparser::ast::FunctionArgExpr::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgExpr", "path": "FunctionArgExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7848, 40], "end": [7848, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7848`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
