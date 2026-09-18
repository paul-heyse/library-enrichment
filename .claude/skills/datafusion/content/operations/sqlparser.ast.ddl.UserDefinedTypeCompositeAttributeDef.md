# `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.UserDefinedTypeCompositeAttributeDef.json).

<a id="op-72233cdbf8ed3fe7b4c69f7d"></a>
## UserDefinedTypeCompositeAttributeDef

`struct` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef` · sqlparser 0.62.0

```rust
struct UserDefinedTypeCompositeAttributeDef
```

Source: `src/ast/ddl.rs:2419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL user defined type attribute definition

<a id="op-f99e6632762c83197cc170b7"></a>
## clone

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserDefinedTypeCompositeAttributeDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2416, 17], "end": [2416, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-547713737db5d5748bbfc22e"></a>
## cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserDefinedTypeCompositeAttributeDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2416, 51], "end": [2416, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ae1858b9237ef9d488ebf26"></a>
## collation

`struct_field` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::collation` · sqlparser 0.62.0

```rust
collation: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:2425`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional collation for the attribute.

<a id="op-d8ec5564e727c5b898989a63"></a>
## data_type

`struct_field` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::data_type` · sqlparser 0.62.0

```rust
data_type: ast::DataType
```

Source: `src/ast/ddl.rs:2423`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attribute data type.

<a id="op-6a8067426d835d1f7f241ab0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2417, 49], "end": [2417, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-613151ca07f3cfe6e1e8d024"></a>
## eq

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserDefinedTypeCompositeAttributeDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2416, 24], "end": [2416, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c82057a762d7fb662567fc4"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2416, 10], "end": [2416, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-997798b13b2222c4887bb634"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2428, 1], "end": [2436, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b782c4a33198da621325c476"></a>
## hash

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2416, 56], "end": [2416, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78a8a2dc144838bd9caf18d9"></a>
## name

`struct_field` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:2421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attribute name.

<a id="op-74d7276e757c48e5f210a2fb"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserDefinedTypeCompositeAttributeDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2416, 35], "end": [2416, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8caf14fc7d378a16e0b69de7"></a>
## serialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2417, 38], "end": [2417, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c05b01daa6fc48602256c9c"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2418, 47], "end": [2418, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d78b11ddceb36bda2b5559f8"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeCompositeAttributeDef", "path": "UserDefinedTypeCompositeAttributeDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2418, 40], "end": [2418, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
