# `datafusion_expr::expr::GroupingSet`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.GroupingSet.json).

<a id="op-8996c6d3b359220ddc0dbd2c"></a>
## GroupingSet

`enum` · `datafusion_expr::expr::GroupingSet` · datafusion-expr 55.1.0

```rust
enum GroupingSet
```

Source: `src/expr.rs:1439`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Grouping sets

See <https://www.postgresql.org/docs/current/queries-table-expressions.html#QUERIES-GROUPING-SETS>
for Postgres definition.
See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-groupby.html>
for Apache Spark definition.

<a id="op-05eca2e4ced0b6dab040b03b"></a>
## Cube

`variant` · `datafusion_expr::expr::GroupingSet::Cube` · datafusion-expr 55.1.0

```rust
Cube
```

Source: `src/expr.rs:1443`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Cube grouping sets

<a id="op-e323280bb1ad09dec715a72a"></a>
## GroupingSets

`variant` · `datafusion_expr::expr::GroupingSet::GroupingSets` · datafusion-expr 55.1.0

```rust
GroupingSets
```

Source: `src/expr.rs:1445`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

User-defined grouping sets

<a id="op-d84add73860b08b8caddf769"></a>
## Rollup

`variant` · `datafusion_expr::expr::GroupingSet::Rollup` · datafusion-expr 55.1.0

```rust
Rollup
```

Source: `src/expr.rs:1441`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rollup grouping sets

<a id="op-f7e5918a3b6384c7ca720bad"></a>
## clone

`function` · `datafusion_expr::expr::GroupingSet::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> GroupingSet
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GroupingSet", "path": "GroupingSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 10], "end": [1438, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fecc3814a268fb2f3cbd7e8"></a>
## distinct_expr

`function` · `datafusion_expr::expr::GroupingSet::distinct_expr` · datafusion-expr 55.1.0

```rust
fn distinct_expr(&self) -> Vec<&Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GroupingSet", "path": "GroupingSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1448, 1], "end": [1468, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1452`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return all distinct exprs in the grouping set. For `CUBE` and `ROLLUP` this
is just the underlying list of exprs. For `GROUPING SET` we need to deduplicate
the exprs in the underlying sets.

<a id="op-560a95c28ebf7989972d0545"></a>
## eq

`function` · `datafusion_expr::expr::GroupingSet::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &GroupingSet) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GroupingSet", "path": "GroupingSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 17], "end": [1438, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11f4e8bb9c6feb57ec6968f1"></a>
## fmt

`function` · `datafusion_expr::expr::GroupingSet::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GroupingSet", "path": "GroupingSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 50], "end": [1438, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc3a89025785d078c0213a49"></a>
## hash

`function` · `datafusion_expr::expr::GroupingSet::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GroupingSet", "path": "GroupingSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 44], "end": [1438, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dc86deeb7587f9e686e83e9"></a>
## partial_cmp

`function` · `datafusion_expr::expr::GroupingSet::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &GroupingSet) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::GroupingSet", "path": "GroupingSet"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1438, 32], "end": [1438, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1438`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
