# `sqlparser::ast::query::TableSampleUnit`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleUnit.json).

<a id="op-7492f046ec6bf0f703d1d79a"></a>
## TableSampleUnit

`enum` · `sqlparser::ast::query::TableSampleUnit` · sqlparser 0.62.0

```rust
enum TableSampleUnit
```

Source: `src/ast/query.rs:1887`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Unit used with a `TABLESAMPLE` quantity (rows or percent).

<a id="op-494403e379286a78f17f9efa"></a>
## Percent

`variant` · `sqlparser::ast::query::TableSampleUnit::Percent` · sqlparser 0.62.0

```rust
Percent
```

Source: `src/ast/query.rs:1891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PERCENT` unit.

<a id="op-6e7ff248f0b1e36c8b2d712f"></a>
## Rows

`variant` · `sqlparser::ast::query::TableSampleUnit::Rows` · sqlparser 0.62.0

```rust
Rows
```

Source: `src/ast/query.rs:1889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROWS` unit.

<a id="op-f0daf6326d4747703ae3651d"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleUnit::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 17], "end": [1883, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68a94b4c207f4a5018e13aef"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleUnit::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleUnit) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 57], "end": [1883, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e59659789d7bded672dd0388"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleUnit::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1884, 49], "end": [1884, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-634f7fe54cbd7e9e8f1e92db"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleUnit::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleUnit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 30], "end": [1883, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-229287af0c9aac2f48e3b6bd"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleUnit::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1894, 1], "end": [1901, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdfbbdee587edbe402b1ae13"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleUnit::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 10], "end": [1883, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ee9b4353590a7e7e9b6bc8e"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleUnit::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 62], "end": [1883, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8185000c3a57a04f92cced65"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleUnit::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleUnit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1883, 41], "end": [1883, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-021d06a981492a62687ed893"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleUnit::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1884, 38], "end": [1884, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-443463ab478de2975ff19aa0"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleUnit::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1885, 47], "end": [1885, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fae9ee46877c5d0a7f6a960b"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleUnit::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleUnit", "path": "TableSampleUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1885, 40], "end": [1885, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
