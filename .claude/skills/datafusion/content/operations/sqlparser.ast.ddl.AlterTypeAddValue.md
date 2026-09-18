# `sqlparser::ast::ddl::AlterTypeAddValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTypeAddValue.json).

<a id="op-044d206525d82f53918166fe"></a>
## AlterTypeAddValue

`struct` · `sqlparser::ast::ddl::AlterTypeAddValue` · sqlparser 0.62.0

```rust
struct AlterTypeAddValue
```

Source: `src/ast/ddl.rs:1098`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [AlterTypeOperation::AddValue](../operations/sqlparser.ast.ddl.AlterTypeOperation.md#op-9f13d2a2f04ce85f88d1ac7b)

<a id="op-4ab109d45faa30f04bd85c38"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTypeAddValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 17], "end": [1095, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fa3bd65a5e3979ac39f7a4b"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTypeAddValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 51], "end": [1095, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91342d23f1215fa5e22694b6"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1096, 49], "end": [1096, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c79a7915a075bd66d23f8805"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTypeAddValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 24], "end": [1095, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33d8d972a2118b749cd29e27"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 10], "end": [1095, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-426f1fa03505299f5685356b"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 56], "end": [1095, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-259429167271a666467a4d9b"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::AlterTypeAddValue::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:1100`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If true, do not error when the value already exists (`IF NOT EXISTS`).

<a id="op-1fae2bebb51dea8b0f032cc5"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTypeAddValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1095, 35], "end": [1095, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25c5761de32b8d19138d62ec"></a>
## position

`struct_field` · `sqlparser::ast::ddl::AlterTypeAddValue::position` · sqlparser 0.62.0

```rust
position: Option<AlterTypeAddValuePosition>
```

Source: `src/ast/ddl.rs:1104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional relative position for the new value (`BEFORE` / `AFTER`).

<a id="op-656a53c0fd2ff91453635eec"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1096, 38], "end": [1096, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d5eec46783c8556f0ba58cc"></a>
## value

`struct_field` · `sqlparser::ast::ddl::AlterTypeAddValue::value` · sqlparser 0.62.0

```rust
value: ast::Ident
```

Source: `src/ast/ddl.rs:1102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The identifier for the new value to add.

<a id="op-5bcf16dff646ccca7cf516ed"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 40], "end": [1097, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1097`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9750f6fb88b65ca481d2cdfe"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeAddValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeAddValue", "path": "AlterTypeAddValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1097, 47], "end": [1097, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1097`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
