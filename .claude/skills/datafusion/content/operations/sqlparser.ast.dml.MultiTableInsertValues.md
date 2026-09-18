# `sqlparser::ast::dml::MultiTableInsertValues`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.MultiTableInsertValues.json).

<a id="op-13f7ff7657fc7c8024715a25"></a>
## MultiTableInsertValues

`struct` · `sqlparser::ast::dml::MultiTableInsertValues` · sqlparser 0.62.0

```rust
struct MultiTableInsertValues
```

Source: `src/ast/dml.rs:868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The VALUES clause in a multi-table INSERT INTO clause.

<a id="op-edad767e7f80de92a38527e3"></a>
## clone

`function` · `sqlparser::ast::dml::MultiTableInsertValues::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> MultiTableInsertValues
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 17], "end": [865, 22], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dml.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-105411186a57669d0ba0bd7e"></a>
## cmp

`function` · `sqlparser::ast::dml::MultiTableInsertValues::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &MultiTableInsertValues) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 51], "end": [865, 54], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dml.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13d1631244f7d364e94619d5"></a>
## deserialize

`function` · `sqlparser::ast::dml::MultiTableInsertValues::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [866, 49], "end": [866, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dml.rs:866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-262c92f8163e43df45d7ebb9"></a>
## eq

`function` · `sqlparser::ast::dml::MultiTableInsertValues::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &MultiTableInsertValues) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 24], "end": [865, 33], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dml.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afe9cfa20894d3caedb81ee4"></a>
## fmt

`function` · `sqlparser::ast::dml::MultiTableInsertValues::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 10], "end": [865, 15], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dml.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36fc9b12567c41a1780b2d70"></a>
## hash

`function` · `sqlparser::ast::dml::MultiTableInsertValues::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 56], "end": [865, 60], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dml.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d3397d35636c87f8aeff748"></a>
## partial_cmp

`function` · `sqlparser::ast::dml::MultiTableInsertValues::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &MultiTableInsertValues) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 35], "end": [865, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dml.rs:865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a84940c7e7bdcaa5d59d235"></a>
## serialize

`function` · `sqlparser::ast::dml::MultiTableInsertValues::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [866, 38], "end": [866, 47], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dml.rs:866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-423236e8bfd7f140877b0a78"></a>
## values

`struct_field` · `sqlparser::ast::dml::MultiTableInsertValues::values` · sqlparser 0.62.0

```rust
values: Vec<MultiTableInsertValue>
```

Source: `src/ast/dml.rs:870`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The values to insert (can be column references, DEFAULT, or NULL)

<a id="op-7c02dad274b434cdd2f2235a"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertValues::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [867, 47], "end": [867, 55], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dml.rs:867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0dd1afb63229a0daf284ee5"></a>
## visit

`function` · `sqlparser::ast::dml::MultiTableInsertValues::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dml::MultiTableInsertValues", "path": "MultiTableInsertValues"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [867, 40], "end": [867, 45], "filename": "src/ast/dml.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dml.rs:867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
