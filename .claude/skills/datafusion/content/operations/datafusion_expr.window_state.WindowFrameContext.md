# `datafusion_expr::window_state::WindowFrameContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_state.WindowFrameContext.json).

<a id="op-6640a81b1bfa2f7459f71ca4"></a>
## WindowFrameContext

`enum` · `datafusion_expr::window_state::WindowFrameContext` · datafusion-expr 55.1.0

```rust
enum WindowFrameContext
```

Source: `src/window_state.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This object stores the window frame state for use in incremental calculations.

<a id="op-d57d5798205e5190a14f6627"></a>
## Groups

`variant` · `datafusion_expr::window_state::WindowFrameContext::Groups` · datafusion-expr 55.1.0

```rust
Groups
```

Source: `src/window_state.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

GROUPS frames are stateful, they store group boundaries and indices
specifying where the previous search left off. This amortizes the
overall cost to O(n) where n denotes the row count.

<a id="op-3aaa259257a61c6b0a151ce3"></a>
## Range

`variant` · `datafusion_expr::window_state::WindowFrameContext::Range` · datafusion-expr 55.1.0

```rust
Range
```

Source: `src/window_state.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

RANGE frames are stateful, they store indices specifying where the
previous search left off. This amortizes the overall cost to O(n)
where n denotes the row count.

<a id="op-b7181e33bb77c30f6b7166c2"></a>
## Rows

`variant` · `datafusion_expr::window_state::WindowFrameContext::Rows` · datafusion-expr 55.1.0

```rust
Rows
```

Source: `src/window_state.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

ROWS frames are inherently stateless.

<a id="op-5dd75487913ace9a12270cf9"></a>
## calculate_range

`function` · `datafusion_expr::window_state::WindowFrameContext::calculate_range` · datafusion-expr 55.1.0

```rust
fn calculate_range(&mut self, range_columns: &[ArrayRef], last_range: &Range<usize>, length: usize, idx: usize) -> Result<Range<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameContext", "path": "WindowFrameContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [269, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This function calculates beginning/ending indices for the frame of the current row.

<a id="op-376e9d9eb5b00314f48dac45"></a>
## clone

`function` · `datafusion_expr::window_state::WindowFrameContext::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFrameContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameContext", "path": "WindowFrameContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 17], "end": [144, 22], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_state.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a8ac1dfa8200ac05b22a5f1"></a>
## fmt

`function` · `datafusion_expr::window_state::WindowFrameContext::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameContext", "path": "WindowFrameContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 10], "end": [144, 15], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_state.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3606e4ac0d1f6e8bfa425a7"></a>
## new

`function` · `datafusion_expr::window_state::WindowFrameContext::new` · datafusion-expr 55.1.0

```rust
fn new(window_frame: Arc<WindowFrame>, sort_options: Vec<SortOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameContext", "path": "WindowFrameContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [164, 1], "end": [269, 2], "filename": "src/window_state.rs"}, "trait": null, "trait_path": null}`

Source: `src/window_state.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new state object for the given window frame.
