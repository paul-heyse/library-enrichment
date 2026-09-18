# `arrow_array::array::primitive_array::Time64NanosecondArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Time64NanosecondArray.json).

<a id="op-866b7b316ebea58d529d9a19"></a>
## Time64NanosecondArray

`type_alias` · `arrow_array::array::primitive_array::Time64NanosecondArray` · arrow-array 59.3.0

```rust
type Time64NanosecondArray = PrimitiveArray<Time64NanosecondType>
```

Source: `src/array/primitive_array.rs:352`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of nanoseconds since midnight stored as `i64`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00.123456789`

Unresolved upstream links (retained, not inferred): ``chrono::NaiveTime``.
