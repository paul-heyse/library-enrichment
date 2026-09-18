# `arrow_arith::temporal::date_part`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.temporal.date_part.json).

<a id="op-1c788fff179b94ab22ae9330"></a>
## date_part

`function` · `arrow_arith::temporal::date_part` · arrow-arith 59.3.0

```rust
fn date_part(array: &dyn Array, part: DatePart) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Source: `src/temporal.rs:207`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Given an array, return a new array with the extracted [`DatePart`](../operations/arrow_arith.temporal.DatePart.md#op-f5e1db29c834d5950775ec3c) as signed 32-bit
integer values.

Currently only supports temporal types:
  - Date32/Date64
  - Time32/Time64
  - Timestamp
  - Interval
  - Duration

Returns an [`Int32Array`](../operations/arrow_array.array.primitive_array.Int32Array.md#op-eabd534f75819dac6cf63c2c) unless input was a dictionary type, in which case returns
the dictionary but with this function applied onto its values.

If array passed in is not of the above listed types (or is a dictionary array where the
values array isn't of the above listed types), then this function will return an error.

# Examples

```
# use arrow_array::{Int32Array, TimestampMicrosecondArray};
# use arrow_arith::temporal::{DatePart, date_part};
let input: TimestampMicrosecondArray =
    vec![Some(1612025847000000), None, Some(1722015847000000)].into();

let week = date_part(&input, DatePart::Week).unwrap();
let week_iso = date_part(&input, DatePart::WeekISO).unwrap();
let expected: Int32Array = vec![Some(4), None, Some(30)].into();
assert_eq!(week.as_ref(), &expected);
assert_eq!(week_iso.as_ref(), &expected);
let year_iso = date_part(&input, DatePart::YearISO).unwrap();
let expected: Int32Array = vec![Some(2021), None, Some(2024)].into();
assert_eq!(year_iso.as_ref(), &expected);
```
