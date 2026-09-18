# `sqlparser::ast::data_type::GeometricTypeKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.GeometricTypeKind.json).

<a id="op-71cf7e21770ff9a88bd9aa31"></a>
## GeometricTypeKind

`enum` · `sqlparser::ast::data_type::GeometricTypeKind` · sqlparser 0.62.0

```rust
enum GeometricTypeKind
```

Source: `src/ast/data_type.rs:1158`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents different types of geometric shapes which are commonly used in
PostgreSQL/Redshift for spatial operations and geometry-related computations.

[PostgreSQL]: https://www.postgresql.org/docs/9.5/functions-geometry.html

<a id="op-5f9bfef263a7b45457e7c748"></a>
## Circle

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::Circle` · sqlparser 0.62.0

```rust
Circle
```

Source: `src/ast/data_type.rs:1172`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Circle geometry

<a id="op-7a5af02966b3cb5f2283e656"></a>
## GeometricBox

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::GeometricBox` · sqlparser 0.62.0

```rust
GeometricBox
```

Source: `src/ast/data_type.rs:1166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Box geometry

<a id="op-3e30c1eff44f4081b995d6f1"></a>
## GeometricPath

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::GeometricPath` · sqlparser 0.62.0

```rust
GeometricPath
```

Source: `src/ast/data_type.rs:1168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Path geometry

<a id="op-00adc71dc26a4eaeabb60b96"></a>
## Line

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::Line` · sqlparser 0.62.0

```rust
Line
```

Source: `src/ast/data_type.rs:1162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Line geometry

<a id="op-d54436f7e846c3c5e2336adb"></a>
## LineSegment

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::LineSegment` · sqlparser 0.62.0

```rust
LineSegment
```

Source: `src/ast/data_type.rs:1164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Line segment geometry

<a id="op-86b5f067267fb1373b81579f"></a>
## Point

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::Point` · sqlparser 0.62.0

```rust
Point
```

Source: `src/ast/data_type.rs:1160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Point geometry

<a id="op-cd3de929030b66b799c18ead"></a>
## Polygon

`variant` · `sqlparser::ast::data_type::GeometricTypeKind::Polygon` · sqlparser 0.62.0

```rust
Polygon
```

Source: `src/ast/data_type.rs:1170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Polygon geometry

<a id="op-9bbb0d1c155b9dfe15f3ebc6"></a>
## clone

`function` · `sqlparser::ast::data_type::GeometricTypeKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GeometricTypeKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 23], "end": [1155, 28], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/data_type.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b93ee9e899b92ed417d89904"></a>
## cmp

`function` · `sqlparser::ast::data_type::GeometricTypeKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GeometricTypeKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 63], "end": [1155, 66], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/data_type.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad333c215adba4c2711c8a20"></a>
## deserialize

`function` · `sqlparser::ast::data_type::GeometricTypeKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1156, 49], "end": [1156, 60], "filename": "src/ast/data_type.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/data_type.rs:1156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0be2bda69812e59e9647f07"></a>
## eq

`function` · `sqlparser::ast::data_type::GeometricTypeKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GeometricTypeKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 30], "end": [1155, 39], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/data_type.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-305d0e473518ac25c21a1197"></a>
## fmt

`function` · `sqlparser::ast::data_type::GeometricTypeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 10], "end": [1155, 15], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/data_type.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b74127a2ee24a43c11af0d1"></a>
## fmt

`function` · `sqlparser::ast::data_type::GeometricTypeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1175, 1], "end": [1187, 2], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/data_type.rs:1176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b78a7aea7de6c40fb854b88d"></a>
## hash

`function` · `sqlparser::ast::data_type::GeometricTypeKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 45], "end": [1155, 49], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/data_type.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa430f2efbfa0785a7277246"></a>
## partial_cmp

`function` · `sqlparser::ast::data_type::GeometricTypeKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GeometricTypeKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1155, 51], "end": [1155, 61], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/data_type.rs:1155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44800fd30c138740380ec18e"></a>
## serialize

`function` · `sqlparser::ast::data_type::GeometricTypeKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1156, 38], "end": [1156, 47], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/data_type.rs:1156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e9d8753cc1fa1617a9a6299"></a>
## visit

`function` · `sqlparser::ast::data_type::GeometricTypeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1157, 40], "end": [1157, 45], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/data_type.rs:1157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6ee0124ef194276c3e9a8b1"></a>
## visit

`function` · `sqlparser::ast::data_type::GeometricTypeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::data_type::GeometricTypeKind", "path": "GeometricTypeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1157, 47], "end": [1157, 55], "filename": "src/ast/data_type.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/data_type.rs:1157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
