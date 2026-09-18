# `arrow_array::array::primitive_array::UInt32Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.UInt32Array.json).

<a id="op-c25dd16dcef9fe574dac428b"></a>
## UInt32Array

`type_alias` · `arrow_array::array::primitive_array::UInt32Array` · arrow-array 59.3.0

```rust
type UInt32Array = PrimitiveArray<UInt32Type>
```

Source: `src/array/primitive_array.rs:168`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `u32`

# Examples

Construction

```
# use arrow_array::UInt32Array;
// Create from Vec<Option<u32>>
let arr = UInt32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u32>
let arr = UInt32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
