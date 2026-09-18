# `sqlparser::ast::RefreshModeKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.RefreshModeKind.json).

<a id="op-4e64b7879fe084d3a746ed2d"></a>
## RefreshModeKind

`enum` · `sqlparser::ast::RefreshModeKind` · sqlparser 0.62.0

```rust
enum RefreshModeKind
```

Source: `src/ast/mod.rs:11858`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the refresh mode for the dynamic table.

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-dynamic-table)

<a id="op-e09f356e10c666a61090aff4"></a>
## Auto

`variant` · `sqlparser::ast::RefreshModeKind::Auto` · sqlparser 0.62.0

```rust
Auto
```

Source: `src/ast/mod.rs:11860`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Automatic refresh mode (`AUTO`).

<a id="op-70c58de207784e9a06119bcd"></a>
## Full

`variant` · `sqlparser::ast::RefreshModeKind::Full` · sqlparser 0.62.0

```rust
Full
```

Source: `src/ast/mod.rs:11862`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Full refresh mode (`FULL`).

<a id="op-d2be1d1a4db402748aa92455"></a>
## Incremental

`variant` · `sqlparser::ast::RefreshModeKind::Incremental` · sqlparser 0.62.0

```rust
Incremental
```

Source: `src/ast/mod.rs:11864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Incremental refresh mode (`INCREMENTAL`).

<a id="op-d933bd7c312618ba2e9ae764"></a>
## clone

`function` · `sqlparser::ast::RefreshModeKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RefreshModeKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11855, 23], "end": [11855, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cfa3ea50c31ce230b236cc2"></a>
## cmp

`function` · `sqlparser::ast::RefreshModeKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RefreshModeKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11855, 57], "end": [11855, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc28969d4152524a784227ba"></a>
## deserialize

`function` · `sqlparser::ast::RefreshModeKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11856, 49], "end": [11856, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca6fb159e3e6bc051b8d4289"></a>
## eq

`function` · `sqlparser::ast::RefreshModeKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RefreshModeKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11855, 30], "end": [11855, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cab3217d66e1717d0f0ba1a"></a>
## fmt

`function` · `sqlparser::ast::RefreshModeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11867, 1], "end": [11875, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d2c485bea66a05a0d0777f"></a>
## fmt

`function` · `sqlparser::ast::RefreshModeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11855, 10], "end": [11855, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a47de43a01542cc83c611505"></a>
## hash

`function` · `sqlparser::ast::RefreshModeKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11855, 62], "end": [11855, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d3f25718ed55ff2fa933c77"></a>
## partial_cmp

`function` · `sqlparser::ast::RefreshModeKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RefreshModeKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11855, 41], "end": [11855, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57960333e57d01ccc7110fd9"></a>
## serialize

`function` · `sqlparser::ast::RefreshModeKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11856, 38], "end": [11856, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdefa318071ba23b34c36460"></a>
## visit

`function` · `sqlparser::ast::RefreshModeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11857, 40], "end": [11857, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11857`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0ca52e9e053221ef1bce062"></a>
## visit

`function` · `sqlparser::ast::RefreshModeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RefreshModeKind", "path": "RefreshModeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11857, 47], "end": [11857, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11857`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
