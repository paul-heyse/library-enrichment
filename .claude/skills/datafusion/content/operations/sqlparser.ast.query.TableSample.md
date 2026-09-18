# `sqlparser::ast::query::TableSample`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSample.json).

<a id="op-98226982f583f50a37ec2f20"></a>
## TableSample

`struct` · `sqlparser::ast::query::TableSample` · sqlparser 0.62.0

```rust
struct TableSample
```

Source: `src/ast/query.rs:1753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a `TABLESAMPLE` clause and its options.

<a id="op-f5d6b6dbdf51ed58faa082f2"></a>
## bucket

`struct_field` · `sqlparser::ast::query::TableSample::bucket` · sqlparser 0.62.0

```rust
bucket: Option<TableSampleBucket>
```

Source: `src/ast/query.rs:1763`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional bucket specification for `BUCKET ... OUT OF ...`-style sampling.

<a id="op-c7a41f0fdd0fee23f2e918b5"></a>
## clone

`function` · `sqlparser::ast::query::TableSample::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSample
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 17], "end": [1749, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0db492db21f3430a5c87f0a4"></a>
## cmp

`function` · `sqlparser::ast::query::TableSample::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSample) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 51], "end": [1749, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c967c1fd42bd54251bec2c2"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSample::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1750, 49], "end": [1750, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-167dcd7769a8a991911fbba8"></a>
## eq

`function` · `sqlparser::ast::query::TableSample::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSample) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 24], "end": [1749, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f7ef7152fbec603590f2c62"></a>
## fmt

`function` · `sqlparser::ast::query::TableSample::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 10], "end": [1749, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dc4a2cc72303b812f0db587"></a>
## fmt

`function` · `sqlparser::ast::query::TableSample::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1925, 1], "end": [1945, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6911c8b17b7bcc0ba33efcb"></a>
## hash

`function` · `sqlparser::ast::query::TableSample::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 56], "end": [1749, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89aee10ac94a724d4850e1f3"></a>
## modifier

`struct_field` · `sqlparser::ast::query::TableSample::modifier` · sqlparser 0.62.0

```rust
modifier: TableSampleModifier
```

Source: `src/ast/query.rs:1755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modifier (e.g. `SAMPLE` or `TABLESAMPLE`).

<a id="op-6814a957f0f70557b725f01e"></a>
## name

`struct_field` · `sqlparser::ast::query::TableSample::name` · sqlparser 0.62.0

```rust
name: Option<TableSampleMethod>
```

Source: `src/ast/query.rs:1757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional sampling method name (e.g. `BERNOULLI`, `SYSTEM`).

<a id="op-0047f4022a59837a0b6a8b2a"></a>
## offset

`struct_field` · `sqlparser::ast::query::TableSample::offset` · sqlparser 0.62.0

```rust
offset: Option<Expr>
```

Source: `src/ast/query.rs:1765`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional offset expression for sampling.

<a id="op-3489243df60e6dbc82304407"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSample::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSample) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1749, 35], "end": [1749, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-269a2926f585df6fe9e63019"></a>
## quantity

`struct_field` · `sqlparser::ast::query::TableSample::quantity` · sqlparser 0.62.0

```rust
quantity: Option<TableSampleQuantity>
```

Source: `src/ast/query.rs:1759`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional sampling quantity (value and optional unit).

<a id="op-329ab8ad031fef243d8e9e21"></a>
## seed

`struct_field` · `sqlparser::ast::query::TableSample::seed` · sqlparser 0.62.0

```rust
seed: Option<TableSampleSeed>
```

Source: `src/ast/query.rs:1761`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional seed clause.

<a id="op-88a5dd850ba8c02c712c83dc"></a>
## serialize

`function` · `sqlparser::ast::query::TableSample::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1750, 38], "end": [1750, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1750`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40c1786c5548fc7b84ac17c9"></a>
## visit

`function` · `sqlparser::ast::query::TableSample::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1751, 40], "end": [1751, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccb900d1eb9e8046752c0b9a"></a>
## visit

`function` · `sqlparser::ast::query::TableSample::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSample", "path": "TableSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1751, 47], "end": [1751, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
