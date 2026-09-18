# `datafusion_functions_window::lead_lag::lag`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.lead_lag.lag.json).

<a id="op-40e308cff8ba80368ff89a79"></a>
## lag

`function` · `datafusion_functions_window::lead_lag::lag` · datafusion-functions-window 55.1.0

```rust
fn lag(arg: datafusion_expr::Expr, shift_offset: Option<i64>, default_value: Option<datafusion_common::ScalarValue>) -> datafusion_expr::Expr
```

Source: `src/lead_lag.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create an expression to represent the `lag` window function

returns value evaluated at the row that is offset rows before the current row within the partition;
if there is no such row, instead return default (which must be of the same type as value).
Both offset and default are evaluated with respect to the current row.
If omitted, offset defaults to 1 and default to null
