# `sqlparser::ast::query::Join`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Join.json).

<a id="op-06e7faf759da953830542044"></a>
## Join

`struct` · `sqlparser::ast::query::Join` · sqlparser 0.62.0

```rust
struct Join
```

Source: `src/ast/query.rs:2652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single `JOIN` clause including relation and join operator/options.

<a id="op-c2651114d6343d895e90af09"></a>
## clone

`function` · `sqlparser::ast::query::Join::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Join
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2648, 17], "end": [2648, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6387bb61de1a5d21a478cff8"></a>
## cmp

`function` · `sqlparser::ast::query::Join::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Join) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2648, 51], "end": [2648, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0d4bf62b662caa505e7e536"></a>
## deserialize

`function` · `sqlparser::ast::query::Join::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 49], "end": [2649, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d8090b6347fe7c4004b722b"></a>
## eq

`function` · `sqlparser::ast::query::Join::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Join) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2648, 24], "end": [2648, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a3ada57f5c87030fe63d476"></a>
## fmt

`function` · `sqlparser::ast::query::Join::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2662, 1], "end": [2797, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b055bae63afdd297033fe5b4"></a>
## fmt

`function` · `sqlparser::ast::query::Join::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2648, 10], "end": [2648, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67c1657687f97a95f4aa45dc"></a>
## global

`struct_field` · `sqlparser::ast::query::Join::global` · sqlparser 0.62.0

```rust
global: bool
```

Source: `src/ast/query.rs:2657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse supports the optional `GLOBAL` keyword before the join operator.
See [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/join)

<a id="op-fc2a499b3c8c8ab6a25e271c"></a>
## hash

`function` · `sqlparser::ast::query::Join::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2648, 56], "end": [2648, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a6df5e8fa1e76f8782d1c46"></a>
## join_operator

`struct_field` · `sqlparser::ast::query::Join::join_operator` · sqlparser 0.62.0

```rust
join_operator: JoinOperator
```

Source: `src/ast/query.rs:2659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The join operator and its constraint (INNER/LEFT/RIGHT/CROSS/ASOF/etc.).

<a id="op-ebb2cf5b520034d20908bb3c"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Join::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Join) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2648, 35], "end": [2648, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9a6c67ccce53055f8c1e619"></a>
## relation

`struct_field` · `sqlparser::ast::query::Join::relation` · sqlparser 0.62.0

```rust
relation: TableFactor
```

Source: `src/ast/query.rs:2654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The joined table factor (table reference or derived table).

<a id="op-d8f9144e18a3bbb153f643b9"></a>
## serialize

`function` · `sqlparser::ast::query::Join::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2649, 38], "end": [2649, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4acaaa58855abde66136e8a"></a>
## span

`function` · `sqlparser::ast::query::Join::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "super::Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2213, 1], "end": [2223, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-616d5c060b11d455608e728a"></a>
## visit

`function` · `sqlparser::ast::query::Join::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2650, 40], "end": [2650, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8381709a3132ef1a682fa776"></a>
## visit

`function` · `sqlparser::ast::query::Join::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Join", "path": "Join"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2650, 47], "end": [2650, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
