# `sqlparser::ast::DropDomain`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DropDomain.json).

<a id="op-b5a3196743f7026584031438"></a>
## DropDomain

`struct` · `sqlparser::ast::DropDomain` · sqlparser 0.62.0

```rust
struct DropDomain
```

Source: `src/ast/mod.rs:7989`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A Drop Domain statement

<a id="op-2240f38914ed1154a14f1f2d"></a>
## clone

`function` · `sqlparser::ast::DropDomain::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropDomain
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7986, 17], "end": [7986, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3100ec02cca48e53c29b055"></a>
## cmp

`function` · `sqlparser::ast::DropDomain::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropDomain) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7986, 51], "end": [7986, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8878a5e4141d50c11e6c5067"></a>
## deserialize

`function` · `sqlparser::ast::DropDomain::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7987, 49], "end": [7987, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dc08569cd2c0dac8d6f90f8"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::DropDomain::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/mod.rs:7995`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The behavior to apply when dropping the domain

<a id="op-d3f695d6603c6c96c17ed555"></a>
## eq

`function` · `sqlparser::ast::DropDomain::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropDomain) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7986, 24], "end": [7986, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31bc32c654db7387af5e46ae"></a>
## fmt

`function` · `sqlparser::ast::DropDomain::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7986, 10], "end": [7986, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b42748317de5916b02831bd"></a>
## hash

`function` · `sqlparser::ast::DropDomain::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7986, 56], "end": [7986, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8575c16f1c62bfe15812f2dd"></a>
## if_exists

`struct_field` · `sqlparser::ast::DropDomain::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/mod.rs:7991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to drop the domain if it exists

<a id="op-c6bc9a6d6dd90d60caa5048b"></a>
## name

`struct_field` · `sqlparser::ast::DropDomain::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:7993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the domain to drop

<a id="op-c523ab7a2f60b5f7d2a4f1c1"></a>
## partial_cmp

`function` · `sqlparser::ast::DropDomain::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropDomain) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7986, 35], "end": [7986, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-924f32136c6efbaf1c779548"></a>
## serialize

`function` · `sqlparser::ast::DropDomain::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7987, 38], "end": [7987, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dba79581aecb7604dbd211c"></a>
## visit

`function` · `sqlparser::ast::DropDomain::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7988, 47], "end": [7988, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7988`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df7d7b2ba926de40c1ac3db6"></a>
## visit

`function` · `sqlparser::ast::DropDomain::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DropDomain", "path": "DropDomain"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7988, 40], "end": [7988, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7988`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
