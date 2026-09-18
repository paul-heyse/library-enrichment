# `sqlparser::ast::ShowObjects`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowObjects.json).

<a id="op-111d7d6e492d753ea99dc6b1"></a>
## ShowObjects

`struct` · `sqlparser::ast::ShowObjects` · sqlparser 0.62.0

```rust
struct ShowObjects
```

Source: `src/ast/mod.rs:10923`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options for a `SHOW OBJECTS` statement.

<a id="op-21656d380e71a3efbd3bcd76"></a>
## clone

`function` · `sqlparser::ast::ShowObjects::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowObjects
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10919, 17], "end": [10919, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-636d9ea5f05fea87af1700b7"></a>
## cmp

`function` · `sqlparser::ast::ShowObjects::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowObjects) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10919, 51], "end": [10919, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cd3beb5083bd30537d37e85"></a>
## deserialize

`function` · `sqlparser::ast::ShowObjects::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10920, 49], "end": [10920, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f24274074695ee9e5bdf8cd1"></a>
## eq

`function` · `sqlparser::ast::ShowObjects::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowObjects) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10919, 24], "end": [10919, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8e7bf215b86fd1de905a943"></a>
## fmt

`function` · `sqlparser::ast::ShowObjects::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10919, 10], "end": [10919, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b54fc3a8f58bccf77ca80d9"></a>
## hash

`function` · `sqlparser::ast::ShowObjects::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10919, 56], "end": [10919, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a60659f04aca6443a8d4b1d9"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowObjects::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowObjects) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10919, 35], "end": [10919, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10919`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fc2304998710af64aaedad8"></a>
## serialize

`function` · `sqlparser::ast::ShowObjects::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10920, 38], "end": [10920, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10920`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3ff276062d2a3913ab4fa1a"></a>
## show_options

`struct_field` · `sqlparser::ast::ShowObjects::show_options` · sqlparser 0.62.0

```rust
show_options: ShowStatementOptions
```

Source: `src/ast/mod.rs:10927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options controlling the SHOW output.

<a id="op-9fcbc30f8dda8546733bf595"></a>
## terse

`struct_field` · `sqlparser::ast::ShowObjects::terse` · sqlparser 0.62.0

```rust
terse: bool
```

Source: `src/ast/mod.rs:10925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to show terse output.

<a id="op-dc216eae98c47e7245a805d0"></a>
## visit

`function` · `sqlparser::ast::ShowObjects::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10921, 40], "end": [10921, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10921`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec61e8c7c7a5dba76501c95d"></a>
## visit

`function` · `sqlparser::ast::ShowObjects::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowObjects", "path": "ShowObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10921, 47], "end": [10921, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10921`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
