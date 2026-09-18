# `datafusion_expr::window_frame`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.window_frame.json).

<a id="op-0d624d9438efdb971f7ebeed"></a>
## window_frame

`module` · `datafusion_expr::window_frame` · datafusion-expr 55.1.0

```rust
mod window_frame
```

Source: `src/window_frame.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Window frame module

The frame-spec determines which output rows are read by an aggregate window function. The frame-spec consists of four parts:
- A frame type - either ROWS, RANGE or GROUPS,
- A starting frame boundary,
- An ending frame boundary,
- An EXCLUDE clause.
