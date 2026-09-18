# `arrow_array::array::primitive_array::Date64Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Date64Array.json).

<a id="op-9fdf1b83ba7913db8f12371d"></a>
## Date64Array

`type_alias` · `arrow_array::array::primitive_array::Date64Array` · arrow-array 59.3.0

```rust
type Date64Array = PrimitiveArray<Date64Type>
```

Source: `src/array/primitive_array.rs:328`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of milliseconds since UNIX epoch stored as `i64`

This type is similar to the [`chrono::NaiveDate`] type and can hold
values such as `2018-11-13`

Unresolved upstream links (retained, not inferred): ``chrono::NaiveDate``.
