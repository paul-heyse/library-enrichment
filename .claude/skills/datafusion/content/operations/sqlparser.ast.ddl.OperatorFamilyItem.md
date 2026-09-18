# `sqlparser::ast::ddl::OperatorFamilyItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorFamilyItem.json).

<a id="op-de6e285ca379bb060339f924"></a>
## OperatorFamilyItem

`enum` · `sqlparser::ast::ddl::OperatorFamilyItem` · sqlparser 0.62.0

```rust
enum OperatorFamilyItem
```

Source: `src/ast/ddl.rs:5104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An item in an ALTER OPERATOR FAMILY ADD statement

<a id="op-e109624af9515380e8fe7cf8"></a>
## Function

`variant` · `sqlparser::ast::ddl::OperatorFamilyItem::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/ddl.rs:5117`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FUNCTION` clause in an operator family modification.

<a id="op-1ae6c5ae94b0034813c2326c"></a>
## Operator

`variant` · `sqlparser::ast::ddl::OperatorFamilyItem::Operator` · sqlparser 0.62.0

```rust
Operator
```

Source: `src/ast/ddl.rs:5106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OPERATOR` clause in an operator family modification.

<a id="op-aadbb65f338e9cfcc8b97bec"></a>
## clone

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OperatorFamilyItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5101, 17], "end": [5101, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2920d0c663374ce64b620e19"></a>
## cmp

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OperatorFamilyItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5101, 51], "end": [5101, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50ab59dc4f6448cba196b935"></a>
## deserialize

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5102, 49], "end": [5102, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb25b677bedceeb190fc4fb3"></a>
## eq

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OperatorFamilyItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5101, 24], "end": [5101, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d2f3b71be1f8a9f9eeb9c6b"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5150, 1], "end": [5187, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5151`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e616165f88c4f7d63b058a"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5101, 10], "end": [5101, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3a26a69f79bc455494dacd0"></a>
## hash

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5101, 56], "end": [5101, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c9ea6d5e05a2cb083cf3555"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OperatorFamilyItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5101, 35], "end": [5101, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7fa1778923b9e174afc7b40"></a>
## serialize

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5102, 38], "end": [5102, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-151ca6c8d855eb0167394cec"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5103, 47], "end": [5103, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64d6a9c52c4877d87a435938"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorFamilyItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyItem", "path": "OperatorFamilyItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5103, 40], "end": [5103, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
