# `datafusion_physical_expr::intervals::utils::convert_duration_type_to_interval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.utils.convert_duration_type_to_interval.json).

<a id="op-0a53e034b0eea4c8265365e6"></a>
## convert_duration_type_to_interval

`function` · `datafusion_physical_expr::intervals::utils::convert_duration_type_to_interval` · datafusion-physical-expr 55.1.0

```rust
fn convert_duration_type_to_interval(interval: &datafusion_expr::interval_arithmetic::Interval) -> Option<datafusion_expr::interval_arithmetic::Interval>
```

Source: `src/intervals/utils.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Converts an [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) of `Duration`s to one of time intervals, if applicable. Otherwise, returns [`None`].

Unresolved upstream links (retained, not inferred): ``None``.
