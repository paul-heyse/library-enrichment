# `sqlparser::ast::query::TableSampleKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableSampleKind.json).

<a id="op-2da1029556982772454a8959"></a>
## TableSampleKind

`enum` · `sqlparser::ast::query::TableSampleKind` · sqlparser 0.62.0

```rust
enum TableSampleKind
```

Source: `src/ast/query.rs:1742`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The table sample modifier options

<a id="op-25ad059e3ca8e364d3789ede"></a>
## AfterTableAlias

`variant` · `sqlparser::ast::query::TableSampleKind::AfterTableAlias` · sqlparser 0.62.0

```rust
AfterTableAlias
```

Source: `src/ast/query.rs:1746`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table sample located after the table alias option

<a id="op-55d69982824c635dfb402d49"></a>
## BeforeTableAlias

`variant` · `sqlparser::ast::query::TableSampleKind::BeforeTableAlias` · sqlparser 0.62.0

```rust
BeforeTableAlias
```

Source: `src/ast/query.rs:1744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table sample located before the table alias option

<a id="op-b6ec362b144a11d1206a3d27"></a>
## clone

`function` · `sqlparser::ast::query::TableSampleKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableSampleKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1739, 17], "end": [1739, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0c92e151a96d22813cfded9"></a>
## cmp

`function` · `sqlparser::ast::query::TableSampleKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableSampleKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1739, 51], "end": [1739, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26d8dd4b025bfbe8443bc014"></a>
## deserialize

`function` · `sqlparser::ast::query::TableSampleKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1740, 49], "end": [1740, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1740`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98fe9febd91834865df5c2fa"></a>
## eq

`function` · `sqlparser::ast::query::TableSampleKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableSampleKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1739, 24], "end": [1739, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3fcf3487ccfed8127fa4fbb"></a>
## fmt

`function` · `sqlparser::ast::query::TableSampleKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1739, 10], "end": [1739, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a2ec72216675eecca81c1ca"></a>
## hash

`function` · `sqlparser::ast::query::TableSampleKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1739, 56], "end": [1739, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cf95100b04b302ec54dd093"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableSampleKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableSampleKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1739, 35], "end": [1739, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9447cb7fcc698139815cc65"></a>
## serialize

`function` · `sqlparser::ast::query::TableSampleKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1740, 38], "end": [1740, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1740`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02261228b5401e433c90e28b"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1741, 40], "end": [1741, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe91ec13d2d34d542166fc02"></a>
## visit

`function` · `sqlparser::ast::query::TableSampleKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableSampleKind", "path": "TableSampleKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1741, 47], "end": [1741, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
