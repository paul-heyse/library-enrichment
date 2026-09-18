# `sqlparser::ast::data_type::ExactNumberInfo`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.ExactNumberInfo.json).

<a id="op-0ee9e529969fe3235a87e6c6"></a>
## ExactNumberInfo

`enum` · `sqlparser::ast::data_type::ExactNumberInfo` · sqlparser 0.62.0

```rust
enum ExactNumberInfo
```

Source: `src/ast/data_type.rs:1017`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional information for `NUMERIC`, `DECIMAL`, and `DEC` data types
following the 2016 [SQL Standard].

[SQL Standard]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#exact-numeric-type

<a id="op-0a5ee6ad1cd60f5cdeb953a4"></a>
## None

`variant` · `sqlparser::ast::data_type::ExactNumberInfo::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/data_type.rs:1019`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No additional information, e.g. `DECIMAL`.

<a id="op-3207b5324e148615a3b84510"></a>
## Precision

`variant` · `sqlparser::ast::data_type::ExactNumberInfo::Precision` · sqlparser 0.62.0

```rust
Precision
```

Source: `src/ast/data_type.rs:1021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only precision information, e.g. `DECIMAL(10)`.

<a id="op-7ee48c1d32fbba0058e855eb"></a>
## PrecisionAndScale

`variant` · `sqlparser::ast::data_type::ExactNumberInfo::PrecisionAndScale` · sqlparser 0.62.0

```rust
PrecisionAndScale
```

Source: `src/ast/data_type.rs:1023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Precision and scale information, e.g. `DECIMAL(10,2)`.

<a id="op-1eb04fe454014ab68450a88f"></a>
## clone

`function` · `sqlparser::ast::data_type::ExactNumberInfo::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExactNumberInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 23], "end": [1014, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:1014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-357a82bfe5ee6eaa3a791c49"></a>
## cmp

`function` · `sqlparser::ast::data_type::ExactNumberInfo::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExactNumberInfo) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 57], "end": [1014, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:1014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa018d66e3b7f7c863afd26c"></a>
## deserialize

`function` · `sqlparser::ast::data_type::ExactNumberInfo::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1015, 49], "end": [1015, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:1015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82780fc1c22a67626acd390f"></a>
## eq

`function` · `sqlparser::ast::data_type::ExactNumberInfo::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExactNumberInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 30], "end": [1014, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:1014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74fe1c401cc49033edb262ec"></a>
## fmt

`function` · `sqlparser::ast::data_type::ExactNumberInfo::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1026, 1], "end": [1040, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:1027`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9c9a68d67b0ddbc6184a28c"></a>
## fmt

`function` · `sqlparser::ast::data_type::ExactNumberInfo::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 10], "end": [1014, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:1014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24c6cf01a25fc869192a86ed"></a>
## hash

`function` · `sqlparser::ast::data_type::ExactNumberInfo::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 62], "end": [1014, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:1014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e068ca1474ebcf7dc2fbb7a4"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::ExactNumberInfo::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExactNumberInfo) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1014, 41], "end": [1014, 51], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:1014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d0869c08008dbfa10075c24"></a>
## serialize

`function` · `sqlparser::ast::data_type::ExactNumberInfo::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1015, 38], "end": [1015, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:1015`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f91920117ac6671aee227e6"></a>
## visit

`function` · `sqlparser::ast::data_type::ExactNumberInfo::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1016, 47], "end": [1016, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:1016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d52a19350205ec196ee5f97"></a>
## visit

`function` · `sqlparser::ast::data_type::ExactNumberInfo::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::ExactNumberInfo", "path": "ExactNumberInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1016, 40], "end": [1016, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:1016`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
