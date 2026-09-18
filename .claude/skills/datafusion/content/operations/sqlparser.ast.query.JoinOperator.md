# `sqlparser::ast::query::JoinOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JoinOperator.json).

<a id="op-e44819ab8f79ab7da5da9604"></a>
## JoinOperator

`enum` · `sqlparser::ast::query::JoinOperator` · sqlparser 0.62.0

```rust
enum JoinOperator
```

Source: `src/ast/query.rs:2803`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operator used for joining two tables, e.g. `INNER`, `LEFT`, `CROSS`, `ASOF`, etc.

<a id="op-eb396e0d51da23f6ec2cc75a"></a>
## Anti

`variant` · `sqlparser::ast::query::JoinOperator::Anti` · sqlparser 0.62.0

```rust
Anti
```

Source: `src/ast/query.rs:2827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ANTI JOIN` (non-standard)

<a id="op-e8394bc69d6604e306cca795"></a>
## ArrayJoin

`variant` · `sqlparser::ast::query::JoinOperator::ArrayJoin` · sqlparser 0.62.0

```rust
ArrayJoin
```

Source: `src/ast/query.rs:2852`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse: `ARRAY JOIN` for unnesting arrays inline.

See <https://clickhouse.com/docs/en/sql-reference/statements/select/array-join>.

<a id="op-7d60598c31ab630839c7d4b3"></a>
## AsOf

`variant` · `sqlparser::ast::query::JoinOperator::AsOf` · sqlparser 0.62.0

```rust
AsOf
```

Source: `src/ast/query.rs:2839`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ASOF` joins are used for joining time-series tables whose timestamp columns do not match exactly.

See <https://docs.snowflake.com/en/sql-reference/constructs/asof-join>.

<a id="op-1648549c474873eebdfa4a35"></a>
## CrossApply

`variant` · `sqlparser::ast::query::JoinOperator::CrossApply` · sqlparser 0.62.0

```rust
CrossApply
```

Source: `src/ast/query.rs:2833`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CROSS APPLY` (non-standard)

<a id="op-042461a6ecdd459c28406f93"></a>
## CrossJoin

`variant` · `sqlparser::ast::query::JoinOperator::CrossJoin` · sqlparser 0.62.0

```rust
CrossJoin
```

Source: `src/ast/query.rs:2819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CROSS JOIN` (constraint usage is non-standard).

<a id="op-e996beb35c4fa32d25110644"></a>
## FullOuter

`variant` · `sqlparser::ast::query::JoinOperator::FullOuter` · sqlparser 0.62.0

```rust
FullOuter
```

Source: `src/ast/query.rs:2817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FULL OUTER JOIN` with an optional constraint.

<a id="op-5132fc9930bcec4417cd872a"></a>
## Inner

`variant` · `sqlparser::ast::query::JoinOperator::Inner` · sqlparser 0.62.0

```rust
Inner
```

Source: `src/ast/query.rs:2807`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`INNER JOIN` with an optional constraint.

<a id="op-438afc96086fddc79fc5a907"></a>
## InnerArrayJoin

`variant` · `sqlparser::ast::query::JoinOperator::InnerArrayJoin` · sqlparser 0.62.0

```rust
InnerArrayJoin
```

Source: `src/ast/query.rs:2856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse: `INNER ARRAY JOIN` for unnesting arrays inline (filters rows with empty arrays).

<a id="op-a6ffbd172cab22ac51e52db5"></a>
## Join

`variant` · `sqlparser::ast::query::JoinOperator::Join` · sqlparser 0.62.0

```rust
Join
```

Source: `src/ast/query.rs:2805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic `JOIN` with an optional constraint.

<a id="op-d701004dc034ecc3944ee198"></a>
## Left

`variant` · `sqlparser::ast::query::JoinOperator::Left` · sqlparser 0.62.0

```rust
Left
```

Source: `src/ast/query.rs:2809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LEFT JOIN` with an optional constraint.

<a id="op-c518b5c668bbb045f083eeec"></a>
## LeftAnti

`variant` · `sqlparser::ast::query::JoinOperator::LeftAnti` · sqlparser 0.62.0

```rust
LeftAnti
```

Source: `src/ast/query.rs:2829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LEFT ANTI JOIN` (non-standard)

<a id="op-3cafaacebfad876e7b993f02"></a>
## LeftArrayJoin

`variant` · `sqlparser::ast::query::JoinOperator::LeftArrayJoin` · sqlparser 0.62.0

```rust
LeftArrayJoin
```

Source: `src/ast/query.rs:2854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse: `LEFT ARRAY JOIN` for unnesting arrays inline (preserves rows with empty arrays).

<a id="op-86ad886dbdfc06ff5dcb3788"></a>
## LeftOuter

`variant` · `sqlparser::ast::query::JoinOperator::LeftOuter` · sqlparser 0.62.0

```rust
LeftOuter
```

Source: `src/ast/query.rs:2811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LEFT OUTER JOIN` with an optional constraint.

<a id="op-b161a61331dce2f9f7bc3143"></a>
## LeftSemi

`variant` · `sqlparser::ast::query::JoinOperator::LeftSemi` · sqlparser 0.62.0

```rust
LeftSemi
```

Source: `src/ast/query.rs:2823`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LEFT SEMI JOIN` (non-standard)

<a id="op-e07f90bb82a2e5b6fce065ef"></a>
## OuterApply

`variant` · `sqlparser::ast::query::JoinOperator::OuterApply` · sqlparser 0.62.0

```rust
OuterApply
```

Source: `src/ast/query.rs:2835`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OUTER APPLY` (non-standard)

<a id="op-a3125d5934d211f33d2e1547"></a>
## Right

`variant` · `sqlparser::ast::query::JoinOperator::Right` · sqlparser 0.62.0

```rust
Right
```

Source: `src/ast/query.rs:2813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RIGHT JOIN` with an optional constraint.

<a id="op-ba15c45d1220373b874b4a16"></a>
## RightAnti

`variant` · `sqlparser::ast::query::JoinOperator::RightAnti` · sqlparser 0.62.0

```rust
RightAnti
```

Source: `src/ast/query.rs:2831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RIGHT ANTI JOIN` (non-standard)

<a id="op-eadbf11488f9cc88808c35fe"></a>
## RightOuter

`variant` · `sqlparser::ast::query::JoinOperator::RightOuter` · sqlparser 0.62.0

```rust
RightOuter
```

Source: `src/ast/query.rs:2815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RIGHT OUTER JOIN` with an optional constraint.

<a id="op-b9e7a0c5c3ff4b3a6825bdec"></a>
## RightSemi

`variant` · `sqlparser::ast::query::JoinOperator::RightSemi` · sqlparser 0.62.0

```rust
RightSemi
```

Source: `src/ast/query.rs:2825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RIGHT SEMI JOIN` (non-standard)

<a id="op-39169438acd09933c5ec8966"></a>
## Semi

`variant` · `sqlparser::ast::query::JoinOperator::Semi` · sqlparser 0.62.0

```rust
Semi
```

Source: `src/ast/query.rs:2821`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SEMI JOIN` (non-standard)

<a id="op-4eed778c55f0f3103ff96b86"></a>
## StraightJoin

`variant` · `sqlparser::ast::query::JoinOperator::StraightJoin` · sqlparser 0.62.0

```rust
StraightJoin
```

Source: `src/ast/query.rs:2848`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`STRAIGHT_JOIN` (MySQL non-standard behavior)

See <https://dev.mysql.com/doc/refman/8.4/en/join.html>.

<a id="op-82491563558c49d30273b10d"></a>
## clone

`function` · `sqlparser::ast::query::JoinOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JoinOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2799, 17], "end": [2799, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e87d8fa73b7d2f250ce4f569"></a>
## cmp

`function` · `sqlparser::ast::query::JoinOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JoinOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2799, 51], "end": [2799, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50e60585da1df2e65c7ddfee"></a>
## deserialize

`function` · `sqlparser::ast::query::JoinOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2800, 49], "end": [2800, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2800`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6257dc19cf3a56e023ba9e5"></a>
## eq

`function` · `sqlparser::ast::query::JoinOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JoinOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2799, 24], "end": [2799, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47da37b84ff8322011101f59"></a>
## fmt

`function` · `sqlparser::ast::query::JoinOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2799, 10], "end": [2799, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5767736d3e98a8c8be1f5d21"></a>
## hash

`function` · `sqlparser::ast::query::JoinOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2799, 56], "end": [2799, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4b96450cba2f9ac183ae7eb"></a>
## partial_cmp

`function` · `sqlparser::ast::query::JoinOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JoinOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2799, 35], "end": [2799, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcce15516defd524ebe64592"></a>
## serialize

`function` · `sqlparser::ast::query::JoinOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2800, 38], "end": [2800, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2800`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efa3c1d8e7351b4b71ffd91d"></a>
## span

`function` · `sqlparser::ast::query::JoinOperator::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "super::JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2231, 1], "end": [2260, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4dd4743b46e536e05468453a"></a>
## visit

`function` · `sqlparser::ast::query::JoinOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2801, 40], "end": [2801, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2801`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59d0e824a14d500f683851b3"></a>
## visit

`function` · `sqlparser::ast::query::JoinOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JoinOperator", "path": "JoinOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2801, 47], "end": [2801, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2801`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
