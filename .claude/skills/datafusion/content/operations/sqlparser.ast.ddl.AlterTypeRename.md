# `sqlparser::ast::ddl::AlterTypeRename`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTypeRename.json).

<a id="op-f05012739c2597c5b567f4dc"></a>
## AlterTypeRename

`struct` · `sqlparser::ast::ddl::AlterTypeRename` · sqlparser 0.62.0

```rust
struct AlterTypeRename
```

Source: `src/ast/ddl.rs:1089`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See [AlterTypeOperation::Rename](../operations/sqlparser.ast.ddl.AlterTypeOperation.md#op-70c500c4d64d35425c941841)

<a id="op-831aa76896a1421a2a168574"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTypeRename::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTypeRename
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 17], "end": [1086, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec7d88eacfc77e6ceec4c28d"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTypeRename::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTypeRename) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 51], "end": [1086, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-905031b05267ac57224920fe"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTypeRename::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 49], "end": [1087, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d9ad507186b84dd0b992e24"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTypeRename::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTypeRename) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 24], "end": [1086, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-136fb60bcd7d341f5b606cea"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTypeRename::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 10], "end": [1086, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e6f8a5a69d4b40e210289d7"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTypeRename::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 56], "end": [1086, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4508f2bf5ec4753c8491daa3"></a>
## new_name

`struct_field` · `sqlparser::ast::ddl::AlterTypeRename::new_name` · sqlparser 0.62.0

```rust
new_name: ast::Ident
```

Source: `src/ast/ddl.rs:1091`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The new name for the type.

<a id="op-41e802db4903db982e456855"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTypeRename::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTypeRename) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1086, 35], "end": [1086, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1086`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac4facdf6e4d663b5b408224"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTypeRename::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1087, 38], "end": [1087, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c1af9206f3008430049ce11"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeRename::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1088, 40], "end": [1088, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d2e1927f4741d4052e42d3d"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTypeRename::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTypeRename", "path": "AlterTypeRename"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1088, 47], "end": [1088, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1088`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
