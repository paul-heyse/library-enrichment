# `sqlparser::ast::query::XmlNamespaceDefinition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.XmlNamespaceDefinition.json).

<a id="op-40a5e5aa61c23bc221902ccb"></a>
## XmlNamespaceDefinition

`struct` · `sqlparser::ast::query::XmlNamespaceDefinition` · sqlparser 0.62.0

```rust
struct XmlNamespaceDefinition
```

Source: `src/ast/query.rs:4317`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a single XML namespace definition in the XMLNAMESPACES clause.

`namespace_uri AS namespace_name`

<a id="op-1eda4108aa78d978ff5d3f9a"></a>
## clone

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> XmlNamespaceDefinition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4314, 17], "end": [4314, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1217ce1e84ebbbaa6284375d"></a>
## cmp

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &XmlNamespaceDefinition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4314, 51], "end": [4314, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b46e84b0441b748a7a26b4d7"></a>
## deserialize

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4316, 49], "end": [4316, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f64e649c260ea4537197b88"></a>
## eq

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &XmlNamespaceDefinition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4314, 24], "end": [4314, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b2a25355d0e67eb83c505f3"></a>
## fmt

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4324, 1], "end": [4328, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c4de0495df0ea7b075a14ea"></a>
## fmt

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4314, 10], "end": [4314, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f6b24df66ee71ab54cabdbe"></a>
## hash

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4314, 56], "end": [4314, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4790f198ab4698fe6b0e6546"></a>
## name

`struct_field` · `sqlparser::ast::query::XmlNamespaceDefinition::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/query.rs:4321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias for the namespace (a simple identifier).

<a id="op-a0602dece4291a507386d5f2"></a>
## partial_cmp

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &XmlNamespaceDefinition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4314, 35], "end": [4314, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4314`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2f20e688a46b600e71afb04"></a>
## serialize

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4316, 38], "end": [4316, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4316`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-860c4a496f07c256f374d9c4"></a>
## uri

`struct_field` · `sqlparser::ast::query::XmlNamespaceDefinition::uri` · sqlparser 0.62.0

```rust
uri: Expr
```

Source: `src/ast/query.rs:4319`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The namespace URI (a text expression).

<a id="op-87a6b0f868235c121de760aa"></a>
## visit

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4315, 47], "end": [4315, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff1fbd778e6107407dc2bbf3"></a>
## visit

`function` · `sqlparser::ast::query::XmlNamespaceDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::XmlNamespaceDefinition", "path": "XmlNamespaceDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4315, 40], "end": [4315, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
