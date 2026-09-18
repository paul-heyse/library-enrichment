# `sqlparser::ast::query::TableSampleMethod`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleMethod.json).

<a id="op-561496d5811e785f15206a61"></a>
## TableSampleMethod

`enum` · `sqlparser::ast::query::TableSampleMethod` · sqlparser 0.62.0

```rust
enum TableSampleMethod
```

Source: `src/ast/query.rs:1823`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The table sample method names
Sampling method used by `TABLESAMPLE`.

<a id="op-3239b8b4ca1837e75ada5c0b"></a>
## Bernoulli

`variant` · `sqlparser::ast::query::TableSampleMethod::Bernoulli` · sqlparser 0.62.0

```rust
Bernoulli
```

Source: `src/ast/query.rs:1827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BERNOULLI` sampling method.

<a id="op-9441ea08118e4702326544f1"></a>
## Block

`variant` · `sqlparser::ast::query::TableSampleMethod::Block` · sqlparser 0.62.0

```rust
Block
```

Source: `src/ast/query.rs:1831`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`BLOCK` sampling method.

<a id="op-4bcb06a09f6bcadd750059b5"></a>
## Row

`variant` · `sqlparser::ast::query::TableSampleMethod::Row` · sqlparser 0.62.0

```rust
Row
```

Source: `src/ast/query.rs:1825`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROW` sampling method.

<a id="op-00519ba7b520833f3b04bd7a"></a>
## System

`variant` · `sqlparser::ast::query::TableSampleMethod::System` · sqlparser 0.62.0

```rust
System
```

Source: `src/ast/query.rs:1829`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SYSTEM` sampling method.

<a id="op-957d3d07604ddf7a541bb06f"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleMethod::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleMethod
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1819, 17], "end": [1819, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d28c9a4d64e733738e38b35"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleMethod::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleMethod) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1819, 57], "end": [1819, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c331cf8160f03a7ce371ce9"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleMethod::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1820, 49], "end": [1820, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b62690850ef083a4689f3724"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleMethod::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleMethod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1819, 30], "end": [1819, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6836d97ebee65afe575c1848"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleMethod::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1819, 10], "end": [1819, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a887881af8ea81a272620d59"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleMethod::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1834, 1], "end": [1843, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1835`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e77d5b6a40f894a953759db"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleMethod::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1819, 62], "end": [1819, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa85436a915e32f59d633d9c"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleMethod::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleMethod) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1819, 41], "end": [1819, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a02728f896de302de3172574"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleMethod::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1820, 38], "end": [1820, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d0c8f4b9099099a26cd4cc"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleMethod::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1821, 47], "end": [1821, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1821`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-987871e05f4690d4497b7cde"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleMethod::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleMethod", "path": "TableSampleMethod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1821, 40], "end": [1821, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1821`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
