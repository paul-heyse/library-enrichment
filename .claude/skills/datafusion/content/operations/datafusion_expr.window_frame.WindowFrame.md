# `datafusion_expr::window_frame::WindowFrame`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_frame.WindowFrame.json).

<a id="op-8950d5a0b2cd14d0f8c71cbe"></a>
## WindowFrame

`struct` · `datafusion_expr::window_frame::WindowFrame` · datafusion-expr 55.1.0

```rust
struct WindowFrame
```

Source: `src/window_frame.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The frame specification determines which output rows are read by an aggregate
window function. The ending frame boundary can be omitted if the `BETWEEN`
and `AND` keywords that surround the starting frame boundary are also omitted,
in which case the ending frame boundary defaults to `CURRENT ROW`.

<a id="op-8b542be1d577bc3124a9f243"></a>
## Error

`assoc_type` · `datafusion_expr::window_frame::WindowFrame::Error` · datafusion-expr 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [143, 2], "filename": "src/window_frame.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/window_frame.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa19dfde826cf54c35667085"></a>
## Error

`assoc_type` · `datafusion_expr::window_frame::WindowFrame::Error` · datafusion-expr 55.1.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "crate::WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [133, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05b2acf6f85e02192a45018a"></a>
## can_accept_multi_orderby

`function` · `datafusion_expr::window_frame::WindowFrame::can_accept_multi_orderby` · datafusion-expr 55.1.0

```rust
fn can_accept_multi_orderby(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns whether the window frame can accept multiple ORDER BY expressions.

<a id="op-70f64bfce208584222b65c09"></a>
## clone

`function` · `datafusion_expr::window_frame::WindowFrame::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFrame
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_frame.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e609b67a78c313495e3ed070"></a>
## end_bound

`struct_field` · `datafusion_expr::window_frame::WindowFrame::end_bound` · datafusion-expr 55.1.0

```rust
end_bound: WindowFrameBound
```

Source: `src/window_frame.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Ending frame boundary

<a id="op-8c231dc3d8342da2025378fc"></a>
## eq

`function` · `datafusion_expr::window_frame::WindowFrame::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WindowFrame) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/window_frame.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a08f7e341842ccda5a0cb58b"></a>
## fmt

`function` · `datafusion_expr::window_frame::WindowFrame::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [115, 2], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_frame.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b06e1db56b54e9994164b98d"></a>
## fmt

`function` · `datafusion_expr::window_frame::WindowFrame::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [104, 2], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/window_frame.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8f7a9bd7bf36f1c3eb48d0f"></a>
## hash

`function` · `datafusion_expr::window_frame::WindowFrame::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 44], "end": [38, 48], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/window_frame.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a1f60995934d668838cbd6b"></a>
## is_causal

`function` · `datafusion_expr::window_frame::WindowFrame::is_causal` · datafusion-expr 55.1.0

```rust
fn is_causal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get whether window frame is causal

<a id="op-afd982eb65e16a4a8b75fade"></a>
## is_ever_expanding

`function` · `datafusion_expr::window_frame::WindowFrame::is_ever_expanding` · datafusion-expr 55.1.0

```rust
fn is_ever_expanding(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Is the window frame ever-expanding (it always grows in the superset sense).
Useful when understanding if set-monotonicity properties of functions can
be exploited.

<a id="op-a292bc63937f747d00e34502"></a>
## new

`function` · `datafusion_expr::window_frame::WindowFrame::new` · datafusion-expr 55.1.0

```rust
fn new(order_by: Option<bool>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new, default window frame (with the meaning of default
depending on whether the frame contains an `ORDER BY` clause and this
ordering is strict (i.e. no ties).

<a id="op-bdd05106d74377bcf538b74d"></a>
## new_bounds

`function` · `datafusion_expr::window_frame::WindowFrame::new_bounds` · datafusion-expr 55.1.0

```rust
fn new_bounds(units: WindowFrameUnits, start_bound: WindowFrameBound, end_bound: WindowFrameBound) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Initializes window frame from units (type), start bound and end bound.

<a id="op-76523d23eab803a82ce6f089"></a>
## partial_cmp

`function` · `datafusion_expr::window_frame::WindowFrame::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowFrame) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 42], "filename": "src/window_frame.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/window_frame.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f177c076e1460aeafb8f0ad"></a>
## regularize_order_bys

`function` · `datafusion_expr::window_frame::WindowFrame::regularize_order_bys` · datafusion-expr 55.1.0

```rust
fn regularize_order_bys(&self, order_by: &mut Vec<Sort>) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Regularizes the ORDER BY clause of the window frame.

<a id="op-68fa73b9f7b4a0f339467ea2"></a>
## reverse

`function` · `datafusion_expr::window_frame::WindowFrame::reverse` · datafusion-expr 55.1.0

```rust
fn reverse(&self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [298, 2], "filename": "src/window_frame.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_frame.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Get reversed window frame. For example
`3 ROWS PRECEDING AND 2 ROWS FOLLOWING` -->
`2 ROWS PRECEDING AND 3 ROWS FOLLOWING`

<a id="op-d73c3e896a2761b284c92244"></a>
## start_bound

`struct_field` · `datafusion_expr::window_frame::WindowFrame::start_bound` · datafusion-expr 55.1.0

```rust
start_bound: WindowFrameBound
```

Source: `src/window_frame.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Starting frame boundary

<a id="op-7fdc6a2f1784e8aa02261905"></a>
## try_from

`function` · `datafusion_expr::window_frame::WindowFrame::try_from` · datafusion-expr 55.1.0

```rust
fn try_from(value: ast::WindowFrame) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [143, 2], "filename": "src/window_frame.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::WindowFrame", "path": "WindowFrame"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/window_frame.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba0a8b6c51ffbd13065d3eea"></a>
## try_from

`function` · `datafusion_expr::window_frame::WindowFrame::try_from` · datafusion-expr 55.1.0

```rust
fn try_from(window: protobuf::WindowFrame) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_frame::WindowFrame", "path": "crate::WindowFrame"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [133, 2], "filename": "src/proto.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::WindowFrame", "path": "WindowFrame"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/proto.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4870d37f7d4ccdbdfd5a15f8"></a>
## units

`struct_field` · `datafusion_expr::window_frame::WindowFrame::units` · datafusion-expr 55.1.0

```rust
units: WindowFrameUnits
```

Source: `src/window_frame.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Frame type - either `ROWS`, `RANGE` or `GROUPS`
