# `arrow_cast::parse::parse_interval_month_day_nano`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.parse_interval_month_day_nano.json).

<a id="op-bee5501c140f5901d4a9815a"></a>
## parse_interval_month_day_nano

`function` · `arrow_cast::parse::parse_interval_month_day_nano` · arrow-cast 59.3.0

```rust
fn parse_interval_month_day_nano(value: &str) -> Result<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Source: `src/parse.rs:1087`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse human-readable interval string to Arrow [IntervalMonthDayNanoType](../operations/arrow_array.types.IntervalMonthDayNanoType.md#op-901b057209e19dc08e92a677)
