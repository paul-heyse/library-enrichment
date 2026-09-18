# `sqlparser::ast::query::TableSampleSeed`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleSeed.json).

<a id="op-f521d58ca198ed6697f3b04c"></a>
## TableSampleSeed

`struct` · `sqlparser::ast::query::TableSampleSeed` · sqlparser 0.62.0

```rust
struct TableSampleSeed
```

Source: `src/ast/query.rs:1849`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SEED` or `REPEATABLE` clause used with sampling.

<a id="op-58ce42ff31116a7fc3c0fac1"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleSeed::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleSeed
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1845, 17], "end": [1845, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11c832247b52f410d988e235"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleSeed::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleSeed) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1845, 51], "end": [1845, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15b6d6523c9f662fa09de749"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleSeed::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1846, 49], "end": [1846, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1de3e8ccdf5bf8b9e417744d"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleSeed::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleSeed) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1845, 24], "end": [1845, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d32390bfffe93532cc9c238"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleSeed::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1845, 10], "end": [1845, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d12c7ecd7b7822978c9e8429"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleSeed::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1856, 1], "end": [1861, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1857`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c1606790e5baae818d83105"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleSeed::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1845, 56], "end": [1845, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d357f97bfae58f38e4510f6"></a>
## modifier

`struct_field` · `sqlparser::ast::query::TableSampleSeed::modifier` · sqlparser 0.62.0

```rust
modifier: TableSampleSeedModifier
```

Source: `src/ast/query.rs:1851`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Seed modifier (e.g. `REPEATABLE` or `SEED`).

<a id="op-5ea9d4dc1e2fb81616d640d4"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleSeed::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleSeed) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1845, 35], "end": [1845, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1845`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13175f173665f5dc28cc9cc3"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleSeed::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1846, 38], "end": [1846, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61d78955b1f0c35cfb149d2b"></a>
## value

`struct_field` · `sqlparser::ast::query::TableSampleSeed::value` · sqlparser 0.62.0

```rust
value: ValueWithSpan
```

Source: `src/ast/query.rs:1853`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The seed value expression.

<a id="op-985074ed18a0cc0ed8c02991"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleSeed::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1847, 47], "end": [1847, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1847`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f5950f9d7c70ababa0197ff"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleSeed::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeed", "path": "TableSampleSeed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1847, 40], "end": [1847, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1847`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
