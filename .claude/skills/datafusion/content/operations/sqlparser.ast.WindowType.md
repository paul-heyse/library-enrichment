# `sqlparser::ast::WindowType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WindowType.json).

<a id="op-c296d0be72cbc27e97f46a77"></a>
## WindowType

`enum` · `sqlparser::ast::WindowType` · sqlparser 0.62.0

```rust
enum WindowType
```

Source: `src/ast/mod.rs:2239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of a window used in `OVER` clauses.

A window can be either an inline specification (`WindowSpec`) or a
reference to a previously defined named window.

- `WindowSpec(WindowSpec)`: An inline window specification, e.g.
  `OVER (PARTITION BY ... ORDER BY ...)`.
- `NamedWindow(Ident)`: A reference to a named window declared elsewhere.

<a id="op-72c86c66b3478955bc04b574"></a>
## NamedWindow

`variant` · `sqlparser::ast::WindowType::NamedWindow` · sqlparser 0.62.0

```rust
NamedWindow
```

Source: `src/ast/mod.rs:2243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A reference to a previously defined named window.

<a id="op-ac3be4023d42b8fb30ce134a"></a>
## WindowSpec

`variant` · `sqlparser::ast::WindowType::WindowSpec` · sqlparser 0.62.0

```rust
WindowSpec
```

Source: `src/ast/mod.rs:2241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An inline window specification.

<a id="op-c8cf199285b7d3c93632f648"></a>
## clone

`function` · `sqlparser::ast::WindowType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WindowType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2236, 17], "end": [2236, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88bbb953141b3853e6cd6efa"></a>
## cmp

`function` · `sqlparser::ast::WindowType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WindowType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2236, 51], "end": [2236, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3a6078aabef14542972f2a7"></a>
## deserialize

`function` · `sqlparser::ast::WindowType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2237, 49], "end": [2237, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17ad95228541307b6dcec1b7"></a>
## eq

`function` · `sqlparser::ast::WindowType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WindowType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2236, 24], "end": [2236, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34d6feaaac57df30debf8a81"></a>
## fmt

`function` · `sqlparser::ast::WindowType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2236, 10], "end": [2236, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7eac0d64b67e72c2a0861a43"></a>
## fmt

`function` · `sqlparser::ast::WindowType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2246, 1], "end": [2259, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2247`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-603be73ac02457d727b604b1"></a>
## hash

`function` · `sqlparser::ast::WindowType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2236, 56], "end": [2236, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d320ea2287de28278bc57633"></a>
## partial_cmp

`function` · `sqlparser::ast::WindowType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WindowType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2236, 35], "end": [2236, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2236`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc74e1519fffb72102303797"></a>
## serialize

`function` · `sqlparser::ast::WindowType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2237, 38], "end": [2237, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38da1f9f4d983e7488ba8e1b"></a>
## visit

`function` · `sqlparser::ast::WindowType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2238, 47], "end": [2238, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ad3fdae96bb81fb2e99472f"></a>
## visit

`function` · `sqlparser::ast::WindowType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowType", "path": "WindowType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2238, 40], "end": [2238, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2238`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
