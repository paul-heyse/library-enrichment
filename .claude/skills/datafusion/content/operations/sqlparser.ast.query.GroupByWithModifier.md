# `sqlparser::ast::query::GroupByWithModifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.GroupByWithModifier.json).

<a id="op-5da4ffaa7493ae509f8ec9a7"></a>
## GroupByWithModifier

`enum` · `sqlparser::ast::query::GroupByWithModifier` · sqlparser 0.62.0

```rust
enum GroupByWithModifier
```

Source: `src/ast/query.rs:3715`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse supports GROUP BY WITH modifiers(includes ROLLUP|CUBE|TOTALS).
e.g. GROUP BY year WITH ROLLUP WITH TOTALS

[ClickHouse]: <https://clickhouse.com/docs/en/sql-reference/statements/select/group-by#rollup-modifier>
Modifiers used with `GROUP BY` such as `WITH ROLLUP` or `WITH CUBE`.

<a id="op-dab9c803238b6966edbdb651"></a>
## Cube

`variant` · `sqlparser::ast::query::GroupByWithModifier::Cube` · sqlparser 0.62.0

```rust
Cube
```

Source: `src/ast/query.rs:3719`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITH CUBE` modifier.

<a id="op-0eeb463ee639c599f9e5306b"></a>
## GroupingSets

`variant` · `sqlparser::ast::query::GroupByWithModifier::GroupingSets` · sqlparser 0.62.0

```rust
GroupingSets
```

Source: `src/ast/query.rs:3725`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive supports GROUPING SETS syntax, e.g. `GROUP BY GROUPING SETS(...)`.

[Hive]: <https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=30151323#EnhancedAggregation,Cube,GroupingandRollup-GROUPINGSETSclause>

<a id="op-b2204607dd1f53729497580d"></a>
## Rollup

`variant` · `sqlparser::ast::query::GroupByWithModifier::Rollup` · sqlparser 0.62.0

```rust
Rollup
```

Source: `src/ast/query.rs:3717`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITH ROLLUP` modifier.

<a id="op-00f00385f5fcaee8d7bb849a"></a>
## Totals

`variant` · `sqlparser::ast::query::GroupByWithModifier::Totals` · sqlparser 0.62.0

```rust
Totals
```

Source: `src/ast/query.rs:3721`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WITH TOTALS` modifier (ClickHouse).

<a id="op-f5e835e0401f853d0ebb0cb4"></a>
## clone

`function` · `sqlparser::ast::query::GroupByWithModifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GroupByWithModifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3711, 17], "end": [3711, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06672e81bf67a5662a4b739b"></a>
## cmp

`function` · `sqlparser::ast::query::GroupByWithModifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GroupByWithModifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3711, 51], "end": [3711, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e15a8a58cdf51c08a7857d9"></a>
## deserialize

`function` · `sqlparser::ast::query::GroupByWithModifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3712, 49], "end": [3712, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bca04d65065079762aecf174"></a>
## eq

`function` · `sqlparser::ast::query::GroupByWithModifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GroupByWithModifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3711, 24], "end": [3711, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a49a0a32f89ddb546d33c9ca"></a>
## fmt

`function` · `sqlparser::ast::query::GroupByWithModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3711, 10], "end": [3711, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3c64ad56209b63ef0020491"></a>
## fmt

`function` · `sqlparser::ast::query::GroupByWithModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3728, 1], "end": [3739, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3729`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22eb9385574ed5aa7593dde9"></a>
## hash

`function` · `sqlparser::ast::query::GroupByWithModifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3711, 56], "end": [3711, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a06d924de561c0e2043cd8a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::GroupByWithModifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GroupByWithModifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3711, 35], "end": [3711, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a42191f09fab3918c0e2738"></a>
## serialize

`function` · `sqlparser::ast::query::GroupByWithModifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3712, 38], "end": [3712, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-132b3a9cc599c168ff3d73ec"></a>
## visit

`function` · `sqlparser::ast::query::GroupByWithModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3713, 40], "end": [3713, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aaf20287215273b542b76193"></a>
## visit

`function` · `sqlparser::ast::query::GroupByWithModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::GroupByWithModifier", "path": "GroupByWithModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3713, 47], "end": [3713, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
