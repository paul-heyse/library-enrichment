# `datafusion_expr::window_frame::WindowFrameBound`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_frame.WindowFrameBound.json).

<a id="op-a7f3fd61a40047b838ee1ecc"></a>
## WindowFrameBound

`enum` · `datafusion_expr::window_frame::WindowFrameBound` · datafusion-expr 55.1.0

```rust
enum WindowFrameBound
```

Source: `src/window_frame.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

There are five ways to describe starting and ending frame boundaries:

1. UNBOUNDED PRECEDING
2. `<expr>` PRECEDING
3. CURRENT ROW
4. `<expr>` FOLLOWING
5. UNBOUNDED FOLLOWING

<a id="op-9325b33a9b8a99870bfa823b"></a>
## CurrentRow

`variant` · `datafusion_expr::window_frame::WindowFrameBound::CurrentRow` · datafusion-expr 55.1.0

```rust
CurrentRow
```

Source: `src/window_frame.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

3. The current row.

For RANGE and GROUPS frame types, peers of the current row are also
included in the frame, unless specifically excluded by the EXCLUDE clause.
This is true regardless of whether CURRENT ROW is used as the starting or ending frame
boundary.

<a id="op-432618fbaec800273fa88e5a"></a>
## Error

`assoc_type` · `datafusion_expr::window_frame::WindowFrameBound::Error` · datafusion-expr 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "crate::WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [82, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrameBound", "path": "WindowFrameBound"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e634f7b99f275d5b7fcbd3e"></a>
## Following

`variant` · `datafusion_expr::window_frame::WindowFrameBound::Following` · datafusion-expr 55.1.0

```rust
Following
```

Source: `src/window_frame.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

4. This is the same as "`<expr>` PRECEDING" except that the boundary is `<expr>` units after the
   current rather than before the current row.

5. UNBOUNDED FOLLOWING
   The frame boundary is the last row in the partition.

<a id="op-60c1e944a94677913da6ad35"></a>
## Preceding

`variant` · `datafusion_expr::window_frame::WindowFrameBound::Preceding` · datafusion-expr 55.1.0

```rust
Preceding
```

Source: `src/window_frame.rs:315`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

1. UNBOUNDED PRECEDING
   The frame boundary is the first row in the partition.

2. `<expr>` PRECEDING
   `<expr>` must be a non-negative constant numeric expression. The boundary is a row that
   is `<expr>` "units" prior to the current row.

<a id="op-a1e0aa613020e5cf3f7e035d"></a>
## clone

`function` · `datafusion_expr::window_frame::WindowFrameBound::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFrameBound
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 17], "end": [307, 22], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_frame.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41ac411401e1490346ce718e"></a>
## eq

`function` · `datafusion_expr::window_frame::WindowFrameBound::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WindowFrameBound) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 24], "end": [307, 33], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/window_frame.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-474765739e389cd39b43f812"></a>
## fmt

`function` · `datafusion_expr::window_frame::WindowFrameBound::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 10], "end": [307, 15], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_frame.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b963fb2251756255cae8f1dd"></a>
## fmt

`function` · `datafusion_expr::window_frame::WindowFrameBound::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [455, 2], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/window_frame.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc8f5ebf8a14854d86933c7d"></a>
## hash

`function` · `datafusion_expr::window_frame::WindowFrameBound::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 51], "end": [307, 55], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/window_frame.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e19a9fe60c6b35a488261a0c"></a>
## is_unbounded

`function` · `datafusion_expr::window_frame::WindowFrameBound::is_unbounded` · datafusion-expr 55.1.0

```rust
fn is_unbounded(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [331, 1], "end": [339, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57cb7a502765fab6581fb418"></a>
## partial_cmp

`function` · `datafusion_expr::window_frame::WindowFrameBound::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowFrameBound) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 39], "end": [307, 49], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/window_frame.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9d6f67996dc9192a6496461"></a>
## try_from

`function` · `datafusion_expr::window_frame::WindowFrameBound::try_from` · datafusion-expr 55.1.0

```rust
fn try_from(bound: protobuf::WindowFrameBound) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrameBound", "path": "crate::WindowFrameBound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [82, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrameBound", "path": "WindowFrameBound"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
