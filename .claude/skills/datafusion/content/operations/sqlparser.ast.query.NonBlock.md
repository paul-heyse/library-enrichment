# `sqlparser::ast::query::NonBlock`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.NonBlock.json).

<a id="op-2b273eeb89a8215f2686cd35"></a>
## NonBlock

`enum` · `sqlparser::ast::query::NonBlock` · sqlparser 0.62.0

```rust
enum NonBlock
```

Source: `src/ast/query.rs:3559`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Non-blocking lock options for `FOR ...` clauses.

<a id="op-2b47751a5132ce5b1f9f7463"></a>
## Nowait

`variant` · `sqlparser::ast::query::NonBlock::Nowait` · sqlparser 0.62.0

```rust
Nowait
```

Source: `src/ast/query.rs:3561`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NOWAIT` — do not wait for the lock.

<a id="op-c317eff7061d98ece42de50e"></a>
## SkipLocked

`variant` · `sqlparser::ast::query::NonBlock::SkipLocked` · sqlparser 0.62.0

```rust
SkipLocked
```

Source: `src/ast/query.rs:3563`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SKIP LOCKED` — skip rows that are locked.

<a id="op-7b218bd3c73b96c40e69ea34"></a>
## clone

`function` · `sqlparser::ast::query::NonBlock::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NonBlock
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3555, 23], "end": [3555, 28], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e62fcb8219f0a1d74030652"></a>
## cmp

`function` · `sqlparser::ast::query::NonBlock::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NonBlock) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3555, 57], "end": [3555, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-342e415441dd5879f50d5331"></a>
## deserialize

`function` · `sqlparser::ast::query::NonBlock::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3556, 49], "end": [3556, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88fca0f833e12ce572258a1a"></a>
## eq

`function` · `sqlparser::ast::query::NonBlock::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NonBlock) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3555, 30], "end": [3555, 39], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b11b3a265a0366e39fe43d3"></a>
## fmt

`function` · `sqlparser::ast::query::NonBlock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3566, 1], "end": [3574, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b464e65e7be50178ae152e64"></a>
## fmt

`function` · `sqlparser::ast::query::NonBlock::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3555, 10], "end": [3555, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-176a108c13b4f1f7f44abca9"></a>
## hash

`function` · `sqlparser::ast::query::NonBlock::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3555, 62], "end": [3555, 66], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4763d21d8ddeeb5e540fa89a"></a>
## partial_cmp

`function` · `sqlparser::ast::query::NonBlock::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NonBlock) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3555, 41], "end": [3555, 51], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3555`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12618e7009bd7e276c6efefe"></a>
## serialize

`function` · `sqlparser::ast::query::NonBlock::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3556, 38], "end": [3556, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12d2fad7cb421db1fe80167e"></a>
## visit

`function` · `sqlparser::ast::query::NonBlock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3557, 47], "end": [3557, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91e77e47bfe30e953b9a9f53"></a>
## visit

`function` · `sqlparser::ast::query::NonBlock::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::NonBlock", "path": "NonBlock"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3557, 40], "end": [3557, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
