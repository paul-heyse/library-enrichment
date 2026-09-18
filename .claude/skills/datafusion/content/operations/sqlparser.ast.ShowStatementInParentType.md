# `sqlparser::ast::ShowStatementInParentType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowStatementInParentType.json).

<a id="op-7219934a063192ee5f827961"></a>
## ShowStatementInParentType

`enum` · `sqlparser::ast::ShowStatementInParentType` · sqlparser 0.62.0

```rust
enum ShowStatementInParentType
```

Source: `src/ast/mod.rs:10840`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parent object types usable with `SHOW ... IN <parent>` clauses.

<a id="op-96bb145b1dd3fb99c0035bd9"></a>
## Account

`variant` · `sqlparser::ast::ShowStatementInParentType::Account` · sqlparser 0.62.0

```rust
Account
```

Source: `src/ast/mod.rs:10842`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ACCOUNT parent type for SHOW statements.

<a id="op-fd841435587c54aa9fdd41ba"></a>
## Database

`variant` · `sqlparser::ast::ShowStatementInParentType::Database` · sqlparser 0.62.0

```rust
Database
```

Source: `src/ast/mod.rs:10844`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DATABASE parent type for SHOW statements.

<a id="op-062393bcc97e326038c1649c"></a>
## Schema

`variant` · `sqlparser::ast::ShowStatementInParentType::Schema` · sqlparser 0.62.0

```rust
Schema
```

Source: `src/ast/mod.rs:10846`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SCHEMA parent type for SHOW statements.

<a id="op-fc73788cc01ba448d4278f02"></a>
## Table

`variant` · `sqlparser::ast::ShowStatementInParentType::Table` · sqlparser 0.62.0

```rust
Table
```

Source: `src/ast/mod.rs:10848`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TABLE parent type for SHOW statements.

<a id="op-98c49b866e0e7c182159683c"></a>
## View

`variant` · `sqlparser::ast::ShowStatementInParentType::View` · sqlparser 0.62.0

```rust
View
```

Source: `src/ast/mod.rs:10850`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

VIEW parent type for SHOW statements.

<a id="op-eef10eaae219e495663047e0"></a>
## clone

`function` · `sqlparser::ast::ShowStatementInParentType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowStatementInParentType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10836, 17], "end": [10836, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b1b97e090f1c717be4686c4"></a>
## cmp

`function` · `sqlparser::ast::ShowStatementInParentType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowStatementInParentType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10836, 51], "end": [10836, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b6ea8fcbbf98b20a43ffc47"></a>
## deserialize

`function` · `sqlparser::ast::ShowStatementInParentType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10837, 49], "end": [10837, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f82b143c7c796469bb5c158"></a>
## eq

`function` · `sqlparser::ast::ShowStatementInParentType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowStatementInParentType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10836, 24], "end": [10836, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ac6fe2f645ff8c1d4ae87db"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementInParentType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10836, 10], "end": [10836, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fc906256e8db19c7fc4e9c5"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementInParentType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10853, 1], "end": [10863, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b63fee8a6b69dad8626abc84"></a>
## hash

`function` · `sqlparser::ast::ShowStatementInParentType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10836, 56], "end": [10836, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b37207e4cbec063048094a78"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowStatementInParentType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowStatementInParentType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10836, 35], "end": [10836, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10836`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68f853370a30375a0d970d20"></a>
## serialize

`function` · `sqlparser::ast::ShowStatementInParentType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10837, 38], "end": [10837, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10837`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e6e5effac211d1739f30695"></a>
## visit

`function` · `sqlparser::ast::ShowStatementInParentType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10838, 40], "end": [10838, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10838`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e0df12274975c0c1cf1c94"></a>
## visit

`function` · `sqlparser::ast::ShowStatementInParentType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementInParentType", "path": "ShowStatementInParentType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10838, 47], "end": [10838, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10838`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
