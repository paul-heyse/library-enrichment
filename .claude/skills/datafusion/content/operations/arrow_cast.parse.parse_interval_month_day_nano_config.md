# `arrow_cast::parse::parse_interval_month_day_nano_config`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.parse_interval_month_day_nano_config.json).

<a id="op-151df2ccbb26517b2d06216f"></a>
## parse_interval_month_day_nano_config

`function` · `arrow_cast::parse::parse_interval_month_day_nano_config` · arrow-cast 59.3.0

```rust
fn parse_interval_month_day_nano_config(value: &str, config: IntervalParseConfig) -> Result<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Source: `src/parse.rs:1075`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse human-readable interval string to Arrow [IntervalMonthDayNanoType](../operations/arrow_array.types.IntervalMonthDayNanoType.md#op-901b057209e19dc08e92a677)
