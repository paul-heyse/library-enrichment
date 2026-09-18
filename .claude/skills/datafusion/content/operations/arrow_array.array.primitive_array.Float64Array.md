# `arrow_array::array::primitive_array::Float64Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Float64Array.json).

<a id="op-cf66296f786b28717e0e42a7"></a>
## Float64Array

`type_alias` · `arrow_array::array::primitive_array::Float64Array` · arrow-array 59.3.0

```rust
type Float64Array = PrimitiveArray<Float64Type>
```

Source: `src/array/primitive_array.rs:252`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `f64`

# Examples

Construction

```
# use arrow_array::Float64Array;
// Create from Vec<Option<f32>>
let arr = Float64Array::from(vec![Some(1.0), None, Some(2.0)]);
// Create from Vec<f32>
let arr = Float64Array::from(vec![1.0, 2.0, 3.0]);
// Create iter/collect
let arr: Float64Array = std::iter::repeat(42.0).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
