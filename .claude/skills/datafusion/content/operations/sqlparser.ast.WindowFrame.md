# `sqlparser::ast::WindowFrame`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WindowFrame.json).

<a id="op-454ad5a072bd35c144616b74"></a>
## WindowFrame

`struct` · `sqlparser::ast::WindowFrame` · sqlparser 0.62.0

```rust
struct WindowFrame
```

Source: `src/ast/mod.rs:2336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies the data processed by a window function, e.g.
`RANGE UNBOUNDED PRECEDING` or `ROWS BETWEEN 5 PRECEDING AND CURRENT ROW`.

Note: The parser does not validate the specified bounds; the caller should
reject invalid bounds like `ROWS UNBOUNDED FOLLOWING` before execution.

<a id="op-b3f73c962c9f376977398931"></a>
## clone

`function` · `sqlparser::ast::WindowFrame::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WindowFrame
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 17], "end": [2333, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea2ae511e96138451690ddc3"></a>
## cmp

`function` · `sqlparser::ast::WindowFrame::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WindowFrame) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 51], "end": [2333, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a3051a06147162a445a14d7"></a>
## default

`function` · `sqlparser::ast::WindowFrame::default` · sqlparser 0.62.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2348, 1], "end": [2359, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/mod.rs:2352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns default value for window frame

See [this page](https://www.sqlite.org/windowfunctions.html#frame_specifications) for more details.

<a id="op-bf96967d8d15aecdb9dbcef7"></a>
## deserialize

`function` · `sqlparser::ast::WindowFrame::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2334, 49], "end": [2334, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a855a289609cb89e6c6d8ac5"></a>
## end_bound

`struct_field` · `sqlparser::ast::WindowFrame::end_bound` · sqlparser 0.62.0

```rust
end_bound: Option<WindowFrameBound>
```

Source: `src/ast/mod.rs:2344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The right bound of the `BETWEEN .. AND` clause. The end bound of `None`
indicates the shorthand form (e.g. `ROWS 1 PRECEDING`), which must
behave the same as `end_bound = WindowFrameBound::CurrentRow`.

<a id="op-a3113d9462a47c8aa692d4c2"></a>
## eq

`function` · `sqlparser::ast::WindowFrame::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WindowFrame) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 24], "end": [2333, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cffc155b425cfc052ec56d9c"></a>
## fmt

`function` · `sqlparser::ast::WindowFrame::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 10], "end": [2333, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24d38fdd74b92df4bbf17c0a"></a>
## hash

`function` · `sqlparser::ast::WindowFrame::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 56], "end": [2333, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4800dc141c0f1eab64dd7bc8"></a>
## partial_cmp

`function` · `sqlparser::ast::WindowFrame::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WindowFrame) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2333, 35], "end": [2333, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21e4c97dfce18fdb8942c9d9"></a>
## serialize

`function` · `sqlparser::ast::WindowFrame::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2334, 38], "end": [2334, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0300fc585b5175f32f720b2d"></a>
## start_bound

`struct_field` · `sqlparser::ast::WindowFrame::start_bound` · sqlparser 0.62.0

```rust
start_bound: WindowFrameBound
```

Source: `src/ast/mod.rs:2340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The start bound of the window frame.

<a id="op-883e1b94ffcc0c774688f0e9"></a>
## units

`struct_field` · `sqlparser::ast::WindowFrame::units` · sqlparser 0.62.0

```rust
units: WindowFrameUnits
```

Source: `src/ast/mod.rs:2338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Units for the frame (e.g. `ROWS`, `RANGE`, `GROUPS`).

<a id="op-9df04dcb1da80bdf0b3a8318"></a>
## visit

`function` · `sqlparser::ast::WindowFrame::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2335, 47], "end": [2335, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8515131e0e0402afcb0801c"></a>
## visit

`function` · `sqlparser::ast::WindowFrame::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2335, 40], "end": [2335, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
