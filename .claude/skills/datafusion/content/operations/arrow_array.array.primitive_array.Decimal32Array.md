# `arrow_array::array::primitive_array::Decimal32Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Decimal32Array.json).

<a id="op-7bc381e3319018860956af21"></a>
## Decimal32Array

`type_alias` · `arrow_array::array::primitive_array::Decimal32Array` · arrow-array 59.3.0

```rust
type Decimal32Array = PrimitiveArray<Decimal32Type>
```

Source: `src/array/primitive_array.rs:430`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of 32-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal32Array;
// Create from Vec<Option<i32>>
let arr = Decimal32Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i32>
let arr = Decimal32Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal32Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
