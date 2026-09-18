# `datafusion_physical_expr::intervals::utils::convert_interval_type_to_duration`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.utils.convert_interval_type_to_duration.json).

<a id="op-1a93db28ad54b1c4592718bc"></a>
## convert_interval_type_to_duration

`function` · `datafusion_physical_expr::intervals::utils::convert_interval_type_to_duration` · datafusion-physical-expr 55.1.0

```rust
fn convert_interval_type_to_duration(interval: &datafusion_expr::interval_arithmetic::Interval) -> Option<datafusion_expr::interval_arithmetic::Interval>
```

Source: `src/intervals/utils.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Converts an [`Interval`](../operations/datafusion_expr_common.interval_arithmetic.Interval.md#op-0e8ed6898e67ba9321c8f53e) of time intervals to one of `Duration`s, if applicable. Otherwise, returns [`None`].

Unresolved upstream links (retained, not inferred): ``None``.
