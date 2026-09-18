# `arrow_array::array::primitive_array::Int8Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Int8Array.json).

<a id="op-a7c0e53f18ba95a4706516e4"></a>
## Int8Array

`type_alias` · `arrow_array::array::primitive_array::Int8Array` · arrow-array 59.3.0

```rust
type Int8Array = PrimitiveArray<Int8Type>
```

Source: `src/array/primitive_array.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `i8`

# Examples

Construction

```
# use arrow_array::Int8Array;
// Create from Vec<Option<i8>>
let arr = Int8Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i8>
let arr = Int8Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int8Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
