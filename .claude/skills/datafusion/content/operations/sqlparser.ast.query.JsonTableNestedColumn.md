# `sqlparser::ast::query::JsonTableNestedColumn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.JsonTableNestedColumn.json).

<a id="op-a639e912360abf31c9eb0e77"></a>
## JsonTableNestedColumn

`struct` · `sqlparser::ast::query::JsonTableNestedColumn` · sqlparser 0.62.0

```rust
struct JsonTableNestedColumn
```

Source: `src/ast/query.rs:4025`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A nested column in a JSON_TABLE column list

See <https://mariadb.com/kb/en/json_table/#nested-paths>
A nested column in a `JSON_TABLE` column list.

<a id="op-4a3bfeac1cef2ea542cef891"></a>
## clone

`function` · `sqlparser::ast::query::JsonTableNestedColumn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonTableNestedColumn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4021, 17], "end": [4021, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:4021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-192a2b529279137ae1bda1d6"></a>
## cmp

`function` · `sqlparser::ast::query::JsonTableNestedColumn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonTableNestedColumn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4021, 51], "end": [4021, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:4021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63b5951da342226e3e7a79ee"></a>
## columns

`struct_field` · `sqlparser::ast::query::JsonTableNestedColumn::columns` · sqlparser 0.62.0

```rust
columns: Vec<JsonTableColumn>
```

Source: `src/ast/query.rs:4029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Columns extracted from the matched nested array.

<a id="op-a32d009a7a0a132d6c980b4f"></a>
## deserialize

`function` · `sqlparser::ast::query::JsonTableNestedColumn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4023, 49], "end": [4023, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:4023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a461e7feea543d1e96c47f34"></a>
## eq

`function` · `sqlparser::ast::query::JsonTableNestedColumn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonTableNestedColumn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4021, 24], "end": [4021, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:4021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2541e78b181411d4ea0fe51b"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableNestedColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4032, 1], "end": [4041, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:4033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-456c6ac179959cc5810f5f18"></a>
## fmt

`function` · `sqlparser::ast::query::JsonTableNestedColumn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4021, 10], "end": [4021, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:4021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a60d88e3b850731bafc444a4"></a>
## hash

`function` · `sqlparser::ast::query::JsonTableNestedColumn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4021, 56], "end": [4021, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:4021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94bf4d1fdf7a3e1e7dfaf1f4"></a>
## partial_cmp

`function` · `sqlparser::ast::query::JsonTableNestedColumn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonTableNestedColumn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4021, 35], "end": [4021, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:4021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f3c0ce0d1ee97ee189047ee"></a>
## path

`struct_field` · `sqlparser::ast::query::JsonTableNestedColumn::path` · sqlparser 0.62.0

```rust
path: ValueWithSpan
```

Source: `src/ast/query.rs:4027`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

JSON path expression (must be a literal `Value`).

<a id="op-b08ea32ad6cf4c3d862fa388"></a>
## serialize

`function` · `sqlparser::ast::query::JsonTableNestedColumn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4023, 38], "end": [4023, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:4023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e9cdd89d7407bf062ec3ff"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableNestedColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4022, 47], "end": [4022, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:4022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdc67db1344a6a49979e50d3"></a>
## visit

`function` · `sqlparser::ast::query::JsonTableNestedColumn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::JsonTableNestedColumn", "path": "JsonTableNestedColumn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4022, 40], "end": [4022, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:4022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
