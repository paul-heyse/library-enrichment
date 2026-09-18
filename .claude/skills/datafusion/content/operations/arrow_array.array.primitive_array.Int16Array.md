# `arrow_array::array::primitive_array::Int16Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Int16Array.json).

<a id="op-108df9d5f08803e85fa8c02b"></a>
## Int16Array

`type_alias` · `arrow_array::array::primitive_array::Int16Array` · arrow-array 59.3.0

```rust
type Int16Array = PrimitiveArray<Int16Type>
```

Source: `src/array/primitive_array.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `i16`

# Examples

Construction

```
# use arrow_array::Int16Array;
// Create from Vec<Option<i16>>
let arr = Int16Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i16>
let arr = Int16Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int16Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
