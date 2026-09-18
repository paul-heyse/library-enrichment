# `sqlparser::ast::Analyze`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Analyze.json).

<a id="op-136586d336fb4e95d6f355ab"></a>
## Analyze

`struct` · `sqlparser::ast::Analyze` · sqlparser 0.62.0

```rust
struct Analyze
```

Source: `src/ast/mod.rs:3475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ANALYZE statement

Supported syntax varies by dialect:
- Hive: `ANALYZE TABLE t [PARTITION (...)] COMPUTE STATISTICS [NOSCAN] [FOR COLUMNS [col1, ...]] [CACHE METADATA]`
- PostgreSQL: `ANALYZE [VERBOSE] [t [(col1, ...)]]` See <https://www.postgresql.org/docs/current/sql-analyze.html>
- General: `ANALYZE [TABLE] t`

<a id="op-02d716ab5d65b98a7bdc5298"></a>
## cache_metadata

`struct_field` · `sqlparser::ast::Analyze::cache_metadata` · sqlparser 0.62.0

```rust
cache_metadata: bool
```

Source: `src/ast/mod.rs:3486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to cache metadata before analyzing.

<a id="op-cefca39aa1baaf012384143a"></a>
## clone

`function` · `sqlparser::ast::Analyze::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Analyze
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3472, 17], "end": [3472, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8fa0b201f151c0645848824"></a>
## cmp

`function` · `sqlparser::ast::Analyze::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Analyze) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3472, 51], "end": [3472, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a994ef55576da9b3de72d2a"></a>
## columns

`struct_field` · `sqlparser::ast::Analyze::columns` · sqlparser 0.62.0

```rust
columns: Vec<Ident>
```

Source: `src/ast/mod.rs:3484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns to analyze.

<a id="op-c88caa74c568f699202d496b"></a>
## compute_statistics

`struct_field` · `sqlparser::ast::Analyze::compute_statistics` · sqlparser 0.62.0

```rust
compute_statistics: bool
```

Source: `src/ast/mod.rs:3490`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to compute statistics during analysis.

<a id="op-86c6885a8c43cbdd0a23c126"></a>
## deserialize

`function` · `sqlparser::ast::Analyze::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3473, 49], "end": [3473, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f6a6bf0222558fe8c83eae4"></a>
## eq

`function` · `sqlparser::ast::Analyze::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Analyze) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3472, 24], "end": [3472, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65b5c25848498e55b2e24346"></a>
## fmt

`function` · `sqlparser::ast::Analyze::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3495, 1], "end": [3529, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3496`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bcda9b21fb06f8a29a02194"></a>
## fmt

`function` · `sqlparser::ast::Analyze::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3472, 10], "end": [3472, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb9eedafedb3f847582b41c2"></a>
## for_columns

`struct_field` · `sqlparser::ast::Analyze::for_columns` · sqlparser 0.62.0

```rust
for_columns: bool
```

Source: `src/ast/mod.rs:3482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when analyzing specific columns (Hive `FOR COLUMNS` syntax).

<a id="op-1bf940c6c941faa313ce78b3"></a>
## has_table_keyword

`struct_field` · `sqlparser::ast::Analyze::has_table_keyword` · sqlparser 0.62.0

```rust
has_table_keyword: bool
```

Source: `src/ast/mod.rs:3492`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the `TABLE` keyword was present.

<a id="op-42e0545a05aa0f4614d3144b"></a>
## hash

`function` · `sqlparser::ast::Analyze::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3472, 56], "end": [3472, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f36b97003cce492fb1837085"></a>
## noscan

`struct_field` · `sqlparser::ast::Analyze::noscan` · sqlparser 0.62.0

```rust
noscan: bool
```

Source: `src/ast/mod.rs:3488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to skip scanning the table.

<a id="op-2d1da98dbbf2e87222c12118"></a>
## partial_cmp

`function` · `sqlparser::ast::Analyze::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Analyze) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3472, 35], "end": [3472, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0bd01b674c41ba5a218dc92"></a>
## partitions

`struct_field` · `sqlparser::ast::Analyze::partitions` · sqlparser 0.62.0

```rust
partitions: Option<Vec<Expr>>
```

Source: `src/ast/mod.rs:3480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition expressions to restrict the analysis.

<a id="op-67d6740fc7f01ab9d7696c5d"></a>
## serialize

`function` · `sqlparser::ast::Analyze::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3473, 38], "end": [3473, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fce26056c8734037aa81d1c0"></a>
## span

`function` · `sqlparser::ast::Analyze::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "super::Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [867, 1], "end": [881, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d890e5d6622fbc66133745e"></a>
## table_name

`struct_field` · `sqlparser::ast::Analyze::table_name` · sqlparser 0.62.0

```rust
table_name: Option<ObjectName>
```

Source: `src/ast/mod.rs:3478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the table to analyze. `None` for bare `ANALYZE`.

<a id="op-4037477b532e5e03bc60fdf5"></a>
## visit

`function` · `sqlparser::ast::Analyze::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3474, 47], "end": [3474, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec2b7e36726f36ea9b29733a"></a>
## visit

`function` · `sqlparser::ast::Analyze::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Analyze", "path": "Analyze"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3474, 40], "end": [3474, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
