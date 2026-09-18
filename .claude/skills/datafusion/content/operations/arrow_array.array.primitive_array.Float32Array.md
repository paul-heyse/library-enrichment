# `arrow_array::array::primitive_array::Float32Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Float32Array.json).

<a id="op-36025714ca37c5fe52c6efee"></a>
## Float32Array

`type_alias` · `arrow_array::array::primitive_array::Float32Array` · arrow-array 59.3.0

```rust
type Float32Array = PrimitiveArray<Float32Type>
```

Source: `src/array/primitive_array.rs:233`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `f32`

# Examples

Construction

```
# use arrow_array::Float32Array;
// Create from Vec<Option<f32>>
let arr = Float32Array::from(vec![Some(1.0), None, Some(2.0)]);
// Create from Vec<f32>
let arr = Float32Array::from(vec![1.0, 2.0, 3.0]);
// Create iter/collect
let arr: Float32Array = std::iter::repeat(42.0).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
