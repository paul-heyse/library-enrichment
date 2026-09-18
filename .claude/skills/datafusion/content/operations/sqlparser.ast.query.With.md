# `sqlparser::ast::query::With`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.With.json).

<a id="op-69d47142f70ced39f839195e"></a>
## With

`struct` · `sqlparser::ast::query::With` · sqlparser 0.62.0

```rust
struct With
```

Source: `src/ast/query.rs:753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `WITH` clause, introducing common table expressions (CTEs).

<a id="op-aeff0bb50f240d6b36ce44bf"></a>
## clone

`function` · `sqlparser::ast::query::With::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> With
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [749, 17], "end": [749, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55848c579ccdfcf831bad86d"></a>
## cmp

`function` · `sqlparser::ast::query::With::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &With) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [749, 51], "end": [749, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0382d746c12026d18d6a4d56"></a>
## cte_tables

`struct_field` · `sqlparser::ast::query::With::cte_tables` · sqlparser 0.62.0

```rust
cte_tables: Vec<Cte>
```

Source: `src/ast/query.rs:759`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of CTEs declared by this `WITH` clause.

<a id="op-0340456dad6402732211a195"></a>
## deserialize

`function` · `sqlparser::ast::query::With::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 49], "end": [750, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a5e510ba54d1d347d90d296"></a>
## eq

`function` · `sqlparser::ast::query::With::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &With) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [749, 24], "end": [749, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a946eb9f68bc9ccef63a18"></a>
## fmt

`function` · `sqlparser::ast::query::With::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [749, 10], "end": [749, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88e57e94514ebec845ed9008"></a>
## fmt

`function` · `sqlparser::ast::query::With::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [762, 1], "end": [771, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:763`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90c7f0ed8b5974862bc23beb"></a>
## hash

`function` · `sqlparser::ast::query::With::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [749, 56], "end": [749, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b145d9f05c4533599568b176"></a>
## partial_cmp

`function` · `sqlparser::ast::query::With::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &With) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [749, 35], "end": [749, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-093a76a67b185477bd025347"></a>
## recursive

`struct_field` · `sqlparser::ast::query::With::recursive` · sqlparser 0.62.0

```rust
recursive: bool
```

Source: `src/ast/query.rs:757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `WITH` is recursive (`WITH RECURSIVE`).

<a id="op-1e38bdadc1d57d6374398eb8"></a>
## serialize

`function` · `sqlparser::ast::query::With::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [750, 38], "end": [750, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-569e114d73fec01dcebcf7d4"></a>
## span

`function` · `sqlparser::ast::query::With::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "super::With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [195, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4adf0ba05ae31e7272f3581b"></a>
## visit

`function` · `sqlparser::ast::query::With::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 47], "end": [751, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b08fd2421859ba9bcfd5cb34"></a>
## visit

`function` · `sqlparser::ast::query::With::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::With", "path": "With"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [751, 40], "end": [751, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfca2877773627cf0056e4ca"></a>
## with_token

`struct_field` · `sqlparser::ast::query::With::with_token` · sqlparser 0.62.0

```rust
with_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/query.rs:755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token for the `WITH` keyword
