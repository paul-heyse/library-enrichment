# `sqlparser::ast::ddl::AlterTypeRenameValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTypeRenameValue.json).

<a id="op-34c4fb36b4c0be939669828a"></a>
## AlterTypeRenameValue

`struct` · `sqlparser::ast::ddl::AlterTypeRenameValue` · sqlparser 0.62.0

```rust
struct AlterTypeRenameValue
```

Source: `src/ast/ddl.rs:1122`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [AlterTypeOperation::RenameValue](../operations/sqlparser.ast.ddl.AlterTypeOperation.md#op-8e4166fd9269a030267acf39)

<a id="op-ba4f3574de0135f6faeee01d"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTypeRenameValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 17], "end": [1119, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-833b321a3a2e135b9fa74d61"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTypeRenameValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 51], "end": [1119, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15d7549168c2bc05e962eb06"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 49], "end": [1120, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf014234be63163f5902ac33"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTypeRenameValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 24], "end": [1119, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6d6664868ba64610d879353"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 10], "end": [1119, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c9345e828e6371bc21d4620"></a>
## from

`struct_field` · `sqlparser::ast::ddl::AlterTypeRenameValue::from` · sqlparser 0.62.0

```rust
from: ast::Ident
```

Source: `src/ast/ddl.rs:1124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Existing value identifier to rename.

<a id="op-79b4090845b29522e02d86fc"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 56], "end": [1119, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89c6be79e20327b3a1e02fae"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTypeRenameValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1119, 35], "end": [1119, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcabb2d80bf3bb78324f4bb5"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 38], "end": [1120, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1120`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d3cc94793e7d833b19f6c78"></a>
## to

`struct_field` · `sqlparser::ast::ddl::AlterTypeRenameValue::to` · sqlparser 0.62.0

```rust
to: ast::Ident
```

Source: `src/ast/ddl.rs:1126`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

New identifier for the value.

<a id="op-884624301f95610878904ecc"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1121, 40], "end": [1121, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7d1453f5c5fe74bfd1afc4f"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeRenameValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRenameValue", "path": "AlterTypeRenameValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1121, 47], "end": [1121, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
