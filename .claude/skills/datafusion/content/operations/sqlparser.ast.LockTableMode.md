# `sqlparser::ast::LockTableMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LockTableMode.json).

<a id="op-ddc5dbcbb4eccbf723c0baab"></a>
## LockTableMode

`enum` · `sqlparser::ast::LockTableMode` · sqlparser 0.62.0

```rust
enum LockTableMode
```

Source: `src/ast/mod.rs:6583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL lock modes for `LOCK TABLE`.

See <https://www.postgresql.org/docs/current/sql-lock.html>

<a id="op-30f87360b27914f77a8d9300"></a>
## AccessExclusive

`variant` · `sqlparser::ast::LockTableMode::AccessExclusive` · sqlparser 0.62.0

```rust
AccessExclusive
```

Source: `src/ast/mod.rs:6599`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ACCESS EXCLUSIVE`

<a id="op-1a142f7c8f0af4d51ba89b24"></a>
## AccessShare

`variant` · `sqlparser::ast::LockTableMode::AccessShare` · sqlparser 0.62.0

```rust
AccessShare
```

Source: `src/ast/mod.rs:6585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ACCESS SHARE`

<a id="op-455683047955886e539b678f"></a>
## Exclusive

`variant` · `sqlparser::ast::LockTableMode::Exclusive` · sqlparser 0.62.0

```rust
Exclusive
```

Source: `src/ast/mod.rs:6597`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EXCLUSIVE`

<a id="op-648ea6e25c4e52066b0a38cb"></a>
## RowExclusive

`variant` · `sqlparser::ast::LockTableMode::RowExclusive` · sqlparser 0.62.0

```rust
RowExclusive
```

Source: `src/ast/mod.rs:6589`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROW EXCLUSIVE`

<a id="op-3c8ccd5f60dc36ea2cb4f411"></a>
## RowShare

`variant` · `sqlparser::ast::LockTableMode::RowShare` · sqlparser 0.62.0

```rust
RowShare
```

Source: `src/ast/mod.rs:6587`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROW SHARE`

<a id="op-3155a5442079ef989135e93e"></a>
## Share

`variant` · `sqlparser::ast::LockTableMode::Share` · sqlparser 0.62.0

```rust
Share
```

Source: `src/ast/mod.rs:6593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHARE`

<a id="op-41e2c8fe890842f07453c377"></a>
## ShareRowExclusive

`variant` · `sqlparser::ast::LockTableMode::ShareRowExclusive` · sqlparser 0.62.0

```rust
ShareRowExclusive
```

Source: `src/ast/mod.rs:6595`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHARE ROW EXCLUSIVE`

<a id="op-7b9c573910a094f48d721f83"></a>
## ShareUpdateExclusive

`variant` · `sqlparser::ast::LockTableMode::ShareUpdateExclusive` · sqlparser 0.62.0

```rust
ShareUpdateExclusive
```

Source: `src/ast/mod.rs:6591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SHARE UPDATE EXCLUSIVE`

<a id="op-29426f374cd41d8787de9840"></a>
## clone

`function` · `sqlparser::ast::LockTableMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LockTableMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6580, 17], "end": [6580, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e96a6eb26188c572d5e16c9c"></a>
## cmp

`function` · `sqlparser::ast::LockTableMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LockTableMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6580, 51], "end": [6580, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b74debcce447ae53c01b4f3"></a>
## deserialize

`function` · `sqlparser::ast::LockTableMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6581, 49], "end": [6581, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f89c19b5502ae0985cd9379b"></a>
## eq

`function` · `sqlparser::ast::LockTableMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LockTableMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6580, 24], "end": [6580, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48907df61e2beced8cae7514"></a>
## fmt

`function` · `sqlparser::ast::LockTableMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6580, 10], "end": [6580, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8c43e3b589e9d8a5feea311"></a>
## fmt

`function` · `sqlparser::ast::LockTableMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6602, 1], "end": [6616, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6603`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6de061372a6e2a2541b3f503"></a>
## hash

`function` · `sqlparser::ast::LockTableMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6580, 56], "end": [6580, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ce74264f65fe506116687d9"></a>
## partial_cmp

`function` · `sqlparser::ast::LockTableMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LockTableMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6580, 35], "end": [6580, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2477dda1ec7787204bd7bd86"></a>
## serialize

`function` · `sqlparser::ast::LockTableMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6581, 38], "end": [6581, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6581`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cf9ec0ca6be8be8c72c6449"></a>
## visit

`function` · `sqlparser::ast::LockTableMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6582, 40], "end": [6582, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd067ba4084bdf128a81d285"></a>
## visit

`function` · `sqlparser::ast::LockTableMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LockTableMode", "path": "LockTableMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6582, 47], "end": [6582, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
