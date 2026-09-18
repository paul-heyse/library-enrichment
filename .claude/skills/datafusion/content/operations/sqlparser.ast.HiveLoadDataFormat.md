# `sqlparser::ast::HiveLoadDataFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveLoadDataFormat.json).

<a id="op-4ab8cc8c5d7b769e1b6a18cb"></a>
## HiveLoadDataFormat

`struct` · `sqlparser::ast::HiveLoadDataFormat` · sqlparser 0.62.0

```rust
struct HiveLoadDataFormat
```

Source: `src/ast/mod.rs:8579`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Format specification for `LOAD DATA` Hive operations.

<a id="op-31180ab74edb2af00944a609"></a>
## clone

`function` · `sqlparser::ast::HiveLoadDataFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveLoadDataFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8575, 17], "end": [8575, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fa773b784666e40656eebcd"></a>
## cmp

`function` · `sqlparser::ast::HiveLoadDataFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveLoadDataFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8575, 51], "end": [8575, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caaee9eac422fb744f7e1ccc"></a>
## deserialize

`function` · `sqlparser::ast::HiveLoadDataFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8576, 49], "end": [8576, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34e14da4ab4fb5cb9aa4ef2a"></a>
## eq

`function` · `sqlparser::ast::HiveLoadDataFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveLoadDataFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8575, 24], "end": [8575, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c6cab31fbdf54541f46c65c"></a>
## fmt

`function` · `sqlparser::ast::HiveLoadDataFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8575, 10], "end": [8575, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d96e482d892c3f22b19ea55"></a>
## hash

`function` · `sqlparser::ast::HiveLoadDataFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8575, 56], "end": [8575, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d951ad970515661964b98af1"></a>
## input_format

`struct_field` · `sqlparser::ast::HiveLoadDataFormat::input_format` · sqlparser 0.62.0

```rust
input_format: Expr
```

Source: `src/ast/mod.rs:8583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Input format expression.

<a id="op-54450b7c2e171af334f8aecd"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveLoadDataFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveLoadDataFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8575, 35], "end": [8575, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-905872897dd603881965d088"></a>
## serde

`struct_field` · `sqlparser::ast::HiveLoadDataFormat::serde` · sqlparser 0.62.0

```rust
serde: Expr
```

Source: `src/ast/mod.rs:8581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SerDe expression used for the table.

<a id="op-b4fc3a951850489b76c0fc9c"></a>
## serialize

`function` · `sqlparser::ast::HiveLoadDataFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8576, 38], "end": [8576, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43ac3cd943b6cf6bf08fecf3"></a>
## visit

`function` · `sqlparser::ast::HiveLoadDataFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8577, 47], "end": [8577, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-825af4b52a914ca2a4498fcb"></a>
## visit

`function` · `sqlparser::ast::HiveLoadDataFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveLoadDataFormat", "path": "HiveLoadDataFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8577, 40], "end": [8577, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
