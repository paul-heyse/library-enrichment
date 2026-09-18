# `sqlparser::ast::WindowFrameUnits`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WindowFrameUnits.json).

<a id="op-b405bbe3edecc78660f01db3"></a>
## WindowFrameUnits

`enum` · `sqlparser::ast::WindowFrameUnits` · sqlparser 0.62.0

```rust
enum WindowFrameUnits
```

Source: `src/ast/mod.rs:2365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Units used to describe the window frame scope.

<a id="op-e76287c147217fa8a2ac8689"></a>
## Groups

`variant` · `sqlparser::ast::WindowFrameUnits::Groups` · sqlparser 0.62.0

```rust
Groups
```

Source: `src/ast/mod.rs:2371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`GROUPS` unit.

<a id="op-44f74f131cc7ce3e8c474ef8"></a>
## Range

`variant` · `sqlparser::ast::WindowFrameUnits::Range` · sqlparser 0.62.0

```rust
Range
```

Source: `src/ast/mod.rs:2369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RANGE` unit.

<a id="op-d9fe3f734c8426b7eb2f021e"></a>
## Rows

`variant` · `sqlparser::ast::WindowFrameUnits::Rows` · sqlparser 0.62.0

```rust
Rows
```

Source: `src/ast/mod.rs:2367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ROWS` unit.

<a id="op-bfe6af6f28248c06347f6444"></a>
## clone

`function` · `sqlparser::ast::WindowFrameUnits::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WindowFrameUnits
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 23], "end": [2361, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b19aa754858714de7a4f8d1"></a>
## cmp

`function` · `sqlparser::ast::WindowFrameUnits::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WindowFrameUnits) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 57], "end": [2361, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41d0460df84e7575d5f8a899"></a>
## deserialize

`function` · `sqlparser::ast::WindowFrameUnits::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2362, 49], "end": [2362, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da0422bcd9ca8d095143ba68"></a>
## eq

`function` · `sqlparser::ast::WindowFrameUnits::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WindowFrameUnits) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 30], "end": [2361, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72b527cf4e1d41bf01c65cd2"></a>
## fmt

`function` · `sqlparser::ast::WindowFrameUnits::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2374, 1], "end": [2382, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fab30021edf8248b06c85a77"></a>
## fmt

`function` · `sqlparser::ast::WindowFrameUnits::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 10], "end": [2361, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d57658b4bcbcdb941173790e"></a>
## hash

`function` · `sqlparser::ast::WindowFrameUnits::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 62], "end": [2361, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2895d379d1550c528f456233"></a>
## partial_cmp

`function` · `sqlparser::ast::WindowFrameUnits::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WindowFrameUnits) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2361, 41], "end": [2361, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b30fc86f547df84f228a8d5e"></a>
## serialize

`function` · `sqlparser::ast::WindowFrameUnits::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2362, 38], "end": [2362, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2362`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-097238900e250e4c2aa811fb"></a>
## visit

`function` · `sqlparser::ast::WindowFrameUnits::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2363, 40], "end": [2363, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb5fad4d16cc3bc87f8e70a3"></a>
## visit

`function` · `sqlparser::ast::WindowFrameUnits::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2363, 47], "end": [2363, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
