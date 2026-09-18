# `arrow_array::array::primitive_array::UInt16Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.UInt16Array.json).

<a id="op-f0a8cd37e931478950e7461b"></a>
## UInt16Array

`type_alias` · `arrow_array::array::primitive_array::UInt16Array` · arrow-array 59.3.0

```rust
type UInt16Array = PrimitiveArray<UInt16Type>
```

Source: `src/array/primitive_array.rs:149`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `u16`

# Examples

Construction

```
# use arrow_array::UInt16Array;
// Create from Vec<Option<u16>>
let arr = UInt16Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u16>
let arr = UInt16Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt16Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
