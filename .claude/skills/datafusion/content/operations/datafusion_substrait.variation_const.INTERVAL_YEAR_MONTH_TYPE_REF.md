# `datafusion_substrait::variation_const::INTERVAL_YEAR_MONTH_TYPE_REF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_substrait.variation_const.INTERVAL_YEAR_MONTH_TYPE_REF.json).

<a id="op-0bfd5ed35c6d6ae2cc777587"></a>
## INTERVAL_YEAR_MONTH_TYPE_REF

`constant` · `datafusion_substrait::variation_const::INTERVAL_YEAR_MONTH_TYPE_REF` · datafusion-substrait 55.1.0

```rust
const INTERVAL_YEAR_MONTH_TYPE_REF: u32 = 1
```

Source: `src/variation_const.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-substrait/55.1.0/json).

For [`DataType::Interval`] with [`IntervalUnit::YearMonth`].

An `i32` for elapsed whole months. See also [`ScalarValue::IntervalYearMonth`]
for the literal definition in DataFusion.

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::YearMonth`]: datafusion::arrow::datatypes::IntervalUnit::YearMonth
[`ScalarValue::IntervalYearMonth`]: datafusion::common::ScalarValue::IntervalYearMonth
