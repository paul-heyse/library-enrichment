# `sqlparser::ast::ShowStatementIn`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowStatementIn.json).

<a id="op-f4a8b91fe960d0e2c8146867"></a>
## ShowStatementIn

`struct` · `sqlparser::ast::ShowStatementIn` · sqlparser 0.62.0

```rust
struct ShowStatementIn
```

Source: `src/ast/mod.rs:10869`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a `SHOW ... IN` clause with optional parent qualifier and name.

<a id="op-762b1a58f7447d006ec0a565"></a>
## clause

`struct_field` · `sqlparser::ast::ShowStatementIn::clause` · sqlparser 0.62.0

```rust
clause: ShowStatementInClause
```

Source: `src/ast/mod.rs:10871`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The clause that specifies what to show (e.g. COLUMNS, TABLES).

<a id="op-75b62f62b6849c08fb958e4d"></a>
## clone

`function` · `sqlparser::ast::ShowStatementIn::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowStatementIn
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10865, 17], "end": [10865, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2107b347908c563456c09fad"></a>
## cmp

`function` · `sqlparser::ast::ShowStatementIn::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowStatementIn) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10865, 51], "end": [10865, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8253454381b5bf4c5b2cb7d"></a>
## deserialize

`function` · `sqlparser::ast::ShowStatementIn::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10866, 49], "end": [10866, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b55fdf482cdb77cfcdae144f"></a>
## eq

`function` · `sqlparser::ast::ShowStatementIn::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowStatementIn) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10865, 24], "end": [10865, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0acca1cae0979e61c5c3b78f"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementIn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10865, 10], "end": [10865, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-965b5c7b76c5ec6f23a181a4"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementIn::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10879, 1], "end": [10890, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77d7ee0aebcf6cd469c3749e"></a>
## hash

`function` · `sqlparser::ast::ShowStatementIn::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10865, 56], "end": [10865, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56d6e0e6079253f736b69998"></a>
## parent_name

`struct_field` · `sqlparser::ast::ShowStatementIn::parent_name` · sqlparser 0.62.0

```rust
parent_name: Option<ObjectName>
```

Source: `src/ast/mod.rs:10876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional parent object name for the SHOW clause.

<a id="op-e4ca1632a7a8658eba765a01"></a>
## parent_type

`struct_field` · `sqlparser::ast::ShowStatementIn::parent_type` · sqlparser 0.62.0

```rust
parent_type: Option<ShowStatementInParentType>
```

Source: `src/ast/mod.rs:10873`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional parent type qualifier (ACCOUNT/DATABASE/...).

<a id="op-9614505fa682cafd4a27f37d"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowStatementIn::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowStatementIn) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10865, 35], "end": [10865, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f781db674b573fd32659e768"></a>
## serialize

`function` · `sqlparser::ast::ShowStatementIn::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10866, 38], "end": [10866, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-061369c6e575e5a03794a323"></a>
## visit

`function` · `sqlparser::ast::ShowStatementIn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10867, 40], "end": [10867, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fe13af5f45949f71e661f3a"></a>
## visit

`function` · `sqlparser::ast::ShowStatementIn::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementIn", "path": "ShowStatementIn"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10867, 47], "end": [10867, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
