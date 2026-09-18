# `sqlparser::ast::RenameTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.RenameTable.json).

<a id="op-dfb19c332edd04d853c2abc9"></a>
## RenameTable

`struct` · `sqlparser::ast::RenameTable` · sqlparser 0.62.0

```rust
struct RenameTable
```

Source: `src/ast/mod.rs:10982`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

rename object definition

<a id="op-aaa0b8580f2523e5fda72182"></a>
## clone

`function` · `sqlparser::ast::RenameTable::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RenameTable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10979, 17], "end": [10979, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2078d614429dd8cc4821d9b"></a>
## cmp

`function` · `sqlparser::ast::RenameTable::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RenameTable) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10979, 51], "end": [10979, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63d1aa7273af32de16c041b2"></a>
## deserialize

`function` · `sqlparser::ast::RenameTable::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10980, 49], "end": [10980, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1122a6175f3483da3e7f8bde"></a>
## eq

`function` · `sqlparser::ast::RenameTable::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RenameTable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10979, 24], "end": [10979, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2502a27a71b8ed1781afe560"></a>
## fmt

`function` · `sqlparser::ast::RenameTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10989, 1], "end": [10994, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10990`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-691a6d69c7046373cd50fd54"></a>
## fmt

`function` · `sqlparser::ast::RenameTable::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10979, 10], "end": [10979, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d61e2068c36e9164f8edf814"></a>
## hash

`function` · `sqlparser::ast::RenameTable::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10979, 56], "end": [10979, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5939f696830ada156a02f9a"></a>
## new_name

`struct_field` · `sqlparser::ast::RenameTable::new_name` · sqlparser 0.62.0

```rust
new_name: ObjectName
```

Source: `src/ast/mod.rs:10986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The new name for the object.

<a id="op-8cc956f37a273dd1cebacaa6"></a>
## old_name

`struct_field` · `sqlparser::ast::RenameTable::old_name` · sqlparser 0.62.0

```rust
old_name: ObjectName
```

Source: `src/ast/mod.rs:10984`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The current name of the object to rename.

<a id="op-8fc8711b08bbc404e23a94f4"></a>
## partial_cmp

`function` · `sqlparser::ast::RenameTable::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RenameTable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10979, 35], "end": [10979, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10979`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2248112a94fc5b44c8bce41"></a>
## serialize

`function` · `sqlparser::ast::RenameTable::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10980, 38], "end": [10980, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10980`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436804e4b38f862a6316d0d8"></a>
## visit

`function` · `sqlparser::ast::RenameTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10981, 40], "end": [10981, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d88db258fc2df2073aee839"></a>
## visit

`function` · `sqlparser::ast::RenameTable::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RenameTable", "path": "RenameTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10981, 47], "end": [10981, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
