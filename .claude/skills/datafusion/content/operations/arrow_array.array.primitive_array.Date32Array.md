# `arrow_array::array::primitive_array::Date32Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Date32Array.json).

<a id="op-66ba6255a42929f8261ccf06"></a>
## Date32Array

`type_alias` · `arrow_array::array::primitive_array::Date32Array` · arrow-array 59.3.0

```rust
type Date32Array = PrimitiveArray<Date32Type>
```

Source: `src/array/primitive_array.rs:322`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of days since UNIX epoch stored as `i32`

This type is similar to the [`chrono::NaiveDate`] type and can hold
values such as `2018-11-13`

Unresolved upstream links (retained, not inferred): ``chrono::NaiveDate``.
