# `arrow_array::array::primitive_array::UInt8Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.UInt8Array.json).

<a id="op-994bdf27bb2cddcd261060b6"></a>
## UInt8Array

`type_alias` · `arrow_array::array::primitive_array::UInt8Array` · arrow-array 59.3.0

```rust
type UInt8Array = PrimitiveArray<UInt8Type>
```

Source: `src/array/primitive_array.rs:130`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of `u8`

# Examples

Construction

```
# use arrow_array::UInt8Array;
// Create from Vec<Option<u8>>
let arr = UInt8Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<u8>
let arr = UInt8Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: UInt8Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
