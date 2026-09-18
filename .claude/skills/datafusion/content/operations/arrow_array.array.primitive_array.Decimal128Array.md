# `arrow_array::array::primitive_array::Decimal128Array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.array.primitive_array.Decimal128Array.json).

<a id="op-d955fd55f06cfa95d3cbe626"></a>
## Decimal128Array

`type_alias` · `arrow_array::array::primitive_array::Decimal128Array` · arrow-array 59.3.0

```rust
type Decimal128Array = PrimitiveArray<Decimal128Type>
```

Source: `src/array/primitive_array.rs:468`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) of 128-bit fixed point decimals

# Examples

Construction

```
# use arrow_array::Decimal128Array;
// Create from Vec<Option<i128>>
let arr = Decimal128Array::from(vec![Some(1), None, Some(2)]);
// Create from Vec<i128>
let arr = Decimal128Array::from(vec![1, 2, 3]);
// Create iter/collect
let arr: Decimal128Array = std::iter::repeat(42).take(10).collect();
```

See [`PrimitiveArray`](../operations/arrow_array.array.primitive_array.PrimitiveArray.md#op-bc88178d283a743ead5dd814) for more information and examples
