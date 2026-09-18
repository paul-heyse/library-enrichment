# `arrow_array::array::primitive_array::Time32MillisecondArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Time32MillisecondArray.json).

<a id="op-bb5011eb11a45337ed1243b2"></a>
## Time32MillisecondArray

`type_alias` · `arrow_array::array::primitive_array::Time32MillisecondArray` · arrow-array 59.3.0

```rust
type Time32MillisecondArray = PrimitiveArray<Time32MillisecondType>
```

Source: `src/array/primitive_array.rs:340`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of milliseconds since midnight stored as `i32`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00.123`

Unresolved upstream links (retained, not inferred): ``chrono::NaiveTime``.
