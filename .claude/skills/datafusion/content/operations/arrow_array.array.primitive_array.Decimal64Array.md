# `arrow_array::array::primitive_array::Decimal64Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Decimal64Array.json).

<a id="op-2c30b51253a4b7f384ef5e60"></a>
## Decimal64Array

`type_alias` · `arrow_array::array::primitive_array::Decimal64Array` · arrow-array 59.3.0

```rust
type Decimal64Array = PrimitiveArray<Decimal64Type>
```

Source: `src/array/primitive_array.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of 64-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal64Array;
// Create from Vec<Option<i64>>
let arr = Decimal64Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i64>
let arr = Decimal64Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal64Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
