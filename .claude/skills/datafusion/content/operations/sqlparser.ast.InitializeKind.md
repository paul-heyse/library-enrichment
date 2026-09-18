# `sqlparser::ast::InitializeKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.InitializeKind.json).

<a id="op-eea50be9c23025ce5f4366ef"></a>
## InitializeKind

`enum` · `sqlparser::ast::InitializeKind` · sqlparser 0.62.0

```rust
enum InitializeKind
```

Source: `src/ast/mod.rs:11883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the behavior of the initial refresh of the dynamic table.

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table)

<a id="op-91431e99750dfc1c4481e1d3"></a>
## OnCreate

`variant` · `sqlparser::ast::InitializeKind::OnCreate` · sqlparser 0.62.0

```rust
OnCreate
```

Source: `src/ast/mod.rs:11885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Initialize on creation (`ON CREATE`).

<a id="op-673636a68ebefbea6269e49c"></a>
## OnSchedule

`variant` · `sqlparser::ast::InitializeKind::OnSchedule` · sqlparser 0.62.0

```rust
OnSchedule
```

Source: `src/ast/mod.rs:11887`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Initialize on schedule (`ON SCHEDULE`).

<a id="op-b69058340161a6a135eba639"></a>
## clone

`function` · `sqlparser::ast::InitializeKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> InitializeKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11880, 23], "end": [11880, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8b3ee9dd80bf2f9923be7ce"></a>
## cmp

`function` · `sqlparser::ast::InitializeKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &InitializeKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11880, 57], "end": [11880, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8085014fd7b853b26fe7529b"></a>
## deserialize

`function` · `sqlparser::ast::InitializeKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11881, 49], "end": [11881, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d413a1d1ee6706bebc6e587b"></a>
## eq

`function` · `sqlparser::ast::InitializeKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &InitializeKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11880, 30], "end": [11880, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d26afb0631ef856d945328ae"></a>
## fmt

`function` · `sqlparser::ast::InitializeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11890, 1], "end": [11897, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe6f43924954eaa41c68cd8e"></a>
## fmt

`function` · `sqlparser::ast::InitializeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11880, 10], "end": [11880, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-580e1f6d3390511ab2212269"></a>
## hash

`function` · `sqlparser::ast::InitializeKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11880, 62], "end": [11880, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8bb5e108e7cb2fcd95129a0"></a>
## partial_cmp

`function` · `sqlparser::ast::InitializeKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &InitializeKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11880, 41], "end": [11880, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdb5f0e35749b5e1366c8ca3"></a>
## serialize

`function` · `sqlparser::ast::InitializeKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11881, 38], "end": [11881, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-366cf4ab465cd402a178b307"></a>
## visit

`function` · `sqlparser::ast::InitializeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11882, 40], "end": [11882, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11882`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d6166381bb2008e6e682420"></a>
## visit

`function` · `sqlparser::ast::InitializeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::InitializeKind", "path": "InitializeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11882, 47], "end": [11882, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11882`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
