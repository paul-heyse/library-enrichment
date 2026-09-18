# `sqlparser::ast::ddl::DropBehavior`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropBehavior.json).

<a id="op-800fab9a43b58907534c1ff3"></a>
## DropBehavior

`enum` · `sqlparser::ast::ddl::DropBehavior` · sqlparser 0.62.0

```rust
enum DropBehavior
```

Source: `src/ast/ddl.rs:2344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<drop behavior> ::= CASCADE | RESTRICT`.

Used in `DROP` statements.

<a id="op-1ca66791c371a03f9c777b3d"></a>
## Cascade

`variant` · `sqlparser::ast::ddl::DropBehavior::Cascade` · sqlparser 0.62.0

```rust
Cascade
```

Source: `src/ast/ddl.rs:2348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE` - automatically drop objects that depend on the object being dropped.

<a id="op-e79a88e5a26ab2a74542cb1b"></a>
## Restrict

`variant` · `sqlparser::ast::ddl::DropBehavior::Restrict` · sqlparser 0.62.0

```rust
Restrict
```

Source: `src/ast/ddl.rs:2346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RESTRICT` - refuse to drop if there are any dependent objects.

<a id="op-07972efedcc4d278a6392243"></a>
## clone

`function` · `sqlparser::ast::ddl::DropBehavior::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropBehavior
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 23], "end": [2341, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e599ca22a4296311f22761bf"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropBehavior::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropBehavior) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 57], "end": [2341, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5b82503727c127632d770f1"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropBehavior::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2342, 49], "end": [2342, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2342`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e89492864164f3fdffd507e5"></a>
## eq

`function` · `sqlparser::ast::ddl::DropBehavior::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropBehavior) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 30], "end": [2341, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03087a2d0fe6bcb587a5f2a0"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropBehavior::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 10], "end": [2341, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd9239587250f01c17c689a4"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropBehavior::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2351, 1], "end": [2358, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dde0b67d528f41f11803a71"></a>
## hash

`function` · `sqlparser::ast::ddl::DropBehavior::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 62], "end": [2341, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f04477f78893e0a6a25b9a0"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropBehavior::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropBehavior) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2341, 41], "end": [2341, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95e9e8214bbe12dfc468e806"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropBehavior::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2342, 38], "end": [2342, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2342`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d5d39229afc8a4870bcd7a2"></a>
## visit

`function` · `sqlparser::ast::ddl::DropBehavior::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2343, 40], "end": [2343, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9722f6b62f287ba513d5a45c"></a>
## visit

`function` · `sqlparser::ast::ddl::DropBehavior::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropBehavior", "path": "DropBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2343, 47], "end": [2343, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
