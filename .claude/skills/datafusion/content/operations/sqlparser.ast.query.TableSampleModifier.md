# `sqlparser::ast::query::TableSampleModifier`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleModifier.json).

<a id="op-5d88bdbb98f9e1f31176ebef"></a>
## TableSampleModifier

`enum` · `sqlparser::ast::query::TableSampleModifier` · sqlparser 0.62.0

```rust
enum TableSampleModifier
```

Source: `src/ast/query.rs:1772`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modifier specifying whether `SAMPLE` or `TABLESAMPLE` keyword was used.

<a id="op-5a5529b00756f738f936278f"></a>
## Sample

`variant` · `sqlparser::ast::query::TableSampleModifier::Sample` · sqlparser 0.62.0

```rust
Sample
```

Source: `src/ast/query.rs:1774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SAMPLE` modifier.

<a id="op-a062e91d69fe5b73daccfc33"></a>
## TableSample

`variant` · `sqlparser::ast::query::TableSampleModifier::TableSample` · sqlparser 0.62.0

```rust
TableSample
```

Source: `src/ast/query.rs:1776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TABLESAMPLE` modifier.

<a id="op-d33b82415814cc3f2616cec9"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleModifier::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleModifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1768, 17], "end": [1768, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d02a666a18d4a8aeab006432"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleModifier::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleModifier) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1768, 57], "end": [1768, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a836df5c9ed37b8fdb05fe99"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleModifier::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1769, 49], "end": [1769, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b9fa55bc2943936cc2a7cc7"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleModifier::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleModifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1768, 30], "end": [1768, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71f4aa1eb0964f5ea93a911a"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1779, 1], "end": [1787, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1780`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7bcb5a6453ea7c19347ea60"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleModifier::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1768, 10], "end": [1768, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e2e559a2bbb148e6414bf2"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleModifier::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1768, 62], "end": [1768, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc81f711edcdd59a85c0fcf0"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleModifier::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleModifier) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1768, 41], "end": [1768, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1768`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62402793a029f33afacbd144"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleModifier::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1769, 38], "end": [1769, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1769`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d8f34fce33d56f75680c685"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1770, 47], "end": [1770, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c45ccf5dcdab8d18759e5c72"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleModifier::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleModifier", "path": "TableSampleModifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1770, 40], "end": [1770, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
