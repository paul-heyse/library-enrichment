# `arrow_array::array::primitive_array::Int64Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Int64Array.json).

<a id="op-3d35f6fcfcf780fcb4b24bb3"></a>
## Int64Array

`type_alias` · `arrow_array::array::primitive_array::Int64Array` · arrow-array 59.3.0

```rust
type Int64Array = PrimitiveArray<Int64Type>
```

Source: `src/array/primitive_array.rs:111`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `i64`

# Examples

Construction

```
# use arrow_array::Int64Array;
// Create from Vec<Option<i64>>
let arr = Int64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i64>
let arr = Int64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Int64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
