# `arrow_array::array::primitive_array::Time32SecondArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Time32SecondArray.json).

<a id="op-740798d2c14117fd402a5835"></a>
## Time32SecondArray

`type_alias` · `arrow_array::array::primitive_array::Time32SecondArray` · arrow-array 59.3.0

```rust
type Time32SecondArray = PrimitiveArray<Time32SecondType>
```

Source: `src/array/primitive_array.rs:334`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of seconds since midnight stored as `i32`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00`

Unresolved upstream links (retained, not inferred): ``chrono::NaiveTime``.
