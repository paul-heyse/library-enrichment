# `arrow_array::array::primitive_array::IntervalYearMonthArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.IntervalYearMonthArray.json).

<a id="op-b8bb28369e9e8d9778eee908"></a>
## IntervalYearMonthArray

`type_alias` · `arrow_array::array::primitive_array::IntervalYearMonthArray` · arrow-array 59.3.0

```rust
type IntervalYearMonthArray = PrimitiveArray<IntervalYearMonthType>
```

Source: `src/array/primitive_array.rs:367`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of “calendar” intervals in whole months

See [`IntervalYearMonthType`](../operations/arrow_array.types.IntervalYearMonthType.md#op-2f3f6cf2fc19abb5faac2f23) for details on representation and caveats.

# Example
```
# use arrow_array::IntervalYearMonthArray;
let array = IntervalYearMonthArray::from(vec![
  2,  // 2 months
  25, // 2 years and 1 month
  -1  // -1 months
]);
```
