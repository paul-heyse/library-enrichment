# `datafusion_expr::window_state::WindowFrameStateGroups`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_state.WindowFrameStateGroups.json).

<a id="op-a8b1906b63d9de9ae24ba2d7"></a>
## WindowFrameStateGroups

`struct` · `datafusion_expr::window_state::WindowFrameStateGroups` · datafusion-expr 55.1.0

```rust
struct WindowFrameStateGroups
```

Source: `src/window_state.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This structure encapsulates all the state information we require as we
scan groups of data while processing window frames.

<a id="op-8b5936383d7959b9dba75c61"></a>
## clone

`function` · `datafusion_expr::window_state::WindowFrameStateGroups::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFrameStateGroups
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameStateGroups", "path": "WindowFrameStateGroups"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 26], "end": [493, 31], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/window_state.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feac1f13162e3ae7dc2b57d1"></a>
## current_group_idx

`struct_field` · `datafusion_expr::window_state::WindowFrameStateGroups::current_group_idx` · datafusion-expr 55.1.0

```rust
current_group_idx: usize
```

Source: `src/window_state.rs:500`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The group index to which the row index belongs.

<a id="op-1bc047aad6b5e22d7b2c23d5"></a>
## default

`function` · `datafusion_expr::window_state::WindowFrameStateGroups::default` · datafusion-expr 55.1.0

```rust
fn default() -> WindowFrameStateGroups
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameStateGroups", "path": "WindowFrameStateGroups"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 17], "end": [493, 24], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/window_state.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0635d07651ef9b7ac11ceaf2"></a>
## fmt

`function` · `datafusion_expr::window_state::WindowFrameStateGroups::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::window_state::WindowFrameStateGroups", "path": "WindowFrameStateGroups"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [493, 10], "end": [493, 15], "filename": "src/window_state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/window_state.rs:493`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab44a91841dd4aa136b30ba8"></a>
## group_end_indices

`struct_field` · `datafusion_expr::window_state::WindowFrameStateGroups::group_end_indices` · datafusion-expr 55.1.0

```rust
group_end_indices: std::collections::VecDeque<(Vec<datafusion_common::ScalarValue>, usize)>
```

Source: `src/window_state.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A tuple containing group values and the row index where the group ends.
Example: [[1, 1], [1, 1], [2, 1], [2, 1], ...] would correspond to
         [([1, 1], 2), ([2, 1], 4), ...].
