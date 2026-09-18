# `sqlparser::ast::OnConflict`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OnConflict.json).

<a id="op-a86c3e7f3ee84ca9defd98ae"></a>
## OnConflict

`struct` · `sqlparser::ast::OnConflict` · sqlparser 0.62.0

```rust
struct OnConflict
```

Source: `src/ast/mod.rs:6727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ON CONFLICT` clause representation.

<a id="op-c31bf41fdb6d7d1b2c6e08a7"></a>
## action

`struct_field` · `sqlparser::ast::OnConflict::action` · sqlparser 0.62.0

```rust
action: OnConflictAction
```

Source: `src/ast/mod.rs:6731`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Action to take when a conflict occurs.

<a id="op-e4c171b4f055ec4f1c516e76"></a>
## clone

`function` · `sqlparser::ast::OnConflict::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OnConflict
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6723, 17], "end": [6723, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6d4ff2d81e3024192440df7"></a>
## cmp

`function` · `sqlparser::ast::OnConflict::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OnConflict) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6723, 51], "end": [6723, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2e56ff245e482dc871b61f9"></a>
## conflict_target

`struct_field` · `sqlparser::ast::OnConflict::conflict_target` · sqlparser 0.62.0

```rust
conflict_target: Option<ConflictTarget>
```

Source: `src/ast/mod.rs:6729`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional conflict target specifying columns or constraint.

<a id="op-e9993bab176537645568315d"></a>
## deserialize

`function` · `sqlparser::ast::OnConflict::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6724, 49], "end": [6724, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5476a4df301292fd7fc0ed4f"></a>
## eq

`function` · `sqlparser::ast::OnConflict::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OnConflict) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6723, 24], "end": [6723, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7d781c24e6060542b50a702"></a>
## fmt

`function` · `sqlparser::ast::OnConflict::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6777, 1], "end": [6785, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6778`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9fc50183f5585bcdc68bd8d"></a>
## fmt

`function` · `sqlparser::ast::OnConflict::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6723, 10], "end": [6723, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b756d7881c7eff26414ca80"></a>
## hash

`function` · `sqlparser::ast::OnConflict::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6723, 56], "end": [6723, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b92e8ab4cb876110c3bca5d3"></a>
## partial_cmp

`function` · `sqlparser::ast::OnConflict::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OnConflict) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6723, 35], "end": [6723, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d46546c15ee1b58643dcd8f4"></a>
## serialize

`function` · `sqlparser::ast::OnConflict::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6724, 38], "end": [6724, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6724`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21012da180406b80bc250a9f"></a>
## span

`function` · `sqlparser::ast::OnConflict::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "super::OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1382, 1], "end": [1393, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d414365d51f01a50c71db82"></a>
## visit

`function` · `sqlparser::ast::OnConflict::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6725, 47], "end": [6725, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b79b7a782c721de662b6df36"></a>
## visit

`function` · `sqlparser::ast::OnConflict::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OnConflict", "path": "OnConflict"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6725, 40], "end": [6725, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
