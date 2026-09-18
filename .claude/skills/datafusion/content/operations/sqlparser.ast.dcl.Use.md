# `sqlparser::ast::dcl::Use`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.Use.json).

<a id="op-3c2ac38eb88a540d35c24148"></a>
## Use

`enum` · `sqlparser::ast::dcl::Use` · sqlparser 0.62.0

```rust
enum Use
```

Source: `src/ast/dcl.rs:245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `USE` (`Statement::Use`) operation

<a id="op-69c279524e6643c0b36ff171"></a>
## Catalog

`variant` · `sqlparser::ast::dcl::Use::Catalog` · sqlparser 0.62.0

```rust
Catalog
```

Source: `src/ast/dcl.rs:247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Switch to the given catalog (e.g. `USE CATALOG ...`).

<a id="op-9211dd7a731b944de1979fe7"></a>
## Database

`variant` · `sqlparser::ast::dcl::Use::Database` · sqlparser 0.62.0

```rust
Database
```

Source: `src/ast/dcl.rs:251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Switch to the given database (e.g. `USE DATABASE ...`).

<a id="op-bc711b662958f9adb321d3d5"></a>
## Default

`variant` · `sqlparser::ast::dcl::Use::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/dcl.rs:261`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Reset to default (e.g. `USE DEFAULT`).

<a id="op-7c4058e12f0895ed195067fe"></a>
## Object

`variant` · `sqlparser::ast::dcl::Use::Object` · sqlparser 0.62.0

```rust
Object
```

Source: `src/ast/dcl.rs:259`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use the specified object (e.g. `USE foo.bar`).

<a id="op-69a8ee11ba86877f154f8db5"></a>
## Role

`variant` · `sqlparser::ast::dcl::Use::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/dcl.rs:255`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Switch to the given role (e.g. `USE ROLE ...`).

<a id="op-87bb6f09b7d1902797ca089d"></a>
## Schema

`variant` · `sqlparser::ast::dcl::Use::Schema` · sqlparser 0.62.0

```rust
Schema
```

Source: `src/ast/dcl.rs:249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Switch to the given schema (e.g. `USE SCHEMA ...`).

<a id="op-7dc104affb657c59f1277c36"></a>
## SecondaryRoles

`variant` · `sqlparser::ast::dcl::Use::SecondaryRoles` · sqlparser 0.62.0

```rust
SecondaryRoles
```

Source: `src/ast/dcl.rs:257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use secondary roles specification (e.g. `USE SECONDARY ROLES ...`).

<a id="op-8ee7f4e0066ac4fb1d666a95"></a>
## Warehouse

`variant` · `sqlparser::ast::dcl::Use::Warehouse` · sqlparser 0.62.0

```rust
Warehouse
```

Source: `src/ast/dcl.rs:253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Switch to the given warehouse (e.g. `USE WAREHOUSE ...`).

<a id="op-c5a83212e73b663f5ab3aa46"></a>
## clone

`function` · `sqlparser::ast::dcl::Use::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Use
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 17], "end": [242, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-410fe020f49a7f4c13dc90a1"></a>
## cmp

`function` · `sqlparser::ast::dcl::Use::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Use) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 51], "end": [242, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed774fdc18b606d7ae101f91"></a>
## deserialize

`function` · `sqlparser::ast::dcl::Use::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 49], "end": [243, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31cbe714f82c1439ad31a6a1"></a>
## eq

`function` · `sqlparser::ast::dcl::Use::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Use) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 24], "end": [242, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f148d339d8794e979e16f2f8"></a>
## fmt

`function` · `sqlparser::ast::dcl::Use::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 10], "end": [242, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa1b621c3961961707926bfe"></a>
## fmt

`function` · `sqlparser::ast::dcl::Use::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [280, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:265`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d2759a016e65aa43d65a569"></a>
## hash

`function` · `sqlparser::ast::dcl::Use::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 56], "end": [242, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1985ce5c1ace96ac5e05f9e4"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::Use::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Use) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [242, 35], "end": [242, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77fd439c8c8cf0f95ae509a7"></a>
## serialize

`function` · `sqlparser::ast::dcl::Use::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [243, 38], "end": [243, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f089af385a3f2a623fd2df65"></a>
## span

`function` · `sqlparser::ast::dcl::Use::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "super::Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [543, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:526`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79d5000469f37aec6f40f56a"></a>
## visit

`function` · `sqlparser::ast::dcl::Use::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 47], "end": [244, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b28ccb0cc9e2bf931422fcb"></a>
## visit

`function` · `sqlparser::ast::dcl::Use::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::Use", "path": "Use"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [244, 40], "end": [244, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:244`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
