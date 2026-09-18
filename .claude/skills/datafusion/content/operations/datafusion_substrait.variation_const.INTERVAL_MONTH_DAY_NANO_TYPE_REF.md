# `datafusion_substrait::variation_const::INTERVAL_MONTH_DAY_NANO_TYPE_REF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.variation_const.INTERVAL_MONTH_DAY_NANO_TYPE_REF.json).

<a id="op-b01ce6ff3a97b60e6c51b47b"></a>
## INTERVAL_MONTH_DAY_NANO_TYPE_REF

`constant` · `datafusion_substrait::variation_const::INTERVAL_MONTH_DAY_NANO_TYPE_REF` · datafusion-substrait 55.1.0

```rust
const INTERVAL_MONTH_DAY_NANO_TYPE_REF: u32 = 3
```

Source: `src/variation_const.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

For [`DataType::Interval`] with [`IntervalUnit::MonthDayNano`].

An `i128` as:
- months: `i32`
- days: `i32`
- nanoseconds: `i64`

See also [`ScalarValue::IntervalMonthDayNano`] for the literal definition in DataFusion.

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::MonthDayNano`]: datafusion::arrow::datatypes::IntervalUnit::MonthDayNano
[`ScalarValue::IntervalMonthDayNano`]: datafusion::common::ScalarValue::IntervalMonthDayNano
