# `datafusion::physical_planner::is_window_frame_bound_valid`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.physical_planner.is_window_frame_bound_valid.json).

<a id="op-e8367e137157407b703a2cbd"></a>
## is_window_frame_bound_valid

`function` · `datafusion::physical_planner::is_window_frame_bound_valid` · datafusion 55.1.0

```rust
fn is_window_frame_bound_valid(window_frame: &datafusion_expr::WindowFrame) -> bool
```

Source: `src/physical_planner.rs:2411`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Check if window bounds are valid after schema information is available, and
window_frame bounds are casted to the corresponding column type.
queries like:
OVER (ORDER BY a RANGES BETWEEN 3 PRECEDING AND 5 PRECEDING)
OVER (ORDER BY a RANGES BETWEEN INTERVAL '3 DAY' PRECEDING AND '5 DAY' PRECEDING)  are rejected
