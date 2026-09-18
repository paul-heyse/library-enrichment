# `arrow_array::array::primitive_array::Int32Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Int32Array.json).

<a id="op-eabd534f75819dac6cf63c2c"></a>
## Int32Array

`type_alias` · `arrow_array::array::primitive_array::Int32Array` · arrow-array 59.3.0

```rust
type Int32Array = PrimitiveArray<Int32Type>
```

Source: `src/array/primitive_array.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `i32`

# Examples

Construction

```
# use arrow_array::Int32Array;
// Create from Vec<Option<i32>>
let arr = Int32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i32>
let arr = Int32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
