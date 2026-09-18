# `sqlparser::ast::query::OrderBy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.OrderBy.json).

<a id="op-b68c1a6eedee2d96a4888e46"></a>
## OrderBy

`struct` · `sqlparser::ast::query::OrderBy` · sqlparser 0.62.0

```rust
struct OrderBy
```

Source: `src/ast/query.rs:2893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an `ORDER BY` clause with its kind and optional `INTERPOLATE`.

<a id="op-35f69c1596225268b717f966"></a>
## clone

`function` · `sqlparser::ast::query::OrderBy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OrderBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2889, 17], "end": [2889, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8db5f96e62f68b0b012c66c"></a>
## cmp

`function` · `sqlparser::ast::query::OrderBy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OrderBy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2889, 51], "end": [2889, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d5ff0486fe57599721eb8ae"></a>
## deserialize

`function` · `sqlparser::ast::query::OrderBy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2890, 49], "end": [2890, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2890`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d5e9ecaee5a6c1fc44e0f98"></a>
## eq

`function` · `sqlparser::ast::query::OrderBy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OrderBy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2889, 24], "end": [2889, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5482fafecfeaf162bd50a37"></a>
## fmt

`function` · `sqlparser::ast::query::OrderBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2889, 10], "end": [2889, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c366df83539a5b6ccf7e12"></a>
## fmt

`function` · `sqlparser::ast::query::OrderBy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2901, 1], "end": [2922, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fe165c1d7f31d9bb742533f"></a>
## hash

`function` · `sqlparser::ast::query::OrderBy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2889, 56], "end": [2889, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8da932e899b15704e2dd382a"></a>
## interpolate

`struct_field` · `sqlparser::ast::query::OrderBy::interpolate` · sqlparser 0.62.0

```rust
interpolate: Option<Interpolate>
```

Source: `src/ast/query.rs:2898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `INTERPOLATE` clause (ClickHouse extension).

<a id="op-312810851613d9e43eb5666c"></a>
## kind

`struct_field` · `sqlparser::ast::query::OrderBy::kind` · sqlparser 0.62.0

```rust
kind: OrderByKind
```

Source: `src/ast/query.rs:2895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The kind of ordering (expressions or `ALL`).

<a id="op-fc6af4107ee647463d2d9ab9"></a>
## partial_cmp

`function` · `sqlparser::ast::query::OrderBy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OrderBy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2889, 35], "end": [2889, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-210076b979c8b32174dfd687"></a>
## serialize

`function` · `sqlparser::ast::query::OrderBy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2890, 38], "end": [2890, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2890`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-916bd32cc08c81d5a8dd86a7"></a>
## span

`function` · `sqlparser::ast::query::OrderBy::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "super::OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1269, 1], "end": [1281, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1270`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646eaf3169d545161e9b8bb0"></a>
## visit

`function` · `sqlparser::ast::query::OrderBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2891, 47], "end": [2891, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97dccd694fba3270c81c0d2c"></a>
## visit

`function` · `sqlparser::ast::query::OrderBy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderBy", "path": "OrderBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2891, 40], "end": [2891, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
