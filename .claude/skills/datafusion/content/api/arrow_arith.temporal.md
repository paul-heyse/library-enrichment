# `arrow_arith::temporal`

Crate `arrow-arith` · 2 public items · structured records in [`model/arrow_arith.temporal.json`](../model/arrow_arith.temporal.json)

## DatePart

`enum` · `arrow_arith::temporal::DatePart`

Also reachable as `arrow::compute::DatePart`, `arrow::compute::kernels::temporal::DatePart`

```rust
enum DatePart
```

**Variants**: `Quarter`, `Year`, `YearISO`, `Month`, `Week`, `WeekISO`, `Day`, `DayOfWeekSunday0`, `DayOfWeekMonday0`, `DayOfWeekSunday1`, `DayOfWeekMonday1`, `DayOfYear`, `Hour`, `Minute`, `Second`, `Millisecond`, `Microsecond`, `Nanosecond`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, ArrowError>
```

Valid parts to extract from date/time/timestamp arrays.

See [`date_part`].

Marked as non-exhaustive as may expand to support more types of
date parts in the future.

---

## date_part

`function` · `arrow_arith::temporal::date_part`

Also reachable as `arrow::compute::date_part`, `arrow::compute::kernels::temporal::date_part`

```rust
fn date_part(array: &dyn Array, part: DatePart) -> Result<ArrayRef, arrow_schema::ArrowError>
```

Given an array, return a new array with the extracted [`DatePart`] as signed 32-bit
integer values.

Currently only supports temporal types:
  - Date32/Date64
  - Time32/Time64
  - Timestamp
  - Interval
  - Duration

Returns an [`Int32Array`] unless input was a dictionary type, in which case returns
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

---
