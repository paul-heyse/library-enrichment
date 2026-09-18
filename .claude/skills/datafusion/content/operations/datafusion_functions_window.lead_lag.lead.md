# `datafusion_functions_window::lead_lag::lead`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.lead_lag.lead.json).

<a id="op-e63e45bbaf5bf1f124dca9f9"></a>
## lead

`function` · `datafusion_functions_window::lead_lag::lead` · datafusion-functions-window 55.1.0

```rust
fn lead(arg: datafusion_expr::Expr, shift_offset: Option<i64>, default_value: Option<datafusion_common::ScalarValue>) -> datafusion_expr::Expr
```

Source: `src/lead_lag.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create an expression to represent the `lead` window function

returns value evaluated at the row that is offset rows after the current row within the partition;
if there is no such row, instead return default (which must be of the same type as value).
Both offset and default are evaluated with respect to the current row.
If omitted, offset defaults to 1 and default to null
