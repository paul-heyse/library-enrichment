# `sqlparser::ast::ddl::RenameTableNameKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.RenameTableNameKind.json).

<a id="op-a53c84139acaf519f84c904b"></a>
## RenameTableNameKind

`enum` · `sqlparser::ast::ddl::RenameTableNameKind` · sqlparser 0.62.0

```rust
enum RenameTableNameKind
```

Source: `src/ast/ddl.rs:3863`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RenameTableNameKind` is the kind used in an `ALTER TABLE _ RENAME` statement.

Note: [MySQL] is the only database that supports the AS keyword for this operation.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html

<a id="op-fe531733753721fb91e3a0aa"></a>
## As

`variant` · `sqlparser::ast::ddl::RenameTableNameKind::As` · sqlparser 0.62.0

```rust
As
```

Source: `src/ast/ddl.rs:3865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`AS new_table_name`

<a id="op-7702806e71cb631880955120"></a>
## To

`variant` · `sqlparser::ast::ddl::RenameTableNameKind::To` · sqlparser 0.62.0

```rust
To
```

Source: `src/ast/ddl.rs:3867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TO new_table_name`

<a id="op-431ecec886acc50d64bb8ee4"></a>
## clone

`function` · `sqlparser::ast::ddl::RenameTableNameKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RenameTableNameKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 17], "end": [3860, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-679d0cd223336ec8c2324202"></a>
## cmp

`function` · `sqlparser::ast::ddl::RenameTableNameKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RenameTableNameKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 51], "end": [3860, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28c7c444e13f9b4fa207d918"></a>
## deserialize

`function` · `sqlparser::ast::ddl::RenameTableNameKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3861, 49], "end": [3861, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3861`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f500070b03133d8fa4c21c4"></a>
## eq

`function` · `sqlparser::ast::ddl::RenameTableNameKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RenameTableNameKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 24], "end": [3860, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3044d597a678e170538b3085"></a>
## fmt

`function` · `sqlparser::ast::ddl::RenameTableNameKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 10], "end": [3860, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b609b0162c0ad5c75ade7cb"></a>
## fmt

`function` · `sqlparser::ast::ddl::RenameTableNameKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3870, 1], "end": [3877, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3871`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb53ecf2f971bf2dc34c978"></a>
## hash

`function` · `sqlparser::ast::ddl::RenameTableNameKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 56], "end": [3860, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84d01d03077cf4bd9c1e0b77"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::RenameTableNameKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RenameTableNameKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3860, 35], "end": [3860, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dfaae8cff067d4e273247fd"></a>
## serialize

`function` · `sqlparser::ast::ddl::RenameTableNameKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3861, 38], "end": [3861, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3861`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c460577798202c82b3e436"></a>
## span

`function` · `sqlparser::ast::ddl::RenameTableNameKind::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3907, 1], "end": [3914, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:3908`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac80473de4b07d481b36990a"></a>
## visit

`function` · `sqlparser::ast::ddl::RenameTableNameKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3862, 40], "end": [3862, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f36fa95e2a097434b7521476"></a>
## visit

`function` · `sqlparser::ast::ddl::RenameTableNameKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::RenameTableNameKind", "path": "RenameTableNameKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3862, 47], "end": [3862, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
