# `sqlparser::ast::query::RowsPerMatch`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.RowsPerMatch.json).

<a id="op-73c2c356b163018faee4a39f"></a>
## RowsPerMatch

`enum` · `sqlparser::ast::query::RowsPerMatch` · sqlparser 0.62.0

```rust
enum RowsPerMatch
```

Source: `src/ast/query.rs:2008`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The rows per match option in a `MATCH_RECOGNIZE` operation.

See <https://docs.snowflake.com/en/sql-reference/constructs/match_recognize#row-s-per-match-specifying-the-rows-to-return>.

<a id="op-1c159c789742bfadc171f2f6"></a>
## AllRows

`variant` · `sqlparser::ast::query::RowsPerMatch::AllRows` · sqlparser 0.62.0

```rust
AllRows
```

Source: `src/ast/query.rs:2012`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALL ROWS PER MATCH <mode>`

<a id="op-f748185c235aecee5714ceb3"></a>
## OneRow

`variant` · `sqlparser::ast::query::RowsPerMatch::OneRow` · sqlparser 0.62.0

```rust
OneRow
```

Source: `src/ast/query.rs:2010`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ONE ROW PER MATCH`

<a id="op-4fcb4d50e78e29ba8c9e4c7d"></a>
## clone

`function` · `sqlparser::ast::query::RowsPerMatch::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RowsPerMatch
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 17], "end": [2005, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa7d8308912eccf4ef2f42f6"></a>
## cmp

`function` · `sqlparser::ast::query::RowsPerMatch::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RowsPerMatch) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 51], "end": [2005, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9e1560fa945b6845a5a4ada"></a>
## deserialize

`function` · `sqlparser::ast::query::RowsPerMatch::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2006, 49], "end": [2006, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2006`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1829e82ec3965f5d942aad3f"></a>
## eq

`function` · `sqlparser::ast::query::RowsPerMatch::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RowsPerMatch) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 24], "end": [2005, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94ee3252bc3f5fb2dfabb2bd"></a>
## fmt

`function` · `sqlparser::ast::query::RowsPerMatch::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2015, 1], "end": [2028, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d627badf022a89302a7bdece"></a>
## fmt

`function` · `sqlparser::ast::query::RowsPerMatch::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 10], "end": [2005, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6121fece78d94e5a47d3552b"></a>
## hash

`function` · `sqlparser::ast::query::RowsPerMatch::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 56], "end": [2005, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4b508b50966274d0be6646a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::RowsPerMatch::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RowsPerMatch) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2005, 35], "end": [2005, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbacc55b6297fccd38729593"></a>
## serialize

`function` · `sqlparser::ast::query::RowsPerMatch::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2006, 38], "end": [2006, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2006`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-462d053f40319ae62f4bddc5"></a>
## visit

`function` · `sqlparser::ast::query::RowsPerMatch::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2007, 47], "end": [2007, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6e884c62c5565401a79c9b9"></a>
## visit

`function` · `sqlparser::ast::query::RowsPerMatch::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::RowsPerMatch", "path": "RowsPerMatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2007, 40], "end": [2007, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
