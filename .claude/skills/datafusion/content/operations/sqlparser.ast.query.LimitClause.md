# `sqlparser::ast::query::LimitClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.LimitClause.json).

<a id="op-203354b1527f1a0d29249ea9"></a>
## LimitClause

`enum` · `sqlparser::ast::query::LimitClause` · sqlparser 0.62.0

```rust
enum LimitClause
```

Source: `src/ast/query.rs:3055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the different syntactic forms of `LIMIT` clauses.

<a id="op-ffa64c1dc8e840c6598893ad"></a>
## LimitOffset

`variant` · `sqlparser::ast::query::LimitClause::LimitOffset` · sqlparser 0.62.0

```rust
LimitOffset
```

Source: `src/ast/query.rs:3059`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Standard SQL `LIMIT` syntax (optionally `BY` and `OFFSET`).

`LIMIT <limit> [BY <expr>,<expr>,...] [OFFSET <offset>]`

<a id="op-2bdd9a88a871e8f5c83c97fe"></a>
## OffsetCommaLimit

`variant` · `sqlparser::ast::query::LimitClause::OffsetCommaLimit` · sqlparser 0.62.0

```rust
OffsetCommaLimit
```

Source: `src/ast/query.rs:3068`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific syntax: `LIMIT <offset>, <limit>` (order reversed).

<a id="op-fd966030ed322a26b7f45be1"></a>
## clone

`function` · `sqlparser::ast::query::LimitClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LimitClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 17], "end": [3051, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a6b0de9c2fe13627678a5d0"></a>
## cmp

`function` · `sqlparser::ast::query::LimitClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LimitClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 51], "end": [3051, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-341ebc2aef59a92958dfbb2c"></a>
## deserialize

`function` · `sqlparser::ast::query::LimitClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3052, 49], "end": [3052, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a6042f5d00662772b8a97fb"></a>
## eq

`function` · `sqlparser::ast::query::LimitClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LimitClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 24], "end": [3051, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25aab0cfc4d8822402233c67"></a>
## fmt

`function` · `sqlparser::ast::query::LimitClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 10], "end": [3051, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4388025229209e482a1c724a"></a>
## fmt

`function` · `sqlparser::ast::query::LimitClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3076, 1], "end": [3101, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3077`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d8bc2a28a4fb97b90e923a0"></a>
## hash

`function` · `sqlparser::ast::query::LimitClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 56], "end": [3051, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8987d72e94b12705aea38121"></a>
## partial_cmp

`function` · `sqlparser::ast::query::LimitClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LimitClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 35], "end": [3051, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee79b884e152620c106b694c"></a>
## serialize

`function` · `sqlparser::ast::query::LimitClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3052, 38], "end": [3052, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0672fb55cdab264c04e6e407"></a>
## span

`function` · `sqlparser::ast::query::LimitClause::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "super::LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [158, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fcf19dab218445b79b6789a"></a>
## visit

`function` · `sqlparser::ast::query::LimitClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 40], "end": [3053, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3053`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20bda6250ad95708da011cfc"></a>
## visit

`function` · `sqlparser::ast::query::LimitClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::LimitClause", "path": "LimitClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 47], "end": [3053, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3053`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
