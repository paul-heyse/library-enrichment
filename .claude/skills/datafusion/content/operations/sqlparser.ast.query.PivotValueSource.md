# `sqlparser::ast::query::PivotValueSource`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PivotValueSource.json).

<a id="op-60fe7e83c818552616bb526b"></a>
## PivotValueSource

`enum` · `sqlparser::ast::query::PivotValueSource` · sqlparser 0.62.0

```rust
enum PivotValueSource
```

Source: `src/ast/query.rs:1951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The source of values in a `PIVOT` operation.

<a id="op-256120af90af9914eed50cc8"></a>
## Any

`variant` · `sqlparser::ast::query::PivotValueSource::Any` · sqlparser 0.62.0

```rust
Any
```

Source: `src/ast/query.rs:1959`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pivot on all distinct values of the pivot column.

See <https://docs.snowflake.com/en/sql-reference/constructs/pivot#pivot-on-all-distinct-column-values-automatically-with-dynamic-pivot>.

<a id="op-3edc4f5e529f4a7bfeeb63ff"></a>
## List

`variant` · `sqlparser::ast::query::PivotValueSource::List` · sqlparser 0.62.0

```rust
List
```

Source: `src/ast/query.rs:1955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pivot on a static list of values.

See <https://docs.snowflake.com/en/sql-reference/constructs/pivot#pivot-on-a-specified-list-of-column-values-for-the-pivot-column>.

<a id="op-4d4a278022552e960baf759d"></a>
## Subquery

`variant` · `sqlparser::ast::query::PivotValueSource::Subquery` · sqlparser 0.62.0

```rust
Subquery
```

Source: `src/ast/query.rs:1963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pivot on all values returned by a subquery.

See <https://docs.snowflake.com/en/sql-reference/constructs/pivot#pivot-on-column-values-using-a-subquery-with-dynamic-pivot>.

<a id="op-1dc25d06636318ac0b4f4036"></a>
## clone

`function` · `sqlparser::ast::query::PivotValueSource::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PivotValueSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1948, 17], "end": [1948, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-482bf192b2717d9c7eefbf9d"></a>
## cmp

`function` · `sqlparser::ast::query::PivotValueSource::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PivotValueSource) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1948, 51], "end": [1948, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0951608d2bf54b79117b4f2f"></a>
## deserialize

`function` · `sqlparser::ast::query::PivotValueSource::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 49], "end": [1949, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c141799f7f6ed3c51ec11068"></a>
## eq

`function` · `sqlparser::ast::query::PivotValueSource::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PivotValueSource) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1948, 24], "end": [1948, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ff6c4b4268401513f8d9a93"></a>
## fmt

`function` · `sqlparser::ast::query::PivotValueSource::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1948, 10], "end": [1948, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b61a5a152ec295a5c58a84f3"></a>
## fmt

`function` · `sqlparser::ast::query::PivotValueSource::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1966, 1], "end": [1980, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e8ab546a6c5f375cec1f6e2"></a>
## hash

`function` · `sqlparser::ast::query::PivotValueSource::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1948, 56], "end": [1948, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4858c69ec9685dc2e97e6ec"></a>
## partial_cmp

`function` · `sqlparser::ast::query::PivotValueSource::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PivotValueSource) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1948, 35], "end": [1948, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be424943bef92de7fd6dc9db"></a>
## serialize

`function` · `sqlparser::ast::query::PivotValueSource::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1949, 38], "end": [1949, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1949`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6993195c3d87a18d823f9e80"></a>
## span

`function` · `sqlparser::ast::query::PivotValueSource::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "super::PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2081, 1], "end": [2089, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c2636b4d53256f06de21046"></a>
## visit

`function` · `sqlparser::ast::query::PivotValueSource::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1950, 47], "end": [1950, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d83b0761aa3e9c5b02c73a5d"></a>
## visit

`function` · `sqlparser::ast::query::PivotValueSource::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PivotValueSource", "path": "PivotValueSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1950, 40], "end": [1950, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1950`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
