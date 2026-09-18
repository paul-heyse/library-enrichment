# `arrow_array::array::primitive_array::UInt64Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.UInt64Array.json).

<a id="op-47e7828263162b82ce927cad"></a>
## UInt64Array

`type_alias` · `arrow_array::array::primitive_array::UInt64Array` · arrow-array 59.3.0

```rust
type UInt64Array = PrimitiveArray<UInt64Type>
```

Source: `src/array/primitive_array.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `u64`

# Examples

Construction

```
# use arrow_array::UInt64Array;
// Create from Vec<Option<u64>>
let arr = UInt64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u64>
let arr = UInt64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
