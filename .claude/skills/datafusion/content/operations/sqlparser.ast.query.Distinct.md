# `sqlparser::ast::query::Distinct`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Distinct.json).

<a id="op-1b07fb1e951f5f01a6955efb"></a>
## Distinct

`enum` · `sqlparser::ast::query::Distinct` · sqlparser 0.62.0

```rust
enum Distinct
```

Source: `src/ast/query.rs:3580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALL`, `DISTINCT`, or `DISTINCT ON (...)` modifiers for `SELECT` lists.

<a id="op-9a618d8f5323dbfffca5a0b7"></a>
## All

`variant` · `sqlparser::ast::query::Distinct::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/query.rs:3585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALL` (keep duplicate rows)

Generally this is the default if omitted, but omission should be represented as
`None::<Option<Distinct>>`

<a id="op-120e6755ebf203b411827f0f"></a>
## Distinct

`variant` · `sqlparser::ast::query::Distinct::Distinct` · sqlparser 0.62.0

```rust
Distinct
```

Source: `src/ast/query.rs:3588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTINCT` (remove duplicate rows)

<a id="op-edb5d66bfe14123df7781488"></a>
## On

`variant` · `sqlparser::ast::query::Distinct::On` · sqlparser 0.62.0

```rust
On
```

Source: `src/ast/query.rs:3591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DISTINCT ON (...)` (Postgres extension)

<a id="op-05cfc66b8000956ec8ee401c"></a>
## clone

`function` · `sqlparser::ast::query::Distinct::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Distinct
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 17], "end": [3576, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31b4ae9ec16f7a970649b1d8"></a>
## cmp

`function` · `sqlparser::ast::query::Distinct::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Distinct) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 51], "end": [3576, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee545664d9ea3f10a4d0c26c"></a>
## deserialize

`function` · `sqlparser::ast::query::Distinct::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3577, 49], "end": [3577, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9344bc6bdf1421718b9800e"></a>
## eq

`function` · `sqlparser::ast::query::Distinct::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Distinct) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 24], "end": [3576, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e2e617cc2bafae9d9f87cca"></a>
## fmt

`function` · `sqlparser::ast::query::Distinct::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3594, 1], "end": [3605, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3595`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fac47f2d886d162fa1532b8"></a>
## fmt

`function` · `sqlparser::ast::query::Distinct::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 10], "end": [3576, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b79d03961efc240ad4683943"></a>
## hash

`function` · `sqlparser::ast::query::Distinct::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 56], "end": [3576, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7aa172c11f46ead33f0a073"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Distinct::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Distinct) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3576, 35], "end": [3576, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7667430558c2b1a7eaa1a69"></a>
## serialize

`function` · `sqlparser::ast::query::Distinct::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3577, 38], "end": [3577, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a62f90be2a49420afa0d5ab"></a>
## visit

`function` · `sqlparser::ast::query::Distinct::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3578, 40], "end": [3578, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ea362b371215ada5fc2782e"></a>
## visit

`function` · `sqlparser::ast::query::Distinct::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Distinct", "path": "Distinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3578, 47], "end": [3578, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
