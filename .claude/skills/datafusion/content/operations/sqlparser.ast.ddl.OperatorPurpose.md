# `sqlparser::ast::ddl::OperatorPurpose`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorPurpose.json).

<a id="op-bacdebad62d8665b0f4bb471"></a>
## OperatorPurpose

`enum` · `sqlparser::ast::ddl::OperatorPurpose` · sqlparser 0.62.0

```rust
enum OperatorPurpose
```

Source: `src/ast/ddl.rs:4905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Purpose of an operator in an operator class

<a id="op-bc2fe64e4c7ba21d5cefabeb"></a>
## ForOrderBy

`variant` · `sqlparser::ast::ddl::OperatorPurpose::ForOrderBy` · sqlparser 0.62.0

```rust
ForOrderBy
```

Source: `src/ast/ddl.rs:4909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Purpose: used for ORDER BY; optionally includes a sort family name.

<a id="op-e946b744dc91a13f1e521bcd"></a>
## ForSearch

`variant` · `sqlparser::ast::ddl::OperatorPurpose::ForSearch` · sqlparser 0.62.0

```rust
ForSearch
```

Source: `src/ast/ddl.rs:4907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Purpose: used for index/search operations.

<a id="op-c69e3a62a2099b356128e969"></a>
## clone

`function` · `sqlparser::ast::ddl::OperatorPurpose::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OperatorPurpose
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4902, 17], "end": [4902, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f086ef75f0c2f047034a65fb"></a>
## cmp

`function` · `sqlparser::ast::ddl::OperatorPurpose::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OperatorPurpose) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4902, 51], "end": [4902, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acd37e40e4d17e79f4574ad0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::OperatorPurpose::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4903, 49], "end": [4903, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75c0dff8d93fa98d36650d0c"></a>
## eq

`function` · `sqlparser::ast::ddl::OperatorPurpose::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OperatorPurpose) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4902, 24], "end": [4902, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0e5c721b030655e7bd9094"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorPurpose::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4956, 1], "end": [4965, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4957`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b0a3a487aa5a6f27bdda93f"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorPurpose::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4902, 10], "end": [4902, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e3d4d2b08e5442de054f22f"></a>
## hash

`function` · `sqlparser::ast::ddl::OperatorPurpose::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4902, 56], "end": [4902, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-412b1f58c09696c5d94dcb4c"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::OperatorPurpose::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OperatorPurpose) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4902, 35], "end": [4902, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a1bdb8772bdfe8c5f7c7a9a"></a>
## serialize

`function` · `sqlparser::ast::ddl::OperatorPurpose::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4903, 38], "end": [4903, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4a81f0102e99bf6cf4f786b"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorPurpose::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4904, 40], "end": [4904, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d73f770a7100837f4a14327a"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorPurpose::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorPurpose", "path": "OperatorPurpose"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4904, 47], "end": [4904, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
