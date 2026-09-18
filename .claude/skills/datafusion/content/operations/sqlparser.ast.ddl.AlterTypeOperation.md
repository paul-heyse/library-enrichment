# `sqlparser::ast::ddl::AlterTypeOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTypeOperation.json).

<a id="op-daea5f0aeaab2c0bbcb89f80"></a>
## AlterTypeOperation

`enum` · `sqlparser::ast::ddl::AlterTypeOperation` · sqlparser 0.62.0

```rust
enum AlterTypeOperation
```

Source: `src/ast/ddl.rs:1076`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An [AlterType](../operations/sqlparser.ast.ddl.AlterType.md#op-3a8ee7ae8d2120bc093b3269) operation

<a id="op-9f13d2a2f04ce85f88d1ac7b"></a>
## AddValue

`variant` · `sqlparser::ast::ddl::AlterTypeOperation::AddValue` · sqlparser 0.62.0

```rust
AddValue
```

Source: `src/ast/ddl.rs:1080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Add a new value to the type (for enum-like types).

<a id="op-70c500c4d64d35425c941841"></a>
## Rename

`variant` · `sqlparser::ast::ddl::AlterTypeOperation::Rename` · sqlparser 0.62.0

```rust
Rename
```

Source: `src/ast/ddl.rs:1078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Rename the type.

<a id="op-8e4166fd9269a030267acf39"></a>
## RenameValue

`variant` · `sqlparser::ast::ddl::AlterTypeOperation::RenameValue` · sqlparser 0.62.0

```rust
RenameValue
```

Source: `src/ast/ddl.rs:1082`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Rename an existing value of the type.

<a id="op-ecaa42e9261de94a00d058ca"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTypeOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTypeOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 17], "end": [1073, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e0efddf18c4c475a92b867d"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTypeOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTypeOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 51], "end": [1073, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdce984c4db7310454b9dcfe"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTypeOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 49], "end": [1074, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1074`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-343cf0bfe699bdda8c598133"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTypeOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTypeOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 24], "end": [1073, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-288fa30c818ba1a4073d9d50"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTypeOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1129, 1], "end": [1161, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d69d2766423af4c6c720137"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTypeOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 10], "end": [1073, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cbbb83866fb74ecb1caa6e6"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTypeOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 56], "end": [1073, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-783039f630219c35b21e72b5"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTypeOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTypeOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1073, 35], "end": [1073, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbc64abbe43f4edc62cdba3f"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTypeOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 38], "end": [1074, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1074`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e489b50819332286bbbaacd"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 40], "end": [1075, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7aba82c5308e770e74af8f96"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeOperation", "path": "AlterTypeOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1075, 47], "end": [1075, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1075`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
