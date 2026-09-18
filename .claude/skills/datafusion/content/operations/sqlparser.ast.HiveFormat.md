# `sqlparser::ast::HiveFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveFormat.json).

<a id="op-76f2ff8487452f9531855dbc"></a>
## HiveFormat

`struct` · `sqlparser::ast::HiveFormat` · sqlparser 0.62.0

```rust
struct HiveFormat
```

Source: `src/ast/mod.rs:8715`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Hive table format and storage-related options.

<a id="op-54aaee0c16278de28c0622f7"></a>
## clone

`function` · `sqlparser::ast::HiveFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 17], "end": [8711, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d6fca45680532a47d3e6be0"></a>
## cmp

`function` · `sqlparser::ast::HiveFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 51], "end": [8711, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-035d0c037c4c9e3f8808198f"></a>
## default

`function` · `sqlparser::ast::HiveFormat::default` · sqlparser 0.62.0

```rust
fn default() -> HiveFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 62], "end": [8711, 69], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a715be76f038a038d342bd9c"></a>
## deserialize

`function` · `sqlparser::ast::HiveFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8712, 49], "end": [8712, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57af599d865adcb7cc0cb097"></a>
## eq

`function` · `sqlparser::ast::HiveFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 24], "end": [8711, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-830ea7d13206fd213069153b"></a>
## fmt

`function` · `sqlparser::ast::HiveFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 10], "end": [8711, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c79fc3c5b63627e86cee31b6"></a>
## hash

`function` · `sqlparser::ast::HiveFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 56], "end": [8711, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6393122a9feba25fcd41e2a8"></a>
## location

`struct_field` · `sqlparser::ast::HiveFormat::location` · sqlparser 0.62.0

```rust
location: Option<String>
```

Source: `src/ast/mod.rs:8723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional location (URI or path) for table data.

<a id="op-8d191ecf13e2ac3daf34cbf2"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8711, 35], "end": [8711, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-774d8e5361ae89220ca8f8e6"></a>
## row_format

`struct_field` · `sqlparser::ast::HiveFormat::row_format` · sqlparser 0.62.0

```rust
row_format: Option<HiveRowFormat>
```

Source: `src/ast/mod.rs:8717`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional row format specification.

<a id="op-c330b71f47bdea1d9fb959d7"></a>
## serde_properties

`struct_field` · `sqlparser::ast::HiveFormat::serde_properties` · sqlparser 0.62.0

```rust
serde_properties: Option<Vec<SqlOption>>
```

Source: `src/ast/mod.rs:8719`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional SerDe properties expressed as SQL options.

<a id="op-dbdcb16bcd562ea6f7e7af59"></a>
## serialize

`function` · `sqlparser::ast::HiveFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8712, 38], "end": [8712, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68910b6f20ee5ea3c6201707"></a>
## storage

`struct_field` · `sqlparser::ast::HiveFormat::storage` · sqlparser 0.62.0

```rust
storage: Option<HiveIOFormat>
```

Source: `src/ast/mod.rs:8721`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional input/output storage format details.

<a id="op-352c4c05df9db392b16f1603"></a>
## visit

`function` · `sqlparser::ast::HiveFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8713, 47], "end": [8713, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c416a6cd943c80029c4f73d1"></a>
## visit

`function` · `sqlparser::ast::HiveFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveFormat", "path": "HiveFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8713, 40], "end": [8713, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8713`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
