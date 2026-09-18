# `sqlparser::ast::UnionField`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.UnionField.json).

<a id="op-89ce10bcd78bd10de59fa34c"></a>
## UnionField

`struct` · `sqlparser::ast::UnionField` · sqlparser 0.62.0

```rust
struct UnionField
```

Source: `src/ast/mod.rs:594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A field definition within a union

[DuckDB]: https://duckdb.org/docs/sql/data_types/union.html

<a id="op-5e12508eef0e33d0d48a2bb2"></a>
## clone

`function` · `sqlparser::ast::UnionField::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UnionField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 17], "end": [591, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82790bd79437a90f8044bfdf"></a>
## cmp

`function` · `sqlparser::ast::UnionField::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UnionField) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 51], "end": [591, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fea49c7370a44375321da298"></a>
## deserialize

`function` · `sqlparser::ast::UnionField::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 49], "end": [592, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b6062f92d89986ac885d865"></a>
## eq

`function` · `sqlparser::ast::UnionField::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UnionField) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 24], "end": [591, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8b6329fd44e196247bdc2a3"></a>
## field_name

`struct_field` · `sqlparser::ast::UnionField::field_name` · sqlparser 0.62.0

```rust
field_name: Ident
```

Source: `src/ast/mod.rs:596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the union field.

<a id="op-06406d356eb03686932199d4"></a>
## field_type

`struct_field` · `sqlparser::ast::UnionField::field_type` · sqlparser 0.62.0

```rust
field_type: DataType
```

Source: `src/ast/mod.rs:598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type of the union field.

<a id="op-462d7505b0838886eab28c47"></a>
## fmt

`function` · `sqlparser::ast::UnionField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 10], "end": [591, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a93eec452e4cc748266913a"></a>
## fmt

`function` · `sqlparser::ast::UnionField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [601, 1], "end": [605, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:602`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-693586b078d8e807f4e859d2"></a>
## hash

`function` · `sqlparser::ast::UnionField::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 56], "end": [591, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75dd6ca7cb3673233f466587"></a>
## partial_cmp

`function` · `sqlparser::ast::UnionField::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UnionField) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 35], "end": [591, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94c672827b4308790b0443cd"></a>
## serialize

`function` · `sqlparser::ast::UnionField::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 38], "end": [592, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72cfe21e3458d52a2b49b952"></a>
## visit

`function` · `sqlparser::ast::UnionField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 40], "end": [593, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f09925ab7765f469e70921"></a>
## visit

`function` · `sqlparser::ast::UnionField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::UnionField", "path": "UnionField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [593, 47], "end": [593, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
