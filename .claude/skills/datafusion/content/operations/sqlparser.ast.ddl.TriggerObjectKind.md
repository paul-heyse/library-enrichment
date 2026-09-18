# `sqlparser::ast::ddl::TriggerObjectKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.TriggerObjectKind.json).

<a id="op-58ec1f54c9302330a0c321a0"></a>
## TriggerObjectKind

`enum` · `sqlparser::ast::ddl::TriggerObjectKind` · sqlparser 0.62.0

```rust
enum TriggerObjectKind
```

Source: `src/ast/ddl.rs:3920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the syntax used for the trigger object (ROW or STATEMENT) is `FOR` or `FOR EACH`.

<a id="op-179275fd554a14b1ca77527a"></a>
## For

`variant` · `sqlparser::ast::ddl::TriggerObjectKind::For` · sqlparser 0.62.0

```rust
For
```

Source: `src/ast/ddl.rs:3922`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `FOR` syntax is used.

<a id="op-45145783eaa1cbe0f8f7fc78"></a>
## ForEach

`variant` · `sqlparser::ast::ddl::TriggerObjectKind::ForEach` · sqlparser 0.62.0

```rust
ForEach
```

Source: `src/ast/ddl.rs:3924`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `FOR EACH` syntax is used.

<a id="op-ba6a3d8174cfe6d940864a9a"></a>
## clone

`function` · `sqlparser::ast::ddl::TriggerObjectKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TriggerObjectKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 17], "end": [3916, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc4b2a7a70942e173749f435"></a>
## cmp

`function` · `sqlparser::ast::ddl::TriggerObjectKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TriggerObjectKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 57], "end": [3916, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a42b07ea9c07b5cd6e704346"></a>
## deserialize

`function` · `sqlparser::ast::ddl::TriggerObjectKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3917, 49], "end": [3917, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e543f84b869313b7282d02d2"></a>
## eq

`function` · `sqlparser::ast::ddl::TriggerObjectKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TriggerObjectKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 30], "end": [3916, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60993c912ca8ff4905b2ecc1"></a>
## fmt

`function` · `sqlparser::ast::ddl::TriggerObjectKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3927, 1], "end": [3934, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf8faf6e8cdb74e96780dec7"></a>
## fmt

`function` · `sqlparser::ast::ddl::TriggerObjectKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 10], "end": [3916, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e48a32dc6e2b0be349bceeb"></a>
## hash

`function` · `sqlparser::ast::ddl::TriggerObjectKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 62], "end": [3916, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95be1be48e67f4aba6c8bbd4"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::TriggerObjectKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TriggerObjectKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3916, 41], "end": [3916, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3916`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d06b44519ef0693dcbf5f13"></a>
## serialize

`function` · `sqlparser::ast::ddl::TriggerObjectKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3917, 38], "end": [3917, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3917`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e4ae9533afddc2db2de9ad8"></a>
## visit

`function` · `sqlparser::ast::ddl::TriggerObjectKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3918, 40], "end": [3918, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7c06f6cb9b7076134f87e45"></a>
## visit

`function` · `sqlparser::ast::ddl::TriggerObjectKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TriggerObjectKind", "path": "TriggerObjectKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3918, 47], "end": [3918, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3918`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
