# `sqlparser::ast::StructField`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.StructField.json).

<a id="op-57c7aa614b0c8ff8d846e329"></a>
## StructField

`struct` · `sqlparser::ast::StructField` · sqlparser 0.62.0

```rust
struct StructField
```

Source: `src/ast/mod.rs:563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A field definition within a struct

[BigQuery]: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#struct_type

<a id="op-4cf982ba74172d2397a31dbe"></a>
## clone

`function` · `sqlparser::ast::StructField::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StructField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 17], "end": [560, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b595c2b824236992f3940b5d"></a>
## cmp

`function` · `sqlparser::ast::StructField::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StructField) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 51], "end": [560, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0520867349ddcafdb4439a05"></a>
## deserialize

`function` · `sqlparser::ast::StructField::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [561, 49], "end": [561, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18dab1edd8e718bcb70e70d1"></a>
## eq

`function` · `sqlparser::ast::StructField::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StructField) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 24], "end": [560, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-604d9721e44a1dafdc0f80bd"></a>
## field_name

`struct_field` · `sqlparser::ast::StructField::field_name` · sqlparser 0.62.0

```rust
field_name: Option<Ident>
```

Source: `src/ast/mod.rs:565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional name of the struct field.

<a id="op-eaadf1fe9889dcf156d05182"></a>
## field_type

`struct_field` · `sqlparser::ast::StructField::field_type` · sqlparser 0.62.0

```rust
field_type: DataType
```

Source: `src/ast/mod.rs:567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The field data type.

<a id="op-dd53fdfeefd3f858c54eca6e"></a>
## fmt

`function` · `sqlparser::ast::StructField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [573, 1], "end": [586, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec5a34866d0989c3569e696e"></a>
## fmt

`function` · `sqlparser::ast::StructField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 10], "end": [560, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c884495be7789a96c155e9bb"></a>
## hash

`function` · `sqlparser::ast::StructField::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 56], "end": [560, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cff75b3dda7082f2185c13a1"></a>
## options

`struct_field` · `sqlparser::ast::StructField::options` · sqlparser 0.62.0

```rust
options: Option<Vec<SqlOption>>
```

Source: `src/ast/mod.rs:570`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Struct field options (e.g., `OPTIONS(...)` on BigQuery).
See [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#column_name_and_column_schema)

<a id="op-5c8b3a7629f9f329691c9075"></a>
## partial_cmp

`function` · `sqlparser::ast::StructField::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StructField) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [560, 35], "end": [560, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:560`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b4f03faa051cbe231680cbc"></a>
## serialize

`function` · `sqlparser::ast::StructField::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [561, 38], "end": [561, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51fb3d5a81e3c64c5d47f06f"></a>
## visit

`function` · `sqlparser::ast::StructField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 40], "end": [562, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ce58f6ffc79a994d0ef034e"></a>
## visit

`function` · `sqlparser::ast::StructField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 47], "end": [562, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
