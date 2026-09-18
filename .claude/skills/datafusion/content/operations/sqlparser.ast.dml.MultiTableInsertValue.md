# `sqlparser::ast::dml::MultiTableInsertValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MultiTableInsertValue.json).

<a id="op-1aefd027288f48202588d22f"></a>
## MultiTableInsertValue

`enum` · `sqlparser::ast::dml::MultiTableInsertValue` · sqlparser 0.62.0

```rust
enum MultiTableInsertValue
```

Source: `src/ast/dml.rs:877`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A value in a multi-table INSERT VALUES clause.

<a id="op-c0305933c80cd252c1dc77d7"></a>
## Default

`variant` · `sqlparser::ast::dml::MultiTableInsertValue::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/dml.rs:881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The DEFAULT keyword

<a id="op-ef0663a660b3cc228fe7a29f"></a>
## Expr

`variant` · `sqlparser::ast::dml::MultiTableInsertValue::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/dml.rs:879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A column reference or expression from the source

<a id="op-cbcdb5e1f9b0ed1d4a64af11"></a>
## clone

`function` · `sqlparser::ast::dml::MultiTableInsertValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MultiTableInsertValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 17], "end": [874, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85ebb24af067ea007c687a71"></a>
## cmp

`function` · `sqlparser::ast::dml::MultiTableInsertValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MultiTableInsertValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 51], "end": [874, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a914b847d0d3f8638679a5ec"></a>
## deserialize

`function` · `sqlparser::ast::dml::MultiTableInsertValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [875, 49], "end": [875, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27834493616a578257fff1d5"></a>
## eq

`function` · `sqlparser::ast::dml::MultiTableInsertValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MultiTableInsertValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 24], "end": [874, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81b00a3c11805f46fbff8f69"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [884, 1], "end": [891, 2], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dml.rs:885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-839fe2ee909621ef2873c3b4"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 10], "end": [874, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-392bb6b6949a7f246f0a1549"></a>
## hash

`function` · `sqlparser::ast::dml::MultiTableInsertValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 56], "end": [874, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-460085564c160a47236632b1"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MultiTableInsertValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MultiTableInsertValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [874, 35], "end": [874, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6545d041a94690762791542"></a>
## serialize

`function` · `sqlparser::ast::dml::MultiTableInsertValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [875, 38], "end": [875, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a09baba8c332a0215bc965b"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [876, 47], "end": [876, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9bd8088c234dc5fecbe2b5b"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValue", "path": "MultiTableInsertValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [876, 40], "end": [876, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
