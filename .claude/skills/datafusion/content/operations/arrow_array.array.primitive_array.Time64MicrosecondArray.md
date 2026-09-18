# `arrow_array::array::primitive_array::Time64MicrosecondArray`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Time64MicrosecondArray.json).

<a id="op-3232703d82081382a896cf54"></a>
## Time64MicrosecondArray

`type_alias` · `arrow_array::array::primitive_array::Time64MicrosecondArray` · arrow-array 59.3.0

```rust
type Time64MicrosecondArray = PrimitiveArray<Time64MicrosecondType>
```

Source: `src/array/primitive_array.rs:346`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of microseconds since midnight stored as `i64`

This type is similar to the [`chrono::NaiveTime`] type and can
hold values such as `00:02:00.123456`

Unresolved upstream links (retained, not inferred): ``chrono::NaiveTime``.
