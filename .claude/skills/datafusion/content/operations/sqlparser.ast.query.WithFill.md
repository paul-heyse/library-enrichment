# `sqlparser::ast::query::WithFill`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.WithFill.json).

<a id="op-7d3197ed4b358db9a0ca0a4a"></a>
## WithFill

`struct` · `sqlparser::ast::query::WithFill` · sqlparser 0.62.0

```rust
struct WithFill
```

Source: `src/ast/query.rs:2965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse `WITH FILL` modifier for `ORDER BY` clause.
Supported by [ClickHouse syntax]

[ClickHouse syntax]: <https://clickhouse.com/docs/en/sql-reference/statements/select/order-by#order-by-expr-with-fill-modifier>
`WITH FILL` options for ClickHouse `ORDER BY` expressions.

<a id="op-299251c0cbf94cc3224451a0"></a>
## clone

`function` · `sqlparser::ast::query::WithFill::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WithFill
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2961, 17], "end": [2961, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:2961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-368b8fe808ef66fe61d2f425"></a>
## cmp

`function` · `sqlparser::ast::query::WithFill::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WithFill) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2961, 51], "end": [2961, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:2961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-656ef57a7e4e290620437ff3"></a>
## deserialize

`function` · `sqlparser::ast::query::WithFill::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2962, 49], "end": [2962, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:2962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a12084866ef38407c277bdb1"></a>
## eq

`function` · `sqlparser::ast::query::WithFill::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WithFill) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2961, 24], "end": [2961, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:2961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac953b5eafbad45003dfcb74"></a>
## fmt

`function` · `sqlparser::ast::query::WithFill::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2974, 1], "end": [2988, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:2975`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8ed883eb635658434cc6ee0"></a>
## fmt

`function` · `sqlparser::ast::query::WithFill::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2961, 10], "end": [2961, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:2961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89dd70c24619e736b34f16c7"></a>
## from

`struct_field` · `sqlparser::ast::query::WithFill::from` · sqlparser 0.62.0

```rust
from: Option<Expr>
```

Source: `src/ast/query.rs:2967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional lower bound expression for the fill range (`FROM <expr>`).

<a id="op-7d4add5fcddb2d9d9cd97204"></a>
## hash

`function` · `sqlparser::ast::query::WithFill::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2961, 56], "end": [2961, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:2961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d08b4d60e83eebfcc9eb8bd"></a>
## partial_cmp

`function` · `sqlparser::ast::query::WithFill::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WithFill) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2961, 35], "end": [2961, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:2961`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e4f173816c0ef9eb26818e9"></a>
## serialize

`function` · `sqlparser::ast::query::WithFill::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2962, 38], "end": [2962, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:2962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24bb77e39f82c19f665b9101"></a>
## span

`function` · `sqlparser::ast::query::WithFill::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "super::WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2134, 1], "end": [2145, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2d17940edbb1d3c298d84c0"></a>
## step

`struct_field` · `sqlparser::ast::query::WithFill::step` · sqlparser 0.62.0

```rust
step: Option<Expr>
```

Source: `src/ast/query.rs:2971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional step expression specifying interpolation step (`STEP <expr>`).

<a id="op-7c010e313cad78a08bc334c1"></a>
## to

`struct_field` · `sqlparser::ast::query::WithFill::to` · sqlparser 0.62.0

```rust
to: Option<Expr>
```

Source: `src/ast/query.rs:2969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional upper bound expression for the fill range (`TO <expr>`).

<a id="op-26918cc825539477c3b2421a"></a>
## visit

`function` · `sqlparser::ast::query::WithFill::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 40], "end": [2963, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2e45fe3204d582b0a313cb7"></a>
## visit

`function` · `sqlparser::ast::query::WithFill::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WithFill", "path": "WithFill"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2963, 47], "end": [2963, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:2963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
