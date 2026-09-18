# `sqlparser::ast::query::NamedWindowDefinition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.NamedWindowDefinition.json).

<a id="op-22e02061093138df34d33bc7"></a>
## NamedWindowDefinition

`struct` · `sqlparser::ast::query::NamedWindowDefinition` · sqlparser 0.62.0

```rust
struct NamedWindowDefinition
```

Source: `src/ast/query.rs:741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A named window definition: `<name> AS <window specification>`

<a id="op-7a4c993244a50a91a888fe7f"></a>
## 0

`struct_field` · `sqlparser::ast::query::NamedWindowDefinition::0` · sqlparser 0.62.0

```rust
0: Ident
```

Source: `src/ast/query.rs:741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f760bd5d8602f1be7d2620c6"></a>
## 1

`struct_field` · `sqlparser::ast::query::NamedWindowDefinition::1` · sqlparser 0.62.0

```rust
1: NamedWindowExpr
```

Source: `src/ast/query.rs:741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc7537ec9b2b76449d1286ad"></a>
## clone

`function` · `sqlparser::ast::query::NamedWindowDefinition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NamedWindowDefinition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 17], "end": [737, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04d9a62a100c0fbadb41fbf7"></a>
## cmp

`function` · `sqlparser::ast::query::NamedWindowDefinition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NamedWindowDefinition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 51], "end": [737, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60ffed946d7fb51892c776c6"></a>
## deserialize

`function` · `sqlparser::ast::query::NamedWindowDefinition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [738, 49], "end": [738, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7435b2b0426ca489d6c62ff"></a>
## eq

`function` · `sqlparser::ast::query::NamedWindowDefinition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NamedWindowDefinition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 24], "end": [737, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-260e2c4af1f5fbfa34284493"></a>
## fmt

`function` · `sqlparser::ast::query::NamedWindowDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [743, 1], "end": [747, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5599d2170178fdbda230ef9"></a>
## fmt

`function` · `sqlparser::ast::query::NamedWindowDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 10], "end": [737, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bb8a8c2b4f8d499141b8f75"></a>
## hash

`function` · `sqlparser::ast::query::NamedWindowDefinition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 56], "end": [737, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73c912f0c35f63f595b8084a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::NamedWindowDefinition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NamedWindowDefinition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [737, 35], "end": [737, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ceecba1c7d24faf82f142f4"></a>
## serialize

`function` · `sqlparser::ast::query::NamedWindowDefinition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [738, 38], "end": [738, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:738`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70a21f2af5a59e7b437f0dca"></a>
## span

`function` · `sqlparser::ast::query::NamedWindowDefinition::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "super::NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2354, 1], "end": [2363, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-331bfb41116d43ac7f742ff7"></a>
## visit

`function` · `sqlparser::ast::query::NamedWindowDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [739, 47], "end": [739, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dd6557f634d1a05c758381a"></a>
## visit

`function` · `sqlparser::ast::query::NamedWindowDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NamedWindowDefinition", "path": "NamedWindowDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [739, 40], "end": [739, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
