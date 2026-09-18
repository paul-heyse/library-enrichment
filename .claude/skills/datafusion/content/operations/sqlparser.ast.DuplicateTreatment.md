# `sqlparser::ast::DuplicateTreatment`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DuplicateTreatment.json).

<a id="op-8262cbd180c87d6214fc24b2"></a>
## DuplicateTreatment

`enum` · `sqlparser::ast::DuplicateTreatment` · sqlparser 0.62.0

```rust
enum DuplicateTreatment
```

Source: `src/ast/mod.rs:8286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

How duplicate values are treated inside function argument lists.

<a id="op-24c0bb916e9360869901ceed"></a>
## All

`variant` · `sqlparser::ast::DuplicateTreatment::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/mod.rs:8290`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Retain all duplicate values (the default).

<a id="op-7a2aa5d8e8e682f92b651fb2"></a>
## Distinct

`variant` · `sqlparser::ast::DuplicateTreatment::Distinct` · sqlparser 0.62.0

```rust
Distinct
```

Source: `src/ast/mod.rs:8288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Consider only unique values.

<a id="op-879f156d7c6a608d6dd2106a"></a>
## clone

`function` · `sqlparser::ast::DuplicateTreatment::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DuplicateTreatment
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8282, 23], "end": [8282, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0407c0947e8c71067baf2f44"></a>
## cmp

`function` · `sqlparser::ast::DuplicateTreatment::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DuplicateTreatment) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8282, 57], "end": [8282, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c6103b31fc8ccae991267b9"></a>
## deserialize

`function` · `sqlparser::ast::DuplicateTreatment::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8283, 49], "end": [8283, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c29bf15f77835d91afa8147"></a>
## eq

`function` · `sqlparser::ast::DuplicateTreatment::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DuplicateTreatment) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8282, 30], "end": [8282, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c26d7af2ab4deb81645300d"></a>
## fmt

`function` · `sqlparser::ast::DuplicateTreatment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8293, 1], "end": [8300, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0ae2422a7a0338376791e13"></a>
## fmt

`function` · `sqlparser::ast::DuplicateTreatment::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8282, 10], "end": [8282, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-184b40a4f029275cff68c4db"></a>
## hash

`function` · `sqlparser::ast::DuplicateTreatment::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8282, 62], "end": [8282, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c4ee46d5212afb59b5feaa3"></a>
## partial_cmp

`function` · `sqlparser::ast::DuplicateTreatment::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DuplicateTreatment) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8282, 41], "end": [8282, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8282`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56f5cf5aa53a034881f1ba17"></a>
## serialize

`function` · `sqlparser::ast::DuplicateTreatment::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8283, 38], "end": [8283, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f2b38838d0f4cdb29ee2c4f"></a>
## visit

`function` · `sqlparser::ast::DuplicateTreatment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8284, 47], "end": [8284, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-747be863923afc57ad1b0af7"></a>
## visit

`function` · `sqlparser::ast::DuplicateTreatment::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DuplicateTreatment", "path": "DuplicateTreatment"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8284, 40], "end": [8284, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8284`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
