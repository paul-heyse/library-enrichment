# `sqlparser::ast::LockTableTarget`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LockTableTarget.json).

<a id="op-9d1936e8749615817709672c"></a>
## LockTableTarget

`struct` · `sqlparser::ast::LockTableTarget` · sqlparser 0.62.0

```rust
struct LockTableTarget
```

Source: `src/ast/mod.rs:6554`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Target of a `LOCK TABLE` command

See <https://www.postgresql.org/docs/current/sql-lock.html>

<a id="op-c2aeef47ff661ed063d6f916"></a>
## clone

`function` · `sqlparser::ast::LockTableTarget::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LockTableTarget
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6551, 17], "end": [6551, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-930ff38105f07b9ddb6ed617"></a>
## cmp

`function` · `sqlparser::ast::LockTableTarget::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LockTableTarget) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6551, 51], "end": [6551, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cc4233d5fa15fc39a48b760"></a>
## deserialize

`function` · `sqlparser::ast::LockTableTarget::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6553, 49], "end": [6553, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6553`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b015977583325dbe7937130f"></a>
## eq

`function` · `sqlparser::ast::LockTableTarget::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LockTableTarget) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6551, 24], "end": [6551, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4518259d7412b48f127fa542"></a>
## fmt

`function` · `sqlparser::ast::LockTableTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6564, 1], "end": [6575, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70ede6d7e601f729c6d8a1e0"></a>
## fmt

`function` · `sqlparser::ast::LockTableTarget::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6551, 10], "end": [6551, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-694fb1ce5d5eadbd63a2165e"></a>
## has_asterisk

`struct_field` · `sqlparser::ast::LockTableTarget::has_asterisk` · sqlparser 0.62.0

```rust
has_asterisk: bool
```

Source: `src/ast/mod.rs:6561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `*` was specified to explicitly include descendant tables.

<a id="op-b8c5e49065613573812dea98"></a>
## hash

`function` · `sqlparser::ast::LockTableTarget::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6551, 56], "end": [6551, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e757a80e81b5d86db6d3b70"></a>
## name

`struct_field` · `sqlparser::ast::LockTableTarget::name` · sqlparser 0.62.0

```rust
name: ObjectName
```

Source: `src/ast/mod.rs:6557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the table being locked.

<a id="op-fa37ccb3409f6dc7d6d71f20"></a>
## only

`struct_field` · `sqlparser::ast::LockTableTarget::only` · sqlparser 0.62.0

```rust
only: bool
```

Source: `src/ast/mod.rs:6559`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `ONLY` was specified to exclude descendant tables.

<a id="op-dd7b9fb7df6d1b2cc984285e"></a>
## partial_cmp

`function` · `sqlparser::ast::LockTableTarget::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LockTableTarget) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6551, 35], "end": [6551, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6551`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6520b147d1ae65cc09a8c6bf"></a>
## serialize

`function` · `sqlparser::ast::LockTableTarget::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6553, 38], "end": [6553, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6553`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36cf55e1dba45bde57944f8d"></a>
## visit

`function` · `sqlparser::ast::LockTableTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6552, 40], "end": [6552, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7eb839f01211f45737d8c3b"></a>
## visit

`function` · `sqlparser::ast::LockTableTarget::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableTarget", "path": "LockTableTarget"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6552, 47], "end": [6552, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
