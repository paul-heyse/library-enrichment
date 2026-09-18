# `sqlparser::ast::FileSize`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FileSize.json).

<a id="op-9d539380a5663918e659ced4"></a>
## FileSize

`struct` · `sqlparser::ast::FileSize` · sqlparser 0.62.0

```rust
struct FileSize
```

Source: `src/ast/mod.rs:9584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SIZE \[ MB | GB \]
```

<a id="op-5ca4c90d0aac8da228bc11eb"></a>
## clone

`function` · `sqlparser::ast::FileSize::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FileSize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9581, 17], "end": [9581, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3df1419aad71f26b52c2599d"></a>
## cmp

`function` · `sqlparser::ast::FileSize::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FileSize) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9581, 51], "end": [9581, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe9a8c02e3440ede0c25805f"></a>
## deserialize

`function` · `sqlparser::ast::FileSize::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9582, 49], "end": [9582, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c7eb209bbded4b10d5ebd94"></a>
## eq

`function` · `sqlparser::ast::FileSize::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FileSize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9581, 24], "end": [9581, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-844e4bd0fba5415c9befab52"></a>
## fmt

`function` · `sqlparser::ast::FileSize::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9581, 10], "end": [9581, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b163eb61dfbd45deaca61428"></a>
## fmt

`function` · `sqlparser::ast::FileSize::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9591, 1], "end": [9599, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-765e7c1631209289e7a03a3d"></a>
## hash

`function` · `sqlparser::ast::FileSize::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9581, 56], "end": [9581, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbfe365e74a2d98e3cfa62dd"></a>
## partial_cmp

`function` · `sqlparser::ast::FileSize::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FileSize) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9581, 35], "end": [9581, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f28885bd8000b4c69b4db2a5"></a>
## serialize

`function` · `sqlparser::ast::FileSize::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9582, 38], "end": [9582, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-652f122380c20ee65997cc66"></a>
## size

`struct_field` · `sqlparser::ast::FileSize::size` · sqlparser 0.62.0

```rust
size: ValueWithSpan
```

Source: `src/ast/mod.rs:9586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Numeric size value.

<a id="op-aab225c969f59b2fd05d6dea"></a>
## unit

`struct_field` · `sqlparser::ast::FileSize::unit` · sqlparser 0.62.0

```rust
unit: Option<FileSizeUnit>
```

Source: `src/ast/mod.rs:9588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional unit for the size (MB or GB).

<a id="op-70cea1c88fa30ee765a94d31"></a>
## visit

`function` · `sqlparser::ast::FileSize::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9583, 40], "end": [9583, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6394d1ec53b5987222fbb63"></a>
## visit

`function` · `sqlparser::ast::FileSize::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FileSize", "path": "FileSize"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9583, 47], "end": [9583, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
