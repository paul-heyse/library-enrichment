# `sqlparser::ast::CreateTableLikeDefaults`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateTableLikeDefaults.json).

<a id="op-5bb68f295c6911fce4b8c727"></a>
## CreateTableLikeDefaults

`enum` · `sqlparser::ast::CreateTableLikeDefaults` · sqlparser 0.62.0

```rust
enum CreateTableLikeDefaults
```

Source: `src/ast/mod.rs:11815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Controls whether defaults are included when creating a table FROM/LILE another.

<a id="op-b7f67f186bc422a9ac3564fb"></a>
## Excluding

`variant` · `sqlparser::ast::CreateTableLikeDefaults::Excluding` · sqlparser 0.62.0

```rust
Excluding
```

Source: `src/ast/mod.rs:11819`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exclude default values from the source table.

<a id="op-04d1aa8cf5d9970e1cb3784d"></a>
## Including

`variant` · `sqlparser::ast::CreateTableLikeDefaults::Including` · sqlparser 0.62.0

```rust
Including
```

Source: `src/ast/mod.rs:11817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Include default values from the source table.

<a id="op-bb99fcb7aa459b2818a4567f"></a>
## clone

`function` · `sqlparser::ast::CreateTableLikeDefaults::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTableLikeDefaults
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11811, 23], "end": [11811, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de041efb15f690aa374a3a90"></a>
## cmp

`function` · `sqlparser::ast::CreateTableLikeDefaults::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateTableLikeDefaults) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11811, 57], "end": [11811, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a0df3341c948181ff11adec"></a>
## deserialize

`function` · `sqlparser::ast::CreateTableLikeDefaults::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11812, 49], "end": [11812, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7aac73ad8019a50a30abe98a"></a>
## eq

`function` · `sqlparser::ast::CreateTableLikeDefaults::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTableLikeDefaults) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11811, 30], "end": [11811, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2992018deb16a222e1218a48"></a>
## fmt

`function` · `sqlparser::ast::CreateTableLikeDefaults::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11822, 1], "end": [11829, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11823`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8db5ad298c6d2c996ed8b24f"></a>
## fmt

`function` · `sqlparser::ast::CreateTableLikeDefaults::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11811, 10], "end": [11811, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-137a924a5ebb282b41d8dfaa"></a>
## hash

`function` · `sqlparser::ast::CreateTableLikeDefaults::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11811, 62], "end": [11811, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd353489f46466c827429a14"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateTableLikeDefaults::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateTableLikeDefaults) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11811, 41], "end": [11811, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b0f24e0658598bd879886d9"></a>
## serialize

`function` · `sqlparser::ast::CreateTableLikeDefaults::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11812, 38], "end": [11812, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-800e449b0cbad4bc0c8e1e26"></a>
## visit

`function` · `sqlparser::ast::CreateTableLikeDefaults::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11813, 40], "end": [11813, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad2f2035f5b47bd855b68838"></a>
## visit

`function` · `sqlparser::ast::CreateTableLikeDefaults::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeDefaults", "path": "CreateTableLikeDefaults"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11813, 47], "end": [11813, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
