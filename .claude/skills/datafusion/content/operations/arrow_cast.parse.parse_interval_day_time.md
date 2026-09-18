# `arrow_cast::parse::parse_interval_day_time`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.parse_interval_day_time.json).

<a id="op-1dd95dcd0c025de82c7780cb"></a>
## parse_interval_day_time

`function` · `arrow_cast::parse::parse_interval_day_time` · arrow-cast 59.3.0

```rust
fn parse_interval_day_time(value: &str) -> Result<<IntervalDayTimeType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Source: `src/parse.rs:1061`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse human-readable interval string to Arrow [IntervalDayTimeType](../operations/arrow_array.types.IntervalDayTimeType.md#op-dc4b5147d6540fd8ad5dff60)
