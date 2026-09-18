# `sqlparser::ast::DictionaryField`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DictionaryField.json).

<a id="op-119c5c26dfa445225edbf1c3"></a>
## DictionaryField

`struct` · `sqlparser::ast::DictionaryField` · sqlparser 0.62.0

```rust
struct DictionaryField
```

Source: `src/ast/mod.rs:613`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A dictionary field within a dictionary.

[DuckDB]: https://duckdb.org/docs/sql/data_types/struct#creating-structs

<a id="op-31b033a5b729abf884ffa997"></a>
## clone

`function` · `sqlparser::ast::DictionaryField::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DictionaryField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 17], "end": [610, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69bd439d8365936c1645139c"></a>
## cmp

`function` · `sqlparser::ast::DictionaryField::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DictionaryField) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 51], "end": [610, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1710f9c1a9549044b7346a96"></a>
## deserialize

`function` · `sqlparser::ast::DictionaryField::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [611, 49], "end": [611, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:611`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c552c668ce341ec8db65d2dd"></a>
## eq

`function` · `sqlparser::ast::DictionaryField::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DictionaryField) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 24], "end": [610, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73084b4dbbad6ca5f98f26a7"></a>
## fmt

`function` · `sqlparser::ast::DictionaryField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 1], "end": [624, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d51b8bcdf142921c65395aba"></a>
## fmt

`function` · `sqlparser::ast::DictionaryField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 10], "end": [610, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4241a3c8436ccaa44543f4c2"></a>
## hash

`function` · `sqlparser::ast::DictionaryField::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 56], "end": [610, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17b64d75ea704ed9fa187787"></a>
## key

`struct_field` · `sqlparser::ast::DictionaryField::key` · sqlparser 0.62.0

```rust
key: Ident
```

Source: `src/ast/mod.rs:615`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dictionary key identifier.

<a id="op-644d48c1796da5069a3278ce"></a>
## partial_cmp

`function` · `sqlparser::ast::DictionaryField::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DictionaryField) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 35], "end": [610, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8484c7de7b3063f9f92cfc94"></a>
## serialize

`function` · `sqlparser::ast::DictionaryField::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [611, 38], "end": [611, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:611`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7fdab83f1795f1212500ba5"></a>
## value

`struct_field` · `sqlparser::ast::DictionaryField::value` · sqlparser 0.62.0

```rust
value: Box<Expr>
```

Source: `src/ast/mod.rs:617`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value expression for the dictionary entry.

<a id="op-01b37caa560e5e9b536a467a"></a>
## visit

`function` · `sqlparser::ast::DictionaryField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 40], "end": [612, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a98bb309334d53e7e5548524"></a>
## visit

`function` · `sqlparser::ast::DictionaryField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DictionaryField", "path": "DictionaryField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 47], "end": [612, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:612`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
