# `arrow_cast::parse::parse_interval_year_month`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.parse_interval_year_month.json).

<a id="op-d20b65d5eed199fc51d393e7"></a>
## parse_interval_year_month

`function` · `arrow_cast::parse::parse_interval_year_month` · arrow-cast 59.3.0

```rust
fn parse_interval_year_month(value: &str) -> Result<<IntervalYearMonthType as ArrowPrimitiveType>::Native, arrow_schema::ArrowError>
```

Source: `src/parse.rs:1045`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Parse human-readable interval string to Arrow [IntervalYearMonthType](../operations/arrow_array.types.IntervalYearMonthType.md#op-2f3f6cf2fc19abb5faac2f23)
