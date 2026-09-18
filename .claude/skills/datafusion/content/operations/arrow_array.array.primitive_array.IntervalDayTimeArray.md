# `arrow_array::array::primitive_array::IntervalDayTimeArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.IntervalDayTimeArray.json).

<a id="op-e05e602ca6d5319ef24fd7c1"></a>
## IntervalDayTimeArray

`type_alias` · `arrow_array::array::primitive_array::IntervalDayTimeArray` · arrow-array 59.3.0

```rust
type IntervalDayTimeArray = PrimitiveArray<IntervalDayTimeType>
```

Source: `src/array/primitive_array.rs:383`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of “calendar” intervals in days and milliseconds

See [`IntervalDayTime`](../operations/arrow_buffer.interval.IntervalDayTime.md#op-e9ed0fecb92cb97c67e9f81d) for details on representation and caveats.

# Example
```
# use arrow_array::IntervalDayTimeArray;
use arrow_array::types::IntervalDayTime;
let array = IntervalDayTimeArray::from(vec![
  IntervalDayTime::new(1, 1000),                 // 1 day, 1000 milliseconds
  IntervalDayTime::new(33, 0),                  // 33 days, 0 milliseconds
  IntervalDayTime::new(0, 12 * 60 * 60 * 1000), // 0 days, 12 hours
]);
```
