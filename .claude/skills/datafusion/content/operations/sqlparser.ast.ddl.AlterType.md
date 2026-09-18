# `sqlparser::ast::ddl::AlterType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterType.json).

<a id="op-3a8ee7ae8d2120bc093b3269"></a>
## AlterType

`struct` · `sqlparser::ast::ddl::AlterType` · sqlparser 0.62.0

```rust
struct AlterType
```

Source: `src/ast/ddl.rs:1065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER TYPE` statement (`Statement::AlterType`)

<a id="op-6ac96fcf481a533134640c66"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 17], "end": [1062, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7c4efb7f50ab4047f734214"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 51], "end": [1062, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1afa8cae9201722c7de67d16"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1063, 49], "end": [1063, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2711558dd20205a1a79a654d"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 24], "end": [1062, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31444481f6725cbc4d71021e"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 10], "end": [1062, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-195de3f52fc6dacdebf3bb58"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 56], "end": [1062, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7fbb9c067d9747e803b4405"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterType::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:1067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the type being altered (may be schema-qualified).

<a id="op-c50895badb662efbd37a2054"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterType::operation` · sqlparser 0.62.0

```rust
operation: AlterTypeOperation
```

Source: `src/ast/ddl.rs:1069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The specific alteration operation to perform.

<a id="op-b8572718cff9fcedfde038f7"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1062, 35], "end": [1062, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce0c8cd7269fc348d4281cbc"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1063, 38], "end": [1063, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4e31fd8944b3044bf4b9e4a"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1064, 40], "end": [1064, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1064`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f07480741a6c3a878d181c59"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterType", "path": "AlterType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1064, 47], "end": [1064, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1064`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
