# `sqlparser::ast::ddl::OperatorFamilyDropItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorFamilyDropItem.json).

<a id="op-82ccdffdd8a0e4a798a6f4a1"></a>
## OperatorFamilyDropItem

`enum` · `sqlparser::ast::ddl::OperatorFamilyDropItem` · sqlparser 0.62.0

```rust
enum OperatorFamilyDropItem
```

Source: `src/ast/ddl.rs:5133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An item in an ALTER OPERATOR FAMILY DROP statement

<a id="op-a959ce9fa297c0918e1b1626"></a>
## Function

`variant` · `sqlparser::ast::ddl::OperatorFamilyDropItem::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/ddl.rs:5142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FUNCTION` clause for DROP within an operator family.

<a id="op-da1bb143df1125e57995e0be"></a>
## Operator

`variant` · `sqlparser::ast::ddl::OperatorFamilyDropItem::Operator` · sqlparser 0.62.0

```rust
Operator
```

Source: `src/ast/ddl.rs:5135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`OPERATOR` clause for DROP within an operator family.

<a id="op-52b82e8e055df04381ed6f6d"></a>
## clone

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OperatorFamilyDropItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5130, 17], "end": [5130, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-392840e797b92c20c4ed0d37"></a>
## cmp

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OperatorFamilyDropItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5130, 51], "end": [5130, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9210257f8ed6676b32b997eb"></a>
## deserialize

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5131, 49], "end": [5131, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-507da878f73b105bdd918cc9"></a>
## eq

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OperatorFamilyDropItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5130, 24], "end": [5130, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a2267568f185ae453ab55ff"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5130, 10], "end": [5130, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f23741088c4acf5e957ca590"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5189, 1], "end": [5214, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-163028b34417056a45fd8e27"></a>
## hash

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5130, 56], "end": [5130, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68675635e166dd885ccfb9d7"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OperatorFamilyDropItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5130, 35], "end": [5130, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-815c7affff22489b15f85287"></a>
## serialize

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5131, 38], "end": [5131, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-676e832bae971b4e40376f2b"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5132, 47], "end": [5132, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9499758c45f754d3a125253"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorFamilyDropItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorFamilyDropItem", "path": "OperatorFamilyDropItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5132, 40], "end": [5132, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5132`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
