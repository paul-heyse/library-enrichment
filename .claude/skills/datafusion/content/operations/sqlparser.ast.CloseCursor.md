# `sqlparser::ast::CloseCursor`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CloseCursor.json).

<a id="op-0c688b0613c769a437e5d452"></a>
## CloseCursor

`enum` · `sqlparser::ast::CloseCursor` · sqlparser 0.62.0

```rust
enum CloseCursor
```

Source: `src/ast/mod.rs:7966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Which cursor(s) to close.

<a id="op-46ca072194c1a2e329ecec99"></a>
## All

`variant` · `sqlparser::ast::CloseCursor::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/mod.rs:7968`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Close all cursors.

<a id="op-f287ca90c6e02543cc05051e"></a>
## Specific

`variant` · `sqlparser::ast::CloseCursor::Specific` · sqlparser 0.62.0

```rust
Specific
```

Source: `src/ast/mod.rs:7970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Close a specific cursor by name.

<a id="op-fb4d37b81e5b9b5409da3b2e"></a>
## clone

`function` · `sqlparser::ast::CloseCursor::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CloseCursor
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7962, 17], "end": [7962, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b25816941be5db4266c8da7"></a>
## cmp

`function` · `sqlparser::ast::CloseCursor::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CloseCursor) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7962, 51], "end": [7962, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39246ff196c8cfd362bd7ec0"></a>
## deserialize

`function` · `sqlparser::ast::CloseCursor::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7963, 49], "end": [7963, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbf1dd7ce12bffddd20eac9d"></a>
## eq

`function` · `sqlparser::ast::CloseCursor::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CloseCursor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7962, 24], "end": [7962, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73b29842c1f4968640452e02"></a>
## fmt

`function` · `sqlparser::ast::CloseCursor::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7962, 10], "end": [7962, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a08e6aa1c4442c6a00d3f1c3"></a>
## fmt

`function` · `sqlparser::ast::CloseCursor::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7976, 1], "end": [7983, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7977`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b95107529a9ec23f4a4426e3"></a>
## hash

`function` · `sqlparser::ast::CloseCursor::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7962, 56], "end": [7962, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc5cb07e39f1e8c5f3790683"></a>
## partial_cmp

`function` · `sqlparser::ast::CloseCursor::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CloseCursor) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7962, 35], "end": [7962, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7962`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-350a5b3281251ed667c87b25"></a>
## serialize

`function` · `sqlparser::ast::CloseCursor::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7963, 38], "end": [7963, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7963`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b4e6ff97bd634f94364777a"></a>
## visit

`function` · `sqlparser::ast::CloseCursor::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7964, 47], "end": [7964, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b508c8f6b93e2d711202f60e"></a>
## visit

`function` · `sqlparser::ast::CloseCursor::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CloseCursor", "path": "CloseCursor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7964, 40], "end": [7964, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
