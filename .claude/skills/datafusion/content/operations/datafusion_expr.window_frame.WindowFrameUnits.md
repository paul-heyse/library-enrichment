# `datafusion_expr::window_frame::WindowFrameUnits`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_frame.WindowFrameUnits.json).

<a id="op-a7813d26482b02871e7a13a7"></a>
## WindowFrameUnits

`enum` · `datafusion_expr::window_frame::WindowFrameUnits` · datafusion-expr 55.1.0

```rust
enum WindowFrameUnits
```

Source: `src/window_frame.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

There are three frame types: ROWS, GROUPS, and RANGE. The frame type determines how the
starting and ending boundaries of the frame are measured.

<a id="op-00fc4776bb794ad14ace30de"></a>
## Groups

`variant` · `datafusion_expr::window_frame::WindowFrameUnits::Groups` · datafusion-expr 55.1.0

```rust
Groups
```

Source: `src/window_frame.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The GROUPS frame type means that the starting and ending boundaries are determine
by counting "groups" relative to the current group. A "group" is a set of rows that all have
equivalent values for all all terms of the window ORDER BY clause.

<a id="op-523a2ad113e1bd4a74820d17"></a>
## Range

`variant` · `datafusion_expr::window_frame::WindowFrameUnits::Range` · datafusion-expr 55.1.0

```rust
Range
```

Source: `src/window_frame.rs:469`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The RANGE frame type requires that the ORDER BY clause of the window have exactly one
term. Call that term "X". With the RANGE frame type, the elements of the frame are
determined by computing the value of expression X for all rows in the partition and framing
those rows for which the value of X is within a certain range of the value of X for the
current row.

<a id="op-ae14917a0baeee1f6b1224e9"></a>
## Rows

`variant` · `datafusion_expr::window_frame::WindowFrameUnits::Rows` · datafusion-expr 55.1.0

```rust
Rows
```

Source: `src/window_frame.rs:463`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The ROWS frame type means that the starting and ending boundaries for the frame are
determined by counting individual rows relative to the current row.

<a id="op-7924343e28ce49ea151cd576"></a>
## clone

`function` · `datafusion_expr::window_frame::WindowFrameUnits::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFrameUnits
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 17], "end": [459, 22], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_frame.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15fe41aa27419c3c1d6f7b25"></a>
## eq

`function` · `datafusion_expr::window_frame::WindowFrameUnits::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WindowFrameUnits) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 30], "end": [459, 39], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/window_frame.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91fb26a22afe73d606e0ff1b"></a>
## fmt

`function` · `datafusion_expr::window_frame::WindowFrameUnits::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 10], "end": [459, 15], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_frame.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94be50ef6d7949abc856f5d4"></a>
## fmt

`function` · `datafusion_expr::window_frame::WindowFrameUnits::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [476, 1], "end": [484, 2], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/window_frame.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04498e1a69b1c91ef1a500c9"></a>
## from

`function` · `datafusion_expr::window_frame::WindowFrameUnits::from` · datafusion-expr 55.1.0

```rust
fn from(units: protobuf::WindowFrameUnits) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "crate::WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [46, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrameUnits", "path": "WindowFrameUnits"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/proto.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-575057e46857dbd4f4bba84c"></a>
## from

`function` · `datafusion_expr::window_frame::WindowFrameUnits::from` · datafusion-expr 55.1.0

```rust
fn from(value: ast::WindowFrameUnits) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [487, 1], "end": [495, 2], "filename": "src/window_frame.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrameUnits", "path": "WindowFrameUnits"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/window_frame.rs:488`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48276fa0e9c0a6690f7df0e8"></a>
## hash

`function` · `datafusion_expr::window_frame::WindowFrameUnits::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 57], "end": [459, 61], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/window_frame.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1d1affdf3b1aa56b57dc819"></a>
## partial_cmp

`function` · `datafusion_expr::window_frame::WindowFrameUnits::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowFrameUnits) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameUnits", "path": "WindowFrameUnits"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 45], "end": [459, 55], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/window_frame.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
