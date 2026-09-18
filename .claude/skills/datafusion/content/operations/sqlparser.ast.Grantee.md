# `sqlparser::ast::Grantee`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Grantee.json).

<a id="op-9b67beb1b563687f21c33f90"></a>
## Grantee

`struct` · `sqlparser::ast::Grantee` · sqlparser 0.62.0

```rust
struct Grantee
```

Source: `src/ast/mod.rs:7411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The principal that receives the privileges

<a id="op-3f3ace4d39f4ee580f626bfa"></a>
## clone

`function` · `sqlparser::ast::Grantee::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Grantee
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7408, 17], "end": [7408, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eadca1c21a356d5a96d306ea"></a>
## cmp

`function` · `sqlparser::ast::Grantee::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Grantee) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7408, 51], "end": [7408, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c532376a2d24fe680ef56f5"></a>
## deserialize

`function` · `sqlparser::ast::Grantee::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7409, 49], "end": [7409, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e862aa2c763a18d01b366012"></a>
## eq

`function` · `sqlparser::ast::Grantee::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Grantee) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7408, 24], "end": [7408, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f6c9428a5e819ec94f7fa64"></a>
## fmt

`function` · `sqlparser::ast::Grantee::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7418, 1], "end": [7452, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-414f81871eed147a450cc7df"></a>
## fmt

`function` · `sqlparser::ast::Grantee::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7408, 10], "end": [7408, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6beb16e6cbb093f65509a0f5"></a>
## grantee_type

`struct_field` · `sqlparser::ast::Grantee::grantee_type` · sqlparser 0.62.0

```rust
grantee_type: GranteesType
```

Source: `src/ast/mod.rs:7413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The category/type of grantee (role, user, share, etc.).

<a id="op-c423b25d3af9701c6e8a8354"></a>
## hash

`function` · `sqlparser::ast::Grantee::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7408, 56], "end": [7408, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f40d383b8082dc2758b2c53c"></a>
## name

`struct_field` · `sqlparser::ast::Grantee::name` · sqlparser 0.62.0

```rust
name: Option<GranteeName>
```

Source: `src/ast/mod.rs:7415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional name of the grantee (identifier or user@host).

<a id="op-2665a670b11148873999a112"></a>
## partial_cmp

`function` · `sqlparser::ast::Grantee::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Grantee) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7408, 35], "end": [7408, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7408`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c78320baadbcaed3439a57ce"></a>
## serialize

`function` · `sqlparser::ast::Grantee::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7409, 38], "end": [7409, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82742b5c5aeb3a59a005f9ec"></a>
## visit

`function` · `sqlparser::ast::Grantee::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7410, 47], "end": [7410, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b93f7a8ebbb16c1b46f3eebd"></a>
## visit

`function` · `sqlparser::ast::Grantee::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Grantee", "path": "Grantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7410, 40], "end": [7410, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7410`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
