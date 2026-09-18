# `datafusion_expr::utils::format_state_name`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.utils.format_state_name.json).

<a id="op-738b514bdebd66d9a4747d71"></a>
## format_state_name

`function` · `datafusion_expr::utils::format_state_name` · datafusion-expr 55.1.0

```rust
fn format_state_name(name: &str, state_name: &str) -> String
```

Source: `src/utils.rs:1418`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Build state name. State is the intermediate state of the aggregate function.
