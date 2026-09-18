# `sqlparser::ast::Lock`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Lock.json).

<a id="op-97e0f56fcda0b9ffb732974a"></a>
## Lock

`struct` · `sqlparser::ast::Lock` · sqlparser 0.62.0

```rust
struct Lock
```

Source: `src/ast/mod.rs:6526`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `LOCK` statement.

See <https://www.postgresql.org/docs/current/sql-lock.html>

<a id="op-cc43532f2008d627afa15620"></a>
## clone

`function` · `sqlparser::ast::Lock::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Lock
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6523, 17], "end": [6523, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a172e44f108faf669170551"></a>
## cmp

`function` · `sqlparser::ast::Lock::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Lock) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6523, 51], "end": [6523, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69118d099c49628897b00d34"></a>
## deserialize

`function` · `sqlparser::ast::Lock::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6525, 49], "end": [6525, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30b10892120bad2a45cd71e9"></a>
## eq

`function` · `sqlparser::ast::Lock::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Lock) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6523, 24], "end": [6523, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-696883fac5d4aba2d791572e"></a>
## fmt

`function` · `sqlparser::ast::Lock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6535, 1], "end": [6546, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6536`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92cef70ee745f4e95de7f73a"></a>
## fmt

`function` · `sqlparser::ast::Lock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6523, 10], "end": [6523, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1c974157e28bbf4cd09c27d"></a>
## hash

`function` · `sqlparser::ast::Lock::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6523, 56], "end": [6523, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdf3072b41a20994a5b8ab9a"></a>
## lock_mode

`struct_field` · `sqlparser::ast::Lock::lock_mode` · sqlparser 0.62.0

```rust
lock_mode: Option<LockTableMode>
```

Source: `src/ast/mod.rs:6530`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Lock mode.

<a id="op-1de62f6dc24397696ebb7d1f"></a>
## nowait

`struct_field` · `sqlparser::ast::Lock::nowait` · sqlparser 0.62.0

```rust
nowait: bool
```

Source: `src/ast/mod.rs:6532`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `NOWAIT` was specified.

<a id="op-2bdf3366842660f164e0ffac"></a>
## partial_cmp

`function` · `sqlparser::ast::Lock::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Lock) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6523, 35], "end": [6523, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a34bde73ff24396a0508727d"></a>
## serialize

`function` · `sqlparser::ast::Lock::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6525, 38], "end": [6525, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-821e35c0792ff49acdda4506"></a>
## tables

`struct_field` · `sqlparser::ast::Lock::tables` · sqlparser 0.62.0

```rust
tables: Vec<LockTableTarget>
```

Source: `src/ast/mod.rs:6528`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of tables to lock.

<a id="op-2733592efcd10ffb517487d2"></a>
## visit

`function` · `sqlparser::ast::Lock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6524, 40], "end": [6524, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b5c9194bcab9af8370d8daa"></a>
## visit

`function` · `sqlparser::ast::Lock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Lock", "path": "Lock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6524, 47], "end": [6524, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
