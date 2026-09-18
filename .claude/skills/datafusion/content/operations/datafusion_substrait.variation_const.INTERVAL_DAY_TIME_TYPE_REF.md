# `datafusion_substrait::variation_const::INTERVAL_DAY_TIME_TYPE_REF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.variation_const.INTERVAL_DAY_TIME_TYPE_REF.json).

<a id="op-9ceb2154660482f01025b201"></a>
## INTERVAL_DAY_TIME_TYPE_REF

`constant` · `datafusion_substrait::variation_const::INTERVAL_DAY_TIME_TYPE_REF` · datafusion-substrait 55.1.0

```rust
const INTERVAL_DAY_TIME_TYPE_REF: u32 = 2
```

Source: `src/variation_const.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

For [`DataType::Interval`] with [`IntervalUnit::DayTime`].

An `i64` as:
- days: `i32`
- milliseconds: `i32`

See also [`ScalarValue::IntervalDayTime`] for the literal definition in DataFusion.

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::DayTime`]: datafusion::arrow::datatypes::IntervalUnit::DayTime
[`ScalarValue::IntervalDayTime`]: datafusion::common::ScalarValue::IntervalDayTime
