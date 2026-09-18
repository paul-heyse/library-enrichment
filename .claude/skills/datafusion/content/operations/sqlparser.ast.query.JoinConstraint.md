# `sqlparser::ast::query::JoinConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JoinConstraint.json).

<a id="op-e7c3078cf95c1469de27da95"></a>
## JoinConstraint

`enum` · `sqlparser::ast::query::JoinConstraint` · sqlparser 0.62.0

```rust
enum JoinConstraint
```

Source: `src/ast/query.rs:2863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents how two tables are constrained in a join: `ON`, `USING`, `NATURAL`, or none.

<a id="op-5f326d79647651bd66353d35"></a>
## Natural

`variant` · `sqlparser::ast::query::JoinConstraint::Natural` · sqlparser 0.62.0

```rust
Natural
```

Source: `src/ast/query.rs:2869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NATURAL` join (columns matched automatically).

<a id="op-cb01606e9484e07e62ec3cea"></a>
## None

`variant` · `sqlparser::ast::query::JoinConstraint::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/query.rs:2871`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No constraint specified (e.g. `CROSS JOIN`).

<a id="op-8054d686ee2fb895e3fcff5b"></a>
## On

`variant` · `sqlparser::ast::query::JoinConstraint::On` · sqlparser 0.62.0

```rust
On
```

Source: `src/ast/query.rs:2865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ON <expr>` join condition.

<a id="op-dd8a2c871c36d1169b9efd8e"></a>
## Using

`variant` · `sqlparser::ast::query::JoinConstraint::Using` · sqlparser 0.62.0

```rust
Using
```

Source: `src/ast/query.rs:2867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`USING(...)` list of column names.

<a id="op-d873881e181e4cb656d870b9"></a>
## clone

`function` · `sqlparser::ast::query::JoinConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JoinConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2859, 17], "end": [2859, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48f10f506c81f85c32016a36"></a>
## cmp

`function` · `sqlparser::ast::query::JoinConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JoinConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2859, 51], "end": [2859, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a90d41970d03aefb2a759825"></a>
## deserialize

`function` · `sqlparser::ast::query::JoinConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2860, 49], "end": [2860, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b381db3802ce87d286045f9f"></a>
## eq

`function` · `sqlparser::ast::query::JoinConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JoinConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2859, 24], "end": [2859, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fd72a541b8637fdb0145b7e"></a>
## fmt

`function` · `sqlparser::ast::query::JoinConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2859, 10], "end": [2859, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ff4ad13747edc27ef4502cc"></a>
## hash

`function` · `sqlparser::ast::query::JoinConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2859, 56], "end": [2859, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbb69d1d4dede4f2aaa3fc69"></a>
## partial_cmp

`function` · `sqlparser::ast::query::JoinConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JoinConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2859, 35], "end": [2859, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78df88405d202b0361ab727e"></a>
## serialize

`function` · `sqlparser::ast::query::JoinConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2860, 38], "end": [2860, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bfdbeba130c8765a067280f"></a>
## span

`function` · `sqlparser::ast::query::JoinConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "super::JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2267, 1], "end": [2276, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2268`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5664273007f258f6cd20727b"></a>
## visit

`function` · `sqlparser::ast::query::JoinConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2861, 47], "end": [2861, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2861`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1d87c21b841bf7e20e189e7"></a>
## visit

`function` · `sqlparser::ast::query::JoinConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinConstraint", "path": "JoinConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2861, 40], "end": [2861, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2861`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
