# `sqlparser::ast::ddl::IdentityProperty`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IdentityProperty.json).

<a id="op-d7477b5e893e2fcb3b34d3d2"></a>
## IdentityProperty

`struct` · `sqlparser::ast::ddl::IdentityProperty` · sqlparser 0.62.0

```rust
struct IdentityProperty
```

Source: `src/ast/ddl.rs:1712`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Properties for the `IDENTITY` / `AUTOINCREMENT` column option.

<a id="op-59aa9e279d3d8793353fb939"></a>
## clone

`function` · `sqlparser::ast::ddl::IdentityProperty::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IdentityProperty
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1709, 17], "end": [1709, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-380b06d354b1f773e576ea45"></a>
## cmp

`function` · `sqlparser::ast::ddl::IdentityProperty::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IdentityProperty) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1709, 51], "end": [1709, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-008530186e13957dc4c3686b"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IdentityProperty::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1710, 49], "end": [1710, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cba9ca9a48ce61d389450dd7"></a>
## eq

`function` · `sqlparser::ast::ddl::IdentityProperty::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IdentityProperty) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1709, 24], "end": [1709, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20c56d3c5d12e8f8f0203930"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityProperty::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1709, 10], "end": [1709, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7aeb846c9e1f449b3ee2538"></a>
## hash

`function` · `sqlparser::ast::ddl::IdentityProperty::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1709, 56], "end": [1709, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a4fdf85ae768d7533d29fd9"></a>
## order

`struct_field` · `sqlparser::ast::ddl::IdentityProperty::order` · sqlparser 0.62.0

```rust
order: Option<IdentityPropertyOrder>
```

Source: `src/ast/ddl.rs:1716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional ordering specifier (`ORDER` / `NOORDER`).

<a id="op-0336dc23cce669d378a83401"></a>
## parameters

`struct_field` · `sqlparser::ast::ddl::IdentityProperty::parameters` · sqlparser 0.62.0

```rust
parameters: Option<IdentityPropertyFormatKind>
```

Source: `src/ast/ddl.rs:1714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional parameters specifying seed/increment for the identity column.

<a id="op-bef77965362a8d53f0a70748"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IdentityProperty::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IdentityProperty) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1709, 35], "end": [1709, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1709`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e7616cab92c7a1bc50b61bf"></a>
## serialize

`function` · `sqlparser::ast::ddl::IdentityProperty::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1710, 38], "end": [1710, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1710`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ea3c5d66ab39675f4503cef"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityProperty::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1711, 40], "end": [1711, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c734bd5142740b53ec28b8b8"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityProperty::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityProperty", "path": "IdentityProperty"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1711, 47], "end": [1711, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1711`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
