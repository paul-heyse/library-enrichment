# `sqlparser::ast::ddl::AlterTableLock`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableLock.json).

<a id="op-c6efaf8b5cea318353140999"></a>
## AlterTableLock

`enum` · `sqlparser::ast::ddl::AlterTableLock` · sqlparser 0.62.0

```rust
enum AlterTableLock
```

Source: `src/ast/ddl.rs:624`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] `ALTER TABLE` lock.

[MySQL]: https://dev.mysql.com/doc/refman/8.4/en/alter-table.html
Locking behavior for `ALTER TABLE` (MySQL-specific).

<a id="op-de7f40abe12e562d7e5bedcc"></a>
## Default

`variant` · `sqlparser::ast::ddl::AlterTableLock::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/ddl.rs:626`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DEFAULT` lock behavior.

<a id="op-a5316a5e271284b388c7a9f0"></a>
## Exclusive

`variant` · `sqlparser::ast::ddl::AlterTableLock::Exclusive` · sqlparser 0.62.0

```rust
Exclusive
```

Source: `src/ast/ddl.rs:632`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXCLUSIVE` lock.

<a id="op-e112b0790a144cafa21aa906"></a>
## None

`variant` · `sqlparser::ast::ddl::AlterTableLock::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/ddl.rs:628`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NONE` lock.

<a id="op-8e1ddb33b0b54efba8660e82"></a>
## Shared

`variant` · `sqlparser::ast::ddl::AlterTableLock::Shared` · sqlparser 0.62.0

```rust
Shared
```

Source: `src/ast/ddl.rs:630`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHARED` lock.

<a id="op-5bd2f82f623f47198f421c52"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTableLock::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTableLock
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 17], "end": [620, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ca10e02cf703eb807c95083"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTableLock::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTableLock) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 57], "end": [620, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a45fde31a5e32162b2a64ef4"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTableLock::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 49], "end": [621, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54a7a5db2af43d06c619c43c"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTableLock::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTableLock) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 30], "end": [620, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ecdb84ce7f6be0acb64828"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableLock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 10], "end": [620, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7671ae2c0f206fe4db8a3b01"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableLock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [635, 1], "end": [644, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:636`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bfe467ab650acc031d7f78b"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTableLock::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 62], "end": [620, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8542f0a5b6e77d62480500c6"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTableLock::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTableLock) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [620, 41], "end": [620, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:620`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12f3e0e0498ec4168a9c5e3a"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTableLock::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [621, 38], "end": [621, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:621`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-987d00f13c33807e0e60b6bc"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableLock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [622, 40], "end": [622, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca7bd527be872f61045e201c"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableLock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableLock", "path": "AlterTableLock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [622, 47], "end": [622, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:622`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
