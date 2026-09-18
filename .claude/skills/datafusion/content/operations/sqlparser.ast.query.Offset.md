# `sqlparser::ast::query::Offset`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Offset.json).

<a id="op-519098f2055e4a742b7ce2dd"></a>
## Offset

`struct` · `sqlparser::ast::query::Offset` · sqlparser 0.62.0

```rust
struct Offset
```

Source: `src/ast/query.rs:3107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OFFSET` clause consisting of a value and a rows specifier.

<a id="op-b007a31019f88b572afe7e60"></a>
## clone

`function` · `sqlparser::ast::query::Offset::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Offset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3103, 17], "end": [3103, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9d305ad0b141cfdb30b3b01"></a>
## cmp

`function` · `sqlparser::ast::query::Offset::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Offset) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3103, 51], "end": [3103, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fee806a20c9497da1987aac"></a>
## deserialize

`function` · `sqlparser::ast::query::Offset::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3104, 49], "end": [3104, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ca15d18809527aed716bf3"></a>
## eq

`function` · `sqlparser::ast::query::Offset::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Offset) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3103, 24], "end": [3103, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4980bb03128972d95c0cd49d"></a>
## fmt

`function` · `sqlparser::ast::query::Offset::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3114, 1], "end": [3118, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2f6dde6253f560f84d79998"></a>
## fmt

`function` · `sqlparser::ast::query::Offset::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3103, 10], "end": [3103, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-714d7884b84a82dce04c0072"></a>
## hash

`function` · `sqlparser::ast::query::Offset::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3103, 56], "end": [3103, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-843be6e8e6b9b17157aa9283"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Offset::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Offset) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3103, 35], "end": [3103, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adc3e8e772d78e1bf95e5d02"></a>
## rows

`struct_field` · `sqlparser::ast::query::Offset::rows` · sqlparser 0.62.0

```rust
rows: OffsetRows
```

Source: `src/ast/query.rs:3111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the offset uses `ROW`/`ROWS` or omits it.

<a id="op-f6d488b96c4bbef28514f6b1"></a>
## serialize

`function` · `sqlparser::ast::query::Offset::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3104, 38], "end": [3104, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2b5590d56a5f2bf519d28bc"></a>
## span

`function` · `sqlparser::ast::query::Offset::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "super::Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [169, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e146df96a833255acb9478f4"></a>
## value

`struct_field` · `sqlparser::ast::query::Offset::value` · sqlparser 0.62.0

```rust
value: Expr
```

Source: `src/ast/query.rs:3109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The numeric expression following `OFFSET`.

<a id="op-43765b151d37d556554a1ea4"></a>
## visit

`function` · `sqlparser::ast::query::Offset::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3105, 40], "end": [3105, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78a148566ebca220a022cf0d"></a>
## visit

`function` · `sqlparser::ast::query::Offset::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Offset", "path": "Offset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3105, 47], "end": [3105, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
