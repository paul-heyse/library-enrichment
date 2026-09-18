# `sqlparser::ast::ExportData`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ExportData.json).

<a id="op-64851c9f71e1682bc39e2707"></a>
## ExportData

`struct` · `sqlparser::ast::ExportData` · sqlparser 0.62.0

```rust
struct ExportData
```

Source: `src/ast/mod.rs:11433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an `EXPORT DATA` statement.

<a id="op-2da752c238fd095b71beeb74"></a>
## clone

`function` · `sqlparser::ast::ExportData::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ExportData
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11429, 17], "end": [11429, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c10718e1307085701d9c5bc2"></a>
## cmp

`function` · `sqlparser::ast::ExportData::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ExportData) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11429, 51], "end": [11429, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca9474551637c92a701f210f"></a>
## connection

`struct_field` · `sqlparser::ast::ExportData::connection` · sqlparser 0.62.0

```rust
connection: Option<ObjectName>
```

Source: `src/ast/mod.rs:11439`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional named connection to use for export.

<a id="op-96f126757824c0b1b9cf098c"></a>
## deserialize

`function` · `sqlparser::ast::ExportData::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11430, 49], "end": [11430, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3403bd5c3ecaab722049e895"></a>
## eq

`function` · `sqlparser::ast::ExportData::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ExportData) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11429, 24], "end": [11429, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de51cc759b60557726755c7c"></a>
## fmt

`function` · `sqlparser::ast::ExportData::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11442, 1], "end": [11460, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbd680f8beb78bba371cc241"></a>
## fmt

`function` · `sqlparser::ast::ExportData::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11429, 10], "end": [11429, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec4054a4c4086cecf41ed297"></a>
## hash

`function` · `sqlparser::ast::ExportData::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11429, 56], "end": [11429, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-569162b14dfe9984ecc751f0"></a>
## options

`struct_field` · `sqlparser::ast::ExportData::options` · sqlparser 0.62.0

```rust
options: Vec<SqlOption>
```

Source: `src/ast/mod.rs:11435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options for the export operation.

<a id="op-4731cbe8f931e85af6f66359"></a>
## partial_cmp

`function` · `sqlparser::ast::ExportData::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ExportData) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11429, 35], "end": [11429, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4f90dc03379599069bf2f6b"></a>
## query

`struct_field` · `sqlparser::ast::ExportData::query` · sqlparser 0.62.0

```rust
query: Box<Query>
```

Source: `src/ast/mod.rs:11437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The query producing the data to export.

<a id="op-26f4e99de2e660e4246ae62a"></a>
## serialize

`function` · `sqlparser::ast::ExportData::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11430, 38], "end": [11430, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11430`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50b8159b2ec6bed1e3c43130"></a>
## visit

`function` · `sqlparser::ast::ExportData::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11431, 40], "end": [11431, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11431`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9950e53975f7535e55c3bc1"></a>
## visit

`function` · `sqlparser::ast::ExportData::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ExportData", "path": "ExportData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11431, 47], "end": [11431, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11431`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
