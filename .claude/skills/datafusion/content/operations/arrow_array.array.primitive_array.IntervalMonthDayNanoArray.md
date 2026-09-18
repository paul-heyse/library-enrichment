# `arrow_array::array::primitive_array::IntervalMonthDayNanoArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.IntervalMonthDayNanoArray.json).

<a id="op-b33fe45608841268b5882a12"></a>
## IntervalMonthDayNanoArray

`type_alias` · `arrow_array::array::primitive_array::IntervalMonthDayNanoArray` · arrow-array 59.3.0

```rust
type IntervalMonthDayNanoArray = PrimitiveArray<IntervalMonthDayNanoType>
```

Source: `src/array/primitive_array.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of “calendar” intervals in  months, days, and nanoseconds.

See [`IntervalMonthDayNano`](../operations/arrow_buffer.interval.IntervalMonthDayNano.md#op-124a76e3b87e96892e283f4a) for details on representation and caveats.

# Example
```
# use arrow_array::IntervalMonthDayNanoArray;
use arrow_array::types::IntervalMonthDayNano;
let array = IntervalMonthDayNanoArray::from(vec![
  IntervalMonthDayNano::new(1, 2, 1000),             // 1 month, 2 days, 1 nanosecond
  IntervalMonthDayNano::new(12, 1, 0),               // 12 months, 1 days, 0 nanoseconds
  IntervalMonthDayNano::new(0, 0, 12 * 1000 * 1000), // 0 days, 12 milliseconds
]);
```
