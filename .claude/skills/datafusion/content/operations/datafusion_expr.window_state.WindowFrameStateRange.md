# `datafusion_expr::window_state::WindowFrameStateRange`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_state.WindowFrameStateRange.json).

<a id="op-c19f04c90083f16996987c1d"></a>
## WindowFrameStateRange

`struct` · `datafusion_expr::window_state::WindowFrameStateRange` · datafusion-expr 55.1.0

```rust
struct WindowFrameStateRange
```

Source: `src/window_state.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This structure encapsulates all the state information we require as we scan
ranges of data while processing RANGE frames.
Attribute `sort_options` stores the column ordering specified by the ORDER
BY clause. This information is used to calculate the range.

<a id="op-e839d43bd18ed6255adf09aa"></a>
## clone

`function` · `datafusion_expr::window_state::WindowFrameStateRange::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFrameStateRange
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameStateRange", "path": "WindowFrameStateRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 26], "end": [310, 31], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_state.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-195d58a22514bff362f376c6"></a>
## default

`function` · `datafusion_expr::window_state::WindowFrameStateRange::default` · datafusion-expr 55.1.0

```rust
fn default() -> WindowFrameStateRange
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameStateRange", "path": "WindowFrameStateRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 17], "end": [310, 24], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/window_state.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb0039e2f9d1b7e1031c315f"></a>
## fmt

`function` · `datafusion_expr::window_state::WindowFrameStateRange::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameStateRange", "path": "WindowFrameStateRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 10], "end": [310, 15], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_state.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
