# `sqlparser::ast::query::TableSampleSeedModifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleSeedModifier.json).

<a id="op-8105d52367f40106486c437f"></a>
## TableSampleSeedModifier

`enum` · `sqlparser::ast::query::TableSampleSeedModifier` · sqlparser 0.62.0

```rust
enum TableSampleSeedModifier
```

Source: `src/ast/query.rs:1867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modifier specifying how the sample seed is applied.

<a id="op-967cc8cde2596cd0b049b0d2"></a>
## Repeatable

`variant` · `sqlparser::ast::query::TableSampleSeedModifier::Repeatable` · sqlparser 0.62.0

```rust
Repeatable
```

Source: `src/ast/query.rs:1869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`REPEATABLE` modifier.

<a id="op-dcff3afd7b241cf10634ac40"></a>
## Seed

`variant` · `sqlparser::ast::query::TableSampleSeedModifier::Seed` · sqlparser 0.62.0

```rust
Seed
```

Source: `src/ast/query.rs:1871`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SEED` modifier.

<a id="op-a2f94052bce694b74ffd1b5f"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleSeedModifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleSeedModifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1863, 17], "end": [1863, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d42b6becc1a09739b7bae176"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleSeedModifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleSeedModifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1863, 57], "end": [1863, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1005205fb80e7cdeb595f41c"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleSeedModifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 49], "end": [1864, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17c5aafc110030a8ed308ae7"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleSeedModifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleSeedModifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1863, 30], "end": [1863, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b006a0f9400b79161f257d1"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleSeedModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1863, 10], "end": [1863, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f77628d95765c540c318aded"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleSeedModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1874, 1], "end": [1881, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb7510a2b2f7224e95cf3987"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleSeedModifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1863, 62], "end": [1863, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d1236a42dbca5fcf329ff5a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleSeedModifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleSeedModifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1863, 41], "end": [1863, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84d58fe8a81d410c1ea2f2c5"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleSeedModifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 38], "end": [1864, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d8030f0986d9bfd9c9e989f"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleSeedModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 40], "end": [1865, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f92fabe3e9a62cbd63fa30d"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleSeedModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleSeedModifier", "path": "TableSampleSeedModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 47], "end": [1865, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
