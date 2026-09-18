# `sqlparser::ast::SetSessionParamIdentityInsert`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionParamIdentityInsert.json).

<a id="op-21c5545e136c54f621a47fd5"></a>
## SetSessionParamIdentityInsert

`struct` · `sqlparser::ast::SetSessionParamIdentityInsert` · sqlparser 0.62.0

```rust
struct SetSessionParamIdentityInsert
```

Source: `src/ast/mod.rs:11122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IDENTITY_INSERT` session parameter for a specific object.

<a id="op-2fe4abcfb92b4ebbb37858ab"></a>
## clone

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionParamIdentityInsert
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11118, 17], "end": [11118, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-008f137b46e8b1f4f6dd3292"></a>
## cmp

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionParamIdentityInsert) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11118, 51], "end": [11118, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29584abb0f9df44ba8c98088"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11119, 49], "end": [11119, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82eb38d142ba56b6635c1d04"></a>
## eq

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionParamIdentityInsert) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11118, 24], "end": [11118, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bb67a0dc4dd9c9e3792b73a"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11129, 1], "end": [11133, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b04e79e9a4acbf9c7fde0c2"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11118, 10], "end": [11118, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-313a35ad4ad2e8f27ed02f9e"></a>
## hash

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11118, 56], "end": [11118, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02dd1ac8638bcbb4a93471af"></a>
## obj

`struct_field` · `sqlparser::ast::SetSessionParamIdentityInsert::obj` · sqlparser 0.62.0

```rust
obj: ObjectName
```

Source: `src/ast/mod.rs:11124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Object name targeted by `IDENTITY_INSERT`.

<a id="op-a0b36d52d6d3875b750a0d0c"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionParamIdentityInsert) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11118, 35], "end": [11118, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11118`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c1ccbc38da35ea3ba6b6241"></a>
## serialize

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11119, 38], "end": [11119, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9de72b9d055f6de0641d0f9"></a>
## value

`struct_field` · `sqlparser::ast::SetSessionParamIdentityInsert::value` · sqlparser 0.62.0

```rust
value: SessionParamValue
```

Source: `src/ast/mod.rs:11126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value (ON/OFF) for the identity insert setting.

<a id="op-2ced9a9119c2df9e8e195321"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11120, 40], "end": [11120, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f47e53b5be3ac67393fd078"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamIdentityInsert::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamIdentityInsert", "path": "SetSessionParamIdentityInsert"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11120, 47], "end": [11120, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
